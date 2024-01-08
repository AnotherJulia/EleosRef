use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Team {
    pub name: String,
    pub members: u32,
    pub points: i32,
    pub turns_needed: i32,
}