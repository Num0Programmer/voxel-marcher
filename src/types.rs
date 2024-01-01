use crate::color::Color;

/// virtual representation of image in space onto which the scene can be
/// projected
pub struct Viewport
{
    /// position in space
    pub pos: Vec3,
    pub width: u32,  // in pixels
    pub height: u32  // in pixels
}


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

    pub fn add(&mut self, b: &Vec3)
    {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    pub fn len(&self) -> f32
    {
        self.square_len().sqrt()
    }

    pub fn normalize(&self) -> Self
    {
        let magnitude = self.len();
        Self
        {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn square_len(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}


pub struct Voxel
{
    pub color: Color
}


/// regular 3-dimensional matrix of voxels
pub struct VoxelGrid
{
    /// position of grid's top left corner in space
    pub pos: Vec3,
    pub width: u32,   // in voxels
    pub height: u32,  // in voxels
    pub length: u32,  // in voxels
    /// matrix of voxels which describe geometry
    pub voxels: Vec<Vec<Voxel>>
}
