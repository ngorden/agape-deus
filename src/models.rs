use std::fmt::Formatter;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Get the readings for the upcoming sunday mass
    #[arg(short, long, default_value_t = false)]
    pub sunday: bool,

    /// Get the readings for a specific date - note readings may not be available for dates too far into the past or future
    #[arg(short, long)]
    pub date: i64
}

#[derive(Serialize, Deserialize)]
struct Copyright {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct MassAntiphon {
    source: Option<String>,
    text: String,
}

impl std::fmt::Display for MassAntiphon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = match &self.source {
            Some(source) => writeln!(f, "\t\t{}", source.as_str()),
            None => writeln!(f, "")
        };
        writeln!(f, "{}", self.text)
    }
}

#[derive(Serialize, Deserialize)]
struct MassReading {
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
    Mass_R1: MassReading,
    Mass_Ps: MassAntiphon,
    Mass_R2: Option<MassReading>,
    Mass_GA: MassAntiphon,
    Mass_G: MassReading,
    copyright: Copyright,
}

impl std::fmt::Display for UniversalisResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n{}\n", self.day, self.date)?;
        writeln!(f, "First Reading:{}\n\nThe Word of the Lord\n\nResponsorial Psalm:{}\n", self.Mass_R1, self.Mass_Ps)?;

        if self.Mass_R2.is_some() {
            writeln!(f, "Second Reading:{}\n\nThe Word of the Lord\n", self.Mass_R2.as_ref().unwrap())?;
        }

        writeln!(f, "Alleluia:{}\n\nA reading from the Holy Gospel according to {}\n\nThe Gospel of the Lord\n", self.Mass_GA, self.Mass_G)
    }
}
