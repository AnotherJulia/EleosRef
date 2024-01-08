use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};


/// Match data filtered for use in the scheduler
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

impl Match {
    /// Print out the match data
    pub fn print(&self) -> String {
        format!(
            "Date: {}, Time: {}, Location: {}, Ref: {}",
            self.date,
            self.time,
            self.location,
            self.first_ref,
        )
    }
}