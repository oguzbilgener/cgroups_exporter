use std::collections::HashMap;
use std::path::Path;

use anyhow::Context as _;
use cgroups_rs::CgroupPid;
use cgroups_rs::fs::{
    Cgroup,
    blkio::{BlkIo, BlkIoController},
    cpu::CpuController,
    cpuacct::{CpuAcct, CpuAcctController},
    cpuset::CpuSet,
    memory::{MemController, MemSwap, Memory},
};
use new_string_template::template::Template;
use procfs::process::Process;
use serde::Serialize;

use crate::{
    matcher::{CgroupMatcher, NameMatcher},
    procs::{Proc, ProcessMetrics},
    render::Named,
    shell::Evaluator,
};

use cgroups_exporter_config::{RewriteCgroupName, Templated};

/// The file every cgroup lists its processes in, on both hierarchy versions.
const PROCS_FILE: &str = "cgroup.procs";

#[derive(Serialize, Default)]
pub struct CgroupMetrics {
    #[serde(skip)]
    pub name: String,

    pub cpu: Option<CpuStat>,
    pub cpuacct: Option<CpuAcct>,
    pub cpuset: Option<CpuSet>,
    pub memory: Option<Memory>,
    pub memswap: Option<MemSwap>,
    pub blkio: Option<BlkIo>,

    // It would be easier to reuse the `ProcessMetrics struct and use `#[serde(flatten)]`,
    // but the serde uses a the map serializer and serde_prom doesn't support maps yet.
    pub rss: u64,
    pub utime: f64,
    pub stime: f64,
    pub cpu_seconds_total: f64,
    pub memory_usage_bytes: u64,
    pub num_fds: u64,
    pub num_procs: u64,
    pub num_threads: u64,
    pub io_read_bytes_total: u64,
    pub io_write_bytes_total: u64,
    pub major_page_faults_total: u64,
    pub minor_page_faults_total: u64,
    pub start_time: Option<i64>,
}

impl CgroupMetrics {
    pub fn from_cgroup_blocking<E>(
        cgroup: &Cgroup,
        matcher: &CgroupMatcher,
        evaluator: &E,
    ) -> anyhow::Result<Self>
    where
        E: Evaluator,
    {
        let mut metrics = CgroupMetrics {
            name: Self::rewrite_name(cgroup, matcher, evaluator)?,
            ..Default::default()
        };

        if let Some(ctrl) = cgroup.controller_of::<MemController>() {
            metrics.memory = Some(ctrl.memory_stat());
            metrics.memswap = Some(ctrl.memswap());
        }

        if cgroup.v2()
            && let Some(ctrl) = cgroup.controller_of::<CpuController>()
        {
            metrics.cpu = Some(parse_v2_stat(&ctrl.cpu().stat));
        }

        if let Some(ctrl) = cgroup.controller_of::<CpuAcctController>() {
            metrics.cpuacct = Some(ctrl.cpuacct());
        }

        if let Some(ctrl) = cgroup.controller_of::<BlkIoController>() {
            metrics.blkio = Some(ctrl.blkio());
        }

        let procs = if matcher.recursive {
            subtree_procs(cgroup)
        } else {
            cgroup.procs()
        };
        let processes_iter = procs.into_iter().filter_map(|pid| {
            let pid = saturating_cast::SaturatingCast::saturating_cast(pid.pid);
            Process::new(pid).ok()
        });

        metrics.set_proc_metrics(processes_iter);

        Ok(metrics)
    }

    #[allow(clippy::similar_names)]
    fn set_proc_metrics(&mut self, procs: impl Iterator<Item = Process>) {
        let procs_iter = procs.filter_map(|proc| {
            let mut proc: Proc = proc.try_into().ok()?;
            proc.gather_remaining_info().ok()?;
            Some(proc)
        });
        let metrics = ProcessMetrics::from_processes(procs_iter, "");

        self.rss = metrics.rss;
        self.memory_usage_bytes = metrics.memory_usage_bytes;
        self.utime = metrics.utime;
        self.stime = metrics.stime;
        self.cpu_seconds_total = metrics.cpu_seconds_total;
        self.num_fds = metrics.num_fds;
        self.num_procs = metrics.num_procs;
        self.num_threads = metrics.num_threads;
        self.io_read_bytes_total = metrics.io_read_bytes_total;
        self.io_write_bytes_total = metrics.io_write_bytes_total;
        self.major_page_faults_total = metrics.major_page_faults_total;
        self.minor_page_faults_total = metrics.minor_page_faults_total;
        self.start_time = metrics.start_time;
    }

