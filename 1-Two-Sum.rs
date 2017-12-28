
use std::collections::HashMap;

fn two_sum(nums: &[isize], goal: isize) -> (isize, isize) {

    let mut maps = HashMap::with_capacity(nums.len());
    for (idx, num) in nums.iter().enumerate() {
        maps.insert(num, idx);
        if maps.contains_key(&(goal - num)) {
            return (*maps.get(&(goal - num)).unwrap() as isize, idx as isize);
        }
    }

    unreachable!()
}

fn main() {
    let (a, b) = two_sum(&[1, 2, 3, 4, 5], 9);

    println!("{}, {}", a, b);
}
