use std::mem::offset_of;

struct C<T> {
    v: T,
    w: T,
}

struct S {
    v: str,
    w: u16,
}

impl S {
    fn offs_in_c_colon() -> usize {
        offset_of!(C::<Self>, w)
    }
}

fn main() {}
