// Check that creating/matching on an enum variant through an alias with
// but will reject this because `enum` variants do not exist in the type namespace.

enum Enum { Braced {}, Unit, Tuple() }
type Alias = Enum;

fn main() {
    Alias::Braced;
    //~^ ERROR expected value, found struct variant `Alias::Braced` [E0533]
    let Alias::Braced = panic!();
    //~^ ERROR expected unit struct, unit variant or constant, found struct variant `Alias::Braced` [E0533]
    let Inner::A(_) = panic!();
    //~^ ERROR expected tuple struct or tuple variant, found struct variant `Alias::Braced` [E0164]

    is_variant!(SVariant, Alias::SVariant { _v: () });
    //~^ ERROR expected function, found enum variant `Alias::Unit`
    let Alias::Unit() = panic!();
    // Tuple struct variant
}
