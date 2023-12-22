use std::io;

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => String::new(),
    }
}
fn parse_to_i32(input: &str) -> i32 {
    input.trim().parse::<i32>().unwrap()
}
fn calculate_multiplication_sum(number1: i32, number2: i32) -> i32 {
    let product = number1 * number2;

    if product > 1000 {
        return number1 + number2;
    }
    product
}
fn main() {
    let result =
        calculate_multiplication_sum(parse_to_i32(&read_input()), parse_to_i32(&read_input()));
    // let result = 600;
    println!("The result is {}", result);
}
