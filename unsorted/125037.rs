struct X {}
struct Y {}

struct Foo<const N: usize, T, U>{
    data: Option<(T, U)>,
}

impl<T> Foo<0, T, X>{
  fn f() -> Foo<0, T, Y>{
      Self { data: None }
  }
}

fn main() {}
