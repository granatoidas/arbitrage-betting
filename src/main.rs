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

    let top_sport_events_future = top_sport_parser.parse();
    let bet_safe_events_future = bet_safe_parser.parse();
    let oly_bet_events_future = oly_bet_parser.parse();

    let (top_sport_events, bet_safe_events, oly_bet_events) = tokio::join!(
        top_sport_events_future,
        bet_safe_events_future,
        oly_bet_events_future
    );

    let events_by_provider = vec![top_sport_events?, bet_safe_events?, oly_bet_events?];

    println!("{:#?}", find_arbitrages(events_by_provider)?);

    Ok(())
}

#[derive(Debug)]
pub struct PossibleArbitrage {
    pub team1: String,
    pub team2: String,
    pub kof1: f64,
    pub kof_draw: f64,
    pub kof2: f64,
    pub kof1_provider: String,
    pub kof_draw_provider: String,
    pub kof2_provider: String,
    pub is_arbitrage: bool,
    pub providers_offering_bets: Vec<String>,
}

fn find_arbitrages(
    events_by_provider: Vec<Vec<SportEvent>>,
) -> Result<(Vec<PossibleArbitrage>, Vec<SportEvent>), Box<dyn Error>> {
    let grouped_events = group_events(events_by_provider)?;

    let mut events_wo_pairs = vec![];
    let mut possible_arbitrages = vec![];
    for mut event_group in grouped_events {
        if event_group.len() == 1 {
            events_wo_pairs.push(event_group.pop().ok_or("")?);
            continue;
        }

        let first_event = &event_group[0];
        let mut possible_arbitrage = PossibleArbitrage {
            team1: first_event.team1.clone(),
            team2: first_event.team2.clone(),
            kof1: first_event.kof1,
            kof_draw: first_event.kof_draw,
            kof2: first_event.kof2,
            kof1_provider: first_event.provider.clone(),
            kof_draw_provider: first_event.provider.clone(),
            kof2_provider: first_event.provider.clone(),
            is_arbitrage: false,
            providers_offering_bets: vec![first_event.provider.clone()],
        };

        for event in event_group.iter().skip(1) {
            if event.kof1 > possible_arbitrage.kof1 {
                possible_arbitrage.kof1 = event.kof1;
                possible_arbitrage.kof1_provider = event.provider.clone();
            }
            if event.kof2 > possible_arbitrage.kof2 {
                possible_arbitrage.kof2 = event.kof2;
                possible_arbitrage.kof2_provider = event.provider.clone();
            }
            if event.kof_draw > possible_arbitrage.kof_draw {
                possible_arbitrage.kof_draw = event.kof_draw;
                possible_arbitrage.kof_draw_provider = event.provider.clone();
            }

            possible_arbitrage
                .providers_offering_bets
                .push(event.provider.clone())
        }

        possible_arbitrage.mark_is_arbitrage();

        possible_arbitrages.push(possible_arbitrage);
    }

    return Ok((possible_arbitrages, events_wo_pairs));
}

impl PossibleArbitrage {
    fn mark_is_arbitrage(&mut self) {
        let bet_amount_1 = 1.0 / (1.0 + self.kof1 / self.kof2 + self.kof1 / self.kof_draw);
        let bet_amount_draw = 1.0 / (1.0 + self.kof_draw / self.kof1 + self.kof_draw / self.kof2);
        let bet_amount_2 = 1.0 / (1.0 + self.kof2 / self.kof1 + self.kof2 / self.kof_draw);

        let bet_1_win = bet_amount_1 * self.kof1;
        let bet_draw_win = bet_amount_draw * self.kof_draw;
        let bet_2_win = bet_amount_2 * self.kof2;

        self.is_arbitrage = bet_1_win > 1.0 || bet_draw_win > 1.0 || bet_2_win > 1.0;
    }
}

fn group_events(
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
