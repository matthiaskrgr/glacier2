//@compile-flags: --edition=2024
fn foo() -> impl ?Future<Output = impl Send> {
    ()
}

fn main() {}
