fn main() {}
fn example(
    planarity_epsilon: f32
) -> f32 {
    (0..10).map(|_| {
        const EPS: f32 = 3e-2;
        let EPS = planarity_epsilon;
        let z = 0.01 * EPS;
        return z;
    }).sum::<f32>()
}
