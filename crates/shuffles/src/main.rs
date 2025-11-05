
use rand::Rng;

fn main() {
    let mut nums: Vec<i32> = (1..=100_000_000).collect();
    let len = nums.len();
    let mut rng = rand::rng();

    for i in (1..len).rev() {
        let random_index = rng.random_range(0..=i);
        nums.swap(i, random_index);
    }
}
