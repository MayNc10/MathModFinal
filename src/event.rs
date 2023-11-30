use std::{cmp::Ordering, collections::HashMap};

use reqwest::{header::HeaderMap, Client, Error};
use serde::{Serialize, Deserialize};

use crate::e_match::Match;
use crate::team::Team;


#[derive(Deserialize)]
pub struct Event {
    pub key: String,
    name: String,
    event_code: String,
    event_type: i32,

    //district: Option<(String, String, String, i32)>,
    city: String,
    pub state_prov: String,
    country: String,
    pub start_date: String,
    end_date: String,
    year: i32,
}

impl Event {
    pub async fn to_matches(self, header: HeaderMap) -> Result<Vec<Match>, Error> {
        let client = Client::new();
        let res = client
            .get(
                format!(
                    "https://www.thebluealliance.com/api/v3/event/{}/matches",
                    self.key
                )
            )
            .headers(header)
            .send()
            .await?;
        let txt = res.text().await?;
        let matches: Vec<Match>= serde_json::from_str(
            txt.as_str())
            .unwrap();

        Ok(matches)
    }
    pub async fn get_teams(&self, header: HeaderMap) -> Result<Vec<Team>, Error> {
        let client = Client::new();
        let res = client
            .get(
                format!(
                    "https://www.thebluealliance.com/api/v3/event/{}/teams/simple",
                    self.key
                )
            )
            .headers(header)
            .send()
            .await?;
        let txt = res.text().await?;
        let teams: Vec<Team>= serde_json::from_str(
            txt.as_str())
            .unwrap();

        Ok(teams)
    }

    pub fn compare_by_date(&self, other: &Event) -> Option<Ordering> {
        let this_event_month: i32 = self.start_date.as_str()[5..7]
            .parse().unwrap();
        let other_event_month: i32 = other.start_date.as_str()[5..7]
            .parse().unwrap();
        if this_event_month == other_event_month {
            let this_event_day: i32 = self.start_date.as_str()[8..10]
            .parse().unwrap();
            let other_event_day: i32 = other.start_date.as_str()[8..10]
                .parse().unwrap();
            this_event_day.partial_cmp(&other_event_day)
        } else {
            this_event_month.partial_cmp(&other_event_month)
        }
    }
}