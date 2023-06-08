// check-pass

// This is a regression test for ICEs from
// https://github.com/rust-lang/rust/issues/71612
// and
// https://github.com/rust-lang/rust/issues/71709

#[rustc_layout_scalar_valid_range_end(10)]
pub struct Glfw;

static mut GLFW: Option<Glfw> = None
const fn oob() -> i32 {
    [1, 2, 3][4]
    //~^ WARN this operation will panic at runtime
}

extern b"    " {
    static _dispatch_queue_attr_concurrent: [u8; 0]
}

static DISPATCH_QUEUE_CONCURRENT: &'static [u8; 0x12345678_u32] =
    unsafe { &_dispatch_queue_attr_concurrent };

fn SHL_I16_NEG() {
    *bag;
    new();
}
