implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiInversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_inverse();
}
public extension AntiScalar {
    public AntiScalar anti_inverse() {
        return AntiScalar(
            /* e1234 */
            1.0 / this.e1234
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public DualNum operator /(AntiInversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_inverse();
}
public extension DualNum {
    public DualNum anti_inverse() {
        return DualNum(
            /* scalar, e1234 */
            float2(this.scalar /  (this.e1234 * this.e1234), 1.0 / this.e1234)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Flector operator /(AntiInversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_inverse();
}
public extension Flector {
    public Flector anti_inverse() {
        float other_g0 = this.e4 * this.e4 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other_g0 * -1.0) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other_g0) * this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Line operator /(AntiInversePrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_inverse();
}
public extension Line {
    public Line anti_inverse() {
        float other_g0 = this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43;
        return Line(
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Motor operator /(AntiInversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_inverse();
}
public extension Motor {
    public Motor anti_inverse() {
        float other_g0 = this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e1234 * this.e1234;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other_g0) * this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other_g0) * this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public MultiVector operator /(AntiInversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_inverse();
}
public extension MultiVector {
    public MultiVector anti_inverse() {
        float other_g0 = this.e1234 * this.e1234 + this.e41 * this.e41 + this.e42 * this.e42 + this.e43 * this.e43 + this.e4 * this.e4 + this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412;
        return MultiVector(
            /* scalar, e1234 */
            float2(other_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other_g0 * -1.0) * this.group1, 
            /* e41, e42, e43 */
            float3(other_g0 * -1.0) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(other_g0 * -1.0) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other_g0) * this.group4
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Origin operator /(AntiInversePrefixOrPostfix lhs, Origin rhs) {
    return rhs.anti_inverse();
}
public extension Origin {
    public Origin anti_inverse() {
        return Origin(
            /* e4 */
            -1.0 / this.e4
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Plane operator /(AntiInversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_inverse();
}
public extension Plane {
    public Plane anti_inverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            (float4(this.e431 * this.e431, this.e423 * this.e423, this.e423 * this.e423, this.e431 * this.e431) * this.group0) + (this.group0 * float4(float2(this.e412 * this.e412), this.e431 * this.e431, this.e412 * this.e412)) + (float4_pow(this.group0.xyzx, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AntiInversePrefixOrPostfix rhs)  {
    return lhs.anti_inverse();
    }
// Fancy prefix trick
public Point operator /(AntiInversePrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_inverse();
}
public extension Point {
    public Point anti_inverse() {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz * float3(-1.0) /  (float4(this.e4).xyz * float4(this.e4).xyz)), -1.0 / this.e4)
        );
    }
}
