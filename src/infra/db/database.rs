#![allow(async_fn_in_trait)]
use crate::config::config::Config;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub trait DatabaseConnect {
    async fn connect(&self) -> Result<DatabaseConnection, DbErr>;
}
pub struct DatabaseConnectImpl<'a> {
    cfg: &'a Config,
}

impl<'a> DatabaseConnectImpl<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg }
    }
}

impl<'a> DatabaseConnect for DatabaseConnectImpl<'a> {
    async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        let mut options = ConnectOptions::new(&self.cfg.db_url);

        options
            .max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(5))
            .acquire_timeout(Duration::from_secs(5));

        Database::connect(options).await
    }
}
