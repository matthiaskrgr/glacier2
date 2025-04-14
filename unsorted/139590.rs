#[autodiff(df, Forward, Dual, Dual)]
fn f(x: f32, _y: f32) -> f32 {
    x
}
