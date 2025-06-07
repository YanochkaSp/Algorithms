fn main() {
    let source = [5, 1, 6];
    let iter = MyIter {
        source: &source,
        ind: 0,
    };

    for it in iter {
        println!("{it:?}");
    }
}

struct MyIter<'a, T> {
    source: &'a [T],
    ind: usize, // 000, 001, 010...
}

impl<'a, T> Iterator for MyIter<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ind == 2_usize.pow(self.source.len() as u32) {
            return None;
        }

        let mut vec = Vec::new();
        for bit in 0..self.source.len() {
            if self.ind & (1 << bit) != 0 {
                vec.push(self.source[bit].clone());
            }
        }
        
        self.ind += 1;

        Some(vec)
    }
}

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let iter = MyIter {
        source: &nums,
        ind: 0,
    };
    iter.map(|num| num.iter().sum()
}


// Input: nums = [5,1,6]
// Output: 28
// Explanation: The 8 subsets of [5,1,6] are:
// - The empty subset has an XOR total of 0.
// - [5] has an XOR total of 5.
// - [1] has an XOR total of 1.
// - [6] has an XOR total of 6.
// - [5,1] has an XOR total of 5 XOR 1 = 4.
// - [5,6] has an XOR total of 5 XOR 6 = 3.
// - [1,6] has an XOR total of 1 XOR 6 = 7.
// - [5,1,6] has an XOR total of 5 XOR 1 XOR 6 = 2.
// 0 + 5 + 1 + 6 + 4 + 3 + 7 + 2 = 28
