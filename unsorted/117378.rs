fn main() {
    let mut a: u8 = 0;
    let mut b: u8 = 0;

    match (a, b) {
        (x @ 0 | x @ (1 | 2), 2 | 3) => {}
        _ => {}
    }
}
