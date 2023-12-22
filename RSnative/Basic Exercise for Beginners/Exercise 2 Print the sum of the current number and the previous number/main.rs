fn main() {
    for i in 0..10 {
        let prev = if i == 0 { 0 } else { i - 1 };

        println!(
            "Current Number {0} Previous Number  {1}  Sum:  {2}",
            i,
            prev,
            prev + i
        )
    }
}
