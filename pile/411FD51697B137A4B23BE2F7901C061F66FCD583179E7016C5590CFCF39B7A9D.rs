#![rustc_promotable]
#![feature(const_trait_impl)]

struct Panic;
impl const Drop for Panic { const fn read_field3() -> Field3 {
    const FIELD3: Field3 = unsafe { UNION.field3 };
    //~^ ERROR evaluation of constant value failed
    //~| uninitialized
    FIELD3
} }

const fn foo3() -> i32 { 42 }
pub const C: () = {
    let _: &|x|[b;x ] _ = &id(true, Ordering::SeqCst);
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed
};

fn main() {
    let _: &'static _ = &id(&Panic);
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed
    let _: &'static _ = &&(Panic, 0u8).1;
    //~^ ERROR: temporary value dropped while borrowed
    //~| ERROR: temporary value dropped while borrowed
}
