use std::error::Error;

use async_trait::async_trait;

use scraper::{Html, Selector};

use crate::{models::SportEvent, parser::BookieParser};

use super::http_client_extensions::DefaultChromeHeaders;

pub struct TopSportParser {
    div_with_content_selector: Selector,
    meta_tag_with_name_selector: Selector,
    league_rate_span_selector: Selector,
}

impl TopSportParser {
    pub fn new() -> Self {
        TopSportParser {
            div_with_content_selector: Selector::parse(
                r#"*[itemtype="http://schema.org/SportsEvent"]"#,
            )
            .expect("Css selector should have been valid."),

            meta_tag_with_name_selector: Selector::parse(r#"meta[itemprop="name"]"#)
                .expect("Css selector should have been valid."),

            league_rate_span_selector: Selector::parse("span.prelive-list-league-rate")
                .expect("Css selector should have been valid."),
        }
    }
}

#[async_trait]
impl BookieParser for TopSportParser {
    async fn parse(&self) -> Result<Vec<SportEvent>, Box<dyn Error>> {
        let client = reqwest::Client::builder().gzip(true).brotli(true).build()?;

        let resp = client
            .get("https://www.topsport.lt/krepsinis/nba")
            .default_chrome_headers()
            .send()
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&resp);

        let upcoming_events = document.select(&self.div_with_content_selector);

        let mut result: Vec<SportEvent> = vec![];

        for event_element in upcoming_events {
            let event_name = event_element
                .select(&self.meta_tag_with_name_selector)
                .next()
                .ok_or("couldn't find meta tag")?
                .value()
                .attr("content")
                .ok_or("meta tag didn't have content atribute")?;

            let team_names = event_name.split(" - ").collect::<Vec<&str>>();

            let kofs = event_element
                .select(&self.league_rate_span_selector)
                .map(|span| span.inner_html())
                .collect::<Vec<_>>();

            let sport_event = SportEvent {
                team1: team_names.get(0).ok_or("can't find team 1")?.to_string(),
                team2: team_names.get(1).ok_or("can't find team 2")?.to_string(),
                kof1: kofs.get(0).ok_or("can't find coefficient 1")?.clone(),
                kof2: kofs.get(1).ok_or("can't find coefficient 2")?.clone(),
                provider: String::from("topSport"),
            };

            result.push(sport_event)
        }

        return Ok(result);
    }
}
