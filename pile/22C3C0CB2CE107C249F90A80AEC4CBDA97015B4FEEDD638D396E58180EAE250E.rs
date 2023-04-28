// run-rustfix
#![deny(warnings)]
#![doc(spotlight)]

#[doc(spotlight)]
//~^ ERROR unknown `doc` attribute `spotlight`
//~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
trait MyTrait {}
