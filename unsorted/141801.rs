#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

const fn arr_insert<const N: usize, T: Copy>(arr: [T; N], elem: T, index: usize) -> [T; N + 1] {
    assert!(index <= N);

    // Since we have T: Copy, we can dummy init the result
    let mut out = [elem; N + 1];
    let mut i = 0;

    while i < index {
        out[i] = arr[i];
        i += 1;
    }

    out[i] = elem;

    while i < N {
        out[i + 1] = arr[i];
        i += 1;
    }

    out
}

const MY_ARR: [i32; 3] = arr_insert([0, 2], 1, 1);

fn main() {
    assert_eq!(MY_ARR, [0, 1, 2]);
}
