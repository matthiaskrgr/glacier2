fn forever2() -> i32 {
  let x: i32 = loop {}; //~ ERROR mismatched types
  x
}

fn main() {}
