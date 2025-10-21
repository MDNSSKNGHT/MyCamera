mod finishing_1 {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "shader/finishing_1.glsl"
    }
}

pub use finishing_1::*;
