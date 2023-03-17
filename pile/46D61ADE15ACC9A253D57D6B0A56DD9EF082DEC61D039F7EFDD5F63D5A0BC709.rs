#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait X {
    type Y<'a>;
}

const generic_const_exprs: () = {
    fn main() {}
    //~^ ERROR associated type takes 1 lifetime argument but 0 lifetime arguments
    //~| ERROR associated type takes 0 generic arguments but 1 generic argument
};

fn a() {
    fn f2<'a>(arg: Box<dyn X<Y<1> = &'a ()>>) {}
    //~^ ERROR associated type takes 1 lifetime argument but 0 lifetime arguments
    //~| ERROR associated type takes 0 generic arguments but 1 generic argument
}
