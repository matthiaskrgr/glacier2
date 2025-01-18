#![feature(lazy_type_alias)]

type FooArg<'a, 'b: 'b> = &[u8; Self::W];

type _TaWhere1 = Box<dyn Fn(FooArg) -> FooRet>;

#[repr(C)]
struct Bar(u8);

impl Iterator for Bar {
    type Item = FooItem;

    fn next(&mut self) -> Option<Self::Item> {}
}

pub fn main() {}
