mod fetch_daily_schedule;
mod fetch_overtime;
mod fetch_submission_types;
mod fetch_weekly_schedule;
mod fetch_work_cards;
mod submit_daily_schedule;
mod submit_overtime;
mod submit_weekly_schedule;
mod submit_work_card;

use anyhow::{Error, Result};
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use ergani::client::ErganiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let ergani_username = env::var("ERGANI_USERNAME")?;
    let ergani_password = env::var("ERGANI_PASSWORD")?;
    let ergani_base_url = env::var("ERGANI_BASE_URL")
        .unwrap_or("https://trialeservices.yeka.gr/WebServicesAPI/api".to_string());

    let client = ErganiClient::new(ergani_username, ergani_password, Some(ergani_base_url)).await?;

    // Submit a work card
    // let result = submit_work_card::submit_work_card(&client).await;

    // Submit an overtime
    // let result = submit_overtime::submit_overtime(&client).await;

    // Submit a daily schedule
    // let result = submit_daily_schedule::submit_daily_schedule(&client).await;

    // Submit a weekly schedule
    // let result = submit_weekly_schedule::submit_weekly_schedule(&client).await;

    // Fetch work cards
    // let result = fetch_work_cards::fetch_work_cards(&client).await;

    // Fetch the weekly schedule
    // let result = fetch_weekly_schedule::fetch_weekly_schedule(&client).await;

    // Fetch the daily schedule
    // let result = fetch_daily_schedule::fetch_daily_schedule(&client).await;

    // Fetch the overtime
    // let result = fetch_overtime::fetch_overtimes(&client).await;

    // Fetch submission types
    let result = fetch_submission_types::fetch_submission_types(&client).await;

    match result {
        Err(e) => pretty_print_error(e),
        _ => {}
    }

    Ok(())
}

fn pretty_print_error(error: Error) {
    let mut error_table = Table::new();
    error_table.set_header(vec![Cell::new("Error")
        .add_attribute(Attribute::Bold)
        .set_alignment(CellAlignment::Center)]);
    error_table.add_row(vec![Cell::new(&error.to_string())]);
    println!("{}", error_table);
}
