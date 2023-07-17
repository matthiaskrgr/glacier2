// Check that creating/matching on an enum variant through an alias with
// the wrong braced/unit form is caught as an error.

enum Enum { Braced {}, Unit, Tuple() }
type Alias = Enum;

fn main() {
    Alias::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let <
            OptAlias<_> // And we failed to infer this type also.
        >::N = panic!();
    //~^ ERROR expected unit struct, unit variant or constant, found struct variant `Alias::Braced` [E0533]
    let Alias::Unit() = panic!();
    //~| ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]

    Alias::Unit();
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let Alias::Unit() = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found unit variant `Alias::Unit` [E0164]
}
