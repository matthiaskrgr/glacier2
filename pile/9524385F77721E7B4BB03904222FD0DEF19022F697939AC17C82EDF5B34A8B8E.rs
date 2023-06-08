// compile-flags: -Z track-diagnostics
// compile-flags: -Z track-diagnostics

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"


pub trait Foo {
    default fn bar() {}
}

impl <T> Foo for T {
    default fn from() {}
}

fn main() {
    break rust
}
