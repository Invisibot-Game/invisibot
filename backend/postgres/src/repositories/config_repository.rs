use sqlx::Transaction;

use crate::{postgres_error::PostgresResult, tables::config::Config};

use super::DB;

pub async fn get_config(
    transaction: &mut Transaction<'_, DB>,
    key: &str,
) -> PostgresResult<Config> {
    Ok(sqlx::query_as!(
        Config,
        r#"
SELECT config_name, config_value, config_type
FROM config
WHERE config_name = $1
        "#,
        key
    )
    .fetch_one(transaction)
    .await?)
}
