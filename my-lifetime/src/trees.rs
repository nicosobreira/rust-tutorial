mod point;

use point::Point;

struct Texture {
    pub image: String,
}

pub struct Tree<'a> {
    texture: &'a Texture,
    position: Point<i32>,
    color: i32,
}

pub struct Trees<'a> {
    texture: Texture,
    trees: Vec<Tree<'a>>,
}

impl Trees {
    fn new(image: String) -> Trees {
        Trees {
            texture: Texture { image: image },
        }
    }
}
