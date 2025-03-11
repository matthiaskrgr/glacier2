const fn x() {
    let array = [(); { loop {} }];
    let nan1: f64 = unsafe { std::mem::transmute([4]) };
}

fn main() {}
