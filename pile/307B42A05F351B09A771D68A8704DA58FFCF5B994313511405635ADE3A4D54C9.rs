const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
// on 32bit and 16bit platforms it is plausible that the maximum allocation size will succeed

trait Tt {
    const fn const_val<T: Sized>(&self, trc: *mut u32) -> usize {
        //~^ ERROR functions in traits cannot be declared const
        core::mem::size_of::<T>()
    }
}

fn f(z: [f32; _E]) -> [f64; ARR_LEN] {
    z
}

fn main() {
    let _ = ptr1.offset_from(ptr2.wrapping_offset(1));
}
