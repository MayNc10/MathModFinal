use std::{io, fs::File};

use serde::Deserialize;

#[repr(transparent)]
#[derive(Deserialize, Debug)]
pub struct Team {
    pub team_number: i64,
}

pub fn write_teams(teams: &Vec<Team>, f: &File) {
    let mut wtr = csv::Writer::from_writer(f);
    wtr.write_record(&["Teams"]).unwrap();
    for team in teams {
        wtr.write_record(&[team.team_number.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}