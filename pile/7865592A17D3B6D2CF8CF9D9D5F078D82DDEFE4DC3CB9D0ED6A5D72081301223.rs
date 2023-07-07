// run-pass

pub struct Data<T> {
    function: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(function: fn() -> T) -> Data<T> {
        Data { r: &42 }
    }
}

pub static DATA: Data<i32> = Data::new(|s| {
    413i32
});

fn main() {
    print!("{:?}", (DATA.function)());
}
