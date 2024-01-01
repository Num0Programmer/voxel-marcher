use crate::color::Color;
use crate::vector::Vec3;

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
            extent: [800, 600],
            plane: [2.0, 2.0]
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
