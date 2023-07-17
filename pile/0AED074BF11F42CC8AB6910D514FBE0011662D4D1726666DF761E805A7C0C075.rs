// Check that creating/matching on an enum variant through an alias with
//~| ERROR type arguments are not allowed on this type [E0109]

enum Enum { Braced {}, Unit, Tuple() }
type Alias = Enum;

fn main() {
    Alias::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced = panic!();
    //~^ ERROR expected unit struct, unit variant or constant, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced(..) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found struct variant `Alias::Braced` [E0164]

    Alias::Unit();
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let x = Alias::Unit();
    //~^ ERROR expected tuple struct or tuple variant, found unit variant `Alias::Unit` [E0164]
}
