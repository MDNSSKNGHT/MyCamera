mod finishing_2 {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shader/finishing_2.glsl"
    }
}

pub use finishing_2::*;
