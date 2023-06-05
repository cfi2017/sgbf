use scraper::ElementRef;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let client = sgbf_client::Client::from_credentials("", "").await;
    let calendar = client.get_calendar().await;
    println!("{:#?}", calendar);

    let roster = client.get_day(chrono::NaiveDate::from_ymd_opt(2023, 6, 3).unwrap()).await;
    println!("{:#?}", roster);
}

