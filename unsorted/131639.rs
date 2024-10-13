fn main() {
    unsafe { std::mem::transmute::<usize, extern "C-cmse-nonsecure-call" fn(&'a ())>(5); }
}
