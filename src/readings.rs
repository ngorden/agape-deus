use std::error::Error;
use chrono::Local;
use regex::Regex;
use substring::Substring;
use crate::models::UniversalisResponse;


const REGULAR_EXPRESSION: &str =
    "(<(div|span) style=.\"(\\w|-|:| |%|;|#|\\.)+.\">)|</*\\w+>|(&#(\\d{3}|(821[0-6,8-9])|(822\\d));)+";

pub async fn get_readings() -> Result<(), Box<dyn Error>> {
    let today = Local::now();

    let reading_string = format!("https://www.universalis.com/{}/jsonpmass.js", today.format("%Y%m%d"));
    let response = reqwest::get(reading_string).await?.text().await?;
    let reading_unformatted = response.substring("universalisCallback(".len(), response.len() - 3);
    let re = Regex::new(REGULAR_EXPRESSION).unwrap();

    let mut clean = re.replace_all(&reading_unformatted, &String::from("")).to_string();
    clean = Regex::new("&#8217;").unwrap().replace_all(&*clean, "'").to_string();
    let reading_formatted: UniversalisResponse = serde_json::from_str(&*clean)?;
    println!("{}", reading_formatted);

    Ok(())
}