// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'static> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Bar(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Enum<'b>)>>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
