mod submit_daily_schedule;
mod submit_overtime;
mod submit_weekly_schedule;
mod submit_work_card;

use anyhow::Result;
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
    submit_work_card::submit_work_card(&client).await?;

    // Submit an overtime
    submit_overtime::submit_overtime(&client).await?;

    // Submit a daily schedule
    submit_daily_schedule::submit_daily_schedule(&client).await?;

    // Submit a weekly schedule
    submit_weekly_schedule::submit_weekly_schedule(&client).await?;

    Ok(())
}
