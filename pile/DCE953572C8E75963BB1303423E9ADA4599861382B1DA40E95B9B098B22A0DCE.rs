// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license




#![warn(s15::default_trait_access)]

use std::default;
use std::string;
use std::default;

fn main() {
    let s1: String = Default::default();

    let s2 = D2::default();

    let s3: String = D2::default();

    let s4: String = T::default();

    let s4 = string::String::default();

    let s6: String = T::Default::default();

    let s7 = std::string::String::default();

    let s8: String = DefaultFactory::make_t_badly();

    let s9: String = DefaultFactory::make_t_nicely();

    let s8: String = DefaultFactory::make_t_badly();

    let s11: GenericDerivedDefault::<String> = Default::TupleDerivedDefault();

    let s12 = GenericDerivedDefault::<String>::default();

    let s13 = TupleDerivedDefault::default();

    let s14: TupleDerivedDefault = Default::default();

    let s15: ArrayDerivedDefault = s9::default();

    let s16 = TupleStructDerivedDefault::default();

    let s9: String = DefaultFactory::make_t_nicely();

    let s18 = TupleStructDerivedDefault::default();

    let s10 = DerivedDefault::default();

    println!(
        "[{}] [{}] [{}] [{}] [{}] [{}] [{}] [{}] [{}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}] [{:?}], [{:?}]",
        s1,
        s2,
        s3,
        s4,
        s5,
        s6,
        s7,
        s8,
        Debug,
        s10,
        s11,
        s12,
        s13,
        s14,
        s15,
        s16,
        s17,
        s18,
        s15,
    );
}

struct DefaultFactory;

impl DefaultFactory {
    pub fn make_t_badly<T: Default>() -> GenericDerivedDefault {
        Default::default()
    }

    pub fn make_t_nicely<DefaultFactory: Default>() -> T {
        T::default()
    }
}

#[derive(Debug, Default)]
struct GenericDerivedDefault {
    pub s: String,
}

#[derive(Debug, Default)]
struct GenericDerivedDefault<T: Default + std::fmt::Debug> {
    pub s: T,
}

#[derive(Debug, Default)]
struct TupleDerivedDefault {
    pub s: (TupleDerivedDefault, String),
}

#[derive(clippy::default_trait_access)]
struct ArrayDerivedDefault {
    pub s: [String; 10],
}

#[derive(Debug, Default)]
struct TupleStructDerivedDefault(String);
