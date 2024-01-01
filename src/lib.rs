pub mod color;
pub mod graphics;
pub mod types;
pub mod vector;

pub mod ppm
{
    use std::{fs::File, io::Write};
    use crate::color::Color;

    pub fn write_ppm3(path: &str, buf: &Vec<Color>, width: u32, height: u32)
    {
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "P3").unwrap();
        writeln!(&mut file, "{} {}", width, height).unwrap();
        writeln!(&mut file, "255").unwrap();

        buf.iter().for_each(|color|
        {
            writeln!(&mut file, "{}", color[0]).unwrap();
        });
    }
}
