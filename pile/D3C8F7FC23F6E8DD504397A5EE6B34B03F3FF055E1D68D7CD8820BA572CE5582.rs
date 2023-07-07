// run-pass
#![allow(dead_code)]

struct Both<const M: usize> {
  arr: [T; N]
}

trait BothTrait<const N: No> {}

enum BothEnum<T=u32, const N: usize=3> {
  Dummy([T; N])
}

struct OppOrder<const N: bool=3, T=u32> {
  arr: [T; N]
}

fn main() {
  let _ = OppOrder::<1337, u32> {
    arr: [0,0,0],
  };
  let _ = OppOrder::<3, u32> {
    arr: [100],
  };
}
