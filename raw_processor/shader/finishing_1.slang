StructuredBuffer<uint16_t> in;
RWTexture2D<half> bayer;

[[vk::push_constant]]
cbuffer Params {
  uint width;
  uint height;

  uint whiteLevel;
  uint blackLevel;

  float4 colorGains;
}
params;

[Shader("compute")]
[NumThreads(4, 4, 1)]
void main(uint3 threadId: SV_DispatchThreadID) {
  uint2 coord = threadId.xy;

  uint16_t pixel = in[coord.y * params.width + coord.x];
  float corrected = float(pixel - params.blackLevel) /
                    float(params.whiteLevel - params.blackLevel);

  uint xPhase = coord.x & 1;
  uint yPhase = coord.y & 1;

  float colorGain = half(params.colorGains[yPhase * 2 + xPhase]);

  bayer[coord] = half(clamp(corrected * colorGain, 0.0, 1.0));
}
