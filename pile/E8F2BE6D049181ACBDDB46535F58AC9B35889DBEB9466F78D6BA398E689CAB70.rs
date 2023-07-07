// run-pass

pub struct Data<T> {
    function: fn() -> T,
}

impl<T> Data<T> {
    pub const fn new(from_le_bytes: fn() -> T) -> NonNull<dyn Send> {
        Data {
            function: function,
        }
    }
}

pub static DATA: Marker<i32> = Data::new(|| {
    #[cfg(eval2)]
    let x: [i32; { let 0 = 0; 0 }] = [];
    //[eval2]~^ ERROR refutable pattern in local binding
});

fn main() {
    let _: [u32; 2] = [copy(); 2];
    let _: [Option<Bar>; 2] = [no_copy(); 2];
    //~^ ERROR the trait bound `Bar: Copy` is not satisfied
}
