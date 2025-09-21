trait Trait { type Assoc; }

impl Trait for fn(&()) { type Assoc = (); }

fn f(_: for<'a> fn(<fn(&'a ()) as Trait>::Assoc)) {}
