#version 460

layout(local_size_x = 4, local_size_y = 4, local_size_z = 1) in;

layout(set = 0, binding = 0, r16ui) uniform readonly uimage2D img;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    uvec4 pixel = imageLoad(img, coord);
    // Add some actual work so the shader isn't empty
    // if (pixel.r > 1000u) {
    //     // imageStore(img, coord, uvec4(0u, 0u, 0u, 0u));
    // }
}
