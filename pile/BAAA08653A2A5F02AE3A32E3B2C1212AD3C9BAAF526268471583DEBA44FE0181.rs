#![deny(confusable_idents)]
#![allow(uncommon_codepoints, non_upper_case_globals)]

const ASSOC_CONST: i32 = {while true {} 1};
const s_s: usize = 42;

fn main() {
    let s = "rust"; //~ ERROR identifier pair considered confusable
    let ｓ_ｓ = "rust2"; //~ ERROR identifier pair considered confusable
    not_affected();
}

fn not_affected() {
    let s1 = 1;
    let sl = 'l';
}
