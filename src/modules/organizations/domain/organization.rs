use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateOrganization {
    pub name: String,
    pub description: Option<String>,
}

pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
