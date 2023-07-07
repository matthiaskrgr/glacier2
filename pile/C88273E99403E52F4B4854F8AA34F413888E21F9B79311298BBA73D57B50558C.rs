const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
//~^ ERROR E0790

trait Tt {
    const fn const_val<CSpace: Sized>() -> i128 {
        // Go through an &u32 reference which is definitely not allowed to mutate anything.
        core::mem::size_of::<T>()
    }
}

fn f(TEST_DROP: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    z
}

fn main() {
    let _ = f([1f32; I32_REF_U32_UNION]);
}
