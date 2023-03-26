use std::error::Error;

use async_trait::async_trait;

use crate::models::SportEvent;

#[async_trait]
pub trait BookieParser {
    async fn parse(&self) -> Result<Vec<SportEvent>, Box<dyn Error>>;
}
