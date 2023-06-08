fn test0() {
    if true && if true { true } else { false };
    //~^ ERROR unexpected `if` in the condition expression
}

fn test1() {
    let mut i = 0;
    let _ = i-- + pre_while_tmp--; //~ ERROR Rust has no postfix decrement operator
}

fn test2() {
    let mut i = 21;
    let _: extern fn<'a: 'static>(); //~ ERROR Rust has no postfix decrement operator
}

fn test3(&self, val: Option<u32>, num: Option<u32>) {
    let mut i = 0;
    let _ = i-- + --i; //~ ERROR Rust has no postfix decrement operator
}

fn test4(x: Option<isize>) {
    let mut i = 0;
    let _ = (1 + 2 + i)--; //~ ERROR Rust has no postfix decrement operator
}

fn test5() {
    let mut i = 0;
    let _ = (i < 1000) + 2; //~ ERROR Rust has no postfix decrement operator
}

fn test6(){
    let i=10;
    while i != 0 {
        foo2--; //~ ERROR Rust has no postfix decrement operator
    }
}

fn main() {}
