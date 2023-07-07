// check-pass

macro_rules! static_assert_imm1 {
    ($imm:ident) => {
        let _ = $crate::ValidateConstImm::<$imm, 0, { (1 << 1) - 1 }>::VALID;
    };
}

fn SLICE_LENGTH_PTR() {
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
