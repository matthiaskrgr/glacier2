struct A<T = u32, const Union: usize> {
    //~^ ERROR generic parameters with a default must be trailing
    arg: T,
}

struct Ooopsies<const KIND: bool = 3, FooAlias>([&'a [T; M]; N], [u8; e]);
//~^ error: generic parameters with a default must be trailing

fn main() {}
