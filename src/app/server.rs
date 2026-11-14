use anyhow::Context;
use db::Database;
use db::prelude::MigrationRunner;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::app::config::Config;
use crate::app::router;
use crate::app::schema;
use crate::app::state::AppState;

pub async fn run() -> anyhow::Result<()> {
    drop(dotenvy::dotenv());

    let mode_hint = std::env::var("APP_MODE").unwrap_or_else(|_| "dev".into());
    init_tracing(&mode_hint);

    let cfg = Config::from_env().context("load config")?;
    let listen = cfg.listen;
    let url_prefix = cfg.api_url_prefix();

    tracing::info!(
        mode = cfg.mode.as_str(),
        version = %cfg.version,
        api = %url_prefix,
        "starting"
    );

    tracing::info!("connecting to database");
       let database = Database::connect(&cfg.database_url)
           .await
           .context("connect database")?;

       let database = database.with_migrations(schema::migrations());
       let applied = database
           .run_pending()
           .await
           .context("run migrations")?;
       tracing::info!(count = applied.len(), "migrations applied");

    let state = AppState::new(cfg, database);
    let app = router::build(state);

    let listener = tokio::net::TcpListener::bind(listen)
        .await
        .with_context(|| format!("bind {listen} failed"))?;

    tracing::info!(addr = %listen, "listening");
    axum::serve(listener, app).await.context("serve failed")?;

    Ok(())
}

fn init_tracing(mode_hint: &str) {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,app=debug"));

    let registry = tracing_subscriber::registry().with(filter);
    let is_prod =
        mode_hint.eq_ignore_ascii_case("prod") || mode_hint.eq_ignore_ascii_case("production");

    if is_prod {
        registry
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}
