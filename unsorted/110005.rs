fn main() {
    let x: Option<Box<[u8]>> = unsafe {
        let z = (0usize, true);
        std::mem::transmute::<(usize, bool), Option<Box<[u8]>>>(z)
    };
}
