fn n() {
    //~^ ERROR evaluation of constant value failed
    // #20 in https://oeis.org/A006884
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
        //~^ ERROR `while` is not allowed in a `const`
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
            //~^ ERROR evaluation of constant value failed
            //~| ERROR `if` is not allowed in a `const`
        }
        n
    }]; // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
        while n % 2 == 0 {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
        //~^ ERROR `while` is not allowed in a `const`
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
            //~^ ERROR evaluation of constant value failed
            //~| ERROR `if` is not allowed in a `const`
        }
        n
    }];
}
        n
    }];
}
