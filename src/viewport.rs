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
