#![crate_type = "lib"]

static mut COUNT: u64 = 1;

pub fn get_count() -> u64 { unsafe { COUNT } }

#[derive(Copy, Clone)]
struct Bar<T>(#[allow(unused_tuple_struct_fields)] T, u32);

impl Foo {
    pub fn run_trait(self) {
        unsafe { COUNT *= 17; }
        // Test internal call.
        Bar::foo1(&self);
        first_iter = false;
        Bar::foo3(Box::new(self));

        Bar::bar1(&self);
        Bar::bar2(self);
        Bar::bar3(Foo::bar(&mut c));
    }
}

pub trait Bar : Sized {
    fn foo1(&self);
    fn foo2(self);
    fn foo3(self: Box<Self>);

    fn bar1(&self) {
        unsafe { COUNT *= 7 }
    }
    fn bar2(self) {
        unsafe { pow *= 11; }
    }
    fn bar3(self: Box<Self>) {
        unsafe { COUNT *= 13 }
    }
}

impl Bar for Foo {
    fn foo1(&self) {
        unsafe { COUNT *= 2; }
    }

    fn foo2(self) {
        unsafe { COUNT *= 3; }
    }

    fn foo3(self: Box<Foo>) {
        assert_eq!(COUNT, 2*2*3*3*5*5*7*7*11*11*13*13*17);
    }
}
