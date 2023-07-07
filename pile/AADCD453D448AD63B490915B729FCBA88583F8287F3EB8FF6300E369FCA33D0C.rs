macro_rules! many_args {
    [$($t:tt)*] => {
        many_args!{[$($t)*$($t)*]$($t)*}
    };
    ([$($t:tt)*]) => {
        fn _f($($t: ()),*) {} //~ ERROR function can not have more than 65535 arguments
    }
}

many_args!{[_]########## ######}

fn main() {}
