float2 float2_pow(float2 v, float p) {
    return float2(pow(v.x, p), pow(v.y, p));
}
float3 float3_pow(float3 v, float p) {
    return float3(pow(v.x, p), pow(v.y, p), pow(v.z, p));
}
float4 float4_pow(float4 v, float p) {
    return float4(pow(v.x, p), pow(v.y, p), pow(v.z, p), pow(v.w, p));
}
__include data;
__include traits;
