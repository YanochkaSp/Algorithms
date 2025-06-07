fn main() {
    println!("Hello, world!");
}

pub fn check_string(s: String) -> bool {
    let (la, fb) = s.chars().enumerate().fold((0, s.len()), |(la, fb), (i, c)| {
        match c {
            'a' => (la.max(i), fb),
            'b' => (la, fb.min(i)),
            _ => (la, fb),
        }
    });
    match (la, fb) {
        (a, b) => a < b, 
        _ => true,
    }
}
