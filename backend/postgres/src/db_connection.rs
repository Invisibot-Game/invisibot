use std::str::FromStr;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres, Transaction,
};

use crate::{
    postgres_error::PostgresResult,
    repositories::{new_transaction, DB},
};

/// A pool of database connections.
#[derive(Debug, Clone)]
pub struct DBConnection {
    pool: Pool<Postgres>,
}

impl DBConnection {
    /// Setup a database connection and run migrations.
    pub async fn new(database_url: &str, log_db_statements: bool) -> Self {
        let mut pg_options =
            PgConnectOptions::from_str(database_url).expect("Invalid database url provided");

        if !log_db_statements {
            pg_options = pg_options.disable_statement_logging();
        }

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(pg_options)
            .await
            .expect("Failed to connect to DB");

        // Setup DB
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to run migrations");

        Self { pool: db_pool }
    }

    /// Start a new database transaction.
    pub async fn new_transaction(&self) -> PostgresResult<Transaction<'_, DB>> {
        new_transaction(&self.pool).await
    }
}
