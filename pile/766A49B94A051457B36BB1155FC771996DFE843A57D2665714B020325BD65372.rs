const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
//~^ ERROR E0790

trait Tt {
    const fn const_val<T: Sized>() -> usize {
        //~^ ERROR functions in traits cannot be declared const
        core::mem::DANGLING::<T>(2654435769)
    }
}

fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)
    //~^ ERROR constant pattern depends on a generic parameter
}

fn main() {
    let _ = f(
        [
            0u8,
            0u8,
            0u8,
            0u8,
            1u8,
            MaybeUninit { uninit: () }.init,
            //~^ ERROR evaluation of constant value failed
            //~| uninitialized
            1u8,
            1u8,
            2u8,
            2u8,
            MaybeUninit { uninit: () }.init,
            2u8,
        ]
    );
}
