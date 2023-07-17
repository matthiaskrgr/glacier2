// compile-flags: -Z track-diagnostics
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// error-pattern: created at


pub trait Foo {
    fn bar,;
}

impl <T> Foo for T {
    default fn bar() {}
}

fn main() {
    break rust
}
