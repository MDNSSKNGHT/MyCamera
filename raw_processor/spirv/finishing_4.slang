//
// gamma correction
//
RWTexture2D<half4> Rgba;

RWTexture2D<float4> QuantizedRgba;

[shader("compute")]
[numthreads(8, 8, 1)]
void computeMain(uint3 threadId: SV_DispatchThreadID) {
  var coords = threadId.xy;

  float3 in = Rgba[coords].rgb;

  float3 l = 12.92 * in;
  float3 h =
      1.055 * pow(in, 0.41666666666 /* approximation of 1/2.4 */) - 0.055;

  float3 out = select(in <= 0.0031308, l, h);

  // quantization to 0..255 is done automatically by device
  QuantizedRgba[coords] = float4(out, 1.0h);
}
