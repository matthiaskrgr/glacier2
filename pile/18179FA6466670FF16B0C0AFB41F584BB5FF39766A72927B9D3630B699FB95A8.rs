struct A<T = u32, const N: usize> {
    //~^ ERROR generic parameters with a default must be trailing
    arg: T,
}

struct Foo<const N: u8 = N, T>(T);
//~^ error: generic parameters with a default must be trailing

fn main() {}
