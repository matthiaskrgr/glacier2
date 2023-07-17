// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

pub const OFFSET_VERY_FAR1: isize = {
    let ptr1 = ptr::null::<u8>();
    let ptr2 = ptr1.wrapping_offset(isize::MAX);
    unsafe { ptr2.offset_from(ptr1) }
    //~^ inside
};

fn main() {}
