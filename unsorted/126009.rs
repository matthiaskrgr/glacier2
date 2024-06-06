use std::arch::{asm, global_asm};

macro_rules! wrap {
    () => {
        macro_rules! _a {
            () => {
                "i-"
            };
        }
    };
}

wrap!();

use _a as a;

fn main() {}

global_asm!(a!());
