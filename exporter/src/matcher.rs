use std::fmt;

use cgroups_exporter_config::{
    CgroupConfig, CgroupMatch, Config, MetricsConfig, NameMatch, ProcessConfig, ProcessMatch,
    RewriteCgroupName, ShellCommandsConfig, Templated,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct MatchableConfig {
    pub cgroups: Vec<MatchableCgroupConfig>,
    pub processes: Vec<MatchableProcessConfig>,
    pub shell_commands: ShellCommandsConfig,
}

/// A mirror of `CgroupConfig` but with parsed Regex and no serialization.
#[derive(Debug, Clone)]
pub struct MatchableCgroupConfig {
    pub match_by: CgroupMatcher,
    pub metrics: MetricsConfig,
}

/// A mirror of `ProcessConfig` but with parsed Regex and no serialization.
#[derive(Debug, Clone)]
pub struct MatchableProcessConfig {
    pub match_by: ProcessMatcher,
    pub metrics: MetricsConfig,
}

/// A mirror of `CgroupMatcher` but with parsed Regex and no serialization.
#[derive(Debug, Clone)]
pub struct CgroupMatcher {
    pub path: NameMatcher,
    pub recursive: bool,
    pub rewrite: Option<RewriteCgroupName>,
}

/// A mirror of `ProcessMatch` but with parsed Regex and no serialization.
#[derive(Debug, Clone)]
pub enum ProcessMatcher {
    Exe { exe: NameMatcher, name: String },
    ExeBase { exe_base: NameMatcher, name: String },
    Comm { comm: NameMatcher, name: String },
    Cmdline { cmdline: NameMatcher, name: String },
}

/// A mirror of `NameMatch` but with parsed Regex and no serialization.
#[derive(Debug, Clone)]
pub enum NameMatcher {
    Glob(glob::Pattern),
    Regex(Regex),
}

impl TryFrom<NameMatch> for NameMatcher {
    type Error = anyhow::Error;

    fn try_from(value: NameMatch) -> Result<Self, Self::Error> {
        match value {
            NameMatch::Glob(glob) => Ok(Self::Glob(glob::Pattern::new(&glob)?)),
            NameMatch::Regex { regex } => Ok(Self::Regex(Regex::new(&regex)?)),
        }
    }
}

impl TryFrom<CgroupMatch> for CgroupMatcher {
    type Error = anyhow::Error;

    fn try_from(value: CgroupMatch) -> Result<Self, Self::Error> {
        let me = Self {
            path: value.path.try_into()?,
            recursive: value.recursive,
            rewrite: value.rewrite,
        };
        if matches!(&me.path, NameMatcher::Glob(_))
            && matches!(
                &me.rewrite,
                Some(RewriteCgroupName::Template {
                    name: Templated::Shell { .. }
                })
            )
        {
            anyhow::bail!(
                "Cgroup matcher with glob path cannot use shell command rewrite for the name"
            );
        }
        Ok(me)
    }
}

impl TryFrom<ProcessMatch> for ProcessMatcher {
    type Error = anyhow::Error;

    fn try_from(value: ProcessMatch) -> Result<Self, Self::Error> {
        match value {
            ProcessMatch::Exe { name, exe } => Ok(Self::Exe {
                exe: exe.try_into()?,
                name,
            }),
            ProcessMatch::ExeBase { name, exe_base } => Ok(Self::ExeBase {
                exe_base: exe_base.try_into()?,
                name,
            }),
            ProcessMatch::Comm { name, comm } => Ok(Self::Comm {
                comm: comm.try_into()?,
                name,
            }),
            ProcessMatch::Cmdline { name, cmdline } => Ok(Self::Cmdline {
                cmdline: cmdline.try_into()?,
                name,
            }),
        }
    }
}

impl TryFrom<CgroupConfig> for MatchableCgroupConfig {
    type Error = anyhow::Error;

    fn try_from(value: CgroupConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            match_by: value.match_by.try_into()?,
            metrics: value.metrics,
        })
    }
}

impl TryFrom<ProcessConfig> for MatchableProcessConfig {
    type Error = anyhow::Error;

    fn try_from(value: ProcessConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            match_by: value.match_by.try_into()?,
            metrics: value.metrics,
        })
    }
}

impl TryFrom<Config> for MatchableConfig {
    type Error = anyhow::Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut cgroups = Vec::new();
        for cgroup in value.cgroups {
            cgroups.push(cgroup.try_into()?);
        }
        let mut processes = Vec::new();
        for process in value.processes {
            processes.push(process.try_into()?);
        }
        Ok(Self {
            cgroups,
            processes,
            shell_commands: value.shell_commands,
        })
    }
}

impl ProcessMatcher {
    /// Returns the group name for this matcher from config.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Cmdline { name, .. }
            | Self::Comm { name, .. }
            | Self::Exe { name, .. }
            | Self::ExeBase { name, .. } => name,
        }
    }

    /// Returns the glob/regex matcher from config for this `ProcessMatcher`.
    #[must_use]
    pub fn matcher(&self) -> &NameMatcher {
        match self {
            Self::Exe { exe, .. } => exe,
            Self::ExeBase { exe_base, .. } => exe_base,
            Self::Comm { comm, .. } => comm,
            Self::Cmdline { cmdline, .. } => cmdline,
        }
    }
}

impl fmt::Display for NameMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Glob(pattern) => write!(f, "Glob({pattern})"),
            Self::Regex(regex) => write!(f, "Regex({regex})"),
        }
    }
}
