use std::cmp::PartialOrd;

mod point;
mod summary;

use point::Point;
use summary::{NewsArticle, PostType, SocialPost};

fn sum_2(a: &mut i32) -> i32 {
    *a += 10;
    *a + 2
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = find_largest(&number_list);

    println!("The largest number is {largest}");

    let p = Point::new(27.2, 98.0);
    let p2 = Point::new(32.0, 18.0);

    println!("Sum = {}", point::sum(&p, &p2));

    println!("Point length is {}", p.length_from_origin());
    p.cmp_display();

    let mut a: i32 = 2;
    println!("{:?}", sum_2(&mut a));
    println!("{:?}", a);

    let article = NewsArticle {
        headline: "Babies are flying!".to_string(),
        location: "Brazil".to_string(),
        author: "Paulo Cardoso".to_string(),
        content: "bla bla bla".to_string(),
    };

    let post = SocialPost {
        username: "Acerola".to_string(),
        content: "Pigs are flying".to_string(),
        post_type: PostType::Repost {
            from_user: "fofocas".to_string(),
        },
    };

    summary::notify(&post);
    summary::notify(&article);
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
