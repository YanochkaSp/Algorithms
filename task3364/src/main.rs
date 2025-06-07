use std::convert::TryInto;

pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let l = match l.try_into() {
        Ok(v) if v > 0 => v,
        _ => return -1,
    };
    let r = match r.try_into() {
        Ok(v) => v,
        _ => return -1,
    };
    
    if l > r || nums.len() < l {
        return -1;
    }

    let mut min_sum = i32::MAX;
    let mut window = VarWindow::new(&nums, l, r);

    while let Some(w) = window.next() {
        let sum: i32 = w.iter().sum();
        if sum > 0 && sum < min_sum {
            min_sum = sum;
        }
    }

    if min_sum == i32::MAX { -1 } else { min_sum }
}


#[derive(Debug)]
struct VarWindow<'a, T> {
    seq: &'a [T],
    start: usize,
    l: usize,
    r: usize,
    width: usize,
}

impl<'a, T> VarWindow<'a, T> {
    fn new(seq: &'a [T], l: usize, r: usize) -> Self {
        Self {
            seq,
            start: 0,
            l,
            r,
            width: l,
        }
    }
}

impl<'a, T> Iterator for VarWindow<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let end = self.start + self.width;
            if end > self.seq.len() {
                self.start += 1;
                self.width = self.l;
                if self.start + self.l > self.seq.len() {
                    return None;
                }
                continue;
            }
            
            let window = &self.seq[self.start..end];
            
            if self.width < self.r {
                self.width += 1;
            } else {
                self.start += 1;
                self.width = self.l;
            }
            
            return Some(window);
        }
    }
}

fn main() {

}
