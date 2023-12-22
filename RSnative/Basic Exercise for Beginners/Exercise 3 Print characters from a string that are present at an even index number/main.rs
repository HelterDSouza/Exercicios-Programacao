use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    input
}
fn main() {
    let user_input = get_input();
    println!("Orginal String is  {}", user_input);
    println!("Printing only even index chars");
    user_input
        .chars()
        .step_by(2)
        .for_each(|c| println!("{}", c));
}
