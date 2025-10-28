//
// normalization of input to 0..1 (half precision)
//
RWTexture2D<uint16_t> rawImage;

RWTexture2D<half> rawNormalizedImage;

[push_constant]
cbuffer Uniforms {
  uint whiteLevel;
  uint blackLevel;
}

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 threadId: SV_DispatchThreadID) {
  var coords = threadId.xy;

  var val = rawImage[coords];

  // remove sensor bias by subtracting the black level
  float norm = float(val - blackLevel) / float(whiteLevel - blackLevel);

  rawNormalizedImage[coords] = half(norm);
}
