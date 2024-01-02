#![allow(non_snake_case, unused_imports, unused_variables)]

use voxel_marcher::{
    color::{Color, BLUE, NO_COLOR, RED},
    graphics::*,
    ppm::write_ppm3,
    viewport::*,
    vector::{Vec3, UP},
    voxel::*
};

fn main()
{
    let viewport = Viewport
    {
        extent: [50, 50],
        plane: [1.0, 1.0]
    };

    let grid = VoxelGrid
    {
        pos: Vec3::new(0.0, 0.0, 5.0),
        width: 1,
        height: 1,
        length: 1,
        voxels: vec![
            vec![
                Voxel { color: RED }
            ],
        ]
    };

    let buffer: Vec<Color> =
    {
        let dx = viewport.plane[0] / viewport.extent[0] as f32;
        let dy = viewport.plane[1] / viewport.extent[1] as f32;
        // top left corner of view plane in space
        let po = Vec3::new(
            -viewport.plane[0] / 2.0,
            viewport.plane[1] / 2.0,
            1.0
        );

        let mut image = Vec::new();

        for j in 0..=viewport.extent[1]
        {
            for i in 0..=viewport.extent[0]
            {
                let pij = Vec3::new(
                    (dx * i as f32) + po.x,
                    (dy * j as f32) - po.y,
                    po.z
                );
                let rij = pij.normalize();
                println!("Pij {:?}", pij);
                image.push(raymarch(pij, rij, &grid));
            }
        }

        image
    };

    write_ppm3("sample.ppm", &buffer, viewport.extent[0], viewport.extent[1]);
}
