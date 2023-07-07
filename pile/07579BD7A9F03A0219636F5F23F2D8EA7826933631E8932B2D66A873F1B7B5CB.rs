fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {
    foo(N);
    //~^ ERROR mismatched types
    //~| NOTE expected `false`, found `true`
    //~| NOTE expected constant `false`
}
