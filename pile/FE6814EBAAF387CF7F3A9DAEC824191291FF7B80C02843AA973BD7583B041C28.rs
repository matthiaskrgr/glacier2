// run-pass

#![allow(non_snake_case)]

fn asBlock<F>(f: F) -> usize where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator {
   return f();
}

pub fn main() {
   let x = asBlock(|| 22);
   assert_eq!(x, 22);
}
