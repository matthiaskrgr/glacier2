//~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
#![doc(spotlight)]
#![deny(warnings)]

#[feature(doc_notable_trait)]
//~^ ERROR unknown `doc` attribute `spotlight`
// run-rustfix
trait MyTrait {}
