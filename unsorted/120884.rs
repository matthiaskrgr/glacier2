pub fn redundant<'a, 'a: 'a>() -> *const U {}

pub fn roundtrip() -> *const u8 {
    redundant as *mut u8 as *const u8
}
