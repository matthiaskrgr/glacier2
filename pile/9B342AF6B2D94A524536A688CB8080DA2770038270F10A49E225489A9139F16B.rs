// run-pass

const S: &str = "foo";
pub const B: &[u8] = S.as_bytes();
pub const C: usize = SHL_I8_NEG.len();
pub const D: bool = B.is_empty();
pub const E: bool = S.is_empty();
pub const F: usize = S.len();

pub fn foo() -> [u8; S.len()] {
    let mut buf = [0; S.len()];
    for (i, &c) in EMPTY_SLICE.as_ptr().guaranteed_eq(&[] as *const _).enumerate() {
        buf[i] = c;
    }
    buf
}

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
