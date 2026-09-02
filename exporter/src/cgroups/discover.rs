use std::sync::{Arc, LazyLock};

use anyhow::Context as _;
use cgroups_explorer::Explorer;
use cgroups_rs::fs::Cgroup;
use tokio::sync::{Semaphore, mpsc};
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tracing::{debug, error};

use crate::{
    cgroups::metrics::CgroupMetrics,
    matcher::{CgroupMatcher, MatchableCgroupConfig, NameMatcher},
    render::MatchGroup,
    shell::ShellEvaluator,
};

const NAMESPACE: &str = "cgroup";

pub(crate) static CONCURRENCY: LazyLock<usize> = LazyLock::new(|| {
    std::env::var("CONCURRENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4)
});

pub fn discover_cgroups_metrics(
    config: &[MatchableCgroupConfig],
    evaluator: &ShellEvaluator,
) -> impl Stream<Item = MatchGroup<CgroupMetrics>> + 'static {
    let permits = Arc::new(Semaphore::new(*CONCURRENCY));
    let (send, recv) = mpsc::channel(*CONCURRENCY);

    for cgroup_config in config {
        let permits = permits.clone();
        let evaluator = evaluator.clone();
        tokio::spawn(discover_cgroup_metrics(
            cgroup_config.clone(),
            permits,
            send.clone(),
            evaluator,
        ));
    }
    drop(send);

    let stream = ReceiverStream::new(recv);
    stream.filter_map(|match_group| match match_group {
        Ok(match_group) => Some(match_group),
        Err(err) => {
            error!(%err, cause =% err.root_cause(), "Error while discovering cgroup metrics");
            None
        }
    })
}

async fn discover_cgroup_metrics(
    config: MatchableCgroupConfig,
    permits: Arc<Semaphore>,
    sender: mpsc::Sender<anyhow::Result<MatchGroup<CgroupMetrics>>>,
    evaluator: ShellEvaluator,
) {
    let Ok(_permit) = permits.acquire_owned().await else {
        return;
    };
    let _join_res = tokio::task::spawn_blocking(move || {
        let series_result = discover_cgroup_metrics_blocking(&config.match_by, &evaluator);
        let _ = sender.blocking_send(series_result.map(|cgroups| {
            let mut metrics_config = config.metrics;
            if metrics_config.namespace.is_none() {
                metrics_config.namespace = Some(NAMESPACE.to_string());
            }
            MatchGroup::new(cgroups, metrics_config)
        }));
    })
    .await;
}

fn discover_cgroup_metrics_blocking(
    matcher: &CgroupMatcher,
    evaluator: &ShellEvaluator,
) -> anyhow::Result<Vec<CgroupMetrics>> {
    let cgroups_iter = discover_cgroup_for_match_blocking(matcher)
        .map_err(|err| {
            error!("Failed to discover cgroups: {}", err);
            err
        })
        .with_context(|| format!("while discovering cgroups for match: {}", matcher.path))?;

    let metrics = cgroups_iter
        .filter_map(|cgroup| {
            match CgroupMetrics::from_cgroup_blocking(&cgroup, matcher, evaluator) {
                Ok(answer) => Some(answer),
                Err(err) => {
                    // Logging at the debug level to avoid cluttering the logs in case of many cgroups
                    debug!(%err, cause =% err.root_cause(), "Failed to create CgroupMetrics");
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(metrics)
}

fn discover_cgroup_for_match_blocking(
    matcher: &CgroupMatcher,
) -> anyhow::Result<impl Iterator<Item = Cgroup>> {
    match &matcher.path {
        NameMatcher::Glob(glob) => {
            let explorer = Explorer::detect_version()
                .include(vec![glob.to_string()])
                .build()?;
            Ok(explorer.iter_cgroups())
        }
        NameMatcher::Regex(regex) => {
            let explorer = Explorer::detect_version()
                .include_regex(vec![regex.to_owned()])
                .build()?;
            Ok(explorer.iter_cgroups())
        }
    }
}
