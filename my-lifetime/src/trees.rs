struct Texture {
    image: String,
}

struct Tree<'a> {
    texture: &'a Texture,
    position: Point<i32>,
    color: i32,
}

struct Trees<'a> {
    texture: Texture,
    trees: Vec<Tree<'a>>,
}
