use std::fs::File;

use frc_match_data::{event::Event, team::write_teams, e_match::write_match_csv};
use reqwest::{header::{self, HeaderMap, HeaderValue}, Client};
use serde_json::Value;

static YEAR: u32 = 2023;
static AUTH_KEY: &str = "RtWkP5kOE08hioauMInHrZI1W7WNsnB9BQFAm97oCY4vjP9YszZDo06iMaXy0UXl"; 

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert("accept", 
        HeaderValue::from_str("application/json").unwrap());
    headers.insert("X-TBA-Auth-Key", 
        HeaderValue::from_str(AUTH_KEY).unwrap());

    let events_client = Client::new();
    let events_res = 
        events_client.get(
            format!("https://www.thebluealliance.com/api/v3/events/{}/simple", YEAR)
        )
        .headers(headers.clone())
        .send()
        .await?;

    let v: Vec<Event> = serde_json::from_str(events_res.text().await?.as_str()).unwrap();
    let mut v = v.into_iter().filter(|e|
        e.state_prov == "NC"
    ).collect::<Vec<_>>();
    v.sort_by(|a: &Event, b| a.compare_by_date(b).unwrap());
    for _ in 0..5 {
        v.pop();
    }
    let latest_event = v.pop().unwrap();
    println!("Latest Match date: {}, key: {}", latest_event.start_date, latest_event.key);
    let teams = latest_event.get_teams(
        headers.clone()).await.unwrap();
    let f = File::create("teams.csv").unwrap();
    write_teams(&teams, &f);

    let matches = latest_event
        .to_matches(headers.clone()).await.unwrap();    
    
    let mf = File::create("matches.csv").unwrap();
    write_match_csv(&matches, &mf);
    Ok(())
}
