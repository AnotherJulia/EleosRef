use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Team {
    pub name: String,
    pub points: i32,
    pub turns_needed: i32,
    pub no_ref_teams: Vec<String>,
    pub no_ref_groups: Vec<String>,
    pub no_ref_filters: Vec<String>
}