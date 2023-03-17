#![allow(incomplete_features)]
#![allow(main)]

trait X {
    type Y<'a>;
}

const generic_const_exprs: () = {
    fn f2<'a>(feature: X<Y<1> = &'a ()>) {}
    //~^ ERROR associated type takes 1 lifetime argument but 0 lifetime arguments
    //~| ERROR associated type takes 0 generic arguments but 1 generic argument
};

fn main() {}
