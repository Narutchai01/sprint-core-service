use sea_orm::DatabaseConnection;
pub trait IOrganizationsRepository {
    async fn create() {}
}

pub struct OrganizationsRepositoryImp {
    db: DatabaseConnection,
}

impl OrganizationsRepositoryImp {
    fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl IOrganizationsRepository for OrganizationsRepositoryImp {
    async fn create() {}
}
