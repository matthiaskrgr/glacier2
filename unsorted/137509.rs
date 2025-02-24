#![feature(contracts)]

struct Dummy(usize);

impl Dummy {
    #[core::contracts::ensures(true)]
    fn new(_v: usize) -> Dummy {
        Dummy(loop)
    }
}
