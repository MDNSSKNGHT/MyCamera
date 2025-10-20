#version 460

layout(local_size_x = 4, local_size_y = 4, local_size_z = 1) in;

layout(set = 0, binding = 0, r16ui) uniform uimage2D bayer;

layout(push_constant) uniform Uniform {
    vec4 color_gains;
    uint white_level;
    uint black_level;
} params;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    uint pixel = imageLoad(bayer, coord).r;

    int x_phase = coord.x & 1;
    int y_phase = coord.y & 1;
    float color_gain = params.color_gains[y_phase * 2 + x_phase];

    float white_factor = 65535.0 / float(params.white_level - params.black_level);
    uint corrected = uint(clamp(
                (pixel - params.black_level) * color_gain * white_factor, 0, 65535.0));

    imageStore(bayer, coord, uvec4(corrected));
}
