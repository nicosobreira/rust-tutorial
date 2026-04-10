pub fn example() {
    let list = vec![0, 2, 3, 1, 4, 9, 8];

    let iter = list.iter();

    let sum: i32 = iter.sum();

    println!("Sum: {sum}");

    let even_list: Vec<_> = list.iter().filter(|x| *x % 2 == 0).collect();

    for e in &even_list {
        print!("{e} ");
    }
    println!();
}
