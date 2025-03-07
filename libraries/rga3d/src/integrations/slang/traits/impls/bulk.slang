implementing rga3d;
// Fancy postfix trick
public Scalar operator /(DualNum lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Scalar operator /(BulkPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.bulk();
}
public extension DualNum {
    public Scalar bulk() {
        return Scalar(
            /* scalar */
            this.scalar
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Flector operator /(BulkPrefixOrPostfix lhs, Flector rhs) {
    return rhs.bulk();
}
public extension Flector {
    public Flector bulk() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Horizon operator /(BulkPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.bulk();
}
public extension Horizon {
    public Horizon bulk() {
        return this;
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Line operator /(BulkPrefixOrPostfix lhs, Line rhs) {
    return rhs.bulk();
}
public extension Line {
    public Line bulk() {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Motor operator /(BulkPrefixOrPostfix lhs, Motor rhs) {
    return rhs.bulk();
}
public extension Motor {
    public Motor bulk() {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public MultiVector operator /(BulkPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.bulk();
}
public extension MultiVector {
    public MultiVector bulk() {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Plane lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Horizon operator /(BulkPrefixOrPostfix lhs, Plane rhs) {
    return rhs.bulk();
}
public extension Plane {
    public Horizon bulk() {
        return Horizon(
            /* e321 */
            this.e321
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Point operator /(BulkPrefixOrPostfix lhs, Point rhs) {
    return rhs.bulk();
}
public extension Point {
    public Point bulk() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, BulkPrefixOrPostfix rhs)  {
    return lhs.bulk();
    }
// Fancy prefix trick
public Scalar operator /(BulkPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.bulk();
}
public extension Scalar {
    public Scalar bulk() {
        return this;
    }
}
