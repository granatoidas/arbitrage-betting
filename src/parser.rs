use std::error::Error;

use crate::models::SportEvent;

pub trait BookieParser {
    fn parse(&self) -> Result<Vec<SportEvent>, Box<dyn Error>>;
}
