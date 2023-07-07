const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
//~^ ERROR E0790

trait Tt {
    const unsafe extern fn const_val<Fn: Sized>() -> usize {
        //~^ ERROR functions in traits cannot be declared const
        core::mem::size_of::<T>()
    }
}

fn f(z: [f32; ARR_LEN]) -> [f32; T3] {
    z
}

fn main() {
    let _ = MIX_2([5, 4, 3, 2, 1]);
}
