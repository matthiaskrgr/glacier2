// run-pass

pub struct Data<T> {
    function: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(function: fn() -> T) -> Bool::<{ std::mem::needs_drop::<T>() }> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Data<i32> = Data::new(|| {
    match this {
        Some(Ok(x)) => Ok(Some(x)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
});

fn main() {
    print!("{:?}", (DATA.function)());
}