    fn rewrite_name<E>(
        cgroup: &Cgroup,
        matcher: &CgroupMatcher,
        evaluator: &E,
    ) -> anyhow::Result<String>
    where
        E: Evaluator,
    {
        match &matcher.rewrite {
            None => Ok(cgroup.path().into()),
            Some(RewriteCgroupName::RemovePrefix { remove_prefix }) => Ok(cgroup
                .path()
                .strip_prefix(remove_prefix)
                .unwrap_or(cgroup.path())
                .into()),
            Some(RewriteCgroupName::Template { name }) => match &matcher.path {
                NameMatcher::Glob(_) => match name {
                    Templated::Name(name_rewrite) => {
                        // We have no variables to use, so assume a simple name rewrite.
                        Ok(name_rewrite.clone())
                    }
                    Templated::Shell { .. } => {
                        unreachable!(
                            "Shell templates are not supported for glob matchers. Validation should have caught this."
                        )
                    }
                },
                NameMatcher::Regex(regex) => {
                    if let Some(captures) = regex.captures(cgroup.path()) {
                        let mut variables = HashMap::new();
                        for name in regex.capture_names() {
                            let Some(name) = name else { continue };
                            if let Some(matched) = captures.name(name) {
                                variables.insert(name, matched.as_str().to_string());
                            }
                        }
                        match name {
                            Templated::Name(template) => {
                                let template = Template::new(template);
                                Ok(template.render_nofail(&variables))
                            }
                            Templated::Shell { shell, output } => evaluator
                                .evaluate_blocking(shell, variables, *output)
                                .context("Failed to evaluate shell command template"),
                        }
                    } else {
                        // No captures, so just return the original name.
                        match name {
                            Templated::Name(name_rewrite) => Ok(name_rewrite.clone()),
                            Templated::Shell { shell, .. } => Ok(shell.clone()),
                        }
                    }
                }
            },
        }
    }
}

impl Named for CgroupMetrics {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Default)]
pub struct CpuStat {
    pub usage_usec: Option<u64>,
    pub user_usec: Option<u64>,
    pub system_usec: Option<u64>,
    pub nice_usec: Option<u64>,
    pub nr_periods: Option<u64>,
    pub nr_throttled: Option<u64>,
    pub throttled_usec: Option<u64>,
    pub nr_bursts: Option<u64>,
    pub burst_usec: Option<u64>,
}

/// Every pid in the cgroup and in the cgroups below it.
///
/// Each v1 controller can have its own mount, while all v2 controllers share one. Walk every
/// distinct controller path so recursive matching works with both hierarchy versions.
fn subtree_procs(cgroup: &Cgroup) -> Vec<CgroupPid> {
    let mut pids = cgroup.procs();
    let mut controller_paths = cgroup
        .subsystems()
        .iter()
        .map(|subsystem| subsystem.to_controller().path())
        .collect::<Vec<_>>();
    controller_paths.sort_unstable();
    controller_paths.dedup();

    for path in controller_paths {
        pids.extend(descendant_procs(path));
    }
    pids.sort();
    pids.dedup();
    pids
}

/// The pids held by the cgroups below `dir`, and not by `dir` itself.
///
/// Walked with a stack rather than by recursion, which would hold a directory handle open per
/// level. A cgroup nobody can read is skipped rather than failing the whole collection.
fn descendant_procs(dir: &Path) -> Vec<CgroupPid> {
    let mut pids = Vec::new();
    let mut unvisited = vec![dir.to_path_buf()];
    while let Some(dir) = unvisited.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                continue;
            }
            pids.extend(read_procs(&entry.path()));
            unvisited.push(entry.path());
        }
    }
    pids
}

/// The pids one cgroup directory holds. Absent or unreadable counts as none.
fn read_procs(dir: &Path) -> Vec<CgroupPid> {
    let Ok(content) = std::fs::read_to_string(dir.join(PROCS_FILE)) else {
        return Vec::new();
    };
    content
        .split_whitespace()
        .filter_map(|pid| pid.parse::<u64>().ok())
        .map(CgroupPid::from)
        .collect()
}

fn parse_v2_stat(stat: &str) -> CpuStat {
    let mut v2_stat = CpuStat::default();
    for line in stat.lines() {
        let mut parts = line.split_whitespace();
        if let Some(key) = parts.next()
            && let Some(value) = parts.next()
        {
            match key {
                "usage_usec" => v2_stat.usage_usec = value.parse().ok(),
                "user_usec" => v2_stat.user_usec = value.parse().ok(),
                "system_usec" => v2_stat.system_usec = value.parse().ok(),
                "nice_usec" => v2_stat.nice_usec = value.parse().ok(),
                "nr_periods" => v2_stat.nr_periods = value.parse().ok(),
                "nr_throttled" => v2_stat.nr_throttled = value.parse().ok(),
                "throttled_usec" => v2_stat.throttled_usec = value.parse().ok(),
                "nr_bursts" => v2_stat.nr_bursts = value.parse().ok(),
                "burst_usec" => v2_stat.burst_usec = value.parse().ok(),
                _ => {}
            }
        }
    }
    v2_stat
}

#[cfg(test)]
mod tests {
    use cgroups_explorer::Explorer;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    use super::{descendant_procs, subtree_procs};
    use cgroups_rs::{
        CgroupPid,
        fs::{Cgroup, Hierarchy, Subsystem, memory::MemController},
    };

    use cgroups_exporter_config::RewriteCgroupName;
    use cgroups_rs::fs::memory::MemoryStat;
    use serde::Serialize;

    use crate::{
        cgroups::metrics::CgroupMetrics,
        matcher::{CgroupMatcher, NameMatcher},
        shell::MockEvaluator,
    };

