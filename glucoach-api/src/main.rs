use axum::Router;
use axum::http::Method;
use axum::routing::get;
use glucoach_lib::config::Config;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// TODO:
// - AppState
// - add supabase JWKs to AppState

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::builder().parse(
            #[cfg(debug_assertions)]
            "glucoach_api=debug,glucoach_lib=debug,axum=debug,tower_http=debug",
            #[cfg(not(debug_assertions))]
            "glucoach_api=info,glucoach_lib=info,axum=warn",
        )?)
        .try_init()?;
    tracing::info!("Starting glucoach_api v{}", env!("CARGO_PKG_VERSION"));

    dotenvy::dotenv().ok();
    let config = Config::try_from_env()?;
    let Config { host, port } = config;

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_methods([Method::GET]));

    let addr = SocketAddr::new(host, port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on http://{}:{}", &host, &port);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
