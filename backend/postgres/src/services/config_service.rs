use crate::{
    db_connection::DBConnection, postgres_error::PostgresResult, repositories::config_repository,
    tables::config::ConfigEntry,
};

pub async fn get_config_entry(conn: &DBConnection, key: &str) -> PostgresResult<ConfigEntry> {
    let mut transaction = conn.new_transaction().await?;
    let config = config_repository::get_config(&mut transaction, key).await?;
    transaction.commit().await?;
    Ok(config.to_config_entry()?)
}
