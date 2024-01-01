use crate::color::Color;

pub struct Viewport
{
    /// dimensions of image in pixels
    pub extent: [u32; 2],
    /// dimensions of view window within virtual space
    pub plane: [f32; 2]
}

impl Viewport
{
    pub fn default() -> Self
    {
        Self
        {
            extent: [1200, 1000],
            plane: [2.0, 2.0]
        }
    }
}


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

    pub fn len(&self) -> f32
    {
        self.square_len().sqrt()
    }

    pub fn normalize(&mut self)
    {
        let magnitude = self.len();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
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


#[derive(Clone, Debug)]
pub struct Voxel
{
    pub color: Color
}


#[derive(Clone, Debug)]
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
