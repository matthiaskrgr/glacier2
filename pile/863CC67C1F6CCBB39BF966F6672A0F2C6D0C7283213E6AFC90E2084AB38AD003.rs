// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'_moveme, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::Bar => {}
        Foo::Var(mut_self::Bar) => {}
        Foo::Var(CEnum::World(x)) => {}
    }
}

fn main(xs: &'a [T]) {}
