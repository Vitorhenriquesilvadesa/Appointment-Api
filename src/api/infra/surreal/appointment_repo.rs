use std::{collections::HashMap, sync::Arc};

use rocket::async_trait;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::api::{
    domain::appointment::Appointment,
    repository::{
        AppointmentRepository, CrudRepository, NewAppointment, RepositoryError, UpdateAppointment,
    },
};

pub struct SurrealAppointRepository {
    client: Arc<Surreal<Client>>,
}

impl SurrealAppointRepository {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }
}

const APPOINTMENT_TABLE_NAME: &str = "appointment";

#[async_trait]
impl CrudRepository for SurrealAppointRepository {
    type Entity = Appointment;
    type NewEntity = NewAppointment;
    type UpdateEntity = UpdateAppointment;

    async fn create(&self, item: Self::NewEntity) -> Result<Self::Entity, RepositoryError> {
        let query_result = self
            .client
            .create(APPOINTMENT_TABLE_NAME)
            .content(item)
            .await
            .map_err(|e| RepositoryError::EntityCreate(e.to_string()))?;

        query_result
            .into_iter()
            .next()
            .ok_or_else(|| RepositoryError::EntityCreate("Could not create entity".into()))
    }

    async fn update(
        &self,
        id: String,
        item: Self::UpdateEntity,
    ) -> Result<Self::Entity, RepositoryError> {
        let mut fields = HashMap::new();

        if let Some(title) = item.title {
            fields.insert("title", surrealdb::sql::Value::from(title));
        }

        if let Some(description) = item.description {
            fields.insert("description", surrealdb::sql::Value::from(description));
        }

        if let Some(end_time) = item.end_time {
            fields.insert(
                "end_time",
                surrealdb::sql::Value::from(end_time.to_string()),
            );
        }

        let query_result = self
            .client
            .update((APPOINTMENT_TABLE_NAME, id))
            .merge(fields)
            .await
            .map_err(|e| RepositoryError::EntityUpdate(e.to_string()))?;

        query_result
            .into_iter()
            .next()
            .ok_or(RepositoryError::NotFound)
    }

    async fn get_all(&self) -> Result<Vec<Self::Entity>, RepositoryError> {
        let query_result: Vec<Appointment> = self
            .client
            .select(APPOINTMENT_TABLE_NAME)
            .await
            .map_err(|e| RepositoryError::EntityRead(e.to_string()))?;

        Ok(query_result)
    }

    async fn delete_by_id(&self, id: String) -> Result<Self::Entity, RepositoryError> {
        let query_result = self
            .client
            .delete((APPOINTMENT_TABLE_NAME, id))
            .await
            .map_err(|e| RepositoryError::DeleteEntity(e.to_string()))?;

        query_result
            .into_iter()
            .next()
            .ok_or(RepositoryError::NotFound)
    }

    async fn get_by_id(&self, id: String) -> Result<Self::Entity, RepositoryError> {
        let query_result = self
            .client
            .select((APPOINTMENT_TABLE_NAME, id))
            .await
            .map_err(|e| RepositoryError::EntityRead(e.to_string()))?;

        query_result
            .into_iter()
            .next()
            .ok_or(RepositoryError::NotFound)
    }
}

impl AppointmentRepository for SurrealAppointRepository {}
