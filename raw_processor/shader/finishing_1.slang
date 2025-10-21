/* 16-bit unsigned integer RAW buffer data */
StructuredBuffer<uint16_t> rawBuffer;

/* 32-bit floating point normalized bayer data */
RWTexture2D<float> bayerBuffer;

[vk::push_constant]
cbuffer Parameters {
  uint stride;
  uint whiteLevel;
  uint blackLevel;
}
params;

[Shader("compute")]
[NumThreads(4, 4, 1)]
void main(uint3 threadId: SV_DispatchThreadID) {
  uint2 coordinates = threadId.xy;

  float pixel = float(rawBuffer[coordinates.y * params.stride + coordinates.x]);
  float normalized = float(pixel - params.blackLevel) /
                     float(params.whiteLevel - params.blackLevel);

  bayerBuffer[coordinates] = normalized;
}
