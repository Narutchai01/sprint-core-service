use super::super::domain::CreateOrganization;
use super::super::port::{CreateOrganizationError, service::OrganizationService};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateOrganizationRequest {
    name: String,
    description: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct OrganizationController<S> {
    organization_service: S,
}

impl<S: OrganizationService + Send + Sync> OrganizationController<S> {
    pub fn new(organization_service: S) -> Self {
        Self {
            organization_service,
        }
    }
    pub async fn create_organization(
        State(controller): State<Arc<Self>>,
        Json(request): Json<CreateOrganizationRequest>,
    ) -> Result<(StatusCode, Json<OrganizationResponse>), StatusCode> {
        let created = controller
            .organization_service
            .create_organization(CreateOrganization {
                name: request.name,
                description: request.description,
            })
            .await
            .map_err(|error| match error {
                CreateOrganizationError::Persistence => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

        let response = OrganizationResponse {
            id: created.id,
            name: created.name,
            description: created.description,
            created_at: created.created_at,
            updated_at: created.updated_at,
        };
        Ok((StatusCode::CREATED, Json(response)))
    }
}
