use crate::modules::organizations::organization_routes;
use axum::Router;
use sea_orm::DatabaseConnection;

pub trait IRouters {
    fn build(self) -> Router;
}

pub struct RoutersImpl {
    db: DatabaseConnection,
    router: Router,
}

impl RoutersImpl {
    pub fn new(router: Router, db: DatabaseConnection) -> Self {
        Self { db, router }
    }
}

impl IRouters for RoutersImpl {
    fn build(self) -> Router {
        let v1 = organization_routes(self.db);
        self.router.nest("/api/v1", v1)
    }
}
