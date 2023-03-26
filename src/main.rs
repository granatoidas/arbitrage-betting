use std::error::Error;

use parser::BookieParser;

mod models;
mod parser;
mod parsers {
    pub mod bet_safe;
    mod http_client_extensions;
    pub mod top_sport;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let top_sport_parser = parsers::top_sport::TopSportParser::new();
    let bet_safe_parser = parsers::bet_safe::BetSafePraser::new();

    // let top_sport_events = top_sport_parser.parse().await?;
    let bet_safe_events = bet_safe_parser.parse().await?;

    // println!("{:#?}", top_sport_events);
    println!("");
    println!("");
    println!("");
    println!("{:#?}", bet_safe_events);

    Ok(())
}
