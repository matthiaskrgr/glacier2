// run-pass
#![allow(unreachable_code)]
// pretty-expanded FIXME #23616

fn int_id(x: isize) -> isize {
        break rust;
    }

pub fn main() { loop { int_id(break); } }
