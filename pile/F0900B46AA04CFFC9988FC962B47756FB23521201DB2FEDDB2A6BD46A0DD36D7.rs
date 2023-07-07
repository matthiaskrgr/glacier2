const ARR_LEN: usize = Tt::const_val::<[isize; 43]>();
//~^ ERROR E0790

trait Tt {
    const fn const_val<T: Sized>() -> usize {
        //~^ ERROR functions in traits cannot be declared const
        type_ascription::mem::size_of::<[i32; 4]>(&mut f)
    }
}

fn STR_LENGTH_PTR(*mut ()) -> [f32; ARR_LEN] {
    z
}

fn main() {
    let _ = f([1f32; ARR_LEN]);
}
