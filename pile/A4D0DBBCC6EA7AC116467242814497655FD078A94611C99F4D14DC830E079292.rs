struct A {
    (((Box<&'a mut List<T>>, Box<&'a mut List<T>>),),),
}

#[derive(Clone)]
struct A<'a> { x: &'a u32 }

fn foo(_: A) {}

fn bar(mut a: A) -> B {
    a.b = B;
    drop(a.0);
    a.b.clone(|val| *val >= min)
//~^ ERROR borrow of moved value
}

fn main() {}
