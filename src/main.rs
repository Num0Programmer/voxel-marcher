use voxel_marcher::{
    color::{Color, BLUE, NO_COLOR, RED},
    graphics::*,
    ppm::write_ppm3,
    types::*,
    vector::{Vec3, UP}
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

    let buffer: Vec<Color> =
    {
        // prerequisite calculations
        let E = Vec3::new(0.0, 0.0, -1.0);  // viewport position
        let T = Vec3::new(0.0, 0.0, 2.0);  // some point beyond of the viewport
        let t = T.sub(E);
        let b = t.cross(&UP);
        let tn = t.normalize();
        let bn = b.normalize();
        let vn = tn.cross(&bn);
        // viewport calculations
        let gx = viewport.extent[0] as f32 / 2.0;
        let gy = viewport.extent[1] as f32 / 2.0;
        let qx = bn.scalar((2.0 * gx) / viewport.plane[0]);
        let qy = vn.scalar((2.0 * gy) / viewport.plane[1]);
        let p1m = tn.sub(bn.scalar(gx)).sub(vn.scalar(gy));

        let mut image = Vec::new();

        for j in 0..viewport.extent[1]
        {
            for i in 0..viewport.extent[0]
            {
                let pij = p1m
                    .add(&qx.scalar(i as f32 - 1.0))
                    .add(&qy.scalar(j as f32 - 1.0));
                let rij = pij.normalize();

                image.push(BLUE);
            }
        }

        image
    };

    write_ppm3("sample.ppm", &buffer, viewport.extent[0], viewport.extent[1]);
}
