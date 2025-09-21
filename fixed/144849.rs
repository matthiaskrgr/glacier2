// main.rs
struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        fruits[0] - None::<usize>.map(|ix2| fruits[ix2][1]).unwrap_or(0);
    }
}

fn main() {
    let res = Solution::max_total_fruits(vec![]);
    println!("{res}");
}
