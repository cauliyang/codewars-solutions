fn descending_order(x: u64) -> u64 {
    let mut temp = x.to_string().chars().collect::<Vec<char>>();
    temp.sort_by(|a, b| b.cmp(a));
    String::from_iter(temp.into_iter()).parse::<u64>().unwrap()
}

fn main() {
    println!("{}", descending_order(14362));
}
