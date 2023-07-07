fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); // run-pass
    }]:
{}

fn main() {
    assert_eq!(R.method::<1u8>(), 1);
}
