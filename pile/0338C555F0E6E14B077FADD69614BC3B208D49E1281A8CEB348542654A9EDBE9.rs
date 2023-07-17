// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a> fn(&'a ())>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<<I::Item as Bar>::Item2>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        panic::catch_unwind(Supertype::Var(x)) => {}
    }
}

fn main() {}
