use vulkano::buffer::BufferContents;

#[derive(BufferContents)]
#[repr(C)]
pub struct Stage1Parameters {
    pub stride: u32,
    pub white_level: u32,
    pub black_level: u32,
}

#[derive(BufferContents)]
#[repr(C)]
pub struct Stage2Parameters {
    pub width: u32,
    pub height: u32,
}
