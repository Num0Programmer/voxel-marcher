use voxel_marcher::{
    color::{Color, NO_COLOR, RED},
    graphics::*,
    types::*
};

fn main()
{
    let viewport = Viewport::default();

    let grid = VoxelGrid
    {
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

    let frame: Vec<Color> =
    {
        let dx = viewport.plane[0] / viewport.extent[0] as f32;
        let dy = viewport.plane[1] / viewport.extent[1] as f32;
        let mut ray = Vec3::new(
            -(viewport.plane[0] / 2.0) + 0.5,
            (viewport.plane[1] / 2.0) - 0.5,
            0.0
        );
        let mut image = Vec::new();

        for _row in 0..viewport.extent[1]
        {
            for _col in 0..viewport.extent[0]
            {
                image.push(raymarch(&ray, &grid));

                ray.x += dx;
                ray.y += dy;
            }
        }

        image
    };
}
