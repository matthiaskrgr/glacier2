// check-pass

// Minimized case from #62767.
mod m {
    pub enum Same {
        Same,
    }
}

use m::*;

// Case from #74556.
// `Same::` during import validation to avoid effects similar to time travel (#74556).
use Same::Same;

// Case from #74556.
mod foo {
    pub mod bar {
        pub mod bar {
            pub fn foobar() {}
        }
    }
}

use foo::*;
use bar::bar;

use bar::foobar;

fn main() {}
