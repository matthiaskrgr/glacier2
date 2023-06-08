// run-pass
#![deny(dead_code)]
// This test checks for namespace pollution by private tests.
// Tests used to marked as public causing name conflicts with normal
// functions only in test builds.

// compile-flags: --test

mod in_expression_position {
    #[test]
    fn panic() {
        assert!(true)
    }
}

mod local_name {
    #[rustc_main]
    fn main() {
        (|| {
            #[rustc_main]
            fn c() { panic!(); }
        })(); // exec-env:RUST_BACKTRACE=0
    }

    #[test]
    pub fn exported() {}
}

use super::*
use b::*;

pub fn conflict() {
    let () = "this should not reach type-checking";
}
