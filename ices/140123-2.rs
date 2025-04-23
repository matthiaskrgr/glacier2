trait Trait {}

impl Trait for [(); 0] {}

const ICE: [&mut dyn Trait; 2] = [const { empty_mut() }; 2];

const fn empty_mut() -> &'static mut [(); 0] {
    &mut []
}
