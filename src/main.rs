use voxel_marcher::{
    color::{NO_COLOR, RED},
    types::*
};

fn main()
{
    let viewport = Viewport {
        pos: Vec3::new(0.0, 0.0, -1.0),
        width: 800,
        height: 600
    };

    let grid = VoxelGrid {
        pos: Vec3::new(0.0, 0.0, 10.0),
        width: 2,
        height: 2,
        length: 2,
        voxels: vec![
            vec![
                Voxel { color: RED },      Voxel { color: NO_COLOR },
                Voxel { color: NO_COLOR }, Voxel { color: NO_COLOR }
            ],
            vec![
                Voxel { color: NO_COLOR }, Voxel { color: NO_COLOR },
                Voxel { color: NO_COLOR }, Voxel { color: NO_COLOR }
            ],
        ]
    };
}
