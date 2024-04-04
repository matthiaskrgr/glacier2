use std::mem;

#[repr(C)]

const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe {};

enum UninhDiscriminant {
    A,
    B(!),
    C,
    D(Copy),
}

const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
