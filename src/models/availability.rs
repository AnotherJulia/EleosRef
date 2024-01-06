use std::collections::{HashMap, HashSet};
use chrono::{NaiveDate, NaiveTime};

pub type TeamName = String;
pub(crate) type Availability = HashMap<String, HashMap<NaiveDate, HashSet<NaiveTime>>>;
