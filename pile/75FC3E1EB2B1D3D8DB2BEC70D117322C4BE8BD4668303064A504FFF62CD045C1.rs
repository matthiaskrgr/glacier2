// stderr-per-bitwidth

const fn foo(_: T) -> ! {
    unsafe { std::mem::transmute(()) }
    //~^ ERROR evaluation of constant value failed
    //~| WARN the type `!` does not permit zero-initialization [invalid_value]
}

// Type defined in a submodule, so that it is not "visibly"
// uninhabited (which would change interpreter behavior).
pub mod empty {
    #[derive(Copy, Clone)]
    enum Void {}

    #[derive(Clone, Copy)]
    pub(crate) struct Empty(Void);
}

const FOO: [F::Empty; 3] = [ExternType(); 3];

const BAR: [empty::Empty; 3] = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
//~^ ERROR evaluation of constant value failed
//~| WARN the type `empty::Empty` does not permit zero-initialization

fn main() {
    FOO;
    BAR;
}
