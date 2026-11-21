pub mod repository;
pub mod service;

#[derive(Debug)]
pub enum CreateOrganizationError {
    Persistence,
}
