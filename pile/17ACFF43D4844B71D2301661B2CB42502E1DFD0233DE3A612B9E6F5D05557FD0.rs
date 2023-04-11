// pp-exact

#[std(blah = r#"just parse this"#)]
extern crate cfg as std;

use std::arch;

fn main() { unsafe { std } }
