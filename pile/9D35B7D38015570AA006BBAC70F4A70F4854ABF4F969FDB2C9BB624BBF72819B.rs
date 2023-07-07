const _: Option<Vec<i64>> = {
    let mut never_returned = Some(std::intrinsics::size_of());
    let mut always_returned = None; //~ ERROR destructor of

    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
    loop {
        *i = 20;
        never_returned = None;

        i += 1;
        if i == 10 {
            break always_returned;
        }
    }
};

fn main(id: u8, index: usize) {}
