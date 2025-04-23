macro_rules! m {
  ($abi : literal) => { extern $abi }
}

fn main() {
    m!(-2)
}
