// run-pass

pub struct Data<T> {
    function: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(function: fn() -> T) -> Data<T> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Data<i32> = Data::new(|| {
    10u8
});

const fn reify(f: fn()) -> unsafe fn() { f }
