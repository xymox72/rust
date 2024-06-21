use crate::service::db::Database;
use crate::service::models::message::Message;
use chrono::{DateTime, Utc};
use sqlx::Error;

pub struct Service {
    db: Database,
}

impl Service {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db = Database::new().await?;
        Ok(Service { db })
    }
    pub async fn get_meesages(&self, date_time: DateTime<Utc>) -> Result<Vec<Message>, Error> {
        let messages = self.db.get_messages(&date_time).await?;
        Ok(messages)
    }

    pub async fn get_count_of_messages(&self, date_time: DateTime<Utc>) -> Result<i64, Error> {
        let count = self.db.count(&date_time).await?;

        Ok(count)
    }

    pub async fn remove_messages(&self, date_time: DateTime<Utc>) -> Result<u64, Error> {
        self.db.remove_messages(&date_time).await?;
        Ok(100)
    }
}
