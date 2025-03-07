implementing rga3d;
// Fancy postfix trick
public Scalar operator /(AntiScalar lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Scalar operator /(RightComplementPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.right_complement();
}
public extension AntiScalar {
    public Scalar right_complement() {
        return Scalar(
            /* scalar */
            this.e1234
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public DualNum operator /(RightComplementPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.right_complement();
}
public extension DualNum {
    public DualNum right_complement() {
        return DualNum(
            /* scalar, e1234 */
            this.group0.yx
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Flector operator /(RightComplementPrefixOrPostfix lhs, Flector rhs) {
    return rhs.right_complement();
}
public extension Flector {
    public Flector right_complement() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
// Fancy postfix trick
public Origin operator /(Horizon lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Origin operator /(RightComplementPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.right_complement();
}
public extension Horizon {
    public Origin right_complement() {
        return Origin(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Line operator /(RightComplementPrefixOrPostfix lhs, Line rhs) {
    return rhs.right_complement();
}
public extension Line {
    public Line right_complement() {
        return Line(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Motor operator /(RightComplementPrefixOrPostfix lhs, Motor rhs) {
    return rhs.right_complement();
}
public extension Motor {
    public Motor right_complement() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public MultiVector operator /(RightComplementPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.right_complement();
}
public extension MultiVector {
    public MultiVector right_complement() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.yx, 
            /* e1, e2, e3, e4 */
            this.group4 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Origin lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Horizon operator /(RightComplementPrefixOrPostfix lhs, Origin rhs) {
    return rhs.right_complement();
}
public extension Origin {
    public Horizon right_complement() {
        return Horizon(
            /* e321 */
            this.e4
        );
    }
}
// Fancy postfix trick
public Point operator /(Plane lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Point operator /(RightComplementPrefixOrPostfix lhs, Plane rhs) {
    return rhs.right_complement();
}
public extension Plane {
    public Point right_complement() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Point lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public Plane operator /(RightComplementPrefixOrPostfix lhs, Point rhs) {
    return rhs.right_complement();
}
public extension Point {
    public Plane right_complement() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Scalar lhs, RightComplementPrefixOrPostfix rhs)  {
    return lhs.right_complement();
    }
// Fancy prefix trick
public AntiScalar operator /(RightComplementPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.right_complement();
}
public extension Scalar {
    public AntiScalar right_complement() {
        return AntiScalar(
            /* e1234 */
            this.scalar
        );
    }
}
