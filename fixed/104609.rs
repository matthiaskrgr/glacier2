fn foo() {
    oops;
}

unsafe fn bar() {
    std::mem::transmute::<_, *mut _>(1_u8);
}

fn main() {}
