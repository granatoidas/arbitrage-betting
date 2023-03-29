use std::{error::Error, vec};

use models::SportEvent;
use parser::BookieParser;
use playwright::Playwright;

mod models;
mod parser;
mod parsers {
    pub mod bet_safe;
    mod http_client_extensions;
    pub mod oly_bet;
    pub mod top_sport;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;

    let top_sport_parser = parsers::top_sport::TopSportParser::new();

    let page = context.new_page().await?;
    let bet_safe_parser = parsers::bet_safe::BetSafePraser::new(page);

    let page = context.new_page().await?;
    let oly_bet_parser = parsers::oly_bet::OlyBetParser::new(page);

    let top_sport_events = top_sport_parser.parse().await?;
    let bet_safe_events = bet_safe_parser.parse().await?;
    let oly_bet_events = oly_bet_parser.parse().await?;

    let events_by_provider = vec![top_sport_events, bet_safe_events, oly_bet_events];

    println!("{:#?}", find_arbitrages(events_by_provider));

    // println!("{:#?}", events_by_provider[0]);
    // println!("");
    // println!("");
    // println!("");
    // println!("{:#?}", events_by_provider[1]);

    Ok(())
}

fn find_arbitrages(
    mut events_by_provider: Vec<Vec<SportEvent>>,
) -> Result<Vec<Vec<SportEvent>>, Box<dyn Error>> {
    let mut grouped_events: Vec<Vec<SportEvent>> = vec![];

    while let Some(mut base_provider_events) = events_by_provider.pop() {
        while let Some(base_sport_event) = base_provider_events.pop() {
            let mut matching_events: Vec<SportEvent> = vec![];

            for provider_events in events_by_provider.iter_mut() {
                let mut found_matching_event: Option<(usize, bool)> = Option::None;
                for (i, sport_event) in provider_events.iter().enumerate() {
                    let (events_match, order_matches) =
                        compare_events(&base_sport_event, sport_event);
                    if events_match {
                        found_matching_event = Option::Some((i, order_matches));
                        break;
                    }
                }

                if let Some((matching_event_index, order_matches)) = found_matching_event {
                    let mut event = provider_events.remove(matching_event_index);
                    if !order_matches {
                        event.switch_teams();
                    }

                    matching_events.push(event);
                }
            }

            matching_events.push(base_sport_event);

            grouped_events.push(matching_events);
        }
    }

    Ok(grouped_events)
}

fn compare_events(event_1: &SportEvent, event_2: &SportEvent) -> (bool, bool) {
    let event_1_team_1 = event_1.team1.to_lowercase();
    let event_1_team_2 = event_1.team2.to_lowercase();
    let event_2_team_1 = event_2.team1.to_lowercase();
    let event_2_team_2 = event_2.team2.to_lowercase();

    if event_1_team_1 == event_2_team_1 && event_1_team_2 == event_2_team_2 {
        return (true, true);
    }

    if event_1_team_2 == event_2_team_1 && event_1_team_1 == event_2_team_2 {
        return (true, false);
    }

    (false, false)
}
