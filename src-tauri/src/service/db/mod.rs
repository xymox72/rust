use std::env;

use crate::service::models::message::Message;
use chrono::{DateTime, Utc};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub struct Database {
    pool: Pool<MySql>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let con_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&con_str)
            .await?;
        Ok(Database { pool })
    }

    pub async fn count(&self, created_at: &DateTime<Utc>) -> Result<i64, sqlx::Error> {
        let conn = &self.pool;

        let query = r#"
        SELECT
            COUNT(*)
        FROM 
            hm_messages 
        WHERE 
            messagecreatetime <= ?
        
    "#;

        let count: (i64,) = sqlx::query_as(query)
            .bind(created_at)
            .fetch_one(conn)
            .await?;

        Ok(count.0)
    }

    pub async fn remove_messages(&self, created_at: &DateTime<Utc>) -> Result<(), sqlx::Error> {
        let conn = &self.pool;

        let query = r#"
        DELETE
        FROM 
            hm_messages 
        WHERE 
            messagecreatetime <= ?
    "#;

        sqlx::query(query).bind(created_at).execute(conn).await?;

        Ok(())
    }

    pub async fn get_messages(
        &self,
        created_at: &DateTime<Utc>,
        is_full: Option<bool>
    ) -> Result<Vec<Message>, sqlx::Error> {
        let conn = &self.pool;
        let mut query  =   r#"
        SELECT
            messagecreatetime, 
            messagefilename, 
            messageflags, 
            messagefolderid, 
            messagefrom, 
            messageid, 
            messageuid 
        FROM 
            hm_messages 
        WHERE 
            messagecreatetime <= ?
        ORDER BY  messagecreatetime desc
        LIMIT 10000
    "#;
        if is_full.is_some_and(|val| val){
          query =  r#"
            SELECT
                messagecreatetime, 
                messagefilename, 
                messageflags, 
                messagefolderid, 
                messagefrom, 
                messageid, 
                messageuid 
            FROM 
                hm_messages 
            WHERE 
                messagecreatetime <= ?
            ORDER BY  messagecreatetime desc
        "#;
        }
      

        let messages = sqlx::query_as::<_, Message>(&query)
            .bind(created_at)
            .fetch_all(conn)
            .await?;

        Ok(messages)
    }
}
