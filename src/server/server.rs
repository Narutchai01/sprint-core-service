#![allow(async_fn_in_trait)]
use crate::config::Config;
use axum::{Router, routing::get};
use std::net::SocketAddr;

pub trait IServer {
    async fn start(&self);
}

pub struct ServerImpl {
    cfg: Config,
}

impl ServerImpl {
    pub fn new(cfg: Config) -> Self {
        ServerImpl { cfg }
    }
}

impl IServer for ServerImpl {
    async fn start(&self) {
        let cfg = &self.cfg;
        let port = cfg.app_port;
        let app = Router::new().route(
            "/",
            get(|| async { "Hello, your going to Sprint managment" }),
        );

        let addr = SocketAddr::from(([0, 0, 0, 0], port as u16));
        println!("🚀 Server listening on http://localhost:{}", port);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
