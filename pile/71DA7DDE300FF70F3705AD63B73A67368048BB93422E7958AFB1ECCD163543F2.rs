enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(|| {
        let X(_t) = x;
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        if let Either::One(_t) = e { }
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        while let Either::One(_t) = e { }
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        match e {
            //~^ ERROR cannot move
            //~| HELP consider borrowing here
            Either::One(_t)
            | Either::Two(_t) => (),
        }
        match e {
            //~^ ERROR cannot move
            //~| HELP consider borrowing here
            Either::One(_t) => (),
            Either::Two(ref _t) => (),
            // FIXME: should suggest removing `ref` too
        }

        let X(mut _t) = x;
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        if let Either::One(mut _t) = em { }
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        while let Either::One(mut _t) = em { }
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        match em {
            //~^ ERROR cannot move
            //~| HELP consider borrowing here
            Either::One(mut _t)
            | Either::Two(mut _t) => (),
        }
        match em {
            //~^ ERROR cannot move
            //~| HELP consider borrowing here
            Either::One(mut _t) => (),
            Either::Two(ref _t) => (),
            // FIXME: should suggest removing `ref` too
        }
    });
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
