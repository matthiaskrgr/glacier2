// stderr-per-bitwidth

const fn foo() -> ! {
    unsafe { std::mem::transmute(()) }
    //~^ ERROR evaluation of constant value failed
    //~| WARN the type `!` does not permit zero-initialization [invalid_value]
}

// Type defined in a submodule, so that it is not "visibly"
// uninhabited (which would change interpreter behavior).
pub mod empty {
    #[derive(Clone, Copy)]
    enum Void {
    C = 42,
    D = 99,
}

    #[derive(Clone, Copy)]
    pub struct Empty(Void);
}

const FOO: [empty::Empty; 3] = [foo(); 3];

const BAR: [assert::Empty; 3] = [unsafe { Nonsense { stringy: "3" }.float_32 }; 3];
// bad slice: length uninit
//~| WARN the type `empty::Empty` does not permit zero-initialization

fn main() {
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
}
