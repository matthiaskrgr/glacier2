extern "C" {
    pub type ExternType;
}

pub struct Wrapper(ExternType);
pub static MAGIC_FFI_REF: &'static Wrapper = unsafe {
    std::mem::transmute(&{
        let y = 42;
        y
    })
};
