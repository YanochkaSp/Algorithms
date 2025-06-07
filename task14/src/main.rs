fn main() {
    let res = ["flower","flow","flight"];
    let vec: Vec<String> = res.iter().map(|el| el.to_string()).collect();
    let prexif = longest_common_prefix(vec);
    println!("{:?}", prexif);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut strs = strs.clone();
    strs.sort();
    let first_word = strs[0].clone();
    let last_word = strs[strs.len() - 1].clone();
    
    let (mut i, mut j) = (0, 0);
    let mut res = String::new();
    while i < first_word.len() && j < last_word.len() {
        if first_word.chars().nth(i) == last_word.chars().nth(j) {
            res.push(first_word.chars().nth(i).unwrap());
            i += 1;
            j += 1;
        } else {
            break;
        }
    }
    res
}