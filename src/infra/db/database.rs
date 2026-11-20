#![allow(async_fn_in_trait)]
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub trait DatabaseConnect {
    async fn connect(&self) -> Result<DatabaseConnection, DbErr>;
}
pub struct DatabaseConnectImpl {
    connection_string: String,
}

impl DatabaseConnectImpl {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl DatabaseConnect for DatabaseConnectImpl {
    async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        let mut options = ConnectOptions::new(self.connection_string.to_owned());

        options
            .max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(5))
            .acquire_timeout(Duration::from_secs(5));

        Database::connect(options).await
    }
}
