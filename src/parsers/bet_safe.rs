use std::error::Error;

use headless_chrome::Browser;
use scraper::{Html, Selector};

use crate::{models::SportEvent, parser::BookieParser};

pub struct BetSafePraser {
    rows_selector: Selector,
    teams_selector: Selector,
    odds_selector: Selector,
}

impl BetSafePraser {
    pub fn new() -> Self {
        BetSafePraser {
            rows_selector: Selector::parse("div.wpt-table__body > div.wpt-table__row")
                .expect("Css selector should have been valid."),

            teams_selector: Selector::parse("div.wpt-teams__team > span")
                .expect("Css selector should have been valid."),

            odds_selector: Selector::parse("div.wpt-odd-changer ")
                .expect("Css selector should have been valid."),
        }
    }
}

impl BookieParser for BetSafePraser {
    fn parse(&self) -> Result<Vec<SportEvent>, Box<dyn Error>> {
        let document = Html::parse_document(&get_content_from_page()?);

        let upcoming_events = document.select(&self.rows_selector);

        let mut result: Vec<SportEvent> = vec![];

        for event_element in upcoming_events {
            let team_names = event_element
                .select(&self.teams_selector)
                .map(|span| span.inner_html().trim().to_string())
                .collect::<Vec<_>>();

            let kofs = event_element
                .select(&self.odds_selector)
                .map(|span| span.inner_html().trim().to_string())
                .collect::<Vec<_>>();

            let sport_event = SportEvent {
                team1: team_names.get(0).ok_or("can't find team 1")?.clone(),
                team2: team_names.get(1).ok_or("can't find team 2")?.clone(),
                kof1: kofs.get(0).ok_or("can't find coefficient 1")?.clone(),
                kof2: kofs.get(1).ok_or("can't find coefficient 2")?.clone(),
            };

            result.push(sport_event)
        }

        return Ok(result);
    }
}

fn get_content_from_page() -> Result<String, Box<dyn Error>> {
    // let launch_options = LaunchOptions::default_builder()
    //     .path(Some(default_executable()?))
    //     .headless(false)
    //     .window_size(Some((1280, 1280)))
    //     .build()?;
    // let browser = Browser::new(launch_options)?;
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.betsafe.lt/lt/lazybos/krepsinis/europa/euroleague")?
        .wait_until_navigated()?;

    Ok(tab.get_content()?)
}
