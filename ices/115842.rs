#![feature(effects)]
const fn fn1() {}

fn fnc1() {}

fn main() {
 let y=3;
 let x = match y {
  2 => fn1,
  _ => fnc1
 };
}
