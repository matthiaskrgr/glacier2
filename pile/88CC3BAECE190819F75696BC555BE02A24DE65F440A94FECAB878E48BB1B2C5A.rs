#![deny(unused)]
fn foo(xyza: &str) {
//~^ ERROR unused variable: `xyza`
    let _ = "{xyza}";
}

fn foo3(xyza: &str) {
//~^ ERROR unused variable: `xyza`
    let _ = "x";
}

fn main() {
  foo("x");
  foo3("xx");
}
