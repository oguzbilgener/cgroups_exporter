use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Context;
use arc_swap::ArcSwap;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
};
use tokio_stream::StreamExt as _;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use crate::{
    cgroups::discover_cgroups_metrics, matcher::MatchableConfig, metadata::METADATA,
    procs::discover_procs_metrics, render::MetricsRenderer, shell::ShellEvaluator,
};

const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct SharedConfig(Arc<ArcSwap<MatchableConfig>>);

impl SharedConfig {
    pub fn new(config: MatchableConfig) -> Self {
        Self(Arc::new(ArcSwap::new(Arc::new(config))))
    }

    pub fn load(&self) -> Arc<MatchableConfig> {
        self.0.load().clone()
    }

    pub fn update(&self, config: Arc<MatchableConfig>) {
        self.0.store(config);
    }
}

struct AppError(anyhow::Error);

pub async fn serve<Fut>(
    listen_addr: SocketAddr,
    config: SharedConfig,
    evaluator: ShellEvaluator,
    shutdown: Fut,
) -> anyhow::Result<()>
where
    Fut: Future<Output = ()> + Send + 'static,
{
    let listener = tokio::net::TcpListener::bind(listen_addr)
        .await
        .context("Failed to bind to listen address")?;

    info!(%listen_addr, "Server listening");

    let app = Router::new()
        .route("/", get(homepage))
        .route("/metrics", get(serve_metrics))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, TIMEOUT_DURATION),
        ))
        .layer(CompressionLayer::new())
        .with_state((config, evaluator));

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
        .context("Server error")
}

async fn homepage() -> Html<&'static str> {
    Html(
        r"<h1>cgroups_exporter</h1>
<p>This is a Prometheus exporter for cgroups and processes.</p>
<p>Visit the <a href='/metrics'>metrics endpoint</a> to get started.</p>
",
    )
}

async fn serve_metrics(
    State((config, evaluator)): State<(SharedConfig, ShellEvaluator)>,
) -> Result<Response<Body>, AppError> {
    #[allow(clippy::explicit_auto_deref)]
    let mut renderer = MetricsRenderer::new(vec![], &*METADATA);
    let config = config.load();
    let cgroup_metrics_stream = discover_cgroups_metrics(config.cgroups.as_slice(), &evaluator);
    let proc_metrics_stream = discover_procs_metrics(config.processes.as_slice());

    tokio::pin!(cgroup_metrics_stream);
    tokio::pin!(proc_metrics_stream);

    let mut cgroups_done = false;
    let mut procs_done = false;
    loop {
        tokio::select! {
            cgroup_metrics = cgroup_metrics_stream.next(), if !cgroups_done => {
                if let Some(cgroup_metrics) = cgroup_metrics {
                    renderer.render(cgroup_metrics)?;
                } else {
                    cgroups_done = true;
                }
            }
            proc_metrics = proc_metrics_stream.next(), if !procs_done => {
                if let Some(proc_metrics) = proc_metrics {
                    renderer.render(proc_metrics)?;
                } else {
                    procs_done = true;
                }
            }
            else => {
                break;
            }
        }
    }

    // We can't stream the response because the recordings from the same metrics families must be contiguous.
    let body = renderer.finish()?;

    Ok(Response::builder()
        .header("Content-Type", "text/plain; version=0.0.4")
        .body(Body::from(body))
        .unwrap())
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        error!(error = %self.0, "Internal error");
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}
