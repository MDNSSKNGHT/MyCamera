//
// Malvar-He-Cutler demosaic
//
RWTexture2D<half> rawNormalizedImage;

RWTexture2D<half4> rgbaImage;

[push_constant]
cbuffer Uniforms { uint4 cfa; }

uint2 clampToEdge(int2 coords) {
  uint2 size;
  rawNormalizedImage.GetDimensions(size.x, size.y);

  uint2 safe = clamp(coords, uint2(0, 0), size - uint2(1, 1));
  return safe;
}

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 threadId: SV_DispatchThreadID) {
  var coords = threadId.xy;

  var xPhase = coords.x & 1;
  var yPhase = coords.y & 1;

  var index = yPhase * 2 + xPhase;

  float R, G, B;

  switch (cfa[index]) {
  case 0:
    R = rawNormalizedImage[coords];
    G = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         4.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 1))] +
                rawNormalizedImage[clampToEdge(coords + int2(-1, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 0))]) +
         8.0 * rawNormalizedImage[coords]) /
        16.0;
    B = (-3.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         4.0 * (rawNormalizedImage[clampToEdge(coords + int2(-1, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(-1, 1))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 1))]) +
         12.0 * rawNormalizedImage[coords]) /
        16.0;
    break;
  case 1:
    R = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         8.0 * (rawNormalizedImage[clampToEdge(coords + int2(-1, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 0))]) -
         2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 2))]) +
         10.0 * rawNormalizedImage[coords]) /
        16.0;
    G = rawNormalizedImage[coords];
    B = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))]) +
         8.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 1))]) -
         2.0 * (rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         10.0 * rawNormalizedImage[coords]) /
        16.0;
    break;
  case 2:
    R = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))]) +
         8.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 1))]) -
         2.0 * (rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         10.0 * rawNormalizedImage[coords]) /
        16.0;
    G = rawNormalizedImage[coords];
    B = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         8.0 * (rawNormalizedImage[clampToEdge(coords + int2(-1, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 0))]) -
         2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 2))]) +
         10.0 * rawNormalizedImage[coords]) /
        16.0;
    break;
  case 3:
    R = (-3.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         4.0 * (rawNormalizedImage[clampToEdge(coords + int2(-1, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(-1, 1))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 1))]) +
         12.0 * rawNormalizedImage[coords]) /
        16.0;
    G = (-2.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(0, 2))] +
                 rawNormalizedImage[clampToEdge(coords + int2(-2, 0))] +
                 rawNormalizedImage[clampToEdge(coords + int2(2, 0))]) +
         4.0 * (rawNormalizedImage[clampToEdge(coords + int2(0, -1))] +
                rawNormalizedImage[clampToEdge(coords + int2(0, 1))] +
                rawNormalizedImage[clampToEdge(coords + int2(-1, 0))] +
                rawNormalizedImage[clampToEdge(coords + int2(1, 0))]) +
         8.0 * rawNormalizedImage[coords]) /
        16.0;
    B = rawNormalizedImage[coords];
    break;
  }

  rgbaImage[coords] = half4(half(R), half(G), half(B), 1.0h);
}
