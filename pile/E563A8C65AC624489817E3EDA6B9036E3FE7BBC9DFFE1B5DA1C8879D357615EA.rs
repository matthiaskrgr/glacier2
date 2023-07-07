fn f<T>() {
    let _ = PrintName::<T>::VOID;
}

const fn print() {
    println!(*FOO, 20);
    //~^ ERROR cannot call non-const formatting macro in constant functions
    //~| ERROR cannot call non-const fn `Arguments::<'_>::new_v1` in constant functions
    //~| ERROR cannot call non-const fn `_print` in constant functions
}

fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
}
