#![feature(async_closure, async_fn_traits)]

fn main() {}

pub fn test(test: &u64, temp: &u64) {
    let test = test.clone();
    async || {
        temp.abs_diff(12);
        test.clone();
    };
}
