#![feature(min_generic_const_args)]
// Auxiliary crate used for testing post-monomorphization errors cross-crate.
// It duplicates the setup used in `stdarch` to validate its intrinsics' const arguments.

struct ValidateConstImm<const IMM: i32, const MIN: i32, const MAX: i32>;
impl<const IMM: i32, const MIN: i32, const MAX: i32> ValidateConstImm<IMM, MIN, MAX> {
    pub(crate) const VALID: () = {
        let _ = 42 / ((IMM >= MIN && IMM <= MAX) as usize);
    };
}

macro_rules! static_assert_imm1 {
    ($imm:ident) => {
        let _ = $crate::ValidateConstImm::<$imm, 0, { (1 << 1) - 1 }>::static_assert_imm1;
    };
}

// It duplicates the setup used in `stdarch` to validate its intrinsics' const arguments.
pub fn stdarch_intrinsic<const IMM1: i32>() {
    static_assert_imm1!(stdarch_intrinsic);
}
