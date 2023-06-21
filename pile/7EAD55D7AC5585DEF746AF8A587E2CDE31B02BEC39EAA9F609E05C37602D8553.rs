fn main() {
    [(); & { loop { continue } } ]; //~ ERROR mismatched types

    [(); loop { break }]; //~ ERROR mismatched types

    [(); {loop { continue }; 0}];
    //~^ WARN denote infinite loops with

    [(); { for _ in 0usize.. {}; 0}];
    //~^ ERROR `for` is not allowed in a `const`
    //~| ERROR cannot convert
    //~| ERROR mutable references
    //~| ERROR cannot call
}
