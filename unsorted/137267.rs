macro_rules! A {
    () => {};
}
fn B() -> [(); {
       A! {};
   }] {
}
fn main() {}
