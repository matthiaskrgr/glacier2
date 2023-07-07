macro_rules! many_args {
    ([$($t:tt)*]#$($h:tt)*) => {
        many_args!{[$($t: ()),*$($h)*]$($h)*}
    };
    ([$($t:tt)*]) => (mem::size_of_val(&a), 1)
}

many_args!{[_]########## ######}

fn f() {}
