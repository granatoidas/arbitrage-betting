use scraper::{Html, Selector};

#[derive(Debug)]
struct SportEvent {
    team1: String,
    team2: String,
    kof1: String,
    kof2: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.topsport.lt/krepsinis/eurolyga")?.text()?;

    let document = Html::parse_document(&resp);

    let selector = Selector::parse("*[itemtype=\"http://schema.org/SportsEvent\"]").unwrap();

    let upcoming_events = document.select(&selector);

    for event_element in upcoming_events {
        let event_name = event_element
            .select(&Selector::parse("meta[itemprop=\"name\"]").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("content")
            .unwrap();

        let team_names = event_name.split(" - ").collect::<Vec<&str>>();

        let team1 = team_names.get(0).unwrap();
        let team2 = team_names.get(1).unwrap();        

        // println!("{}", event_name);

        let kofs = event_element
            .select(&Selector::parse("span.prelive-list-league-rate").unwrap())
            .map(|span| span.inner_html())
            .collect::<Vec<_>>();

        // println!("{:#?}", kofs);

        let sport_event = SportEvent {
            team1: team1.to_string(),
            team2: team2.to_string(),
            kof1: kofs[0].clone(),
            kof2: kofs[1].clone(),
        };        

        println!("{:#?}", sport_event);
    }

    let elements = document.select(&selector).collect::<Vec<_>>();

    println!("{}", elements.len());

    Ok(())
}
