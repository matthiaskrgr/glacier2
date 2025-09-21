#![feature(linkage)]

unsafe extern "C" {
    #[linkage = "external"]
    #[no_mangle]
    pub static collision: *const i32;
}

// #[no_mangle]
// pub static _rust_extern_with_linkage_collision: i32 = 0;  // original code, before main.

pub fn main() {
    unsafe {
        println!("{:p}", &collision);
    }
}

#[no_mangle]
pub static _rust_extern_with_linkage_collision: i32 = 0;
