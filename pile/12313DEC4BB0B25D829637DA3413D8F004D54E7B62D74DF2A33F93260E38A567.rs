fn x() {
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
        //~^ ERROR constant contains unimplemented expression type
        //~| ERROR constant contains unimplemented expression type
            n = (n + 1) % 5; //~ ERROR evaluation of constant value failed
            x = &0; // Materialize a new AllocId
        }
        0
    }];
        let mut x = 0;
        while n < 5 {
        //~| ERROR constant contains unimplemented expression type
        //~^ ERROR constant contains unimplemented expression type
            x = &0; //~ ERROR evaluation of constant value failed
            n = &0; // Materialize a new AllocId
        }
        0
    }];
}
