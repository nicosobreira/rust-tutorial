fn main() {
    let mut v = vec![0, 9, 8, 2];

    for n in &mut v {
        *n += 80;
        print!("{n} ");
    }

    println!();

    // Failed at compile time
    // let does_not_exist = &v[100]

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("No third element"),
    }

    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("The string is: {s}");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s = "áôîÊ";
    println!("Length = {}", s.len());

    for c in s.chars() {
        print!("{c} ");
    }
    println!();
}
