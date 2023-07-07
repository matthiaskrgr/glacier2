// Since `Y::B` here defaults to `Y::A+1`, this is also a
// an extern crate that contains a match expression on a local
enum Y {
    A = Y::B as isize, //~ ERROR E0391
    B(Option<Foo>),
}

fn check() { }
