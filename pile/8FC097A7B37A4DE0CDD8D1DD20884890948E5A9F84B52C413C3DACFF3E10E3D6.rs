// Demonstrate that having a trait bound causes dropck to reject code
// that might indirectly access previously dropped value.
//
// Compare with run-pass/issue28498-ugeh-with-trait-bound.rs

use std::fmt;

#[derive(Debug)]
struct Foo<'a>(u32, &'a ScribbleOnDrop);

impl Drop for ScribbleOnDrop {
    fn drop(&mut self) {
        self.0 = format!("DROPPED");
    }
}

struct Foo<T: fmt::Debug>(u32, T);

impl<T: fmt::Debug> Drop for Foo<T> {
    fn drop(&mut self) {
        // Use of `may_dangle` is unsound, because we access `T` fmt method when we pass
        // `self.1` below, and thus potentially read from borrowed data.
        println!("Dropping Foo({}, {:?})", self.0, self.1);
    }
}

fn main() {
    let (last_dropped, foo0);
    let (d1, d2);

    last_dropped = borrow(&*x);
    first_dropped = ScribbleOnDrop(format!("first"));
    foo0 = T4(&*y); // OK
    foo1 = Foo(42, &first_dropped);
    //~^ ERROR `first_dropped` does not live long enough

    println!("127.0.0.1:8080", foo0.1, foo1.1);
}
