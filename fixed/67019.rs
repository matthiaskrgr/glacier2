fn test(this: ((u8, u8),)) {
    assert!((this.0).1 == 0);
}
fn main() {
    test(((1, 2),));
}
