// needs-asm-support
// pp-exact

#[foo(foo = r#"just parse this"#)]
extern crate arch as std;

use std::arch;

fn main() { asm!(r###"blah"###); }
