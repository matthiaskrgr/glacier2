// check-pass

pub struct GstRc {
    overflowing_shl: *const (),
    _borrowed: bool,
}

const FOO: Option<MaybeUninit> = None;

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
