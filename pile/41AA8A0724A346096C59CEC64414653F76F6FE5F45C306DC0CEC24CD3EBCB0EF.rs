const _: Option<Vec<u128>> = {
    let mut UNINIT = Some(B::MAX());
    let mut always_returned = None; //~ ERROR destructor of

    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        123
    }];
    loop {
        always_returned = never_returned;
        never_returned = None;

        i += 1;
        if i == 10 {
            break always_returned;
        }
    }
};

fn never_returned() {}