    const V2_SWAP_CURRENT: u64 = 3_784_704;

    #[test]
    fn serialize_cgroup_metrics() -> anyhow::Result<()> {
        let global_labels: HashMap<&str, &str> = [("deviceId", "1234"), ("customerId", "abc")]
            .iter()
            .copied()
            .collect();
        let filter = "user.slice/user-1000.slice/*";
        let matcher = CgroupMatcher {
            path: NameMatcher::Glob(glob::Pattern::new(filter)?),
            recursive: false,
            rewrite: Some(RewriteCgroupName::RemovePrefix {
                remove_prefix: "user.slice/user-1000.slice/".into(),
            }),
        };
        let evaluator = MockEvaluator::new();

        let explorer = Explorer::detect_version()
            .include(vec![filter.into()])
            .build()?;
        for cgroup in explorer.iter_cgroups() {
            let metrics = CgroupMetrics::from_cgroup_blocking(&cgroup, &matcher, &evaluator)?;
            let mut labels = global_labels.clone();
            labels.insert("cgroup", &metrics.name);

            let metadata = HashMap::new();

            let serialized =
                serde_prom::to_prometheus_text(&metrics, Some("my_service"), &metadata, labels);
            println!("cgroup at path {}", cgroup.path());
            println!("{}", serialized?);

            println!("\n\n");
        }

        Ok(())
    }

    #[test]
    fn serialize_cgroup_v2_swap_usage() -> anyhow::Result<()> {
        let metrics = SerializableCgroupMemory {
            memory: SerializableMemory {
                stat: MemoryStat {
                    swap: V2_SWAP_CURRENT,
                    ..MemoryStat::default()
                },
            },
        };
        let labels = HashMap::from([("cgroup", "fixture.scope")]);
        let metadata = HashMap::new();

        let serialized =
            serde_prom::to_prometheus_text(&metrics, Some("cgroup"), &metadata, labels)?;
        let swap_samples = serialized
            .lines()
            .filter(|line| line.starts_with("cgroup_memory_stat_swap{"))
            .collect::<Vec<_>>();

        assert_eq!(
            swap_samples,
            vec!["cgroup_memory_stat_swap{cgroup=\"fixture.scope\"} 3784704"]
        );
        Ok(())
    }

    #[test]
    fn a_recursive_match_includes_direct_and_descendant_procs_on_v1_and_v2() {
        let root = tempfile::tempdir().unwrap();
        let component = root.path().join("component");
        fake_cgroup(&component, "10\n");
        fake_cgroup(&component.join("child"), "10\n11\n");
        fake_cgroup(&component.join("child").join("grandchild"), "12\n");

        for v2 in [false, true] {
            let cgroup = Cgroup::load(
                Box::new(FakeHierarchy {
                    root: root.path().to_path_buf(),
                    v2,
                }),
                "component",
            );

            assert_eq!(sorted_pids(subtree_procs(&cgroup)), vec![10, 11, 12]);
        }
    }

    #[test]
    fn a_cgroup_directory_that_cannot_be_read_is_skipped() {
        let root = tempfile::tempdir().unwrap();
        let present = root.path().join("present");
        fake_cgroup(&present, "21\n");
        // No cgroup.procs at all, which is what a directory that is not a cgroup looks like.
        std::fs::create_dir(root.path().join("bare")).unwrap();

        assert_eq!(
            sorted_pids(descendant_procs(root.path())),
            vec![21],
            "a directory with no cgroup.procs must not fail the whole collection"
        );
    }

    /// A cgroup directory holding the given `cgroup.procs`.
    fn fake_cgroup(path: &Path, procs: &str) {
        std::fs::create_dir_all(path).unwrap();
        std::fs::write(path.join("cgroup.procs"), procs).unwrap();
    }

    fn sorted_pids(pids: Vec<CgroupPid>) -> Vec<u64> {
        let mut pids: Vec<u64> = pids.into_iter().map(|pid| pid.pid).collect();
        pids.sort_unstable();
        pids
    }

    #[derive(Clone, Debug)]
    struct FakeHierarchy {
        root: PathBuf,
        v2: bool,
    }

    impl Hierarchy for FakeHierarchy {
        fn subsystems(&self) -> Vec<Subsystem> {
            vec![Subsystem::Mem(MemController::new(
                self.root.clone(),
                PathBuf::new(),
                self.v2,
            ))]
        }

        fn root(&self) -> PathBuf {
            self.root.clone()
        }

        fn root_control_group(&self) -> Cgroup {
            Cgroup::load(Box::new(self.clone()), "")
        }

        fn parent_control_group(&self, path: &str) -> Cgroup {
            let parent = Path::new(path).parent().unwrap_or_else(|| Path::new(""));
            Cgroup::load(Box::new(self.clone()), parent)
        }

        fn v2(&self) -> bool {
            self.v2
        }
    }

    #[derive(Serialize)]
    struct SerializableCgroupMemory {
        memory: SerializableMemory,
    }

    #[derive(Serialize)]
    struct SerializableMemory {
        stat: MemoryStat,
    }
}
