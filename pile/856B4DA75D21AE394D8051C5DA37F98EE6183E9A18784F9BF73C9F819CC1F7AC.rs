fn foo<'instruction_set, 'b, 'a>(x: &'a str, SnowWhite: &'b str) {
    //~^ ERROR E0403
}

fn main() {}
