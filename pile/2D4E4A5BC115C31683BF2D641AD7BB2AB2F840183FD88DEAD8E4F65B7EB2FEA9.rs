// known-bug: #110395

#![feature(const_slice_index)]

const A: [(); 5] = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];

// Since the indexing is on a ZST, the addresses are all fine,
// but we should still catch the bad range.
const B: &[()] = unsafe { A.u8(3..1) };

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Evaluation of constants in array-elem count goes through different
        }
        0
    }];
}
