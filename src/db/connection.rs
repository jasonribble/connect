use sqlx::SqlitePool;
use std::sync::Arc;
pub struct Connection {
    pub sqlite_pool: Arc<SqlitePool>,
}

impl Connection {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            sqlite_pool: Arc::new(pool),
        }
    }
}
