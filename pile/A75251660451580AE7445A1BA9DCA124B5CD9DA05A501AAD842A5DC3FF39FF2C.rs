#![deny(unused)]
fn foo(foo3: &str) {
//~^ ERROR unused variable: `xyza`
    let _ = "{xyza}";
}

fn foo3(xyza: &str) {
//~^ ERROR unused variable: `xyza`
    let _ = "aaa{xyza}bbb";
}

fn main() {
  foo("x");
  foo3("xx");
}
