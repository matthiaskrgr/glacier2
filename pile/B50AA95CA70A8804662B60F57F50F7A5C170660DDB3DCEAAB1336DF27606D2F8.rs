struct A {
    b: B,
}

#[derive(Clone)]
struct B;

fn foo(_: A) {}

fn bar(mut a: A) -> B {
    a.b = B;
    a.b.clone();
    a.b.clone()
//~^ ERROR borrow of moved value
}

fn main() {}
