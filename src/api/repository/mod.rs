pub mod appointment_repo;

pub use appointment_repo::*;
use rocket::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Not found")]
    NotFound,

    #[error("Database error")]
    DatabaseError,

    #[error("Cannot update entity: {0}")]
    EntityUpdate(String),

    #[error("Cannot create entity: {0}")]
    EntityCreate(String),

    #[error("Cannot read entity: {0}")]
    EntityRead(String),

    #[error("Cannot delete entity: {0}")]
    DeleteEntity(String),
}

#[async_trait]
pub trait CrudRepository {
    type Entity;
    type NewEntity;
    type UpdateEntity;

    async fn create(&self, item: Self::NewEntity) -> Result<Self::Entity, RepositoryError>;
    async fn update(
        &self,
        id: String,
        item: Self::UpdateEntity,
    ) -> Result<Self::Entity, RepositoryError>;
    async fn delete_by_id(&self, id: String) -> Result<Self::Entity, RepositoryError>;
    async fn get_by_id(&self, id: String) -> Result<Self::Entity, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<Self::Entity>, RepositoryError>;
}
