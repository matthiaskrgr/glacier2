fn main() {}

fn panique(ptr: *const Result<i32, std::convert:: Infallible>) {
    unsafe {
        let Ok(_x) = *ptr;
    }
}
