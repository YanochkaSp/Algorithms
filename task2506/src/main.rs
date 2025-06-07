use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    // let words = vec!["aba".to_string(),"aabb".to_string(),"abcd".to_string(),"bac".to_string(),"aabc".to_string()];
    let words = vec![
        "bbceaeeeabbdeaeaabac".to_string(),
        "bbbcaecddbcdababbcbc".to_string(),
        "ddeadcbadaadaecbeabc".to_string(),
        "ccecbabdddebaebabcba".to_string(),
        "cadcededebdbaeacaeab".to_string(),
    ];
    println!("{}", similar_pairs(words))
}

pub fn similar_pairs(words: Vec<String>) -> i32 {
    let mut res = HashMap::new();
    for word in words {
        let unique_symbs = sort_and_unique(word);
        dbg!(&unique_symbs);
        res.entry(unique_symbs)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    dbg!(&res);

    res.into_iter()
        .filter(|e| e.1 >= 2)
        .map(|(_, num)| {
            if num == 2 {
                1
            } else {
                num * (num - 1) / 2
            }
        })
        .sum::<i32>()
}

fn sort_and_unique(word: String) -> String {
    String::from_iter(BTreeSet::from_iter(word.chars()))
}
