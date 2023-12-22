fn main() {
    for x in 1..=5 {
        let value = format!("{} ", x).repeat(x);
        println!("{}", value)
    }
}
