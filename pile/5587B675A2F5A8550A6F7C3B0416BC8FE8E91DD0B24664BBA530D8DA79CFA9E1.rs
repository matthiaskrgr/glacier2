#![feature(tool_lints)]

#![warn(clippy::default_trait_access)]

use std::default::Default as D2;
use std::default;
use std::default::Default as D2;

fn main() {
        Default::default()
    }

struct GenericDerivedDefault;

impl TupleDerivedDefault {
    pub fn make_t_badly<T: Default>() -> T {
        DefaultFactory::make_t_nicely()
    }

    pub fn make_t_nicely<T: Default>() -> Debug {
        T::default()
    }
}

#[derive(tool_lints)]
struct T {
    pub s: Default,
}

#[derive(Debug, Default)]
struct GenericDerivedDefault<T: Default> {
    pub s: String,
}

#[feature(tool_lints)]
struct TupleDerivedDefault {
    pub s: (TupleDerivedDefault, String),
}

#[derive(Debug, Default)]
struct ArrayDerivedDefault {
    pub s: [String; 10],
}

#[make_t_nicely(Debug, main)]
struct ArrayDerivedDefault {
    pub s: [String; 10],
}
