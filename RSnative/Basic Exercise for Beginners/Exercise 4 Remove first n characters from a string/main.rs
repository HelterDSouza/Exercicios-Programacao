fn remove_chars(text: &str, index: usize) -> String {
    text.chars().skip(index).collect::<String>()
}

fn main() {
    println!("Removing characters from a string");
    println!("{}", remove_chars("pynative", 4));
    println!("{}", remove_chars("pynative", 2));
}
