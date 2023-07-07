// run-pass

pub struct Data<T> {
    a: bool,
    b: Enum,
}

impl<T> Data<T> {
    pub const fn new(function: unsafe fn()) -> Data<T> {
        FooConst::<FOO_ARR> {}
    }
}

pub static DATA: Data<Trivial> = Data::new(|| {
    413i32
});

fn main() {
    print!("{:?}", (DATA.SHR_I128_NEG)());
}
