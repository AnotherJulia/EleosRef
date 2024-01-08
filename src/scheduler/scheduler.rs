use std::collections::HashMap;
use std::collections::hash_map::Entry;
use chrono::{NaiveDate, NaiveTime};
use crate::models::availability::{Availability, Timeslots};
use crate::models::matches::Match;
use crate::models::team::Team;
use crate::utils::hash::find_team_from_timeset;


pub fn create_schedule(matches: Vec<Match>, filtered_matches: Vec<Match>, teams: Vec<Team>) -> Vec<Match> {

    // Availability hashmap for all the teams
    let availability: Availability = determine_availability(matches, &filtered_matches, teams);

    let schedule: Vec<Match> = Vec::new();

    // Go over the matches that need to be divided across the teams -> check what teams are available
    // score the teams for "best fit" -> go along the list who is "eligible" to ref.
    // TODO: Balancing -> option is going over the teams again if over amount of req. turns (adjustment loop)

    // filtered matches are the home-only
    for m in filtered_matches {
        if m.first_ref == "" {
            let teams_available = find_available_teams(&m.date, &m.time, &availability);

            // Sort in order of "best"
            let sorted_teams = sort_by_score(teams_available, &filtered_matches);

        }
    }

    schedule
}

fn determine_availability(matches: Vec<Match>, filtered_matches: &Vec<Match>, teams: Vec<Team>) -> Availability {
    // TODO: Availability right now has the problem that 15 minutes in-to the game can possibly count as "available"

    // Hashmap with for each team, the times that they are available
    let mut availability: Availability = HashMap::new();

    // The timeslots we'll need to check availability for
    let timeslots = determine_timeslots(&matches);

    // We need to go over teams, and for each team, populate their map with all timeslots
    // and then remove the timeslots where they have their own match (only home matches)
    for t in teams {

        // populate the personal team availability vector (in this case we're using filtered matches since we only
        // want the teams who have "home" matches to referee)
        let team_availability: Vec<(NaiveDate, NaiveTime)> = populate_team_availability(&t, &filtered_matches, &timeslots);

        // After populating the teams_availability vector of tuples, we can insert it into our availability hashmap
        availability.insert(t, team_availability);
    }

    availability
}

fn determine_timeslots(matches: &Vec<Match>) -> Timeslots {
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


fn populate_team_availability (team: &Team, matches: &Vec<Match>, timeslots: &Timeslots) -> Vec<(NaiveDate, NaiveTime)> {
    let mut team_availability: Vec<(NaiveDate, NaiveTime)> = Vec::new();

    for m in matches {
        // Check if the team is playing at home
        if m.home_team == team.name  {
            // add that day to their availability -> remove the time they are playing

            // get the date timeslots from the timeslots hashmap
            let mut times = timeslots.get(&m.date).unwrap();

            // keep only the times where the team is free
            let times = times.into_iter().filter(|time| **time != m.time).cloned().collect::<Vec<NaiveTime>>();

            // add the (date, time) combo to the team_availability hashmap
            for t in times {
                team_availability.push((m.date, t));
            }


        }
    }

    team_availability
}


fn find_available_teams(date: &NaiveDate, time: &NaiveTime, availability: &Availability) -> Vec<Team> {

    // filter out the time and date from the hashmap -> find available teams
    let timeset: (NaiveDate, NaiveTime) = (*date, *time);
    // problem We have (NaiveDate, Vec<NaiveTime>)

    let teams_available: Vec<Team> = find_team_from_timeset(availability, timeset);
    println!("{:?}", teams_available);

    teams_available
}

fn sort_by_score(available_teams: Vec<Team>, matches: &Vec<Match>) -> Vec<Team> {
    let sorted: Vec<Team> = Vec::new();





    sorted
}

