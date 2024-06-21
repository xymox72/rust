use crate::service::db::Database;
use crate::service::models::message::Message;



pub struct Service{

  pub db: Database
}

impl Service{

    pub async fn new(&self) -> Result<Self, sqlx::Error> {
        let db = Database::new(connection_str).await?;
        Ok(Service {db })
    }
    pub fn get_meesages(&self) -> Result<Vec<Message>, sqlx::Error>{
        
    }
}