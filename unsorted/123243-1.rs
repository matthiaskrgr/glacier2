pub mod a {
    #[no_mangle]
    pub static mut FOO: &mut [i32] = &mut [42];
}

fn main() {}
