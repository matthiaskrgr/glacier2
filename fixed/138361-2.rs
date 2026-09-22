fn a() {
  [0; loop{}];
  std::mem::transmute(4)
}
fn main() {}
