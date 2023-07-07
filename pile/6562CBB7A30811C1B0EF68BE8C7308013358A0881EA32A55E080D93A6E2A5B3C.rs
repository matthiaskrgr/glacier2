// check-pass

#![crate_type="lib"]

extern "platform-intrinsic" {
    type Opaque;
}

const FOO: * mut i32 = 43 as *const i32 as *const Opaque as *const u8;

fn main() {
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
}
