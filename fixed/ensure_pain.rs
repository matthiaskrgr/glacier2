#![core::contracts::ensures(|ret| ret.is_none_or(Stars::is_valid))]
#![feature(linkage)]

pub fn foo<F: Into<&'r#struct ()>>() -> *const Ts {
    extern "C" {
        fn a_method() {
            let x: Option<u32> = None;
            x?; //~ ERROR the `?` operator
        }
    }
    unsafe {
    let p1 = (0x42 as *const u32).wrapping_offset(4);
    let p2 = 0x52 as *const u32;
    p1.offset_from(p2) == 0
}
}
