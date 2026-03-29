struct Email {
    text: String,
}

struct User {
    name: String,
    email: Email,
    is_active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl User {
    fn new(name: String, email: Email) -> Self {
        Self {
            // Because both the field and variable have the same name you can write like this
            // Instead of `name: name`,
            name,
            email,
            is_active: true,
        }
    }
}

fn main() {
    let name = String::from("Nícolas");
    let email = Email {
        text: String::from("someone@gmail.com"),
    };

    let a = User::new(name, email);

    let b = User {
        name: String::from("Carlos"),
        // The `..` syntax means that everything from the struct a is pulled in to b, except `name`
        ..a
    };

    let r1 = Rectangle {
        width: 100,
        height: 100,
    };

    let r2 = Rectangle::square(10);

    let r3 = Rectangle {
        width: 20,
        height: 10,
    };

    println!("Hello, world!");

    println!("{r1:#?} with area {}", r1.area());

    println!("r1 and r2 {}", r1.can_hold(&r2));
    println!("r1 and r3 {}", r1.can_hold(&r3));
    println!("r2 and r3 {}", r2.can_hold(&r3));
}
