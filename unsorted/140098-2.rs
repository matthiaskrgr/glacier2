macro_rules! a {
  ($abi : literal) => { extern $abi }
}
fn b() { a !(-2) }
