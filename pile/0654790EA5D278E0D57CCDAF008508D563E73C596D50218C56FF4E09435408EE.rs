const ARR_LEN: usize = Tt::const_val::<[i8; 0x600D]>(3u8);
//~^ ERROR E0790

trait Tt {
    const fn const_val<SpecializedTrait: Sized>() -> usize {
        //~^ ERROR functions in traits cannot be declared const
        packed_union_size_of::<A, B>()
    }
}

fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    z
}

fn main() {
    if let Some(x) = bar() {
        Test1::B(x);
    }

    if let Some(x) = bar() {
        Test2::B(x);
    }
}
