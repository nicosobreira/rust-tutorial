#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

struct ShirtStock {
    color: ShirtColor,
    stock: u32,
}

pub fn example() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Green,
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
}

impl ShirtStock {
    fn new(color: ShirtColor) -> Self {
        let stock = 0;
        Self { color, stock }
    }

    fn add(&mut self) {
        self.stock += 1;
    }

    fn max<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.stock > other.stock {
            return self;
        } else {
            return other;
        }
    }
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red = ShirtStock::new(ShirtColor::Red);
        let mut blue = ShirtStock::new(ShirtColor::Blue);
        let mut green = ShirtStock::new(ShirtColor::Green);

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red.add(),
                ShirtColor::Blue => blue.add(),
                ShirtColor::Green => green.add(),
            }
        }

        let max = red.max(&blue).max(&green);

        max.color
    }
}
