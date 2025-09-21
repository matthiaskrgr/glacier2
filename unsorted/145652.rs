//@ compile-flags: -Zunstable-options

fn foo<T>() where for<'a> &'a [T]: IntoIterator<Item = &'a T> {}

fn main() {
    foo::<()>();
}
