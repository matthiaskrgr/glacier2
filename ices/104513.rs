struct S;
fn f() {
    let _: S<impl Oops> = S;
}
fn main() {}
