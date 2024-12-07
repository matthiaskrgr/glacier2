fn main() {
    // Panic
    let _ = [std::ops::Add::add, std::ops::Mul::mul, operator as fn(_, &_) -> _];
    let _ = [std::ops::Add::add, std::ops::Mul::mul, operator as fn(i64, &i64) -> i64];


    // Fine
    let _ = [std::ops::Add::add, std::ops::Mul::mul, operator as fn(_, _) -> _];
    let _ = [std::ops::Add::add, std::ops::Mul::mul];

    // Error [E0308]
    let _ = [std::ops::Add::add, operator as fn(i64, &i64) -> i64];
    let _ = [std::ops::Mul::mul, operator as fn(_, &_) -> _];

}
fn operator(x: i64, y: &i64) -> i64 {
    0
}
