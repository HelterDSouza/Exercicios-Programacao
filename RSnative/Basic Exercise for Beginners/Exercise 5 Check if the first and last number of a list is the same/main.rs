fn check_first_lest_equal(array: &[i32]) -> bool {
    array.first() == array.last()
}
fn main() {
    let numbers_x = [10, 20, 30, 40, 10];
    let numbers_y = [75, 65, 35, 75, 30];
    println!("Given list: {:?}", numbers_x);
    println!("result is {}", check_first_lest_equal(&numbers_x));
    println!("Given list: {:?}", numbers_y);
    println!("result is {}", check_first_lest_equal(&numbers_y));
}
