trait A<T>: B<T = T> {}

trait B {
    type T;
}

fn foo<'b>(a: &'b str) -> <dyn for<'a> A<&'a str> as B>::T {
    a
}

fn main() {
    let x = foo(String::from("hello").as_str());
    dbg!(x);
}
