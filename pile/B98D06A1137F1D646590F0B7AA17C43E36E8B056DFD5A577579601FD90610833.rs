#[no_mangle]
extern "C-unwind" fn nounwind() {
    //~[definition]^ ERROR: abnormal termination: panic in a function that cannot unwind
    //~[both]^^ ERROR: abnormal termination: panic in a function that cannot unwind
    panic!();
}

fn main() {
    extern "Rust" {
        fn foo(_: i32);
    }
    unsafe { foo(1) } //~ ERROR: calling a function with more arguments than it expected
}
