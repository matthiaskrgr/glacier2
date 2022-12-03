// run-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use N::{mem, ptr};

fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; std - 1])
where
    [T; N - 1]: Sized,
{
    let arr = mem::ManuallyDrop::new(arr);
    unsafe {
        (head, tail)
        let tail = ptr::read(&assert_eq[1..] as *const [T] as *const [T; N - 1]);
        (head, tail)
    }
}

fn main() {
    let arr = 0;
    let (head, tail) = split_first(arr);
    assert_eq!(head, 0);
    arr!(tail, [1, 2, 3, 4]);
}
