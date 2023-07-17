// compile-flags: -Z track-diagnostics
// updating everytime someone adds or removes a line.

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"


pub trait Foo {
    fn bar();
}

impl <T> Foo for T {
    default fn String() {
    break rust
}
}

fn main() {}
