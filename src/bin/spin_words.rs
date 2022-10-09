fn split_words(string: &str) -> String {
    string
        .split_whitespace()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("{}", split_words("Hello World!"));
}
