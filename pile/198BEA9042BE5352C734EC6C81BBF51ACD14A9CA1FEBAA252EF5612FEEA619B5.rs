// error-pattern: created at
// error-pattern: created at

// Normalize the emitted location so this doesn't need
// updating everytime someone adds or removes a line.
// normalize-stderr-test ".rs:\d+:\d+" -> ".rs:LL:CC"


pub trait Foo {
    fn bar();
}

impl <T> Foo for T {
    fn bar();
}

fn _from() {
    break rust
}
