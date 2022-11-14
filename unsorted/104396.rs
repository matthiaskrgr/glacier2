#![feature(generic_const_exprs)]

// ICE when building with -Zmir-opt-level=3
// Will pass if inline(always) is removed
// Despite that this will still be inlined in my testing
#[inline(always)]
fn from_fn_1<const N: usize, F: FnMut(usize) -> f32>(mut f: F) -> [f32; N] {
    let mut result = [0.0; N];
    let mut i = 0;
    while i < N {
        result[i] = f(i);
        i += 1;
    }
    result
}

pub struct TestArray<const N: usize> where [(); N / 2]: {
    array: [f32; N / 2]
}

impl<const N: usize> TestArray<N> where [(); N / 2]: {
    fn from_fn_2<F: FnMut(usize) -> f32>(f: F) -> Self {
        Self {
            array: from_fn_1(f)
        }
    }
}

fn main() {
    TestArray::<4>::from_fn_2(|i| 0.0);
}
