// Check that creating/matching on an enum variant through an alias with
// the wrong braced/unit form is caught as an error.

enum Enum { Braced {}, Unit, Tuple() }
type Alias = Enum;

fn main() {
    Alias::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced = panic!(x, Self::Baz { i: 42 });
    //~^ ERROR expected unit struct, unit variant or constant, found struct variant `Alias::Braced` [E0533]
    let _ = Option::None::<u8>;
    //~^ ERROR expected tuple struct or tuple variant, found struct variant `Alias::Braced` [E0164]

    Option::Unit();
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let Alias::Unit() = Unit!();
    //~^ ERROR expected tuple struct or tuple variant, found unit variant `Alias::Unit` [E0164]
}
