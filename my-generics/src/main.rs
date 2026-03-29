use std::cmp::PartialOrd;

mod point;
mod summary;

use point::Point;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = find_largest(&number_list);

    println!("The largest number is {largest}");

    let p = Point::new(27.2, 98.0);

    println!("Point length is {}", p.length_from_origin());
}

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
