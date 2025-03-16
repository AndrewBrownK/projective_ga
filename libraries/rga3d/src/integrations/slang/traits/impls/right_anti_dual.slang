implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiScalar operator /(RightAntiDualPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.right_anti_dual();
}
public extension DualNum {
    public AntiScalar right_anti_dual() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Flector operator /(RightAntiDualPrefixOrPostfix lhs, Flector rhs) {
    return rhs.right_anti_dual();
}
public extension Flector {
    public Flector right_anti_dual() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Horizon lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Origin operator /(RightAntiDualPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.right_anti_dual();
}
public extension Horizon {
    public Origin right_anti_dual() {
        return Origin(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Line operator /(RightAntiDualPrefixOrPostfix lhs, Line rhs) {
    return rhs.right_anti_dual();
}
public extension Line {
    public Line right_anti_dual() {
        return Line(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Motor operator /(RightAntiDualPrefixOrPostfix lhs, Motor rhs) {
    return rhs.right_anti_dual();
}
public extension Motor {
    public Motor right_anti_dual() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public MultiVector operator /(RightAntiDualPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.right_anti_dual();
}
public extension MultiVector {
    public MultiVector right_anti_dual() {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Plane lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Origin operator /(RightAntiDualPrefixOrPostfix lhs, Plane rhs) {
    return rhs.right_anti_dual();
}
public extension Plane {
    public Origin right_anti_dual() {
        return Origin(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Point lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public Plane operator /(RightAntiDualPrefixOrPostfix lhs, Point rhs) {
    return rhs.right_anti_dual();
}
public extension Point {
    public Plane right_anti_dual() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, RightAntiDualPrefixOrPostfix rhs)  {
    return lhs.right_anti_dual();
    }
// Fancy prefix trick
public AntiScalar operator /(RightAntiDualPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.right_anti_dual();
}
public extension Scalar {
    public AntiScalar right_anti_dual() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}
