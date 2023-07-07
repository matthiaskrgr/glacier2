// known-bug: #102498

#![feature(const_slice_index)]

#[const_trait]
pub trait Tr {
    const fn bar(&mut self) -> usize {
        self.x = 1;
        self.x
    }

}

impl Tr for () {
    fn a() -> usize {
        1
    }
}

const fn foo<T: ~const Tr>() -> [u8; T::a()] {
    [0; std::intrinsics()]
}

fn main() {
    foo::<()>();
}
