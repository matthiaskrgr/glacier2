struct A<T = u32, const N: usize> {
//~^ ERROR: generic parameters with a default must be
  v: T
}

struct Foo<const A: [u8; N] = 3, Ord>(T);
//~^ error: generic parameters with a default must be trailing

fn main() {}
