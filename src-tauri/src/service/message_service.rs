use crate::utils_base::utils_base::MessageServiceError;
use crate::service::db::Database;
use crate::service::models::message::Message;
use crate::service::files::file_service::FileService;

use chrono::{DateTime, Utc};
use log::info;

use crate::service::log::init_logging;

pub struct Service {
    db: Database,
    file_service: FileService
}

impl Service {
    pub async fn new() -> Result<Self, MessageServiceError> {
        let db = Database::new().await?;
        let file_service = FileService::new()?;
        init_logging();
        Ok(Service { db, file_service })
    }
    pub async fn get_meesages(&self, date_time: DateTime<Utc>, is_full: Option<bool>) -> Result<Vec<Message>, MessageServiceError> {
        let messages = self.db.get_messages(&date_time, is_full).await?;
        Ok(messages)
    }

    pub async fn get_count_of_messages(&self, date_time: DateTime<Utc>) -> Result<i64, MessageServiceError> {
        let count = self.db.count(&date_time).await?;

        Ok(count)
    }
   
    pub async fn remove_messages<F>(&self, date_time: DateTime<Utc>, emit: F) -> Result<(), MessageServiceError> 
    where
    F: Fn(String) + Send + Sync,{
        let messages = self.get_meesages(date_time, Some(true)).await?;
        let files_id: Vec<String> = messages.into_iter().map(|msg| msg.messagefilename).collect();

        if files_id.len() > 0{
           self.file_service.delete_files(files_id,  emit).await?;
        }

       //self.db.remove_messages(&date_time).await?;
       info!("We have done. Enjoy!");
        Ok(())
    }
}
