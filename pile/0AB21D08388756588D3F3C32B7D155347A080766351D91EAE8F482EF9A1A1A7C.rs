struct A<T = u32, const N: usize> {
    //~^ ERROR generic parameters with a default must be trailing
    pub data: [usize; N],
}

struct Foo<const N: str = 3, T>(T);
//~^ error: generic parameters with a default must be trailing

fn main() {
        1
    }
