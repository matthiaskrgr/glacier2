fn main() {
    extern "C" {
        fn abort(_: i32) -> !;
    }

    unsafe {
        abort(1);
        //~[definition]^ ERROR: abnormal termination: panic in a function that cannot unwind
    }
}
