use itertools::Itertools;

fn count_duplicate(text: &str) -> u32 {
    text.to_lowercase()
        .chars()
        .counts()
        .values()
        .filter(|&&i| i > 1)
        .count() as u32
}

fn main() {
    let string = "Hello World";
    let count = count_duplicate(string);
    println!("{} has {} duplicate characters", string, count);
}
