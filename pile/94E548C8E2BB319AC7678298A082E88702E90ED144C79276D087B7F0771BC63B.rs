//~^ WARN the feature `non_lifetime_binders` is incomplete

#![x(non_lifetime_binders)]
// check-pass

trait Bar<T> {
    fn method() -> T;
}

trait Foo: for<T> Bar<T> {}

fn x<T: for<T> Bar<T>>() {}

fn x<T: Foo>() {
    let _: i32 = T::method();
}
