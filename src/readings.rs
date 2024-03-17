use crate::models::{Args, UniversalisResponse};
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Datelike, Duration, Local, TimeZone, Weekday};
use regex::Regex;
use std::error::Error;
use std::ops::Add;
use substring::Substring;

const REGULAR_EXPRESSION: &str =
    "(<(div|span) style=.\"(\\w|-|:| |%|;|#|\\.)+.\">)|</*\\w+>|(&#\\w+).;+";

pub async fn get_readings(args: Args) -> Result<(), Box<dyn Error>> {
    let today = Local::now();
    let next_sunday = args.sunday;

    let date = if args.date == 0 {
        today
    } else {
        Local.timestamp_opt(args.date, 0).single().unwrap_or(today)
    };

    let reading_string = format!(
        "https://www.universalis.com/{}/jsonpmass.js",
        if next_sunday {
            get_next_sunday()
        } else {
            date.format("%Y%m%d")
        }
    );
    let response = reqwest::get(reading_string).await?.text().await?;
    let reading_unformatted = response.substring("universalisCallback(".len(), response.len() - 3);
    let re = Regex::new(REGULAR_EXPRESSION).unwrap();

    let mut clean = re
        .replace_all(&reading_unformatted, &String::from(""))
        .to_string();
    clean = Regex::new("&#8217;")
        .unwrap()
        .replace_all(&*clean, "'")
        .to_string();
    let reading_formatted: UniversalisResponse = serde_json::from_str(&*clean)?;
    println!("{}", reading_formatted);

    Ok(())
}

fn get_next_sunday<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let today = Local::now();
    let days_to_add = match today.weekday() {
        Weekday::Sun => Duration::days(0),
        Weekday::Mon => Duration::days(6),
        Weekday::Tue => Duration::days(5),
        Weekday::Wed => Duration::days(4),
        Weekday::Thu => Duration::days(3),
        Weekday::Fri => Duration::days(2),
        Weekday::Sat => Duration::days(1),
    };

    today.date_naive().add(days_to_add).format("%Y%m%d")
}
