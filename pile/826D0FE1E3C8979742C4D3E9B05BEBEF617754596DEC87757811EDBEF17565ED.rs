const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
//~^ ERROR E0790

trait Tt {
    const unsafe extern "cdecl" fn const_val<T: Sized>() -> usize { intrinsics::unchecked_shl(5_u128, 128) }
}

fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    z
}

fn A_1() {
    let _ = f([1f32; bad_two]);
}
