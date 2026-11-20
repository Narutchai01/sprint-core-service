pub use sea_orm_migration::prelude::*;

mod m20261119_161623_create_organizations;
mod m20261119_165015_drop_organizations_column_name;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261119_161623_create_organizations::Migration),
            Box::new(m20261119_165015_drop_organizations_column_name::Migration),
        ]
    }
}
