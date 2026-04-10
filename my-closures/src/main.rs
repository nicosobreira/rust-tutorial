mod borrow;
mod iter;
mod shirts;
mod sort;

fn main() {
    shirts::example();

    println!();

    borrow::example();

    println!();

    let mut list = vec![1, 2, 3];
    println!("Before thread spawn: {list:?}");

    std::thread::spawn(move || {
        list.push(5);
        println!("Inside thread: {list:?}");
    })
    .join()
    .unwrap();

    println!();

    sort::example();

    println!();

    iter::example();
}
