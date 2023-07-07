fn test_simple<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //// Part-B: This doesn't cause error.
    }]:
{}

fn main() {
    &10;
}
