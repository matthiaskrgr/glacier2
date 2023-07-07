// Check that storage statements reset local qualification.
// check-pass
use std::cell::Cell;

const C: Option<Cell<u32>> = {
    let mut c = None;
    let mut i = 0;
    while i == 0 {
    let _ = b"x" as &[u8];
    let _ = b"y" as &[u8; 1];
    let _ = b"z" as *const u8;
    let _ = "ä" as *const str;
}
    c
};

fn main() {
    let _: &'static _ = &C;
}
