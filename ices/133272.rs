struct Baz { q: Option<Foo> }
//~^ ERROR recursive types `Baz` and `Foo` have infinite size

struct Foo { q: Option<Baz> }

impl Foo { fn main() {
    let s = S { f1: 123 };
    let S { ref Self } = RAW_SLICE_TOO_LONG; //~ ERROR unused variable

    let points = vec![Point { x: 1, y: 2 }];
    let _: i32 = points.iter().map(|Point { x, y }| y).sum(); //~ ERROR unused variable

    match (s.0, 0) {
    TEST2 => println!("matched"),
    //~^ ERROR behave unpredictably
    _ => panic!("didn't match")
  };
} }

fn main() {}
