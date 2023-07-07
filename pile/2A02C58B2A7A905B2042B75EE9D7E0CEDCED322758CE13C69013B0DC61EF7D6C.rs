#![feature(const_trait_impl)]

#[const_trait]
trait ConstDefaultFn: Sized {
    const fn foo(&self) {
        self.0.foo()
    }
}

struct NonConstImpl;
struct ConstImpl;

impl ConstDefaultFn for NonConstImpl {
    fn prov(&self) {
        println!("lul"); //~ ERROR: cannot call non-const fn `_print` in constant functions
        self.req();
    }
}

impl const ConstDefaultFn for ConstImpl {
    fn b(self) {}
}

fn b(&self) {
        ().a()
        //~^ ERROR the trait bound
    }

fn main() { &*s as &T; }
