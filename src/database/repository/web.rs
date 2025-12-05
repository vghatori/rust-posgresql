

use sqlx::{PgPool, Row};
use async_trait::async_trait;
use sqlx::Error;

use crate::database::models::message::Message;
#[derive(Clone)]
pub struct WebRepository {
    pub pool: PgPool
}
#[async_trait]
pub trait PgWebRepository {
    async fn create(&self, message : Message) -> Result<(), Error>;
    async fn read_all(&self) -> Result<Vec<Message>, Error>;
}

impl WebRepository {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pool : pg_pool
        }
    }

}
#[async_trait]
impl PgWebRepository for WebRepository {
    async fn create(&self, message: Message) -> Result<(), Error> {
        sqlx::query!("INSERT INTO Message (title, description) VALUES ($1, $2)", message.title, message.description)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn read_all(&self) -> Result<Vec<Message>, sqlx::Error> {
        let query = sqlx::query_as::<_,Message>("select * from message")
            .fetch_all(&self.pool)
            .await?;
        Ok(query)
    }
}