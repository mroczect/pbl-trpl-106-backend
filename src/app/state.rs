use std::sync::Arc;

use db::Database;

use crate::app::config::Config;

#[derive(Clone, Debug)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: Arc<Database>,
}

impl AppState {
    #[must_use]
    pub fn new(cfg: Config, db: Database) -> Self {
        Self {
            cfg: Arc::new(cfg),
            db: Arc::new(db),
        }
    }
}
