#![feature(lazy_type_alias)]
fn b() {
  type C where
#[cfg(a)] D: E = D;
}
