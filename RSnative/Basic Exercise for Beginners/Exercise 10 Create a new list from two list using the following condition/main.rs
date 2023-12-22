fn main() {
    let list1 = [10, 20, 25, 30, 35];
    let list2 = [40, 45, 60, 75, 90];
    let new_list: Vec<_> = list1
        .iter()
        .filter(|x| *x % 2 == 1)
        .chain(list2.iter().filter(|x| *x % 2 == 0).collect::<Vec<_>>())
        .collect();
    println!("result list: {new_list:?}");
}
