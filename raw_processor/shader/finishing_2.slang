/* 32-bit floating point normalized bayer data */
Texture2D<float> bayerBuffer;

/* 32-bit floating point normalized RGB debayered data */
RWTexture2D<float3> rgbBuffer;

[vk_push_constant]
cbuffer Params {
  uint width;
  uint height;
}
params;

uint2 offsetCoordinates(uint2 coordinates, int offsetX, int offsetY) {
  uint clampX = clamp(coordinates.x + offsetX, 0, params.width - 1);
  uint clampY = clamp(coordinates.y + offsetY, 0, params.height - 1);
  return uint2(clampX, clampY);
}

[Shader("compute")]
[NumThreads(4, 4, 1)]
void main(uint3 threadId: SV_DispatchThreadID) {
  uint2 coordinates = threadId.xy;

  uint xPhase = coordinates.x & 1;
  uint yPhase = coordinates.y & 1;

  uint index = yPhase * 2 + xPhase;

  float3 rgb = float3(0.0);

  /* Pass color filter arrangement */
  switch (index) {
  case 0:
    rgb.r = bayerBuffer[coordinates];
    rgb.g = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)] +
                     bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             4.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 1)] +
                    bayerBuffer[offsetCoordinates(coordinates, -1, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 0)]) +
             8.0 * bayerBuffer[coordinates]) /
            16.0;
    rgb.b = (-3.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)] +
                     bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             4.0 * (bayerBuffer[offsetCoordinates(coordinates, -1, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, -1, 1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 1)]) +
             12.0 * bayerBuffer[coordinates]) /
            16.0;
    break;
  case 1:
    rgb.r = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             8.0 * (bayerBuffer[offsetCoordinates(coordinates, -1, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 0)]) -
             2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) +
             10.0 * bayerBuffer[coordinates]) /
            16.0;
    rgb.g = bayerBuffer[coordinates];
    rgb.b = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) +
             8.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 1)]) -
             2.0 * (bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             10.0 * bayerBuffer[coordinates]) /
            16.0;
    break;
  case 2:
    rgb.r = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) +
             8.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 1)]) -
             2.0 * (bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             10.0 * bayerBuffer[coordinates]) /
            16.0;
    rgb.g = bayerBuffer[coordinates];
    rgb.b = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             8.0 * (bayerBuffer[offsetCoordinates(coordinates, -1, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 0)]) -
             2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) +
             10.0 * bayerBuffer[coordinates]) /
            16.0;
    break;
  case 3:
    rgb.r = (-3.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)] +
                     bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             4.0 * (bayerBuffer[offsetCoordinates(coordinates, -1, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, -1, 1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 1)]) +
             12.0 * bayerBuffer[coordinates]) /
            16.0;
    rgb.g = (-2.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] +
                     bayerBuffer[offsetCoordinates(coordinates, 0, 2)] +
                     bayerBuffer[offsetCoordinates(coordinates, -2, 0)] +
                     bayerBuffer[offsetCoordinates(coordinates, 2, 0)]) +
             4.0 * (bayerBuffer[offsetCoordinates(coordinates, 0, -1)] +
                    bayerBuffer[offsetCoordinates(coordinates, 0, 1)] +
                    bayerBuffer[offsetCoordinates(coordinates, -1, 0)] +
                    bayerBuffer[offsetCoordinates(coordinates, 1, 0)]) +
             8.0 * bayerBuffer[coordinates]) /
            16.0;
    rgb.b = bayerBuffer[coordinates];
    break;
  }

  // switch (index) {
  // case 0:
  //   data.r = bayerBuffer[coordinates];
  //   data.g = (-bayerBuffer[offsetCoordinates(coordinates, 0, -2)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, -1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 0)] * 4.0 +
  //             bayerBuffer[coordinates] * 8.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 0)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)] * 2.0) /
  //            16.0;
  //   data.b = (-bayerBuffer[offsetCoordinates(coordinates, 0, -2)] * 3.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 4.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 3.0 +
  //             bayerBuffer[coordinates] * 12.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 3.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 4.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)] * 3.0) /
  //            16.0;

  //   break;
  // case 1:
  //   data.r = (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 0)] * 8.0 +
  //             bayerBuffer[coordinates] * 10.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 0)] * 8.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) /
  //            16.0;
  //   data.g = bayerBuffer[coordinates];
  //   data.b = (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, -1)] * 8.0 +
  //             bayerBuffer[coordinates] * 10.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 1)] * 8.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) /
  //            16.0;
  //   break;
  // case 2:
  //   data.r = (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, -1)] * 8.0 +
  //             bayerBuffer[coordinates] * 10.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 1)] * 8.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) /
  //            16.0;
  //   data.g = bayerBuffer[coordinates];
  //   data.b = (bayerBuffer[offsetCoordinates(coordinates, 0, -2)] -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 0)] * 8.0 +
  //             bayerBuffer[coordinates] * 10.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 0)] * 8.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 2.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)]) /
  //            16.0;
  //   break;
  // case 3:
  //   data.r = (-bayerBuffer[offsetCoordinates(coordinates, 0, -2)] * 3.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, -1)] * 4.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, -1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 3.0 +
  //             bayerBuffer[coordinates] * 12.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 3.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 1)] * 4.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)] * 3.0) /
  //            16.0;
  //   data.g = (-bayerBuffer[offsetCoordinates(coordinates, 0, -2)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, -1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, -2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, -1, 0)] * 4.0 +
  //             bayerBuffer[coordinates] * 8.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 1, 0)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 2, 0)] * 2.0 +
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 1)] * 4.0 -
  //             bayerBuffer[offsetCoordinates(coordinates, 0, 2)] * 2.0) /
  //            16.0;
  //   data.b = bayerBuffer[coordinates];
  //   break;
  // }

  rgbBuffer[coordinates] = rgb;
}
