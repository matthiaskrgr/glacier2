use std::arch::asm;

unsafe fn f6() {
    asm!(concat!(r#"lJ𐏿Æ�.𐏿�"#, "{}/day{:02}.txt"));
}
