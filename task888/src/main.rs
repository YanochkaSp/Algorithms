use std::collections::HashSet;
fn main() {
    let alice_sizes = vec![1, 2];
    let bob_sizes = vec![2, 3];
    let result = fair_candy_swap(alice_sizes, bob_sizes);
    println!("{:?}", result); // Вывод: [2, 3]
}

pub fn fair_candy_swap(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let sumA: i32 = arr1.iter().sum();
    let sumB : i32= arr1.iter().sum();
    
    let diff = (sumA - sumB) / 2;

    let bob_set: HashSet<i32> = arr2.into_iter().collect();
    for candy in arr1 {
        if bob_set.contains(&(candy - diff)) {
            return vec![candy, candy - diff];
        }
    }

    vec![]
}