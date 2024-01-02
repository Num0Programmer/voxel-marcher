use crate::color::Color;
use crate::vector::Vec3;


pub const MIN_DIST: f32 = 2.0;
pub const VOXEL_DIM: f32 = 1.0;


#[derive(Clone, Debug)]
pub struct Voxel
{
    pub color: Color
}


/// regular 3-dimensional matrix of voxels
#[derive(Clone, Debug)]
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

impl VoxelGrid
{
    pub fn distance_from(&self, p: Vec3) -> f32
    {
        (
            ((p.x - self.pos.x) * (p.x - self.pos.x))
            + ((p.y - self.pos.y) * (p.y - self.pos.y))
            + ((p.z - self.pos.z) * (p.z - self.pos.z))
        )
        .sqrt()
    }
}
