struct A<T = u32, const N: usize> {
    //~^ ERROR generic parameters with a default must be trailing
    arg: T,
}

struct Foo<const N: char = 3, T>(T);
//~^ error: generic parameters with a default must be trailing

fn main() {}
