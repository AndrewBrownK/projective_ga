implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiReversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_reverse();
}
public extension AntiScalar {
    public AntiScalar anti_reverse() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public DualNum operator /(AntiReversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_reverse();
}
public extension DualNum {
    public DualNum anti_reverse() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Flector operator /(AntiReversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_reverse();
}
public extension Flector {
    public Flector anti_reverse() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Horizon operator /(AntiReversePrefixOrPostfix lhs, Horizon rhs) {
    return rhs.anti_reverse();
}
public extension Horizon {
    public Horizon anti_reverse() {
        return this;
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Line operator /(AntiReversePrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_reverse();
}
public extension Line {
    public Line anti_reverse() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Motor operator /(AntiReversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_reverse();
}
public extension Motor {
    public Motor anti_reverse() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public MultiVector operator /(AntiReversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_reverse();
}
public extension MultiVector {
    public MultiVector anti_reverse() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Origin operator /(AntiReversePrefixOrPostfix lhs, Origin rhs) {
    return rhs.anti_reverse();
}
public extension Origin {
    public Origin anti_reverse() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Plane operator /(AntiReversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_reverse();
}
public extension Plane {
    public Plane anti_reverse() {
        return this;
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Point operator /(AntiReversePrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_reverse();
}
public extension Point {
    public Point anti_reverse() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AntiReversePrefixOrPostfix rhs)  {
    return lhs.anti_reverse();
    }
// Fancy prefix trick
public Scalar operator /(AntiReversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_reverse();
}
public extension Scalar {
    public Scalar anti_reverse() {
        return this;
    }
}
