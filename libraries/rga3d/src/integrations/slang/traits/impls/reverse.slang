implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public AntiScalar operator /(ReversePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.reverse();
}
public extension AntiScalar {
    public AntiScalar reverse() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public DualNum operator /(ReversePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.reverse();
}
public extension DualNum {
    public DualNum reverse() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Flector operator /(ReversePrefixOrPostfix lhs, Flector rhs) {
    return rhs.reverse();
}
public extension Flector {
    public Flector reverse() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Horizon operator /(ReversePrefixOrPostfix lhs, Horizon rhs) {
    return rhs.reverse();
}
public extension Horizon {
    public Horizon reverse() {
        return Horizon(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Line operator /(ReversePrefixOrPostfix lhs, Line rhs) {
    return rhs.reverse();
}
public extension Line {
    public Line reverse() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Motor operator /(ReversePrefixOrPostfix lhs, Motor rhs) {
    return rhs.reverse();
}
public extension Motor {
    public Motor reverse() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public MultiVector operator /(ReversePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.reverse();
}
public extension MultiVector {
    public MultiVector reverse() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Origin operator /(ReversePrefixOrPostfix lhs, Origin rhs) {
    return rhs.reverse();
}
public extension Origin {
    public Origin reverse() {
        return this;
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Plane operator /(ReversePrefixOrPostfix lhs, Plane rhs) {
    return rhs.reverse();
}
public extension Plane {
    public Plane reverse() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Point operator /(ReversePrefixOrPostfix lhs, Point rhs) {
    return rhs.reverse();
}
public extension Point {
    public Point reverse() {
        return this;
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, ReversePrefixOrPostfix rhs)  {
    return lhs.reverse();
    }
// Fancy prefix trick
public Scalar operator /(ReversePrefixOrPostfix lhs, Scalar rhs) {
    return rhs.reverse();
}
public extension Scalar {
    public Scalar reverse() {
        return this;
    }
}
