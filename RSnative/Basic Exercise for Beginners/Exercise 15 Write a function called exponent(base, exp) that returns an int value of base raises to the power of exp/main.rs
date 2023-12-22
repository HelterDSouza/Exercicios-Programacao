fn exponent(base: i32, exp: i32) -> i32 {
    let exp = match exp {
        n if n < 0 => 0,
        0 => return 1,
        _ => exp,
    };

    (1..exp).fold(base, |acc, _| acc * base)
}
fn main() {
    let base = 2;
    let exp = 1;
    let result = exponent(base, exp);
    println!("{base} raises to the power of {exp}: {result}");
}
