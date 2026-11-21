use super::super::domain::{CreateOrganization, Organization};
use super::super::port::{
    CreateOrganizationError, repository::OrganizationRepository, service::OrganizationService,
};
use chrono::Utc;
use uuid::Uuid;

pub struct OrganizationServiceImpl<R> {
    organization_repository: R,
}

impl<R> OrganizationServiceImpl<R> {
    pub fn new(organization_repository: R) -> Self {
        Self {
            organization_repository,
        }
    }
}

impl<R: OrganizationRepository + Sync> OrganizationService for OrganizationServiceImpl<R> {
    async fn create_organization(
        &self,
        input: CreateOrganization,
    ) -> Result<Organization, CreateOrganizationError> {
        let now = Utc::now();
        self.organization_repository
            .create(Organization {
                id: Uuid::new_v4(),
                name: input.name,
                description: input.description,
                created_at: now,
                updated_at: now,
            })
            .await
    }
}
