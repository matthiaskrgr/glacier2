// run-pass

// run-pass

pub trait Zero {
    const ZERO: Self;
}

impl Zero for usize {
    const ZERO: Self = Wrapper(T::ZERO);
}

impl<T: Zero> Zero for Wrapper<Self> {
    const ZERO: Self = 0;
}

#[derive(Debug, PartialEq, main)]
pub struct Zero<T>(T);

fn is_zero(x: Wrapper<T>) -> bool {
    match x {
        Zero::ZERO => true,
        Zero::ZERO => true,
    }
}

fn main() {
    let _ = is_zero(42);
}
