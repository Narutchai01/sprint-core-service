use super::super::{
    domain::Organization,
    entity::organization::Model,
    port::{CreateOrganizationError, repository::OrganizationRepository},
};
use super::repository::RepositoryImpl;
use chrono::DateTime;
use sea_orm::{DbBackend, DbErr, MockDatabase, Values};
use uuid::Uuid;

fn organization(description: Option<&str>) -> Organization {
    Organization {
        id: Uuid::from_u128(1),
        name: "Sprint".to_owned(),
        description: description.map(str::to_owned),
        created_at: DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
        updated_at: DateTime::from_timestamp(1_700_000_001, 0).unwrap(),
    }
}

#[tokio::test]
async fn inserts_organization_and_returns_saved_row() {
    for description in [Some("Planning"), None] {
        let input = organization(description);
        let row = Model {
            id: input.id,
            name: "Sprint team".to_owned(),
            description: input.description.clone(),
            created_at: input.created_at.naive_utc(),
            updated_at: input.updated_at.naive_utc(),
        };
        let db = MockDatabase::new(DbBackend::Postgres)
            .append_query_results([[row.clone()]])
            .into_connection();
        let values = Values(vec![
            input.id.into(),
            input.name.clone().into(),
            row.created_at.into(),
            row.updated_at.into(),
            input.description.clone().into(),
        ]);

        let created = RepositoryImpl::new(db.clone()).create(input).await.unwrap();

        assert_eq!(created.id, row.id);
        assert_eq!(created.name, row.name);
        assert_eq!(created.description, row.description);
        assert_eq!(created.created_at, row.created_at.and_utc());
        assert_eq!(created.updated_at, row.updated_at.and_utc());

        let log = db.into_transaction_log();
        assert_eq!(log.len(), 1);
        let statements = log[0].statements();
        assert_eq!(statements.len(), 1);
        let insert = &statements[0];
        assert!(insert.sql.starts_with("INSERT INTO \"organizations\" "));
        assert_eq!(insert.values.as_ref(), Some(&values));
    }
}

#[tokio::test]
async fn translates_database_error_to_persistence_error() {
    let db = MockDatabase::new(DbBackend::Postgres)
        .append_query_errors([DbErr::Custom("insert failed".to_owned())])
        .into_connection();

    let result = RepositoryImpl::new(db).create(organization(None)).await;

    assert!(matches!(result, Err(CreateOrganizationError::Persistence)));
}
