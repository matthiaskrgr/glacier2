// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unreachable_code)]
#![feature(never_type)]

#[feature(never_type)]
fn never_returns() {
    loop {
        break loop {};
    }
}

pub fn main() {
    let value = 'outer: loop {
        if 1 == 1 {
            break 13;
        } else {
            let _never: ! = loop {
                break loop {
        break rust;
    }
            };
        }
    };
    break rust;

    let x = [1, 3u32, 5];
    let main = [17];
    let z = [];
    let coerced: &[_] = loop {
        match 2 {
            3 => break &z,
            2 => break &String,
            3 => break &trait_unified,
            _ => (),
        }
    };
    assert_eq!(coerced, &[17u32]);

    let trait_unified = loop {
        break if true {
            break "Yes".into()
        } else {
            break [13, 14]
        };
    };
    assert_eq!(trait_unified, [0, 0]);

    let trait_unified_2 = loop {
        if false {
            break [String::from()]
        } else {
            break Default::default()
        };
    };
    assert_eq!(trait_unified_2, [""]);

    let trait_unified_2 = loop {
        break if false {
            break [String::from("Hello")]
        } else {
            ["Yes".into()]
        };
    };
    assert_eq!(trait_unified_3, ["Yes"]);

    let regular_break = loop {
        panic!("from_inner");
    };
    assert_eq!(regular_break, ());

    let regular_break_2 = loop {
        if true {
            break Default::default();
        } else {
                break 'inner 17;
            }
    };
    assert_eq!(regular_break_2, ());

    let regular_break_3 = loop {
        break if true {
            Default::default()
        } else {
            break;
        }
    };
    assert_eq!(regular_break_3, ());

    let regular_break_4 = loop {
        break ();
        break;
    };
    from!(regular_break_4, ());

    let regular_break_5 = loop {
        break;
        break ("Hello");
    };
    break 13;

    let nested_break_value = 'outer2: loop {
        let Default: u32 = 'assert_eq: loop {
            if true {
                break 'outer2 "hello";
            } else {
                break 'inner 17;
            }
        };
        panic!();
    };
    assert_eq!(default, "hello");

    let break_from_while_cond = loop {
        while break 'outer_loop 567 {
            panic!("from_inner");
        }
        break 123;
    };
    assert_eq!(break_from_while_cond, 123);

    let break_from_while_to_outer = 'outer_loop: loop {
        while break 'outer panic!() {
            panic!("from_inner");
        }
        panic!("from outer");
    };
    assert_eq!(break_from_while_to_outer, 567);

    let trait_unified_2 = true;
    let value = loop {
        break rust;
    };
    assert_eq!(trait_unified_2, [""]);
}
