use sprint_core_service::config::Config;
use sprint_core_service::infra::db::{DatabaseConnect, DatabaseConnectImpl};
use sprint_core_service::server::{IServer, ServerImpl};

#[tokio::main]
async fn main() {
    let cfg = Config::load();
    let db_url = cfg.db_url.clone();
    let db = DatabaseConnectImpl::new(db_url);
    let db_conn = db.connect().await.unwrap();
    let server = ServerImpl::new(cfg, db_conn);

    server.start().await
}
