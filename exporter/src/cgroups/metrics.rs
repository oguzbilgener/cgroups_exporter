use std::collections::HashMap;

use anyhow::Context as _;
use cgroups_rs::{
    Cgroup,
    blkio::{BlkIo, BlkIoController},
    cpu::CpuController,
    cpuacct::{CpuAcct, CpuAcctController},
    cpuset::CpuSet,
    memory::{MemController, Memory},
};
use new_string_template::template::Template;
use procfs::process::Process;
use saturating_cast::SaturatingCast as _;
use serde::Serialize;

use crate::{
    matcher::{CgroupMatcher, NameMatcher},
    procs::{Proc, ProcessMetrics},
    render::Named,
    shell::Evaluator,
};

use cgroups_exporter_config::{RewriteCgroupName, Templated};

#[derive(Serialize, Default)]
pub struct CgroupMetrics {
    #[serde(skip)]
    pub name: String,

    pub cpu: Option<CpuStat>,
    pub cpuacct: Option<CpuAcct>,
    pub cpuset: Option<CpuSet>,
    pub memory: Option<Memory>,
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
        }

        if cgroup.v2() {
            if let Some(ctrl) = cgroup.controller_of::<CpuController>() {
                metrics.cpu = Some(parse_v2_stat(&ctrl.cpu().stat));
            }
        }

        if let Some(ctrl) = cgroup.controller_of::<CpuAcctController>() {
            metrics.cpuacct = Some(ctrl.cpuacct());
        }

        if let Some(ctrl) = cgroup.controller_of::<BlkIoController>() {
            metrics.blkio = Some(ctrl.blkio());
        }

        let processes_iter = cgroup.procs().into_iter().filter_map(|pid| {
            let pid = pid.pid.saturating_cast();
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

fn parse_v2_stat(stat: &str) -> CpuStat {
    let mut v2_stat = CpuStat::default();
    for line in stat.lines() {
        let mut parts = line.split_whitespace();
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
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
    }
    v2_stat
}

#[cfg(test)]
mod tests {
    use cgroups_explorer::Explorer;
    use cgroups_exporter_config::RewriteCgroupName;
    use std::collections::HashMap;

    use crate::shell::MockEvaluator;

    use super::*;

    #[test]
    fn serialize_cgroup_metrics() -> anyhow::Result<()> {
        let global_labels: HashMap<&str, &str> = [("deviceId", "1234"), ("customerId", "abc")]
            .iter()
            .copied()
            .collect();
        let filter = "user.slice/user-1000.slice/*";
        let matcher = CgroupMatcher {
            path: NameMatcher::Glob(glob::Pattern::new(filter)?),
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
}
