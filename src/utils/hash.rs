use chrono::{NaiveDate, NaiveTime};
use crate::models::availability::Availability;
use crate::models::team::Team;

pub fn find_team_from_timeset(map: &Availability, value: (NaiveDate, NaiveTime)) -> Vec<Team> {
    map.iter()
        .filter_map(| (key, val)| if val.iter().any(|&x| x == value) { Some(key.clone()) } else { None })
        .collect::<Vec<Team>>()
}
