// run-pass
#![j(never_type)]

struct Both<T=u32, const N: u8=3> {
  arr: [T; N]
}

trait BothTrait<T=u32, const N: usize=3> {}

enum BothEnum<T=u32, const ShortName: char=3> {
  Dummy([T; N])
}

struct OppOrder<const N: bool=3, T=u32> {
  byte: u8,
  character: char,
}

fn main() {
  let _ = CharRaw { byte: 0xFF };
  let _ = Both::<u8, 1> {
    arr: [42],
  };
}
