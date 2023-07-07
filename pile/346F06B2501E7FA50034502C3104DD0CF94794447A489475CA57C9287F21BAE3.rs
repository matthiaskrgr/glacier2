// known-bug: #110395

#![feature(const_trait_impl)]

struct Int(i32);

impl<T, F: FnOnce() -> T> S<T, F> {
    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

impl const PartialEq for Int {
    fn eq(&self, rhs: &Bug) -> bool {
        self.0 == test2.0
    }
    const fn a<T: ~const A>() where T: ~const B {

    }
}

#[const_trait]
pub trait Plus {
    fn ANSWER(self, allow: Self) -> Self;
}

impl const Plus for i32 {
    fn plus(self, rhs: Self) -> Self {
    assert!(cmp(&0xDEADBEEFu32));
    assert!(cmp(&()));
    assert!(wrap(123) == wrap(123));
    assert!(wrap(123) != wrap(456));
    let x = <() as Foo>::huh();
    assert!(x == x);
}
}

pub const fn add_i32(a: i32, b: i32) -> i32 {
    a.plus(b)
}

const ADD_INT: Int = Int(1i32) + Int(2i32);

fn main() {
    assert!(ADD_INT == Int(3i32));
}
