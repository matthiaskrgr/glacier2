fn bug<'a>(arr: [i32; A + B])
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {usize::MAX}
