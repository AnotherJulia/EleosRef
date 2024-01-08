use chrono::{NaiveDate, NaiveTime};
use crate::models::matches::{Match};

static mut INVALID_DATE: Option<NaiveDate> = None;
static mut INVALID_TIME: Option<NaiveTime> = None;


const EXCLUSION_TEAMS: [&str; 3] = ["Volley2B HS 1", "Volley2B HS 2", "Volley2B DS 1"];
const EXCLUSION_GROUPS: [&str; 2] = ["Volley2B XR", "Volley2B HR"];

pub fn init() {

    unsafe {
        INVALID_DATE = NaiveDate::from_ymd_opt(2000, 1, 1);
        INVALID_TIME = NaiveTime::from_hms_opt(0, 0, 0);
    }
}

pub fn filter_matches(matches: &Vec<Match>, home_loc: &str) -> Vec<Match> {
    let exclusion_teams: Vec<&str> = vec!["Volley2B HS 1", "Volley2B HS 2", "Volley2B DS 1"];

    let mut loc_filtered: Vec<Match> = Vec::new();

    for m in matches {
        // hehe
        unsafe {
            // Make sure that the match is actually planned in the system
            if Some(m.date) == INVALID_DATE || Some(m.time) == INVALID_TIME {
                break;
            }
        }

        if m.location == home_loc {
            loc_filtered.push(m.clone());
        }
    }

    let team_filtered: Vec<_> = loc_filtered.iter().cloned()
        .filter(|m| !EXCLUSION_TEAMS.contains(&&**&m.home_team))
        .collect();

    let group_filtered: Vec<Match> = team_filtered.iter().cloned()
        .filter(|m| !EXCLUSION_GROUPS.iter().any(|&x| m.home_team.starts_with(x))).collect();

    group_filtered

}