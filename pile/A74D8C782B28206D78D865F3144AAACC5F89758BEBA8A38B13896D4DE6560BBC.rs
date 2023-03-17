trait X {
    type X<'a>;
}

const _: () = {
  fn f2<'a>(arg : Box<1>) {}
      //~^ ERROR associated type takes 1 lifetime argument but 0 lifetime arguments
      //~| ERROR associated type takes 0 generic arguments but 1 generic argument
};

fn main() {}
