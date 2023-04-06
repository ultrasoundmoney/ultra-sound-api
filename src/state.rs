use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
}
