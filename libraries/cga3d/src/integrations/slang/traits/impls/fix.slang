implementing cga3d;
// Fancy postfix trick
public AntiFlatPoint operator /(AntiFlatPoint lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(FixPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.fix();
}
public extension AntiFlatPoint {
    public AntiFlatPoint fix() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.group0.xyz / float4(this.e321).xyz), 1.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(AntiPlane lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public AntiPlane operator /(FixPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.fix();
}
public extension AntiPlane {
    public AntiPlane fix() {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e2 * this.e2, this.e1 * this.e1, this.e1 * this.e1, this.e2 * this.e2) * this.group0) + (this.group0 * float4(float2(this.e3 * this.e3), this.e2 * this.e2, this.e3 * this.e3)) + (float4_pow(this.group0.xyzx, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public AntiScalar operator /(FixPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.fix();
}
public extension AntiScalar {
    public AntiScalar fix() {
        return AntiScalar(
            /* e12345 */
            -1.0
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(FlatPoint lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public FlatPoint operator /(FixPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.fix();
}
public extension FlatPoint {
    public FlatPoint fix() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.group0.xyz * float3(-1.0) / float4(this.e45).xyz), -1.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Plane operator /(FixPrefixOrPostfix lhs, Plane rhs) {
    return rhs.fix();
}
public extension Plane {
    public Plane fix() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            -(float4(this.e4315 * this.e4315, this.e4235 * this.e4235, this.e4235 * this.e4235, this.e4315 * this.e4315) * this.group0) - (this.group0 * float4(float2(this.e4125 * this.e4125), this.e4315 * this.e4315, this.e4125 * this.e4125)) - (float4_pow(this.group0.xyzx, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(RoundPoint lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public RoundPoint operator /(FixPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.fix();
}
public extension RoundPoint {
    public RoundPoint fix() {
        float geometric_product_g0 = this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 - 2.0 * (this.e4 * this.e5);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(geometric_product_g0) * this.group0, 
            /* e5 */
            geometric_product_g0 * this.e5
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Scalar operator /(FixPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.fix();
}
public extension Scalar {
    public Scalar fix() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Sphere lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Sphere operator /(FixPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.fix();
}
public extension Sphere {
    public Sphere fix() {
        float geometric_product_g0 = 2.0*(this.e3215 * this.e1234) - this.e4235 * this.e4235 - this.e4315 * this.e4315 - this.e4125 * this.e4125;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_product_g0) * this.group0, 
            /* e1234 */
            geometric_product_g0 * this.e1234
        );
    }
}
