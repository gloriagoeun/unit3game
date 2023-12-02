use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpriteOption {
    Storage,
    Uniform,
    VertexBuffer,
}
#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUSprite {
    pub screen_region: [f32; 4],
    pub sheet_region: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUCamera {
    pub screen_pos: [f32; 2],
    pub screen_size: [f32; 2],
}

#[cfg(all(not(feature = "uniforms"), not(feature = "vbuf")))]
pub const SPRITES: SpriteOption = SpriteOption::Storage;
#[cfg(feature = "uniforms")]
pub const SPRITES: SpriteOption = SpriteOption::Uniform;
#[cfg(feature = "vbuf")]
pub const SPRITES: SpriteOption = SpriteOption::VertexBuffer;
#[cfg(all(feature = "vbuf", feature = "uniform"))]
compile_error!("Can't choose both vbuf and uniform sprite features");