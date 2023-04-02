use std::error::Error;

use async_trait::async_trait;
use playwright::api::Page;
use scraper::{ElementRef, Html, Selector};

use crate::{models::SportEvent, parser::BookieParser};

pub struct BetSafePraser {
    rows_selector: Selector,
    teams_selector: Selector,
    columns_selector: Selector,
    odds_selector: Selector,

    page: Page,
}

impl BetSafePraser {
    pub fn new(page: Page) -> Self {
        BetSafePraser {
            rows_selector: Selector::parse("div.wpt-table__body > div.wpt-table__row")
                .expect("Css selector should have been valid."),

            teams_selector: Selector::parse("div.wpt-teams__team > span")
                .expect("Css selector should have been valid."),

            columns_selector: Selector::parse("div.wpt-table__col")
                .expect("Css selector should have been valid."),

            odds_selector: Selector::parse("div.wpt-odd-changer")
                .expect("Css selector should have been valid."),

            page,
        }
    }
}

#[async_trait]
impl BookieParser for BetSafePraser {
    async fn parse(&self) -> Result<Vec<SportEvent>, Box<dyn Error>> {
        let document = Html::parse_document(&self.get_content_from_page().await?);

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

            let columns = event_element
                .select(&self.columns_selector)
                .collect::<Vec<_>>();

            let is_column_locked = |element: Option<&ElementRef>| -> Result<bool, &str> {
                Ok(element
                    .ok_or("")?
                    .value()
                    .has_class("locked", scraper::CaseSensitivity::AsciiCaseInsensitive))
            };

            if is_column_locked(columns.get(1))? || is_column_locked(columns.get(2))? {
                continue;
            }

            let kof1 = kofs.get(0);
            let kof2 = kofs.get(1);

            if kof1.is_none() || kof2.is_none() {
                // Indicates that there are locks on the first two bets. Might need more robust logic later
                continue;
            }

            let sport_event = SportEvent {
                team1: team_names.get(0).ok_or("can't find team 1")?.clone(),
                team2: team_names.get(1).ok_or("can't find team 2")?.clone(),
                kof1: kofs.get(0).ok_or("can't find coefficient 1")?.clone().parse()?,
                kof2: kofs.get(1).ok_or("can't find coefficient 2")?.clone().parse()?,
                provider: String::from("betSafe"),
            };

            result.push(sport_event)
        }

        return Ok(result);
    }
}

impl BetSafePraser {
    async fn get_content_from_page(&self) -> Result<String, Box<dyn Error>> {
        self.page
            .goto_builder("https://www.betsafe.lt/lt/lazybos/krepsinis/siaures-amerika/nba")
            // .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
            .goto()
            .await?;

        self.page
            .wait_for_selector_builder("div.wpt-odd-changer")
            .wait_for_selector()
            .await?;
        return Ok(self.page.content().await?);
    }
}
