use super::super::domain::Organization;
use super::super::entity::organization::ActiveModel;
use super::super::port::{CreateOrganizationError, repository::OrganizationRepository};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub struct RepositoryImpl {
    db: DatabaseConnection,
}

impl RepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl OrganizationRepository for RepositoryImpl {
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, CreateOrganizationError> {
        let model = ActiveModel {
            id: Set(organization.id),
            name: Set(organization.name),
            description: Set(organization.description),
            created_at: Set(organization.created_at.naive_utc()),
            updated_at: Set(organization.updated_at.naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|_| CreateOrganizationError::Persistence)?;

        Ok(Organization {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at.and_utc(),
            updated_at: model.updated_at.and_utc(),
        })
    }
}
