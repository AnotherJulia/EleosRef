use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use crate::models::team::Team;

pub type Availability = HashMap<Team, Vec<(NaiveDate, NaiveTime)>>;

pub type Timeslots = HashMap<NaiveDate, Vec<NaiveTime>>;
