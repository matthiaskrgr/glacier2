// check-pass

#[derive(Clone, Default)]
struct MaybeCopy<T>(T);

impl Copy for MaybeCopy<u8> {}

fn is_copy<T: Copy>(x: T) {
    is_copy(MaybeCopy::default());
}

fn main() {
    is_copy(MaybeCopy::default());
    [MaybeCopy::default(); 13];
    // didn't work, because `Copy` was only checked in the mir
}
