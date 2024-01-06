use std::collections::{HashMap, HashSet};
use chrono::{NaiveDate, NaiveTime};
use crate::models::matches::Match;
use crate::models::availability::Availability;

pub fn assign_referees(matches: Vec<Match>, filtered_matches: Vec<Match>) {

    let default_times = vec![NaiveTime::from_hms(9, 0, 0), NaiveTime::from_hms(10, 30, 0),
                             NaiveTime::from_hms(14, 0, 0), NaiveTime::from_hms(16, 0, 0)];

    // Lets determine the availability for each team
    let mut availability: Availability = HashMap::new();
    populate_availability(matches, &mut availability, default_times);
    availability.retain(|key, _| key.starts_with("Volley2B"));
    let rewritten_availability = rewrite_filter_hashmap(availability);

    println!("{:?}", rewritten_availability);

}


fn populate_availability(matches: Vec<Match>, availability: &mut Availability, default_times: Vec<NaiveTime>) {
    // Initialize availability for each team with all default times for each match date
    for m in &matches {
        if m.home_team.starts_with("Volley2B") {
            let team_availability = availability.entry(m.home_team.clone()).or_default();
            let date_availability = team_availability.entry(m.date).or_insert_with(HashSet::new);
            if date_availability.is_empty() {
                // If no times are set yet for this date, initialize with default times
                *date_availability = default_times.iter().cloned().collect();
            }
            // Remove the match time from the availability set for this date
            date_availability.remove(&m.time);
        }
    }
}

fn rewrite_filter_hashmap(availability: Availability) -> HashMap<String, Vec<(NaiveDate, NaiveTime)>> {
    let mut rewritten_availability: HashMap<String, Vec<(NaiveDate, NaiveTime)>> = HashMap::new();

    let prefixes = vec!["HS", "DS", "XR", "JA", "JB", "JC", "MA", "MB", "MC"];

    for (key, value) in availability {
        if let Some(new_key) = key.strip_prefix("Volley2B ") {
            // Convert new_key to String and replace prefixes
            let mut new_key_str = new_key.to_string();
            for prefix in &prefixes {
                let find = format!("{} ", prefix); // e.g., "DS "
                new_key_str = new_key_str.replace(&find, prefix); // Replace "DS " with "DS"
            }

            // Insert the updated key with its corresponding date-time values
            rewritten_availability.insert(new_key_str, value);
        }
    }

    rewritten_availability
}

pub fn is_team_available(team: &String, match_date: NaiveDate, match_time: NaiveTime, availability: &Availability) -> bool {
    if let Some(unavailable_datetimes) = availability.get(team) {
        !unavailable_datetimes.contains(&(match_date, match_time))
    } else {
        true // Team is available if no unavailable dates and times are recorded
    }
}



