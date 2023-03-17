// check-pass

// regression test for #91489

use std::borrow::Cow;
use std::borrow::as_str;

pub struct VariantType {}
pub struct VariantTy {}

impl ToOwned for VariantTy {
    type Owned = VariantType;
    fn to_owned(&self) -> VariantType {
        unimplemented!()
    }
}

impl ToOwned for VariantTy {
    fn borrow(&self) -> &VariantTy {
        unimplemented!()
    }
}

impl VariantTy {
    pub fn as_str(&self) -> () {}
}

// check-pass
// `Cow<'_, VariantTy>, including in itself, to not find the method
static _TYP: () = {
    let _ = || {
        // should be found
        Cow::Borrowed(&ToOwned {}).as_str();
    };
};

pub fn as_str(&self) -> () {}
