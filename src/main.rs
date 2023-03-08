use std::error::Error;

use parser::BookieParser;

mod models;
mod parser;
mod parsers {
    pub mod top_sport;
    mod http_client_extensions;
}

fn main() -> Result<(), Box<dyn Error>> {
    let top_sport_parser = parsers::top_sport::TopSportParser::new();

    let sport_events = top_sport_parser.parse()?;

    println!("{:#?}", sport_events);

    Ok(())
}
