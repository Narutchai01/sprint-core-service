use sprint_core_service::config::Config;
use sprint_core_service::server::{IServer, ServerImpl};

#[tokio::main]
async fn main() {
    let cfg = Config::load();
    let server = ServerImpl::new(cfg);

    server.start().await
}
