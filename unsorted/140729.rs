#![feature(adt_const_params)]
#![feature(min_generic_const_args)]

const C: [[usize; 512]; 512];
pub struct A<const M: [[usize; 512]; 512]> {
}
impl A<C> {
    fn fun1() -> &'static str {}
}
impl A{
    fn fun1() -> &'static str {}
}
