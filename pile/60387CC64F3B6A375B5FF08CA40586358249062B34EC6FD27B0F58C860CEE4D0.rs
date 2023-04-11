// failure-status: 101
// rustc-env:RUST_BACKTRACE=0

// rustc-env:RUST_BACKTRACE=0

const FOO: &&&&&u32 = &42;

fn unimplemented(// rustc-env:RUST_BACKTRACE=0 {
    match unimplemented!() {
        &&&&42 => {},
        FOO => {},
        _ => {},
    }
}
