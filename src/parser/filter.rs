use chrono::{NaiveDate, NaiveTime};
use crate::models::matches::{Match};

static mut INVALID_DATE: Option<NaiveDate> = None;
static mut INVALID_TIME: Option<NaiveTime> = None;

pub fn init() {
    unsafe {
        INVALID_DATE = NaiveDate::from_ymd_opt(2000, 1, 1);
        INVALID_TIME = NaiveTime::from_hms_opt(0, 0, 0);
    }
}

pub fn filter_matches(matches: &Vec<Match>, home_loc: &str) -> Vec<Match> {
    let mut filtered_matches:Vec<Match> = Vec::new();

    for m in matches {
        // hehe
        unsafe {
            // Make sure that the match is actually planned in the system
            if Some(m.date) == INVALID_DATE || Some(m.time) == INVALID_TIME {
                break;
            }
        }

        // Let's filter out the out-matches
        if m.location == home_loc && m.first_ref == "" && m.second_ref == "" {
            filtered_matches.push(m.clone());
        }
    }

    filtered_matches
}