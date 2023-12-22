fn reverse(mut number: i32) -> String {
    let mut reversed_string = String::new();
    while number != 0 {
        let last_digit = number % 10;
        reversed_string.push_str(&format!("{} ", last_digit));
        number /= 10;
    }
    reversed_string
}

fn main() {
    let number = 7536;

    println!(
        "The digits of {} in reverse order are: {}",
        number,
        reverse(number)
    );
}
