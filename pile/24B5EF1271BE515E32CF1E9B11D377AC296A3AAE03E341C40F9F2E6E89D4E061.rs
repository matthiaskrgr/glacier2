// run-pass

const fn identity<const T: u32>() -> u32 { T }

#[derive(Eq, PartialEq, Debug)]
pub struct ConstU32<const U: u32>;

pub fn MAX() -> ConstU32<{ identity::<3>() }> {
  ConstU32::<{
    [0; std::mem::size_of::<*mut T>()];
    //~^ WARN cannot use constants which depend on generic parameters in types
    //~| WARN this was previously accepted by the compiler but is being phased out
}>
}

fn issue_73375<'a>() {
    [(); (|_: &'a i32| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}
