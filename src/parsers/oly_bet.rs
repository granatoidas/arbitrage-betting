use std::error::Error;

use async_trait::async_trait;
use playwright::api::Page;
use scraper::{Html, Selector};

use crate::{models::SportEvent, parser::BookieParser};

pub struct OlyBetParser {
    rows_selector: Selector,
    teams_selector: Selector,
    odds_selector: Selector,

    page: Page,
}

impl OlyBetParser {
    pub fn new(page: Page) -> Self {
        OlyBetParser {
            rows_selector: Selector::parse("table.aic-hdp-row")
                .expect("Css selector should have been valid."),

            teams_selector: Selector::parse("div.aic-team-names > p")
                .expect("Css selector should have been valid."),

            odds_selector: Selector::parse("span").expect("Css selector should have been valid."),
            page,
        }
    }
}

#[async_trait]
impl BookieParser for OlyBetParser {
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

            let kof1 = match kofs.get(1) {
                Some(value) => {
                    if value.is_empty() {
                        continue;
                    }
                    value
                }
                None => continue,
            };
            let kof_draw = match kofs.get(2) {
                Some(value) => {
                    if value.is_empty() {
                        continue;
                    }
                    value
                }
                None => continue,
            };
            let kof2 = match kofs.get(3) {
                Some(value) => {
                    if value.is_empty() {
                        continue;
                    }
                    value
                }
                None => continue,
            };

            let sport_event = SportEvent {
                team1: team_names.get(0).ok_or("can't find team 1")?.clone(),
                team2: team_names.get(1).ok_or("can't find team 2")?.clone(),
                kof1: kof1.clone().parse()?,
                kof_draw: kof_draw.clone().parse()?,
                kof2: kof2.clone().parse()?,
                provider: String::from("olyBet"),
            };

            result.push(sport_event)
        }

        return Ok(result);
    }
}

impl OlyBetParser {
    async fn get_content_from_page(&self) -> Result<String, Box<dyn Error>> {
        self.page.goto_builder(
            "https://sportsbook-lt.orakulas.lt/#/sport/?type=0&sport=1&region=20001&competition=18286520&game=22723272",
        )
        // .wait_until(playwright::api::DocumentLoadState::DomContentLoaded)
        .goto()
        .await?;

        self.page
            .wait_for_selector_builder("table.aic-hdp-row")
            .wait_for_selector()
            .await?;
        return Ok(self.page.content().await?);
    }
}
