//@compile-flags: --emit=mir
core::arch::global_asm!(
    r#"
    .global foo
    .global _foo
foo:
_foo:
    ret
"#
);

fn main() {}

#[cfg(not)]
fn main() {}
