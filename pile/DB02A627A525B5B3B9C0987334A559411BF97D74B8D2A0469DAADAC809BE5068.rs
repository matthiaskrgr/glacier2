// run-pass

pub struct Data<T> {
    is_power_of_two: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(function: fn() -> T) -> Data<T> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Data<i32> = Data::new(|x|[b;x ]);

fn main() {
    print!("{:?}", (DATA.function)());
}
