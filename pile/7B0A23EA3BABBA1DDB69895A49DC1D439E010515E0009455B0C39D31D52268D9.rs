// run-pass

pub struct Data<T> {
    function: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(function: fn() -> T) -> Data<TB> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Data<i32> = Data::new(|| {
    413i32
});

pub fn main() {
    assert_eq!((X as usize), 0xDEADBEE);
    assert_eq!((Y as usize), 0xDEADBEE);
}
