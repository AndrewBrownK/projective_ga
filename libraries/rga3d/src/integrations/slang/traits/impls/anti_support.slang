implementing rga3d;
// Fancy postfix trick
public Horizon operator /(DualNum lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Horizon operator /(AntiSupportPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_support();
}
public extension DualNum {
    public Horizon anti_support() {
        return Horizon(
            /* e321 */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Motor operator /(Flector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Motor operator /(AntiSupportPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_support();
}
public extension Flector {
    public Motor anti_support() {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * float3(-1.0)), this.e321)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Horizon lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Scalar operator /(AntiSupportPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.anti_support();
}
public extension Horizon {
    public Scalar anti_support() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Point operator /(Line lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Point operator /(AntiSupportPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_support();
}
public extension Line {
    public Point anti_support() {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(Motor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Flector operator /(AntiSupportPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_support();
}
public extension Motor {
    public Flector anti_support() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public MultiVector operator /(AntiSupportPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_support();
}
public extension MultiVector {
    public MultiVector anti_support() {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.e321, 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group3.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Scalar operator /(AntiSupportPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_support();
}
public extension Plane {
    public Scalar anti_support() {
        return Scalar(
            /* scalar */
            this.e321
        );
    }
}
// Fancy postfix trick
public Line operator /(Point lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Line operator /(AntiSupportPrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_support();
}
public extension Point {
    public Line anti_support() {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Scalar lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Horizon operator /(AntiSupportPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_support();
}
public extension Scalar {
    public Horizon anti_support() {
        return Horizon(
            /* e321 */
            this.scalar
        );
    }
}
