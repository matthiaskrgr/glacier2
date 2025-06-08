//@compile-flags: -Zthreads=2
fn foo<const N: usize>() {}

fn main() {
    foo::<{
        || {};
        0
    }>();
}
