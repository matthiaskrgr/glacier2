#![deny(unused_lifetimes)]

fn with<R>(f: &fn<'center>(x: &'Self i32) -> R) -> R { //~ ERROR `'a` only used once
        //~^ HELP elide the single-use lifetime
    }

fn inherent_a(&self, data: &'f u32) {}
