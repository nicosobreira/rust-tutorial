pub mod garden;

use garden::Vegetable;

mod parent {
    struct Parent {
        a: i32,
    }

    mod child {
        struct Child {
            p: crate::parent::Parent,
        }
    }
}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

fn main() {
    garden::teste::execucao();
    println!("Hello, world!");

    let v = Vegetable {};
    println!("I'm a {v:?}");
}
