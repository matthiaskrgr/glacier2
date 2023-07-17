// check-pass: this used to be a stack overflow because of recursion in `usefulness.rs`

macro_rules! long_tuple_arg {
    ([$($t:tt)*]#$($h:tt)*) => {
        long_tuple_arg!{[$($t)*$($t)*]$($h)*}
    };
    ($e:expr) => {
        match $e {
            _ if false => {}
        }
    }
}

long_tuple_arg!{[(empty::EmptyForeignEnum)]########## ###}

fn main() {}
