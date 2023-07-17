// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'b> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'SpecialsRes> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::_t1 => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
