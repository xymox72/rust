use crate::utils_base::utils_base::MessageServiceError;
use crate::service::db::Database;
use crate::service::models::message::Message;
use crate::service::files::file_service::FileService;

use chrono::{DateTime, Utc};

pub struct Service {
    db: Database,
    file_service: FileService
}

impl Service {
    pub async fn new() -> Result<Self, MessageServiceError> {
        let db = Database::new().await?;
        let file_service = FileService::new()?;
        Ok(Service { db, file_service })
    }
    pub async fn get_meesages(&self, date_time: DateTime<Utc>) -> Result<Vec<Message>, MessageServiceError> {
        let messages = self.db.get_messages(&date_time).await?;
        Ok(messages)
    }

    pub async fn get_count_of_messages(&self, date_time: DateTime<Utc>) -> Result<i64, MessageServiceError> {
        let count = self.db.count(&date_time).await?;

        Ok(count)
    }
   
    pub async fn remove_messages(&self, date_time: DateTime<Utc>) -> Result<u64, MessageServiceError> {
        let messages = self.get_meesages(date_time).await?;
        let files_id: Vec<String> = messages.into_iter().map(|msg| msg.messagefilename).collect();

        if files_id.len() > 0{
           self.file_service.delete_files(files_id).await?;
        }

       // self.db.remove_messages(&date_time).await?;
        Ok(100)
    }
}
