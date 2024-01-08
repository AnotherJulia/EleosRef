use std::collections::hash_map::Entry;
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use crate::models::availability::Availability;
use crate::models::matches::Match;
use crate::models::team::Team;

/// For a specific timeset (date, time) find the available teams from the availability hashmap
pub fn find_team_from_timeset(map: &Availability, value: (NaiveDate, NaiveTime)) -> Vec<Team> {
    map.iter()
        .filter_map(| (key, val)| if val.iter().any(|&x| x == value) { Some(key.clone()) } else { None })
        .collect::<Vec<Team>>()
}

/// Create a hashmap of type TeamMatches
/// for matches: only home-matches should be inserted
pub fn determine_team_hashmap(matches: &Vec<Match>, teams: &Vec<Team>) {
    let mut team_map: HashMap<Team, Vec<Match>> = HashMap::new();

    // Go over all the matches and add the match data to the corresponding key
    for m in matches {
        // Find the corresponding home team object from the string
        let home_team = match find_team_from_str(&m.home_team, &teams) {
            Ok(team) => team,
            Err(e) => { print!("Error: {}", e); continue }
        };

        // Add the match to the team_map (for the corresponding team)
        match team_map.entry(home_team) {
            Entry::Vacant(e) => { e.insert(vec![m.clone()]); },
            Entry::Occupied(mut e) => {e.get_mut().push(m.clone())}
        }
    }
}

pub fn find_team_from_str(name: &str, teams: &Vec<Team>) -> Result<Team, &'static str> {
   for t in teams {
       if t.name == name {
           return Ok(t.clone())
       }
   } 

    Err("No team found matching the given name")
}

pub fn determine_date_hashmap(matches: Match, teams: &Vec<Team>) {

}