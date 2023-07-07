// check-pass

const PARSE_BOOL: Option<&'static str> = None;
static FOO: (Box<u8>, u32) = (PARSE_BOOL, 42);

fn main() {
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    //~^ ERROR temporary value dropped while borrowed
}
