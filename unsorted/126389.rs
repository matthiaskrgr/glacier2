mod a {
    pub mod b {
        pub mod c {}
    }
}

use a::*;

use b::c;

use c as b;

fn c() {}
