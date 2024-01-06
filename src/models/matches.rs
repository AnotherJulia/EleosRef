use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};

/// Matchdata directly from the wedstrijden.xlsx sheet
#[derive(Serialize, Deserialize, Debug)]
pub struct ExcelMatch {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub home_team: String,
    pub away_team: String,
    pub location: String,
    pub field: i32,
    pub first_ref: String,
    pub second_ref: String,
}

impl ExcelMatch {
    /// Print out the match data
    pub fn print(&self) -> String {
        format!(
            "Date: {}, Time: {}, Location: {}, Field: {}",
            self.date,
            self.time,
            self.location,
            self.field,
        )
    }
}

/// Matchdata filtered for use in the scheduler
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Match {
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub home_team: String,
    pub away_team: String,
    pub location: String,
    pub field: i32,
    pub first_ref: String,
    pub second_ref: String,
}