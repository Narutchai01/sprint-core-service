use super::super::{
    domain::Organization,
    port::{CreateOrganizationError, repository::OrganizationRepository},
};
use super::routes_with_repository;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

struct FakeRepository {
    fails: bool,
}

impl OrganizationRepository for FakeRepository {
    async fn create(
        &self,
        organization: Organization,
    ) -> Result<Organization, CreateOrganizationError> {
        if self.fails {
            return Err(CreateOrganizationError::Persistence);
        }
        Ok(organization)
    }
}

fn create_request(body: &'static str) -> Request<Body> {
    Request::post("/organizations")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap()
}

#[tokio::test]
async fn creates_organization_with_optional_description() {
    let router = routes_with_repository(FakeRepository { fails: false });

    for (request, description) in [
        (
            r#"{"name":"Sprint","description":"Planning"}"#,
            Some("Planning"),
        ),
        (r#"{"name":"Sprint","description":null}"#, None),
        (r#"{"name":"Sprint"}"#, None),
    ] {
        let response = router
            .clone()
            .oneshot(create_request(request))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        let id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();
        let created_at: DateTime<Utc> = body["createdAt"].as_str().unwrap().parse().unwrap();
        let updated_at: DateTime<Utc> = body["updatedAt"].as_str().unwrap().parse().unwrap();

        assert_eq!(body.as_object().unwrap().len(), 5);
        assert_eq!(id.get_version_num(), 4);
        assert_eq!(body["name"], "Sprint");
        assert_eq!(body["description"], json!(description));
        assert_eq!(created_at, updated_at);
    }
}

#[tokio::test]
async fn rejects_malformed_json_and_missing_name() {
    let router = routes_with_repository(FakeRepository { fails: false });

    for (request, status) in [
        ("{", StatusCode::BAD_REQUEST),
        (
            r#"{"description":"Planning"}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
    ] {
        let response = router
            .clone()
            .oneshot(create_request(request))
            .await
            .unwrap();
        assert_eq!(response.status(), status);
    }
}

#[tokio::test]
async fn returns_empty_500_when_persistence_fails() {
    let response = routes_with_repository(FakeRepository { fails: true })
        .oneshot(create_request(r#"{"name":"Sprint"}"#))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        to_bytes(response.into_body(), 4096)
            .await
            .unwrap()
            .is_empty()
    );
}
