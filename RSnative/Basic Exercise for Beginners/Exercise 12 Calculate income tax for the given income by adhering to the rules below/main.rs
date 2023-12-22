fn calc_tax(income: i32) -> i32 {
    match income {
        0..=9_999 => 0,
        10_000..=20_000 => (income - 10000) * 20 / 100,
        _ => (10000 * 10 / 100) + (income - 20000) * 20 / 100,
    }
}
fn main() {
    let income = 45_000;
    let tax_payable = calc_tax(income);
    println!("Total tax to pay is {tax_payable}")
}
