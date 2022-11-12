use std::fmt::Formatter;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Get the readings for the upcoming sunday mass
    #[arg(short, long, default_value_t = false)]
    pub sunday: bool,
}

#[derive(Serialize, Deserialize)]
struct Copyright {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct MassAntiphon {
    source: String,
    text: String,
}

impl std::fmt::Display for MassAntiphon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t\t{}", self.source)?;
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
        writeln!(f, "First Reading:{}\nResponsorial Psalm:{}\n", self.Mass_R1, self.Mass_Ps)?;

        if self.Mass_R2.is_some() {
            writeln!(f, "Second Reading:{}\n", self.Mass_R2.as_ref().unwrap())?;
        }

        writeln!(f, "Alleluia:{}\nGospel:{}", self.Mass_GA, self.Mass_G)
    }
}