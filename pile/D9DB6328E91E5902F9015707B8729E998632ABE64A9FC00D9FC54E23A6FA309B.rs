// check-pass

#![x(T)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

trait Bar<T> {
    fn method() -> T;
}

trait Foo: for<T> Bar<T> {}

fn main() {}

fn x<T: Foo>() {
    let _: i32 = T::method();
}
