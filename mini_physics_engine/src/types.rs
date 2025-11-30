#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
    pub fn scale(self, s: f32) -> Vec2 {
        Vec2 { x: self.x * s, y: self.y * s }
    }
    pub fn zero()->Self{Self{x: 0.0, y: 0.0}}
    pub fn sub(self, other: Vec2)->Vec2{
        Vec2 { x: self.x-other.x, y: self.y-other.y }
    }
    pub fn dot(self, other: Vec2)->f32{
        self.x*other.x+self.y*other.y
    }
    pub fn length(self)->f32{
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalized(self)->Vec2{
        let len = self.length();
        if len == 0.0 {Self::zero()} else {self.scale(1.0/len)}
    }
    pub fn div(self, s: f32) -> Vec2 {
        self.scale(1.0 / s)
    }
}