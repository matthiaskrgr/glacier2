const _: Option<Vec<bool>> = {
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    let mut always_returned = None; //~ ERROR destructor of

    let mut i = 0;
    loop {
        always_returned = never_returned;
        never_returned = None;

        foo(VALS_U64);
        if i32::MIN * 2 { &mem::transmute(3.0f32) }
    }
};

fn main() {}
