use anyhow::Context as _;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

/// The main application config.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// A list of configs to use when collecting metrics from cgroups.
    pub cgroups: Vec<CgroupConfig>,
    /// A list of configs to use when collecting metrics from processes.
    #[serde(default)]
    pub processes: Vec<ProcessConfig>,
    /// Configuration for the shell commands executor used when rewriting cgroup names with `Templated::Shell`.
    #[serde(default, rename = "shell")]
    pub shell_commands: ShellCommandsConfig,
}

/// The config for a cgroup. This includes the matcher and the metrics config.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CgroupConfig {
    /// The matcher for the cgroup(s). This can be a glob or a regex.
    #[serde(rename = "match")]
    pub match_by: CgroupMatch,
    /// The metrics config for the cgroup(s). This includes the label map and namespace.
    #[serde(default)]
    pub metrics: MetricsConfig,
}

/// The config for a process. This includes the matcher and the metrics config.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessConfig {
    /// The matcher for the process(es). This can be a glob or a regex.
    #[serde(rename = "match")]
    pub match_by: ProcessMatch,
    /// The metrics config for the process(es). This includes the label map and namespace.
    #[serde(default)]
    pub metrics: MetricsConfig,
}

/// Rules for how to generate metrics from discovered process and cgroup groups.
#[derive(Debug, Clone, Deserialize, JsonSchema, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MetricsConfig {
    #[serde(default = "default_label_map")]
    /// The label map to use for the metrics. This is used to rename labels in the metrics.
    /// For example, if the label map is `{"name": "process"}`, the label "name" will be renamed to "process".
    pub label_map: HashMap<String, String>,
    /// The namespace to use for the metrics. This is used to group metrics together.
    /// For example, if the namespace is `my_service`, the metrics will be prefixed with `my_service_`
    /// instead of `process_` or `cgroup_`.
    pub namespace: Option<String>,
}

/// A matcher for cgroups. It can match a single cgroup or a group of cgroups.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CgroupMatch {
    /// The name matcher for the cgroup(s). This can be a glob or a regex. Use the exact glob `/`
    /// to match the root cgroup.
    pub path: NameMatch,
    /// Sum the processes of the whole subtree under a matched cgroup, not just the ones it holds
    /// directly. The controller files a cgroup exposes already cover its descendants, so turn this
    /// on when the matched cgroup keeps its processes one level down.
    #[serde(default)]
    pub recursive: bool,
    /// Group name rewrite rules.
    #[serde(flatten, default)]
    pub rewrite: Option<RewriteCgroupName>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum RewriteCgroupName {
    RemovePrefix {
        /// The prefix to remove from the cgroup name. Useful when the cgroup is nested in a hierarchy.
        remove_prefix: String,
    },
    Template {
        /// A template string to rewrite the cgroup name with. Use this in conjunction with regex capture groups.
        name: Templated,
    },
}

#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Templated {
    /// A template string to rewrite the cgroup name with. Use this in conjunction with regex capture groups.
    Name(String),
    /// A shell command to run to rewrite the cgroup name. The command can include regex capture groups.
    /// This is useful for more complex rewrites.
    Shell {
        /// The shell command to run. This can include regex capture groups.
        /// The command will be run with `sh -c` and the output will be captured.
        /// The command must exit with a 0 status code.
        shell: String,
        /// The stream to use for the shell command. This can be `stdout` or `stdin`.
        #[serde(default)]
        output: ShellCommandStream,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShellCommandStream {
    #[default]
    Stdout,
    Stderr,
}

/// A matcher for process names. This can be a glob or a regex.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum NameMatch {
    /// A glob pattern or a simple string to match a process or cgroup against.
    Glob(String),
    /// A regex pattern to match a process or cgroup against.
    Regex {
        /// The regex. If this is not a valid regex, this matcher will be ignored.
        regex: String,
    },
}

/// Rule to match processes by their executable name, command name, or command line,
/// and group them together.
/// The group name can use template variables to divide the group into subgroups.
/// The variables include `comm`, `exe`, `pid`, as well as any regex capture groups.
#[derive(Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
#[schemars(deny_unknown_fields)]
pub enum ProcessMatch {
    /// Match a process by the full path to the executable.
    Exe {
        /// The path of the executable. If this matches multiple processes, they are grouped together.
        exe: NameMatch,
        /// The path to use for the whole match group. This can use template variables to divide the group into subgroups.
        name: String,
    },
    /// Match a process by its executable name.
    ExeBase {
        /// The name of the executable. If this matches multiple processes, they are grouped together.
        exe_base: NameMatch,
        /// The name to use for the whole match group. This can use template variables to divide the group into subgroups.
        name: String,
    },
    /// Match a process by its command name.
    Comm {
        /// The name of the process. If this matches multiple processes, they are grouped together.
        comm: NameMatch,
        /// The name to use for the whole match group. This can use template variables to divide the group into subgroups.
        name: String,
    },
    /// Match a process by its whole command line.
    Cmdline {
        /// The command line of the process. If this matches multiple processes, they are grouped together.
        cmdline: NameMatch,
        /// The name to use for the whole match group. This can use template variables to divide the group into subgroups.
        name: String,
    },
}

