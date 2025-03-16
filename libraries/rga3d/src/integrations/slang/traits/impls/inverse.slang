implementing rga3d;
// Fancy postfix trick
public DualNum operator /(DualNum lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public DualNum operator /(InversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.inverse();
}
public extension DualNum {
    public DualNum inverse() {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0 / this.scalar, this.e1234 /  (this.scalar * this.scalar))
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Flector operator /(InversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.inverse();
}
public extension Flector {
    public Flector inverse() {
        float other_g0 = this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e321 * this.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * -1.0) * this.group1
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Horizon operator /(InversePrefixOrPostfix lhs, Horizon rhs) {
    return rhs.inverse();
}
public extension Horizon {
    public Horizon inverse() {
        return Horizon(
            /* e321 */
            -1.0 / this.e321
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Line operator /(InversePrefixOrPostfix lhs, Line rhs) {
    return rhs.inverse();
}
public extension Line {
    public Line inverse() {
        float other_g0 = this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12;
        return Line(
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Motor operator /(InversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.inverse();
}
public extension Motor {
    public Motor inverse() {
        float other_g0 = this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.scalar * this.scalar;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public MultiVector operator /(InversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.inverse();
}
public extension MultiVector {
    public MultiVector inverse() {
        float other_g0 = this.scalar * this.scalar + this.e1 * this.e1 + this.e2 * this.e2 + this.e3 * this.e3 + this.e23 * this.e23 + this.e31 * this.e31 + this.e12 * this.e12 + this.e321 * this.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0) * this.group1, 
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_g0 * -1.0) * this.group4
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Plane operator /(InversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.inverse();
}
public extension Plane {
    public Plane inverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e423 /  (this.e321 * this.e321), this.e431 /  (this.e321 * this.e321), this.e412 /  (this.e321 * this.e321), 1.0 / this.e321) * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Point operator /(InversePrefixOrPostfix lhs, Point rhs) {
    return rhs.inverse();
}
public extension Point {
    public Point inverse() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 3), pow(this.e2, 3), pow(this.e3, 3), this.e3 * this.e3 * this.e4) + (float4_pow(this.group0.yxxx, 2) * this.group0) + (float4_pow(this.group0.zzyy, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, InversePrefixOrPostfix rhs)  {
    return lhs.inverse();
    }
// Fancy prefix trick
public Scalar operator /(InversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.inverse();
}
public extension Scalar {
    public Scalar inverse() {
        return Scalar(
            /* scalar */
            1.0 / this.scalar
        );
    }
}
