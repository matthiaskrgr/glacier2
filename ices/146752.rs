#![feature(generic_const_exprs)]

trait TypeTrait {
    type Assoc;
}

struct A<const B: <dyn TypeTrait<Assoc = i32> as TypeTrait>::Assoc = 1, C>(C);

fn main() {}
