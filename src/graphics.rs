use crate::color::{Color, NO_COLOR, RED};
use crate::vector::Vec3;
use crate::voxel::*;


const MAX_ITERS: u32 = 10;


pub fn raymarch(point: Vec3, ray: Vec3, grid: &VoxelGrid) -> Color
{
    let mut p = point;
    for _ in 0..MAX_ITERS
    {
        let d = grid.pos.sub(point);

        println!("Dist {}", d.len());

        if d.len() <= MIN_DIST
        {
            return RED;
        }

        p = p.add(ray.scalar(d.len()));
    }
    println!();

    NO_COLOR
}
