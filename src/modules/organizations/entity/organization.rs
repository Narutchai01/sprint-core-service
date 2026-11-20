use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "organizations"]
pub struct Organization {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
