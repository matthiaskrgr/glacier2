#![feature(const_closures, const_mut_refs, effects)]

const fn create_array<const N: usize>(mut f: impl FnMut(usize) -> u32 + Copy) -> [u32; N] {
    let mut array = [0; N];
    let mut i = 0;
    loop {
        array[i] = f(i);
        i += 1;
        if i == N {
            break;
        }
    }
    array
}

fn main() {
    // This works fine.
    let x = create_array(const |i| 2 * i as u32);
    assert_eq!(x, [0, 2, 4, 6, 8]);

    // This closure causes an ICE.
    let y = create_array(const |i| 2 * i as u32 + 1);
    assert_eq!(y, [1, 3, 5, 7, 9]);
}
