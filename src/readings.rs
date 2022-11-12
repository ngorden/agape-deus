use std::error::Error;
use std::ops::Add;
use chrono::{Datelike, Duration, Local, Weekday};
use chrono::format::{DelayedFormat, StrftimeItems};
use regex::Regex;
use substring::Substring;
use crate::models::UniversalisResponse;


const REGULAR_EXPRESSION: &str =
    "(<(div|span) style=.\"(\\w|-|:| |%|;|#|\\.)+.\">)|</*\\w+>|(&#(\\d{3}|(821[0-6,8-9])|(822\\d));)+";

pub async fn get_readings(next_sunday: bool) -> Result<(), Box<dyn Error>> {
    let today = Local::now();

    let reading_string = format!("https://www.universalis.com/{}/jsonpmass.js", if next_sunday { get_next_sunday() } else { today.format("%Y%m%d") });
    let response = reqwest::get(reading_string).await?.text().await?;
    let reading_unformatted = response.substring("universalisCallback(".len(), response.len() - 3);
    let re = Regex::new(REGULAR_EXPRESSION).unwrap();

    let mut clean = re.replace_all(&reading_unformatted, &String::from("")).to_string();
    clean = Regex::new("&#8217;").unwrap().replace_all(&*clean, "'").to_string();
    let reading_formatted: UniversalisResponse = serde_json::from_str(&*clean)?;
    println!("{}", reading_formatted);

    Ok(())
}

fn get_next_sunday<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let today = Local::now();
    match today.weekday() {
        Weekday::Sun => today.format("%Y%m%d"),
        Weekday::Mon => today.date().add(Duration::days(6)).format("%Y%m%d"),
        Weekday::Tue => today.date().add(Duration::days(5)).format("%Y%m%d"),
        Weekday::Wed => today.date().add(Duration::days(4)).format("%Y%m%d"),
        Weekday::Thu => today.date().add(Duration::days(3)).format("%Y%m%d"),
        Weekday::Fri => today.date().add(Duration::days(2)).format("%Y%m%d"),
        Weekday::Sat => today.date().add(Duration::days(1)).format("%Y%m%d"),
    }
}