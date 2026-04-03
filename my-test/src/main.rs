#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width: width,
            height: height,
        }
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

fn main() {
    println!("Hello, world! {}", wrong_add(1, 3));
}

fn wrong_add(a: i32, b: i32) -> i32 {
    a + b - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_will_fail() {
        let a = 5;
        let b = 9;

        let result = wrong_add(a, b);

        assert_eq!(
            result, 14,
            "The function `add` has a result diferent than 14"
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(20, 30);
        let smaller = Rectangle::new(10, 5);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let smaller = Rectangle::new(10, 5);
        let larger = Rectangle::new(20, 30);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn guess_greater_than_100() {
        Guess::new(242);
    }
}
