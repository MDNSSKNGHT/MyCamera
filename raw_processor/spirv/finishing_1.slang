// 16-bit RAW data
RWTexture2D<uint16_t> RAW;

// 16-bit floating point RGBA data (after demosaic)
RWTexture2D<half4> RGBA;

uint2 offsetCoordinates(uint2 coordinates, int offsetX, int offsetY) {
  uint width, height;
  RAW.GetDimensions(width, height);

  uint clampX = clamp(coordinates.x + offsetX, 0, width - 1);
  uint clampY = clamp(coordinates.y + offsetY, 0, height - 1);

  return uint2(clampX, clampY);
}

[Shader("compute")]
[NumThreads(8, 8, 1)]
void main(uint3 threadId: SV_DispatchThreadID) {
  uint2 coordinates = threadId.xy;

  uint xPhase = coordinates.x & 1;
  uint yPhase = coordinates.y & 1;

  uint index = yPhase * 2 + xPhase;

  float R, G, B;

  // Pass color filter arrangement
  switch (index) {
  case 0:
    R = RAW[coordinates];
    G = (-2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)] +
                 RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         4.0 * (RAW[offsetCoordinates(coordinates, 0, -1)] +
                RAW[offsetCoordinates(coordinates, 0, 1)] +
                RAW[offsetCoordinates(coordinates, -1, 0)] +
                RAW[offsetCoordinates(coordinates, 1, 0)]) +
         8.0 * RAW[coordinates]) /
        16.0;
    B = (-3.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)] +
                 RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         4.0 * (RAW[offsetCoordinates(coordinates, -1, -1)] +
                RAW[offsetCoordinates(coordinates, 1, -1)] +
                RAW[offsetCoordinates(coordinates, -1, 1)] +
                RAW[offsetCoordinates(coordinates, 1, 1)]) +
         12.0 * RAW[coordinates]) /
        16.0;
    break;
  case 1:
    R = (-2.0 * (RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         8.0 * (RAW[offsetCoordinates(coordinates, -1, 0)] +
                RAW[offsetCoordinates(coordinates, 1, 0)]) -
         2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                RAW[offsetCoordinates(coordinates, 0, 2)]) +
         10.0 * RAW[coordinates]) /
        16.0;
    G = RAW[coordinates];
    B = (-2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)]) +
         8.0 * (RAW[offsetCoordinates(coordinates, 0, -1)] +
                RAW[offsetCoordinates(coordinates, 0, 1)]) -
         2.0 * (RAW[offsetCoordinates(coordinates, -2, 0)] +
                RAW[offsetCoordinates(coordinates, 2, 0)]) +
         10.0 * RAW[coordinates]) /
        16.0;
    break;
  case 2:
    R = (-2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)]) +
         8.0 * (RAW[offsetCoordinates(coordinates, 0, -1)] +
                RAW[offsetCoordinates(coordinates, 0, 1)]) -
         2.0 * (RAW[offsetCoordinates(coordinates, -2, 0)] +
                RAW[offsetCoordinates(coordinates, 2, 0)]) +
         10.0 * RAW[coordinates]) /
        16.0;
    G = RAW[coordinates];
    B = (-2.0 * (RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         8.0 * (RAW[offsetCoordinates(coordinates, -1, 0)] +
                RAW[offsetCoordinates(coordinates, 1, 0)]) -
         2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                RAW[offsetCoordinates(coordinates, 0, 2)]) +
         10.0 * RAW[coordinates]) /
        16.0;
    break;
  case 3:
    R = (-3.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)] +
                 RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         4.0 * (RAW[offsetCoordinates(coordinates, -1, -1)] +
                RAW[offsetCoordinates(coordinates, 1, -1)] +
                RAW[offsetCoordinates(coordinates, -1, 1)] +
                RAW[offsetCoordinates(coordinates, 1, 1)]) +
         12.0 * RAW[coordinates]) /
        16.0;
    G = (-2.0 * (RAW[offsetCoordinates(coordinates, 0, -2)] +
                 RAW[offsetCoordinates(coordinates, 0, 2)] +
                 RAW[offsetCoordinates(coordinates, -2, 0)] +
                 RAW[offsetCoordinates(coordinates, 2, 0)]) +
         4.0 * (RAW[offsetCoordinates(coordinates, 0, -1)] +
                RAW[offsetCoordinates(coordinates, 0, 1)] +
                RAW[offsetCoordinates(coordinates, -1, 0)] +
                RAW[offsetCoordinates(coordinates, 1, 0)]) +
         8.0 * RAW[coordinates]) /
        16.0;
    B = RAW[coordinates];
    break;
  }

  // Normalization
  R = (R - 0.0) / 1023.0;
  G = (R - 0.0) / 1023.0;
  B = (R - 0.0) / 1023.0;

  RGBA[coordinates] = half4(half(R), half(G), half(B), 1.0h);
}