/// Configuration for the shell commands executor used when rewriting cgroup names with `Templated::Shell`.
#[derive(Default, Debug, Clone, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ShellCommandsConfig {
    /// The capacity of the LRU cache for the results of the shell command executions.
    #[serde(default = "default_cache_size")]
    pub cache_size: usize,
}

fn default_label_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "name".to_string());
    map
}

fn default_cache_size() -> usize {
    100
}

impl Config {
    /// Creates a new `Config` from a YAML string.
    ///
    /// # Errors
    ///
    /// If the YAML string is not valid, an error is returned.
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Loads a config from a YAML or JSON file.
///
/// # Errors
///
/// If the file does not exist or is not valid YAML/JSON, an error is returned.
#[cfg(feature = "tokio")]
pub async fn load_config(path: &Path) -> anyhow::Result<Config> {
    let config_str = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read config file")?;
    if path.extension() == Some("json".as_ref()) {
        return Config::from_json(&config_str).context("Failed to parse config");
    }
    Config::from_yaml(&config_str).context("Failed to parse config")
}

pub fn load_config_blocking(path: &Path) -> anyhow::Result<Config> {
    let file = std::fs::File::open(path).context("Failed to open config file")?;
    if path.extension() == Some("json".as_ref()) {
        serde_json::from_reader(file).context("Failed to parse config")
    } else {
        serde_yaml::from_reader(file).context("Failed to parse config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    #[test]
    fn test_load_config1() {
        let config = load_config_blocking(Path::new("../tests/test_data/config1.yml")).unwrap();
        assert_eq!(3, config.cgroups.len());
        assert_eq!(1, config.processes.len());
        assert_eq!(
            config,
            Config {
                cgroups: vec![
                    CgroupConfig {
                        match_by: CgroupMatch {
                            path: NameMatch::Glob("services.scope/*".to_string()),
                            recursive: false,
                            rewrite: Some(RewriteCgroupName::RemovePrefix {
                                remove_prefix: "services.scope/".to_string()
                            })
                        },
                        metrics: MetricsConfig {
                            label_map: vec![("name".to_string(), "name".to_string())]
                                .into_iter()
                                .collect(),
                            namespace: Some("my_services".to_string())
                        }
                    },
                    CgroupConfig {
                        match_by: CgroupMatch {
                            path: NameMatch::Regex {
                                regex: "^system.slice/docker-(?<containerId>\\w+)\\.scope$"
                                    .to_string()
                            },
                            recursive: false,
                            rewrite: Some(RewriteCgroupName::Template {
                                name: Templated::Shell {
                                    shell: "docker ps --filter \"id={containerId}\" --format \"{{.Names}}\"".to_string(),
                                    output: ShellCommandStream::Stdout,
                                }
                            })
                        },
                        metrics: MetricsConfig {
                            label_map: vec![("name".to_string(), "name".to_string())]
                                .into_iter()
                                .collect(),
                            namespace: Some("container".to_string())
                        }
                    },
                    CgroupConfig {
                        match_by: CgroupMatch {
                            path: NameMatch::Regex {
                                regex: "^system.slice/docker-(?<containerId>\\w+)\\.scope$"
                                    .to_string()
                            },
                            recursive: false,
                            rewrite: Some(
                                RewriteCgroupName::Template {
                                    name: Templated::Name("{containerId}".to_string())
                                },
                            ),
                        },
                        metrics: MetricsConfig {
                            label_map: vec![("name".to_string(), "id".to_string())]
                                .into_iter()
                                .collect(),
                            namespace: Some("container".to_string())
                        }
                    }
                ],
                processes: vec![ProcessConfig {
                    match_by: ProcessMatch::Comm {
                        comm: NameMatch::Glob("firefox".to_string()),
                        name: "firefox".to_string()
                    },
                    metrics: MetricsConfig {
                        label_map: vec![("name".to_string(), "name".to_string())]
                            .into_iter()
                            .collect(),
                        namespace: Some("my_services".to_string())
                    }
                }],
                shell_commands: ShellCommandsConfig {
                    cache_size: 1024
                }
            }
        );
    }
}
