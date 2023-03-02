fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.topsport.lt/krepsinis/eurolyga")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
