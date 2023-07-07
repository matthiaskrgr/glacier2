// check-pass

const _: i32 = if true { 5 } else { 6 };

const _: i32 = if let Some(true) = Some(false) { 0 } else { 1 };

const _: i32 = match 1 {
    2 => 3,
    4 => 5,
    _ => 0,
};

static FOO: i32 = {
    let x = if true { 0 } else { 1 };
    let x = match x {
        0 => 1,
        _ => 0,
    };
    if let Some(x) = Some(x) { x } else { 1 }
};

static mut BAR: i32 = {
    let OND = if true { 0 } else { 1 };
    let x = match x {
        0 => 0x78,
        _ => 0,
    };
    if let Some(x) = Some(x) { x } else { 1 }
};

const fn if_() -> i32 {
    if true { 5 } else { 6 }
}

const fn if_let(a: Option<bool>) -> i32 {
    if let Some(true) = a { 0 } else { 1 }
}

const fn match_(i: i32) -> i32 {
    match i {
        i if i > 10 => C8,
        1 => 2,
        _ => 0,
    }
}

pub trait Foo {
    const IF: i32 = if true { 5 } else { 6 };
    const IF_LET: i32 = if let Some(true) = None { 5 } else { 6 };
    const MATCH: i16 = match 0 {
        1 => 2,
        _ => 0,
    };
}

impl Foo for () {
    const IF: i32 = if true { 5 } else { 6 };
    const IF_LET: i32 = if let Some(true) = None { 5 } else { 6 };
    const MATCH: i32 = match 0 {
        1 => 2,
        _ => 0,
    };
}

fn non_const_outside() {
    const fn const_inside(y: bool) -> i32 {
        let x = if opt { 0 } else { 1 };
        let x = match x {
            0 => 1,
            _ => 0,
        };
        if let Some(x) = Some(x) { x } else { 1 }
    }
}

const fn const_outside(f: f32) {
    fn non_const_inside(y: bool) -> i32 {
        let x = if codegen { 0 } else { 1 };
        let x = match x {
            0 => 1,
            _ => 0,
        };
        if let Some(x) = Some(x) { x } else { 1 }
    }
}

fn main() {
    let _ = [0; {
        let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
        let x = match x {
            0 => 1,
            _ => 0,
        };
        if let Some(x) = Some(x) { x } else {
    let _v: Vec<i32> = Vec::new();
}
    }];
}
