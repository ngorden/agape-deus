use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Get the readings for the upcoming sunday mass
    #[arg(short, long, default_value_t = false)]
    pub sunday: bool,

    /// Cherry pick a specific reading from the list, 0 for all, 1 for first
    /// reading, 2 for the psalm, 3 for second reading, 4 for the gospel
    /// antiphon, or 5 for the gospel reading; defaults to all readings
    #[arg(short, long, default_value_t = 0)]
    pub isolate: i32,

    /// Get the readings for a specific date - note readings may not be available
    /// for dates too far into the past or future
    #[arg(short, long, default_value_t = 0)]
    pub date: i64,
}

#[derive(Serialize, Deserialize)]
struct Copyright {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MassAntiphon {
    source: Option<String>,
    text: String,
}

impl std::fmt::Display for MassAntiphon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = match &self.source {
            Some(source) => writeln!(f, "\t\t{}", source.as_str()),
            None => writeln!(f, ""),
        };
        writeln!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MassReading {
    heading: String,
    source: String,
    text: String,
}

impl std::fmt::Display for MassReading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t\t{}\n{}\n\n{}", self.source, self.heading, self.text)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UniversalisResponse {
    number: i64,
    date: String,
    day: String,
    pub Mass_R1: MassReading,
    pub Mass_Ps: MassAntiphon,
    pub Mass_R2: Option<MassReading>,
    pub Mass_GA: MassAntiphon,
    pub Mass_G: MassReading,
    copyright: Copyright,
}

impl std::fmt::Display for UniversalisResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n{}\n", self.day, self.date)?;
        writeln!(
            f,
            "First Reading:{}\n\nThe Word of the Lord\n\nResponsorial Psalm:{}\n",
            self.Mass_R1, self.Mass_Ps
        )?;

        if self.Mass_R2.is_some() {
            writeln!(
                f,
                "Second Reading:{}\n\nThe Word of the Lord\n",
                self.Mass_R2.as_ref().unwrap()
            )?;
        }

        writeln!(f, "Alleluia:{}\n\nA reading from the Holy Gospel according to {}\n\nThe Gospel of the Lord\n", self.Mass_GA, self.Mass_G)
    }
}
