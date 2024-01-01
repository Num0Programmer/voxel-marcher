pub const FORWARD: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
pub const UP: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

#[derive(Clone, Debug)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Self
    {
        Self { x, y, z }
    }

    pub fn add(&self, b: &Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z
        }
    }

    pub fn cross(&self, b: &Vec3) -> Vec3
    {
        Vec3
        {
            x: (self.y * b.z) - (self.z * b.y),
            y: (self.z * b.x) - (self.x * b.z),
            z: (self.x * b.y) - (self.y * b.x)
        }
    }

    pub fn dot(&self, b: &Vec3) -> f32
    {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
    }

    pub fn len(&self) -> f32
    {
        self.square_len().sqrt()
    }

    pub fn normalize(&self) -> Vec3
    {
        let magnitude = self.len();
        Vec3
        {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }

    pub fn scalar(&self, scalar: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    pub fn square_len(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn sub(&self, b: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z
        }
    }
}
