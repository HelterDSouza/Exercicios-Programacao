fn main() {
    let array = [10, 20, 33, 46, 55];
    println!("Given list is  {:?}", array);
    println!("Divisible by 5");
    array
        .into_iter()
        .filter(|&x| x % 5 == 0)
        .for_each(|x| println!("{}", x));
}
