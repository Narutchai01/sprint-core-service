use super::super::domain::{CreateOrganization, Organization};
use super::CreateOrganizationError;
use std::future::Future;

pub trait OrganizationService {
    fn create_organization(
        &self,
        organization: CreateOrganization,
    ) -> impl Future<Output = Result<Organization, CreateOrganizationError>> + Send;
}
