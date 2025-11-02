//
// normalization of input to 0..1 (half precision)
//
RWTexture2D<uint16_t> Raw;
RWTexture2D<half> RawNormalized;

[push_constant]
cbuffer Uniforms {
  float4 colorGains;
  int4 blackLevel;
  uint whiteLevel;
}

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 threadId: SV_DispatchThreadID) {
  int x = threadId.x;
  int y = threadId.y;

  // // Assuming RGGB pattern: [R G; G B]
  uint xPhase = x & 1;
  uint yPhase = y & 1;

  // remove sensor bias by subtracting the black level
  float norm = float(Raw[int2(x, y)] - blackLevel[0]) /
               float(whiteLevel - blackLevel[0]);

  RawNormalized[int2(x, y)] =
      (yPhase == 0) ? ((xPhase == 0) ? half(norm * colorGains[0])
                                     : half(norm * colorGains[1]))
                    : ((xPhase == 0) ? half(norm * colorGains[2])
                                     : half(norm * colorGains[3]));
}
