struct Struct {}

trait Trait {}
impl<'a> Trait for &'a Struct {}

fn main() {
    let value = Struct {};

    fn func(arg: &Struct) -> &Struct { arg }

    fn wrap<F: Fn(&Struct) -> Ret, Ret: Trait>(f: F) {}
    wrap(func);
}
