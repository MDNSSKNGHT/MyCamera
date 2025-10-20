/* Malvar-He-Cutler */

RWTexture2D<half4> bayer;

[[vk::push_constant]]
cbuffer Params {
  uint width;
  uint height;
}
params;

float sample_bayer(uint2 coord, uint x, uint y) {
  uint new_x = clamp(coord.x + x, 0, params.width - 1);
  uint new_y = clamp(coord.y + y, 0, params.height - 1);
  return bayer[uint2(new_x, new_y)].a;
}

[Shader("compute")]
[NumThreads(4, 4, 1)]
void main(uint3 threadId: SV_DispatchThreadID) {
  uint2 coord = threadId.xy;

  uint x_phase = coord.x & 1;
  uint y_phase = coord.y & 1;

  float4 pixel = float4(0.0);

  switch (y_phase * 2 + x_phase) {
  case 0: /* Red */
    pixel.r = sample_bayer(coord, 0, 0);
    pixel.g =
        (-sample_bayer(coord, 0, -2) * 2.0 + sample_bayer(coord, 0, -1) * 4.0 -
         sample_bayer(coord, -2, 0) * 2.0 + sample_bayer(coord, -1, 0) * 4.0 +
         sample_bayer(coord, 0, 0) * 8.0 + sample_bayer(coord, 1, 0) * 4.0 -
         sample_bayer(coord, 2, 0) * 2.0 + sample_bayer(coord, 0, 1) * 4.0 -
         sample_bayer(coord, 0, 2) * 2.0) /
        16.0;
    pixel.b =
        (-sample_bayer(coord, 0, -2) * 3.0 + sample_bayer(coord, -1, -1) * 4.0 +
         sample_bayer(coord, 1, -1) * 4.0 - sample_bayer(coord, -2, 0) * 3.0 +
         sample_bayer(coord, 0, 0) * 12.0 - sample_bayer(coord, 2, 0) * 3.0 +
         sample_bayer(coord, -1, 1) * 4.0 + sample_bayer(coord, 1, 1) * 4.0 -
         sample_bayer(coord, 0, 2) * 3.0) /
        16.0;
    pixel.a = sample_bayer(coord, 0, 0);
    break;
  case 1: /* Green odd */
    pixel.r =
        (sample_bayer(coord, 0, -2) - sample_bayer(coord, -1, -1) * 2.0 -
         sample_bayer(coord, 1, -1) * 2.0 - sample_bayer(coord, -2, 0) * 2.0 +
         sample_bayer(coord, -1, 0) * 8.0 + sample_bayer(coord, 0, 0) * 10.0 -
         sample_bayer(coord, 1, 0) * 8.0 - sample_bayer(coord, 2, 0) * 2.0 -
         sample_bayer(coord, -1, 1) * 2.0 - sample_bayer(coord, 1, 1) * 2.0 +
         sample_bayer(coord, 0, 2)) /
        16.0;
    pixel.g = sample_bayer(coord, 0, 0);
    pixel.b =
        (sample_bayer(coord, 0, -2) - sample_bayer(coord, -1, -1) * 2.0 -
         sample_bayer(coord, 1, -1) * 2.0 - sample_bayer(coord, -2, 0) * 2.0 +
         sample_bayer(coord, 0, -1) * 8.0 + sample_bayer(coord, 0, 0) * 10.0 -
         sample_bayer(coord, 0, 1) * 8.0 - sample_bayer(coord, 2, 0) * 2.0 -
         sample_bayer(coord, -1, 1) * 2.0 - sample_bayer(coord, 1, 1) * 2.0 +
         sample_bayer(coord, 0, 2)) /
        16.0;
    pixel.a = sample_bayer(coord, 0, 0);
    break;
  case 2: /* Green even */
    pixel.r =
        (sample_bayer(coord, 0, -2) - sample_bayer(coord, -1, -1) * 2.0 -
         sample_bayer(coord, 1, -1) * 2.0 - sample_bayer(coord, -2, 0) * 2.0 +
         sample_bayer(coord, 0, -1) * 8.0 + sample_bayer(coord, 0, 0) * 10.0 -
         sample_bayer(coord, 0, 1) * 8.0 - sample_bayer(coord, 2, 0) * 2.0 -
         sample_bayer(coord, -1, 1) * 2.0 - sample_bayer(coord, 1, 1) * 2.0 +
         sample_bayer(coord, 0, 2)) /
        16.0;
    pixel.g = sample_bayer(coord, 0, 0);
    pixel.b =
        (sample_bayer(coord, 0, -2) - sample_bayer(coord, -1, -1) * 2.0 -
         sample_bayer(coord, 1, -1) * 2.0 - sample_bayer(coord, -2, 0) * 2.0 +
         sample_bayer(coord, -1, 0) * 8.0 + sample_bayer(coord, 0, 0) * 10.0 -
         sample_bayer(coord, 1, 0) * 8.0 - sample_bayer(coord, 2, 0) * 2.0 -
         sample_bayer(coord, -1, 1) * 2.0 - sample_bayer(coord, 1, 1) * 2.0 +
         sample_bayer(coord, 0, 2)) /
        16.0;
    pixel.a = sample_bayer(coord, 0, 0);
    break;
  case 3: /* Blue */
    pixel.r =
        (-sample_bayer(coord, 0, -2) * 3.0 + sample_bayer(coord, -1, -1) * 4.0 +
         sample_bayer(coord, 1, -1) * 4.0 - sample_bayer(coord, -2, 0) * 3.0 +
         sample_bayer(coord, 0, 0) * 12.0 - sample_bayer(coord, 2, 0) * 3.0 +
         sample_bayer(coord, -1, 1) * 4.0 + sample_bayer(coord, 1, 1) * 4.0 -
         sample_bayer(coord, 0, 2) * 3.0) /
        16.0;
    pixel.g =
        (-sample_bayer(coord, 0, -2) * 2.0 + sample_bayer(coord, 0, -1) * 4.0 -
         sample_bayer(coord, -2, 0) * 2.0 + sample_bayer(coord, -1, 0) * 4.0 +
         sample_bayer(coord, 0, 0) * 8.0 + sample_bayer(coord, 1, 0) * 4.0 -
         sample_bayer(coord, 2, 0) * 2.0 + sample_bayer(coord, 0, 1) * 4.0 -
         sample_bayer(coord, 0, 2) * 2.0) /
        16.0;
    pixel.b = sample_bayer(coord, 0, 0);
    pixel.a = sample_bayer(coord, 0, 0);
    break;
  }

  bayer[coord] = half4(pixel);
}
