#![allow(async_fn_in_trait)]
use crate::config::Config;
use crate::routers::{IRouters, RoutersImpl};
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;

pub trait IServer {
    async fn start(&self);
}

pub struct ServerImpl<'a> {
    cfg: &'a Config,
    db: DatabaseConnection,
}

impl<'a> ServerImpl<'a> {
    pub fn new(cfg: &'a Config, db: DatabaseConnection) -> Self {
        ServerImpl { cfg, db }
    }
}

impl IServer for ServerImpl<'_> {
    async fn start(&self) {
        let cfg = &self.cfg;
        let port = cfg.app_port;
        let app = Router::new().route(
            "/",
            get(|| async { "Hello, your going to Sprint managment" }),
        );

        let app = RoutersImpl::new(app, self.db.clone()).build();

        let addr = SocketAddr::from(([0, 0, 0, 0], port as u16));
        println!("🚀 Server listening on http://localhost:{}", port);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
