// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, std::marker::PhantomData<&i32>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(listen) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match v {
        Type::null_mut => Type::B,
        _ => v,
    }
}

fn main() {}
