#![allow(incomplete_features)]
#![feature(ref_pat_everywhere)]

pub fn main() {
    if let Some(Some(&x)) = &Some(&Some(0)) {
        let _: u32 = x;
    }
}
