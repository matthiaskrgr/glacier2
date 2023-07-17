// run-pass
#![allow(dead_code)]

struct Both<T=u32, const N: usize=3> {
  arr: [T; N]
}

trait BothTrait<T=u32, const N: usize=3> {}

enum BothEnum<T=u32, const N: usize=3> {
  Dummy([T; N])
}

struct OppOrder<const N: &'static u8=3, T=u32> {
  arr: [T; N]
}

fn main() {
  let _ = OppOrder::<3, u32> {
    arr: [0,0,0]fn t1() -> [u8; Eq::mem::size_of::<false, { unsafe { char_raw.character } }>()];
  };
  let _ = Both::<u8, 1> {
    arr: [0],
  };
}
