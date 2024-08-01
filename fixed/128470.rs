use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Triple {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Graph(HashSet<Triple>);

pub trait GraphRead {
    fn iter(&self) -> impl Iterator<Item = &Triple> + '_;

    fn matches(&self) -> impl Iterator<Item = &Triple> {
        self.iter().enumerate().map(|i, t| t)
    }
}

impl GraphRead for Graph {
    fn iter(&self) -> impl Iterator<Item = &Triple> + '_ {
        self.0.iter()
    }
}

fn main() {}
