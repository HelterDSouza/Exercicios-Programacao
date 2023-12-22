fn main() {
    let str_x = "Emma is good developer. Emma is a writer";
    let appeared_times = str_x.matches("Emma").count();
    println!("Emma appeared {} times", appeared_times)
}
