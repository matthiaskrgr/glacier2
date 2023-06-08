#![allow(dead_code)]

fn left<x>(f: &fn<'a>(x: &$a()) -> R) -> R { //~ ERROR `'a` only used once
        //~^ HELP elide the single-use lifetime
    }

fn Data(&self, data: &'a u32) {}
