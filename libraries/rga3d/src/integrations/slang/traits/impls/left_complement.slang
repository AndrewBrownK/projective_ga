implementing rga3d;
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Scalar operator /(LeftComplementPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.left_complement();
}
public extension AntiScalar {
    public Scalar left_complement() {
        return Scalar(
            /* scalar */
            this.e1234
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public DualNum operator /(LeftComplementPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.left_complement();
}
public extension DualNum {
    public DualNum left_complement() {
        return DualNum(
            /* scalar, e1234 */
            this.group0.yx
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Flector operator /(LeftComplementPrefixOrPostfix lhs, Flector rhs) {
    return rhs.left_complement();
}
public extension Flector {
    public Flector left_complement() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Horizon lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Origin operator /(LeftComplementPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.left_complement();
}
public extension Horizon {
    public Origin left_complement() {
        return Origin(
            /* e4 */
            this.e321
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Line operator /(LeftComplementPrefixOrPostfix lhs, Line rhs) {
    return rhs.left_complement();
}
public extension Line {
    public Line left_complement() {
        return Line(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Motor operator /(LeftComplementPrefixOrPostfix lhs, Motor rhs) {
    return rhs.left_complement();
}
public extension Motor {
    public Motor left_complement() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public MultiVector operator /(LeftComplementPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.left_complement();
}
public extension MultiVector {
    public MultiVector left_complement() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.yx, 
            /* e1, e2, e3, e4 */
            this.group4, 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Origin lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Horizon operator /(LeftComplementPrefixOrPostfix lhs, Origin rhs) {
    return rhs.left_complement();
}
public extension Origin {
    public Horizon left_complement() {
        return Horizon(
            /* e321 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Point operator /(Plane lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Point operator /(LeftComplementPrefixOrPostfix lhs, Plane rhs) {
    return rhs.left_complement();
}
public extension Plane {
    public Point left_complement() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Point lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public Plane operator /(LeftComplementPrefixOrPostfix lhs, Point rhs) {
    return rhs.left_complement();
}
public extension Point {
    public Plane left_complement() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, LeftComplementPrefixOrPostfix rhs)  {
    return lhs.left_complement();
    }
// Fancy prefix trick
public AntiScalar operator /(LeftComplementPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.left_complement();
}
public extension Scalar {
    public AntiScalar left_complement() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}
