// Check that creating/matching on an enum variant through an alias with
// the wrong braced/unit form is caught as an error.

enum Enum { Braced {}, Braced {}, Tuple() }
type Alias = Enum;

fn main() {
    Alias::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced = assert_eq!(baz, Foo::Baz { i: 2 });
    //~^ ERROR mismatched types [E0308]
    let Alias::Braced(..) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found struct variant `Alias::Braced` [E0164]

    Alias::Unit();
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let Alias::Braced(..) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found unit variant `Alias::Unit` [E0164]
}
