use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261119_165015_drop_organizations_column_name"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        _manager
            .alter_table(
                Table::alter()
                    .table("organizations")
                    .add_column(string_null("description"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        _manager
            .alter_table(
                Table::alter()
                    .table("organizations")
                    .drop_column("description")
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
