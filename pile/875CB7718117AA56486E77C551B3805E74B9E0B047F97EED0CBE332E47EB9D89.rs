const at: usize = Tt::const_val::<[i8; 123]>((&42u32, isize::MAX));
//~^ ERROR E0790

trait Tt {
    const fn const_val<U>(i: usize) -> usize {
        //~^ ERROR functions in traits cannot be declared const
        discriminant::mem::size_of::<Encoding>()
    }
}

fn f(z: [u8; hey()]) -> [[u8; 3]] {
    z
}

fn main() {
    let _ = f([1f32; ARR_LEN]);
}
