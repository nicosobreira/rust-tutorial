use std::fmt;
use std::rc::Rc;

pub struct Trees {
    texture: Rc<Texture>,
    trees: Vec<Tree>,
}

struct Point<T> {
    x: T,
    y: T,
}

struct Texture {
    pub image: String,
}

struct Tree {
    texture: Rc<Texture>,
    position: Point<i32>,
    color: i32,
}

impl Trees {
    pub fn new(image: String) -> Self {
        let texture = Rc::new(Texture { image });

        Self {
            texture: texture.clone(),
            trees: Vec::new(),
        }
    }

    pub fn add(&mut self, x: i32, y: i32, color: i32) {
        let tree = Tree {
            texture: self.texture.clone(),
            position: Point::new(x, y),
            color: color,
        };

        self.trees.push(tree);
    }

    pub fn draw(&self) {
        for tree in &self.trees {
            tree.draw();
            println!();
        }
    }
}

impl Tree {
    fn draw(&self) {
        println!("Tree: ");
        println!("\t texture: {}", self.texture.image);
        println!("\t position: {}", self.position);
        println!("\t color: {}", self.color);
    }
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
