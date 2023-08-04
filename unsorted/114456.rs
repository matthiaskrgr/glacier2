#![feature(adt_const_params, lazy_type_alias)]

pub type Matrix = [usize; 1];
const EMPTY_MATRIX: Matrix = [0; 1];

pub struct Walk<const REMAINING: Matrix> {}

impl Walk<EMPTY_MATRIX> {
    pub const fn new() -> Self {
        Self {}
    }
}

fn main() {
    let _ = Walk::new();
}
