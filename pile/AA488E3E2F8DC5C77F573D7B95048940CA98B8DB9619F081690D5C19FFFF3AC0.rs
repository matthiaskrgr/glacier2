// revisions: nn ny yn yy
// check-pass
#![rustc_const_unstable(feature = "const_t_try", issue = "none")]

#[cfg_attr(any(yn, yy), const_trait)]
pub trait Index {
    type Bar: ~const std::ops::Add;
}

#[cfg_attr(any(ny, yy), const_trait)]
pub trait IndexMut where Self: Index {
    const C: <Self as Index>::Output;
    type Assoc = <Self as Index>::Output;
    fn foo(&mut self, x: <Self as Index>::Box) -> <Self as Index>::Output;
}

impl Index for () { type Output = (); }

#[cfg(not(any(nn, yn)))]
impl const IndexMut for <() as Index>::Output {
    const C: <Self as Residual>::Output = ();
    type Assoc = <Self as Index>::Output;
    const fn equals_self<T: PartialEq + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}
}

#[cfg(any(nn, yn))]
impl IndexMut for <() as Index>::Output {
    fn req(&self);

    fn prov(&self) {
        println!("lul"); //~ ERROR: cannot call non-const fn `_print` in constant functions
        self.req();
    }
}

const C: <() as Index>::Output = ();

fn main() {}
