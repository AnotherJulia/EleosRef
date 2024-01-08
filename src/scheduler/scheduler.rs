use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use chrono::{NaiveDate, NaiveTime};
use crate::{MAX_GAP_THRESHOLD, WEIGHT_REMAINING_TURNS};
use crate::models::availability::{Availability, HashmapSet, PlayTimes, TeamSchedule, Timeslots};
use crate::models::matches::Match;
use crate::models::team::Team;
use crate::utils::hash::{find_team_from_timeset, determine_team_hashmap, find_team_from_str};


pub struct ScheduleResults {
    pub(crate) schedule: Vec<Match>,
    pub(crate) teams: Vec<Team>,
    pub(crate) team_infos: HashMap<String, SchedulingTeamInfo>, // Added field
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct SchedulingTeamInfo {
    pub team: Team,
    pub turns_completed: i32,
    pub turns_needed: i32,  // Added field
    pub last_play_time: Option<(NaiveDate, NaiveTime)>,
}

pub fn create_schedule(matches: Vec<Match>, filtered_matches: Vec<Match>, teams: Vec<Team>) -> ScheduleResults {
    let hashmapset = determine_availability(matches.clone(), &filtered_matches, teams.clone());

    let mut schedule: Vec<Match> = Vec::new();

    // Create a mutable map to track scheduling information for each team
    let mut team_infos = teams.into_iter().map(|team| {
        (team.name.clone(), SchedulingTeamInfo {
            team: team.clone(),
            turns_completed: 0,
            turns_needed: team.turns_needed,
            last_play_time: None,
        })
    }).collect::<HashMap<_, _>>();

    for mut m in filtered_matches {
        if m.first_ref == "" {
            let teams_available = find_available_teams(&m.date, &m.time, &hashmapset.availability);

            let mut best_score = None;
            let mut selected_team_name = None;

            for team in teams_available {
                let team_info = team_infos.get(&team.name).unwrap();

                // Check if the team still has refereeing turns left
                if team_info.turns_completed < team_info.turns_needed {
                    let score = calculate_score(&team_info.team, m.date, m.time, &matches);

                    if best_score.is_none() || score > best_score.unwrap() {
                        best_score = Some(score);
                        selected_team_name = Some(team.name.clone());
                    }
                }
            }

            if let Some(team_name) = selected_team_name {
                m.first_ref = team_name.clone();

                // Update the turns information for the selected team
                if let Some(team_info) = team_infos.get_mut(&team_name) {
                    team_info.turns_completed += 1;
                }
            }

            schedule.push(m);
        }
    }

    // Convert team_infos back to Vec<Team> if needed
    let updated_teams = team_infos.clone().into_iter().map(|(_name, info)| info.team).collect::<Vec<_>>();

    ScheduleResults {
        schedule,
        teams: updated_teams, // This might be the original or updated list of teams
        team_infos, // Now includes detailed refereeing information
    }
}


fn determine_availability(matches: Vec<Match>, filtered_matches: &Vec<Match>, teams: Vec<Team>) -> HashmapSet {
    // TODO: Availability right now has the problem that 15 minutes in-to the game can possibly count as "available"

    // Hashmap with for each team, the times that they are available

    let mut hashmapset = HashmapSet {
        availability: Default::default(),
        playtimes: Default::default(),
    };

    // The timeslots we'll need to check availability for
    let timeslots = determine_timeslots(&matches);

    // We need to go over teams, and for each team, populate their map with all timeslots
    // and then remove the timeslots where they have their own match (only home matches)
    for t in teams {

        // populate the personal team availability vector (in this case we're using filtered matches since we only
        // want the teams who have "home" matches to referee)
        let team_schedule: TeamSchedule = populate_team_availability(&t, &filtered_matches, &timeslots);

        // After populating the teams_availability vector of tuples, we can insert it into our availability hashmap
        // TODO: creating two instances --> instead use references
        hashmapset.availability.insert(t.clone(), team_schedule.available_times);
        hashmapset.playtimes.insert(t, team_schedule.play_times);
    }

    hashmapset
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


fn populate_team_availability(team: &Team, matches: &Vec<Match>, timeslots: &Timeslots) -> TeamSchedule {
    let mut play_times_set = HashSet::new();
    let mut team_schedule = TeamSchedule {
        available_times: Vec::new(),
        play_times: Vec::new(),
    };

    // First, collect all the play times
    for m in matches {
        if m.home_team == team.name {
            play_times_set.insert((m.date, m.time));
            team_schedule.play_times.push((m.date, m.time));
        }
    }

    // Then, iterate through the timeslots to determine availability
    for (&date, times) in timeslots {
        for &time in times {
            if !play_times_set.contains(&(date, time)) {
                team_schedule.available_times.push((date, time));
            }
        }
    }

    team_schedule
}


fn find_available_teams(date: &NaiveDate, time: &NaiveTime, availability: &Availability) -> Vec<Team> {

    // filter out the time and date from the hashmap -> find available teams
    let timeset: (NaiveDate, NaiveTime) = (*date, *time);
    // problem We have (NaiveDate, Vec<NaiveTime>)

    let teams_available: Vec<Team> = find_team_from_timeset(availability, timeset);

    teams_available
}

fn calculate_score(team: &Team, referee_match_date: NaiveDate, referee_match_time: NaiveTime, matches: &Vec<Match>) -> i32 {
    let mut score = 0;

    // Score based on turns remaining
    let turns_remaining = team.turns_needed;
    score += turns_remaining * WEIGHT_REMAINING_TURNS; // Weight for turns remaining

    // Find the team's match on the same day and calculate the time gap
    if let Some(team_match) = matches.iter().find(|m| m.date == referee_match_date && (m.home_team == team.name || m.away_team == team.name)) {
        let gap = (referee_match_time - team_match.time).num_minutes().abs();
        if gap <= MAX_GAP_THRESHOLD.into() {
            score += 10; // Within threshold, add points
        } else {
            score -= 10; // Exceeds threshold, subtract points
        }
    }

    score
}