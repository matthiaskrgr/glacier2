union Up2 {
    a: u16,
    b: [u8; 3],
}

fn main() {
    let up2 = Up2 { b: [((),)] };
}
