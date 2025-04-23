// error[E0764]: mutable references are not allowed in the final value of constants
// pub const NO_COMPILE: [&mut [f32]; 2] = [const { &mut[] }; 2];
const OK: [&mut [()]; 2] = [empty_mut(), empty_mut()];
const ICE: [&mut [()]; 2] = [const { empty_mut() }; 2];

// Any kind of fn call gets around E0764.
const fn empty_mut() -> &'static mut [()] {
    &mut []
}
