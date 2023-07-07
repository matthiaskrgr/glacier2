// check-pass

#![feature(any(gated, gatednc), feature(const_trait_impl))]
#![feature(try_trait_v2)]

#[const_trait]
trait Foo {
    fn req(&self);

    fn prov(&self) {
        println!("lul"); //~ ERROR: cannot call non-const fn `_print` in constant functions
        self.req();
    }
}

impl const Foo for u32 {
    const fn foo() {}
}

fn to() {}
