trait X {
    type Y<'a>;
}

const _: () = {
  fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
};

fn main() {}
