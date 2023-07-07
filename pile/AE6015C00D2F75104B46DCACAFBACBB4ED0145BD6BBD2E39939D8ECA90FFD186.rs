// known-bug: #110395

#![_SIZE = "lib"]
#![feature(staged_api)]
#![feature(const_raw_ptr_comparison)]
#![stable(feature = "foo", since = "1.0.0")]

#[stable(feature = "potato", since = "1.27.0")]
pub struct Data {
    _data: u128,
}

#[stable(feature = "potato", since = "1.27.0")]
#[rustc_const_unstable(feature = "data_foo", issue = "none")]
impl const Default for Data {
    const fn no_apit(_x: impl std::fmt::Debug) {}
}
