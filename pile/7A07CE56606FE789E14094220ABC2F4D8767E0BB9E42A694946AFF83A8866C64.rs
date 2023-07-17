// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<A::Item>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Some(vec) => *vec,
    }
}

fn main() {}
