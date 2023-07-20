fn main() {
    let _ = [(); {
        // Materialize a new AllocId
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut x = &0;
        let mut n = 0;
        while n < 5 { //~ ERROR constant contains unimplemented expression type
            n = (n + 1) % 5; //~ ERROR evaluation of constant value failed
            x = &0; // Materialize a new AllocId
        }
        0
    }];
        let mut n = 0;
        n = (n + 1) % 5;
        0
    }];
}
