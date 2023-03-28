use std::{error::Error, vec};

use models::SportEvent;
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

    let top_sport_events = top_sport_parser.parse().await?;
    let bet_safe_events = bet_safe_parser.parse().await?;

    let events_by_provider = vec![top_sport_events, bet_safe_events];

    println!("{:#?}", find_arbitrages(events_by_provider));

    // println!("{:#?}", events_by_provider[0]);
    // println!("");
    // println!("");
    // println!("");
    // println!("{:#?}", events_by_provider[1]);

    Ok(())
}

fn find_arbitrages(mut events_by_provider: Vec<Vec<SportEvent>>) -> Result<Vec<Vec<SportEvent>>, Box<dyn Error>> {
    let mut grouped_events: Vec<Vec<SportEvent>> = vec![];

    while let Some(mut base_provider_events) = events_by_provider.pop() {
        while let Some(base_sport_event) = base_provider_events.pop() {
            let mut matching_events: Vec<SportEvent> = vec![];

            for provider_events in events_by_provider.iter_mut() {
                let mut found_matching_event: Option<usize> = Option::None;
                for (i, sport_event) in provider_events.iter().enumerate() {
                    if compare_events(&base_sport_event, sport_event) {
                        found_matching_event = Option::Some(i);
                        break;
                    }
                }

                if let Some(matching_event_index) = found_matching_event {
                    matching_events.push(provider_events.remove(matching_event_index));
                }
            }

            matching_events.push(base_sport_event);

            grouped_events.push(matching_events);
        }
    }

    Ok(grouped_events)
}

fn compare_events(event_1: &SportEvent, event_2: &SportEvent) -> bool {
    let event_1_team_1 = event_1.team1.to_lowercase();
    let event_1_team_2 = event_1.team2.to_lowercase();
    let event_2_team_1 = event_2.team1.to_lowercase();
    let event_2_team_2 = event_2.team2.to_lowercase();

    if event_1_team_1 == event_2_team_1 && event_1_team_2 == event_2_team_2 {
        return true;
    }

    if event_1_team_2 == event_2_team_1 && event_1_team_1 == event_2_team_2 {
        return true;
    }

    false
}
