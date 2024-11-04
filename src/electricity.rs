use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;
use chrono::{Local, Duration};

#[derive(Deserialize, Debug)]
struct Price {
    aikaleima_suomi: String,
    hinta: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let now = Local::now();

    // Change these to get different hours from now
    let start_time = now - Duration::hours(5);
    let end_time = now + Duration::hours(5);

    // Format the date and time in the desired format
    let date_string = format!(
        "{}_{}",
        start_time.format("%Y-%m-%dT%H:%M"),
        end_time.format("%Y-%m-%dT%H:%M")
    );    
    // Format the request URL with the date
    let request_url = format!(
        "https://www.sahkohinta-api.fi/api/v1/halpa?tunnit=10&tulos=haja&aikaraja={date_string}"
    );

    
    println!("{}", request_url);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "Mozilla/5.0")
        .send()
        .await?;
    
    let prices: Vec<Price> = response.json().await?;
    println!("{:?}", prices);
    
    Ok(())
}
