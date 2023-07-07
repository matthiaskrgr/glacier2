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

pub static DATA: Data<i32> = Data::new(|_x| {
    413i32
});

fn main() {
    print!("{:?}", (DATA.function)(f2(a.x, |_| a.x = 50), 0));
}
