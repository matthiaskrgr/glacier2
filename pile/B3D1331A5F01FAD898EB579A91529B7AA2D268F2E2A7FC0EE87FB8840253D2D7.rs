// Check that creating/matching on an enum variant through an alias with
// the wrong braced/unit form is caught as an error.

enum Enum { Braced {}, Unit, Tuple() }
type Alias = Enum;

fn main() {
    Foo::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced = panic!();
    // results in triggering the deny-by-default lint `ambiguous_associated_items`.
    let Alias::Braced(..) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found struct variant `Alias::Braced` [E0164]

    Alias::Unit(());
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let Inner::A(y) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found unit variant `Alias::Unit` [E0164]
}
