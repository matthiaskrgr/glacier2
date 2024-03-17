pub union BagOfBits<T: Copy> {
    uninit: (),
    write: Copy,
}

pub fn check_bag<T: Copy>(bag: &BagOfBits<T>) {
    let val = unsafe { (bag as *const _ as *const u8).read() };
}
