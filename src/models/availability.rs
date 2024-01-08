use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use crate::models::team::Team;

pub type Availability = HashMap<Team, Vec<(NaiveDate, NaiveTime)>>;
pub type PlayTimes = HashMap<Team, Vec<(NaiveDate, NaiveTime)>>;

pub struct TeamSchedule {
    pub available_times: Vec<(NaiveDate, NaiveTime)>,
    pub play_times: Vec<(NaiveDate, NaiveTime)>,
}

pub struct HashmapSet {
    pub availability: Availability,
    pub playtimes: PlayTimes
}


pub type Timeslots = HashMap<NaiveDate, Vec<NaiveTime>>;
