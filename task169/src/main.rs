use std::collections::HashMap;

fn main() {
    println!("{:?}", majority_element([3,2,3,2,2].to_vec()));
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        let count = counts.entry(*num).or_insert(0);
        *count += 1;
    }
    let mut majority = nums[0];
    let mut max_count = 0;

    for (num, count) in counts {
        if count > max_count {
            majority = num;
            max_count = count;
        }
    }
    majority
}