#![feature(fn_align)]
#![crate_type = "lib"]

trait MyTrait {
    #[repr(align)]
    fn myfun();
}
