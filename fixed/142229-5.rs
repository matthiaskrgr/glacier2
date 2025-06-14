use std::pin::{pin, Pin};
pub const C: () = ignore(pin!(1));
const fn ignore(_: Pin<&mut i32>) {}
