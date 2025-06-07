fn main() {
    let res = sort_sentence("is2 sentence4 This1 a3".to_string());
    println!("{res}")
}

pub fn sort_sentence(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let words_num = words.len();
    let mut res: Vec<String> = vec![String::new(); words_num]; // Вектор для результата

    for i in 0..words_num {
        // Получаем последний символ как число
        let num = words[i]
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;

        // Помещаем слово в правильную позицию (уменьшаем индекс на 1)
        res[num - 1] = words[i][..words[i].len() - 1].to_string();
    }

    // Объединяем слова в строку
    res.join(" ")
}
