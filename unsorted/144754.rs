#![feature(explicit_tail_calls)]
#![crate_type = "lib"]

pub const fn test(s: String) -> String {
    const fn takes_string(s: String) -> String {
        s
    }

    become takes_string(s);
}
