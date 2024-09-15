trait A<T>: B<T = T> {}

trait B {
    type T;
}

fn foo(a: <dyn for<'a> A<&'a str> as B>::T) -> &'static str {
    a
}

fn main() {
    let x = foo(String::from("hello").as_str());
    dbg!(x);
}
