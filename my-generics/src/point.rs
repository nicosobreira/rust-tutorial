use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point<f64> {
    pub fn length_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

pub fn sum<T>(a: &Point<T>, b: &Point<T>) -> Point<T>
where
    T: Add<Output = T> + Copy,
{
    let result = Point::new(a.x + b.x, a.y + b.y);

    result
}
