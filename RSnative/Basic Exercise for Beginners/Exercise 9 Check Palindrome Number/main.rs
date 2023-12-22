fn invert_number(number: usize) -> usize {
    let mut x_number = number;
    let mut reversed = 0;
    while x_number > 0 {
        let last_digit = x_number % 10;
        reversed = (reversed * 10) + last_digit;
        x_number /= 10;
    }
    reversed
}
fn main() {
    let number = 125;
    println!("original number {}", number);
    let reversed_number = invert_number(number);
    match number == reversed_number {
        true => println!("Yes. given number is palindrome number"),
        false => println!("No. given number is not palindrome number"),
    }
}
