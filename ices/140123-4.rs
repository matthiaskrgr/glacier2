const ICE: [&mut [(); 0]; 2] = [const { empty_mut() }; 2];

const fn empty_mut() -> &'static mut [(); 0] {
    &mut []
}
