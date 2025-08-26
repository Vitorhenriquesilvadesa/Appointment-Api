use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_id: String,
}

fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let thing = Thing::deserialize(deserializer)?;
    Ok(thing.id.to_string())
}
