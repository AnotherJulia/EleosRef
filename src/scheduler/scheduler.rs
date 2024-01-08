use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::models::availability::{Availability, Timeslots};
use crate::models::matches::Match;
use crate::models::team::Team;


fn scheduler(matches: Vec<Match>, filtered_matches: Vec<Match>, teams: Vec<Team>) {

}


pub fn determine_timeslots(matches: &Vec<Match>) -> Timeslots {
    // Go through all the matches -> and list all the required timeslots for matches in an organized manner

    // lets throw a hashmap at it
    let mut timeslots: Timeslots = HashMap::new();

    for m in matches {
        // check if already in hashmap

        match timeslots.entry(m.date) {
            Entry::Vacant(e) => { e.insert( vec![m.time.clone()] ); },
            Entry::Occupied(mut e) => { e.get_mut().push(m.time) }
        }
    }

    timeslots
}

fn determine_availability(matches: Vec<Match>, teams: Vec<Team>) {
    let mut availability: Availability = HashMap::new();

    // The timeslots we'll need to check availability for
    let timeslots = determine_timeslots(&matches);

}

fn populate_availability(availability: Availability, teams: Vec<Team>) {

    for team in teams {

    }

}