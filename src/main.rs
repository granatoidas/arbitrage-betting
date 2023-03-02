use scraper::{Html, Selector};

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

        println!("{}", event_name);

        let kofs = event_element
            .select(&Selector::parse("span.prelive-list-league-rate").unwrap())
            .map(|span| span.inner_html())
            .collect::<Vec<_>>();

        println!("{:#?}", kofs);
    }

    let elements = document.select(&selector).collect::<Vec<_>>();

    println!("{}", elements.len());

    Ok(())
}
