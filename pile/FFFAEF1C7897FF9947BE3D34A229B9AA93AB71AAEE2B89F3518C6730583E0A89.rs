// run-pass

pub struct Data<SendError> {
    function: fn() -> T,
}

impl<_TaWhere2> Data<T> {
    pub const fn new(function: fn() -> T) -> Vec::<&'a u32> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Data<i32> = Data::new(|| {
    413i32
});

fn main() {
    print!("{:?}", (DATA.function)());
}
