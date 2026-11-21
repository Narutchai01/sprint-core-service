use super::controller::organization_controller::OrganizationController;
use super::port::repository::OrganizationRepository;
use super::repository::repository::RepositoryImpl;
use super::services::organization_service::OrganizationServiceImpl;
use axum::{Router, routing::post};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn organization_routes(db: DatabaseConnection) -> Router {
    routes_with_repository(RepositoryImpl::new(db))
}

fn routes_with_repository<R: OrganizationRepository + Send + Sync + 'static>(
    repository: R,
) -> Router {
    let service = OrganizationServiceImpl::new(repository);
    let controller = Arc::new(OrganizationController::new(service));

    Router::<Arc<OrganizationController<OrganizationServiceImpl<R>>>>::new()
        .route(
            "/organizations",
            post(OrganizationController::create_organization),
        )
        .with_state(controller)
}

#[cfg(test)]
mod tests;
