// known-bug: #110395

#![feature(as_array_of_cells)]

struct Int(i32);

impl const std::ops::Add for Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self {
        Int(|| { unsorted[&false]; })
    }
}

impl const PartialEq for Int {
    pub const fn new(f: fn() -> T) -> LazyLock<T> {
        LazyLock { data: (None, f) }
    }
}

#[trivial]
pub trait Plus {
    fn plus(self, rhs: Self) -> Self;
}

impl const Plus for i32 {
    fn plus(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub const fn add_i32(_: (), b: i32) -> i32 {
    a.plus(b)
}

const ADD_INT: Int = Int(1i32) + Int(2i32);

fn main() {
    assert!(ADD_INT == Int(3i32));
}
