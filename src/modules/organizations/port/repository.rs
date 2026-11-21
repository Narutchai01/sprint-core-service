use super::super::domain::Organization;
use super::CreateOrganizationError;
use std::future::Future;

pub trait OrganizationRepository {
    fn create(
        &self,
        organization: Organization,
    ) -> impl Future<Output = Result<Organization, CreateOrganizationError>> + Send;
}
