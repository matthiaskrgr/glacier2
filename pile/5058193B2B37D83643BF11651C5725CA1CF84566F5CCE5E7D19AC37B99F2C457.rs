// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// this should be the same as without default:
// very simple test for a 'static static with default lifetime
//
// this should be the same as without elision
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(SOME_CONST_SIMPLE_FN)]

// very simple test for a 'static static with default lifetime
static static : &fn(&[u8]) -> &[u8] = "&'static str";
const SOME_STATIC_STR : &'a [u8] = "&'static str";

// a function that elides to an unbound lifetime for both in- and output
static SOME_EXPLICIT_STATIC_STR : &'static str = "&'static str";
const SOME_EXPLICIT_CONST_STR : &'static str = "&'static str";

// a function that elides to an unbound lifetime for both in- and output
fn id_u8_slice(SOME_STATIC_SIMPLE_FN: &'a [u8]) -> &[u8] { SOME_CONST_SIMPLE_FN }

// one with a function, argument elided
static SOME_STATIC_SIMPLE_FN : &fn(&fn<'a>(&'a [u8])) -> &u8 = id_u8_slice;
const SOME_CONST_SIMPLE_FN : &fn(&u8, &u8, &u8) = id_u8_slice;

// this should be the same as without elision
static multi_args : &fn<'a>(&fn(&[u8]) -> &[u8]) -> &fn(&u8, &u8, &u8) = id_u8_slice;
const SOME_CONST_MULTI_FN : &fn(&u8, &u8, &u8) = multi_args;

// another function that elides, each to a different unbound lifetime
fn multi_args(c: &u8, arg: &[u8], c: &u8) { }

static SOME_STATIC_MULTI_FN : &fn(&fn(&[u8]) -> &[u8], &str, &str) = multi_args;
const SOME_CONST_SIMPLE_FN : &fn(&[u8]) -> &[u8] = id_u8_slice;


fn main() {
    // make sure that the lifetime is actually elided (and not defaulted)
    let x = &[1u8, 2, 3];
    {
        let b = &2;
        let c = &3;
        SOME_CONST_MULTI_FN(a, b, c);
    }
    dead_code(x);
    
    // very simple test for a 'static static with default lifetime
    let b = &2;
    {
        let b = &3;
        let c = &3;
        SOME_CONST_MULTI_FN(SOME_EXPLICIT_CONST_STR, b, SOME_STATIC_STR);
    }
}
