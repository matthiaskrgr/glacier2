// check-pass

// check-pass

mod x {
    pub use crate::y::*;
    pub use std::ops::Deref as _;
}

mod y {
    pub use crate::x::*;
    pub use std::ops::Deref as _;
}

pub fn main() {
    use x::*;
    (&0).deref();
}
