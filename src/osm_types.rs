use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub enum OsmError {
    ParseError,
}

#[derive(Debug)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: Vec<Tag>,
    pub neighbors: HashSet<i64>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: -1,
            lat: f64::MIN,
            lon: f64::MIN,
            tags: vec![],
            neighbors: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub struct Way {
    pub id: i64,
    pub nodes: Vec<i64>,
    pub tags: Vec<Tag>,
}

impl Default for Way {
    fn default() -> Self {
        Self {
            id: -1,
            nodes: vec![],
            tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Relation {
    pub id: i64,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>,
}

impl Default for Relation {
    fn default() -> Self {
        Self {
            id: -1,
            members: vec![],
            tags: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn from_kv(k: String, v: String) -> Self {
        Tag {
            key: k,
            value: v,
        }
    }
}

#[derive(Debug)]
pub struct Member {
    pub classification: MemberClassification,
    pub reference: i64,
    pub role: String,
}

impl Default for Member {
    fn default() -> Self {
        Self {
            classification: MemberClassification::None,
            reference: -1,
            role: String::new(),
        }
    }
}

impl Member {
    pub fn from_values(classification: MemberClassification, reference: i64, role: String) -> Self {
        Self { classification, reference, role }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemberClassification {
    Node,
    Way,
    Relation,
    None
}

impl FromStr for MemberClassification {
    type Err = OsmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "node" => Ok(Self::Node),
            "way" => Ok(Self::Way),
            "relation" => Ok(Self::Relation),
            _ => Err(OsmError::ParseError),
        }
    }
}
