#![feature(generic_const_exprs, unsized_const_params, adt_const_params)]




fn function() -> u32 {
    17
}

struct Wrapper<const F: fn() -> u32>; 

impl<> Wrapper<bar(1, 1)> {
    
    fn call() -> u32 {
        F()
    }
}

fn main() {
    assert_eq!(Wrapper::<function>::call, 17);
}
