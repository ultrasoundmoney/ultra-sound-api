use crate::env;
use sqlx::PgPool;

fn get_db_url() -> String {
    env::get_env_var_unsafe("DATABASE_URL")
}

pub async fn get_db_pool() -> PgPool {
    let db_url = get_db_url();
    PgPool::connect(&db_url)
        .await
        .expect("expect DB to be available to connect")
}
