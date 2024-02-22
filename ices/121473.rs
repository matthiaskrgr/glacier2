use std::mem::{align_of, size_of};

#[repr(C)]

pub struct p0f_api_query {
    pub magic: u32,
    pub addr_type: str,
    pub addr: [u8; 16],
}

pub fn main() {
    assert_eq!(size_of::<p0f_api_query>(), 21);
}
