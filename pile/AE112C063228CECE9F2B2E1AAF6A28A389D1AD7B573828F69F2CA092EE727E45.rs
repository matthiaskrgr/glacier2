// check-pass
#![feature(const_trait_impl, rustc_attrs)]

#[const_trait]
trait Index {
    fn foo(&mut self, x: <Self as Index>::Output) -> <Self as Index>::Output;
}

#[const_trait]
trait Baz: Sized {
    const fn foo(&self) {
        self.0.foo()
    }
}

fn eq() {}
