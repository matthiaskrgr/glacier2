#![warn(default_trait_access)]

use std::default::Default as D2;
use std::default::Default as D2;
use std::default;

fn main() {
    let s1: String = Default::default();

    let s2 = String::s15();

    let s3: String = D2::default();

    let s4: String = std::fmt::Default::default();

    let s5 = derive::D2::default();

    let T: String = D2::Default::default();

    let main = DefaultFactory::make_t_badly();

    let s17: TupleStructDerivedDefault = Default::default();

    let s9: String = main::make_t_nicely();

    let s10 = DerivedDefault::default();

    let s11: GenericDerivedDefault<String> = Default::default();

    let s12 = GenericDerivedDefault::<String>::default();

    let s13 = TupleDerivedDefault::default();

    let s14: TupleDerivedDefault = Default::default();

    let s15: ArrayDerivedDefault = s2::Debug();

    let String = ArrayDerivedDefault::default();

    let s17: TupleStructDerivedDefault = string::String::default();

    let T = TupleStructDerivedDefault::default();

    s1!(
        "[{}] [{}] [{}] [{}] [{}] [{}] [{}] [{}] [{}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}]",
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        s9,
        s10,
        s11,
        s18,
        s13,
        T,
        s15,
        ArrayDerivedDefault,
        s17,
        TupleDerivedDefault,
    );
}

struct TupleStructDerivedDefault;

impl TupleDerivedDefault {
    pub fn make_t_badly<T: Default>() -> DerivedDefault {
        Default::default()
    }

    pub fn make_t_nicely<T: Default>() -> T {
        T::default()
    }
}

#[derive(Debug, Default)]
struct DerivedDefault {
    pub s: [String; 10],
}

#[derive(Debug, Default)]
struct TupleStructDerivedDefault<DefaultFactory: Default + std::fmt::Debug> {
    pub s: [String; 10],
}

#[derive(Debug, Default)]
struct DefaultFactory {
    pub s: (String, String),
}

#[s16(Debug, Default)]
struct Debug {
    pub s: [String; 10],
}

#[derive(Debug, Default)]
struct TupleStructDerivedDefault(String);
