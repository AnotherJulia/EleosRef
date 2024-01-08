// use std::collections::{HashMap, HashSet};
// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
// use rocket::form::validate::Contains;
// use crate::models::matches::Match;
// use crate::models::availability::Availability;
// use crate::models::team::Team;
//
// pub fn assign_referees(matches: &mut Vec<Match>, filtered_matches: Vec<Match>, teams: Vec<Team>) -> &mut Vec<Match> {
//     let default_times = vec![NaiveTime::from_hms_opt(11, 15, 0).unwrap(), NaiveTime::from_hms_opt(13, 00, 0).unwrap(),NaiveTime::from_hms_opt(15, 00, 0).unwrap(), NaiveTime::from_hms_opt(17, 00, 0).unwrap(), NaiveTime::from_hms_opt(19, 00, 0).unwrap(), NaiveTime::from_hms_opt(21, 00, 0).unwrap()];
//     let season_length = 6;
//     // Let's determine the availability for each team
//     let mut availability: Availability = HashMap::new();
//     populate_availability(matches, &mut availability, default_times);
//     availability.retain(|key, _| key.starts_with("Volley2B"));
//     let rewritten_availability = rewrite_filter_hashmap(availability.clone());
//     // Initialize a HashMap to track the number of turns each team has refereed
//     let mut team_referee_counts: HashMap<String, i32> = teams
//         .iter()
//         .map(|team| (team.name.clone(), 0))
//         .collect();
//     // With this availability, let's see if we can assign referees as best as we can
//     for (match_index, m) in matches.iter_mut().enumerate() {
//         let season_progress = match_index as f32 / season_length as f32;
//         let available_teams = find_available_teams(m.date, m.time, &availability);
//         if let (best_team) = select_best_team(&available_teams, &team_referee_counts, &teams, season_progress) {
//             // Assign this team as the referee for the match
//             m.first_ref = Some(best_team.clone()).unwrap();
//             *team_referee_counts.get_mut(&best_team).unwrap() += 1;
//         }
//     }
//
//     matches
// }
//
//
// fn populate_availability(matches: &mut Vec<Match>, availability: &mut Availability, default_times: Vec<NaiveTime>) {
//     // NOTE: ONLY AVAILABLE WHEN THE TEAM HAS A "HOME" MATCH (AWAY MATCHES NOT USED)
//
//     // Initialize availability for each team with all default times for each match date
//     for(match_index, m) in matches.iter_mut().enumerate() {
//         if m.home_team.starts_with("Volley2B") {
//             let team_availability = availability.entry(m.home_team.clone()).or_default();
//             let date_availability = team_availability.entry(m.date).or_insert_with(HashSet::new);
//             if date_availability.is_empty() {
//                 // If no times are set yet for this date, initialize with default times
//                 *date_availability = default_times.iter().cloned().collect();
//             }
//             // Remove the match time from the availability set for this date
//             date_availability.remove(&m.time);
//         }
//     }
// }
//
// fn rewrite_filter_hashmap(availability: 98Availability) -> HashMap<String, HashMap<NaiveDate, HashSet<NaiveTime>>> {
//     let mut rewritten_availability: HashMap<String, HashMap<NaiveDate, HashSet<NaiveTime>>> = HashMap::new();
//
//     let prefixes = vec!["HS", "DS", "XR", "JA", "JB", "JC", "MA", "MB", "MC"];
//
//     for (key, value) in availability {
//         if let Some(new_key) = key.strip_prefix("Volley2B ") {
//             // Convert new_key to String and replace prefixes
//             let mut new_key_str = new_key.to_string();
//             for prefix in &prefixes {
//                 let find = format!("{} ", prefix); // e.g., "DS "
//                 new_key_str = new_key_str.replace(&find, prefix); // Replace "DS " with "DS"
//             }
//
//             // Insert the updated key with its corresponding date-time values
//             rewritten_availability.insert(new_key_str, value);
//         }
//     }
//
//     rewritten_availability
// }
//
// fn is_team_available(
//     team: &str,
//     match_date: NaiveDate,
//     match_time: NaiveTime,
//     availability: &HashMap<String, HashMap<NaiveDate, HashSet<NaiveTime>>>
// ) -> bool {
//     if let Some(dates_available) = availability.get(team) {
//         if let Some(times_available) = dates_available.get(&match_date) {
//             times_available.contains(&match_time)
//         } else {
//             // If the team doesn't have an entry for this date, they are available
//             true
//         }
//     } else {
//         // If the team isn't in the availability map, they are available
//         true
//     }
// }
//
// fn find_available_teams(
//     date: NaiveDate,
//     time: NaiveTime,
//     availability: &HashMap<String, HashMap<NaiveDate, HashSet<NaiveTime>>>,
// ) -> Vec<String> {
//     availability.iter()
//         .filter(|(team, dates)| is_team_available(team, date, time, availability))
//         .map(|(team, _)| team.clone())
//         .collect()
// }
//
// fn select_best_team(
//     available_teams: &[String],
//     team_referee_counts: &HashMap<String, i32>,
//     teams: &[Team],
//     season_progress: f32,
// ) -> String {
//     available_teams.iter()
//         .filter_map(|team_name| {
//             let team = teams.iter().find(|t| &t.name == team_name)?;
//             let turns_done = team_referee_counts.get(team_name).unwrap_or(&0);
//             let expected_turns = (team.turns_needed as f32 * season_progress).round() as i32;
//             let turns_behind = expected_turns - turns_done;
//             if turns_behind > 0 { Some((team_name, turns_behind)) } else { None }
//         })
//         .max_by_key(|&(_, turns_behind)| turns_behind)
//         .map(|(team_name, _)| team_name.clone())
//         .unwrap_or_else(|| "".to_string()) // use unwrap_or_else(|| None) instead of unwrap()
// }
