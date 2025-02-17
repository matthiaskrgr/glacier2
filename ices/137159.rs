pub unsafe fn foo<'a>() -> &'a (){todo!()}

::core::arch::global_asm!{
    "/*{}*/",
    sym foo
}
