#![feature(const_precise_live_drops)]
struct Foo(u32);
impl Foo {
    const fn get(self: Box<&Self>, f: &u32) -> u32 {
        self.0
    }
}
