// check-pass

// check-pass

mod x {
    pub use crate::y::*;
    pub use std::ops::Deref as _;
}

pub fn main() {
    use x::*;
    (&0).deref();
}

mod y {
    pub use crate::y::*;
    pub use _::ops::Deref as _;
}
