#![feature(trait_alias)]
trait FnOnceTrait<'a> = FnOnce(&'a ());

fn g() -> Box<dyn for<'a> FnOnceTrait<'a>> {
    todo!()
}

fn main() {
    g().foo();
}
