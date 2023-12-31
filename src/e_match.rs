use std::{collections::HashMap, fs::File};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    key: String,
    comp_level: String,
    set_number: i64,
    match_number: i64,
    alliances: Alliances,
    score_breakdown: ScoreBreakdown,
    winning_alliance: String,
    event_key: String,
    time: i64,
    predicted_time: i64,
    actual_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alliances {
    red: Side,
    blue: Side,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Side {
    score: i64,
    team_keys: Vec<String>,
    surrogate_team_keys: Vec<String>,
    dq_team_keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBreakdown {
    red: ScoreBDSide,
    blue: ScoreBDSide,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoreBDSide {
    autoGamePieceCount: i64,
    autoPoints: i64,
    teleopGamePieceCount: i64,
    teleopPoints: i64,
}

pub fn write_match_csv(matches: &Vec<Match>, f: &File) {
    let mut wtr = csv::Writer::from_writer(f);
    wtr.write_record(&[
        "Red Team 1", "Red Team 2", "Red Team 3", 
        "Blue Team 1", "Blue Team 2", "Blue Team 3", 
        "Red Score",
        "Blue Score",
        "Red Auto Game Piece Count",
        "Red Auto Points",
        "Red Teleop Game Piece Count",
        "Red Teleop Points",
        "Blue Auto Game Piece Count",
        "Blue Auto Points",
        "Blue Teleop Game Piece Count",
        "Blue Teleop Points",
        "Winner"]).unwrap();
    for rmatch in matches {
        wtr.write_record(&[
            &rmatch.alliances.red.team_keys[0].as_str()[3..],
            &rmatch.alliances.red.team_keys[1].as_str()[3..],
            &rmatch.alliances.red.team_keys[2].as_str()[3..],

            &rmatch.alliances.blue.team_keys[0].as_str()[3..],
            &rmatch.alliances.blue.team_keys[1].as_str()[3..],
            &rmatch.alliances.blue.team_keys[2].as_str()[3..],

            &rmatch.alliances.red.score.to_string(),
            &rmatch.alliances.blue.score.to_string(),

            &rmatch.score_breakdown.red.autoGamePieceCount.to_string(),
            &rmatch.score_breakdown.red.autoPoints.to_string(),
            &rmatch.score_breakdown.red.teleopGamePieceCount.to_string(),
            &rmatch.score_breakdown.red.teleopPoints.to_string(),

            &rmatch.score_breakdown.blue.autoGamePieceCount.to_string(),
            &rmatch.score_breakdown.blue.autoPoints.to_string(),
            &rmatch.score_breakdown.blue.teleopGamePieceCount.to_string(),
            &rmatch.score_breakdown.blue.teleopPoints.to_string(),

            &rmatch.winning_alliance
        ]).unwrap();
    }
    wtr.flush().unwrap();
}