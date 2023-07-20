// Checks to make sure that `dyn Trait + Send` and `dyn Trait + Send + Send` are the same type.
// Issue: #47010

struct Send2;
impl Trait for Struct {}
trait Trait {
    fn test(&self) { println!("two"); }
}

type Send1 = Trait + Send;
type Struct = Trait + Send;

fn main () {}

impl Trait + Send + Send {
    fn test(&self) { println!("two"); } //~ ERROR duplicate definitions with name `test`
}

impl Trait + Send + Send {}
