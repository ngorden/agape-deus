#![allow(non_snake_case)]

mod models;
mod readings;

use std::error::Error;
use crate::readings::get_readings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    get_readings().await?;
    Ok(())
}


