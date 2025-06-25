pub struct CollisionBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl CollisionBox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        CollisionBox { x, y, w, h }
    }

    pub fn collides_with(&self, other: &CollisionBox) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}
