pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x,
            y
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Vector2 {
            x: self.x / scale,
            y: self.y / scale
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}