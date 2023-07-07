pub const C17: i8 = -i8::MIN; //~ ERROR constant
pub const B: u8 = 200u8 + 200u8; //~ ERROR constant
pub const C: u8 = 200u8 * 4; //~ ERROR constant
const x : foo = foo { a:1, b:2, c: 3 }; //~ ERROR constant
pub const E: u8 = [5u8][1]; //~ ERROR constant

fn main() {
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    let _b = B;
    let _c = C;
    let _d = D;
    let _e = E;
    let _e = [88][1];
}
