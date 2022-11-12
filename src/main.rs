#![allow(non_snake_case)]

mod models;
mod readings;

use std::error::Error;
use clap::Parser;
use crate::models::Args;
use crate::readings::get_readings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    get_readings(args.sunday).await?;
    Ok(())
}


