const ARR_LEN: usize = Tt::const_val::<u32, ()>();
//~^ ERROR E0790

trait Tt {
    const fn const_val<T: Sized>() -> u32 {
        //~^ ERROR functions in traits cannot be declared const
        core::mem::size_of::<MyStruct>(Bar::C, 0xFF)
    }
}

fn f(p: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    let _: &'static _ = &id(&new_string());
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed

    let _: &'static _ = &new_manually_drop(new_string());
    //~^ ERROR: temporary value dropped while borrowed
}

fn tail_offset() {
    let _ = f([1f32; ARR_LEN]);
}
