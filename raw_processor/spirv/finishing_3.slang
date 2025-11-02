//
// sensor color space to CIE XYZ and then to linear sRGB
//
RWTexture2D<half4> Rgba;

[push_constant]
cbuffer Uniforms {
  float3x3 forwardMatrix1;
  float3x3 forwardMatrix2;
}

static const float3x3 M_XYZ_to_sRGB = float3x3(
    3.2406, -1.5372, -0.4986, -0.9689, 1.8758, 0.0415, 0.0557, -0.2040, 1.0570);

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 threadId: SV_DispatchThreadID) {
  int x = threadId.x;
  int y = threadId.y;

  float3 RGB = Rgba[int2(x, y)].rgb;

  float3 XYZ = mul(RGB, forwardMatrix1);

  float3 sRGB = mul(M_XYZ_to_sRGB, XYZ);

  Rgba[int2(x, y)] = half4(half3(sRGB), 1.0h);
}
