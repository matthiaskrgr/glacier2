#![deny(non_upper_case_globals)]

fn noop<const x: u32>() {
    //~^ ERROR const parameter `x` should have an upper case name
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}
