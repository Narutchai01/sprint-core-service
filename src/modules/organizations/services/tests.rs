use super::super::{
    domain::CreateOrganization, repository::OrganizationRepository,
    services::OrganizationServiceImpl,
};

fn organization(org: CreateOrganization) -> CreateOrganization {
    CreateOrganization {
        name: org.name,
        description: org.description,
    }
}

#[tokio::test]
async fn test_organization_service() {
    let org = organization(CreateOrganization {
        name: "Sprint".to_owned(),
        description: None,
    });
    assert_eq!(org.name, "Sprint");
}
