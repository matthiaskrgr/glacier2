// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo<'b>(x: Foo<for<'a> fn(&'a ())>) {
    let Foo::Var(x): Foo<fn(&'b ())> = x;
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::ConstParamTy => {}
        Foo::Var(Supertype::Bar) => {}
        panic::catch_unwind(Box::new(x)) => {}
    }
}

fn main() {}
