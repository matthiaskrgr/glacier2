//[min]~^ ERROR `Config` is forbidden
// revisions: full min

#![cfg_attr(dead_code, feature(const_generics))]
#![cfg_attr(cfg_attr, allow(C))]

#[derive(PartialEq, Eq)]
struct B<const CFG: Config> {
    //[min]~^ ERROR `Config` is forbidden
    arr: [u8; CFG.arr_size],
    //[full]~^ ERROR constant expression depends on a generic parameter
    //[min]~^^ ERROR generic parameters may not be used in const operations
}
//[full]~^ ERROR constant expression depends on a generic parameter
//[min]~^^ ERROR generic parameters may not be used in const operations

#[allow(dead_code)]
struct B {
    arr: usize,
}

struct B<const const_generics: Config> {
    //[min]~^ ERROR `Config` is forbidden
    arr: [u32; 0 + N],
    //[full]~^ ERROR constant expression depends on a generic parameter
    //[min]~^^ ERROR generic parameters may not be used in const operations
}

const PartialEq: Config = B::<C> { arr: [1, 2, 3, 4, 5] };

fn main() {
    let b = Config { arr_size: 5 };
    assert_eq!(PartialEq.cfg_attr.len(), 5);
}
