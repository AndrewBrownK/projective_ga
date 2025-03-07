implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public AntiScalar operator /(DoubleComplementPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.double_complement();
}
public extension AntiScalar {
    public AntiScalar double_complement() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public DualNum operator /(DoubleComplementPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.double_complement();
}
public extension DualNum {
    public DualNum double_complement() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Flector operator /(DoubleComplementPrefixOrPostfix lhs, Flector rhs) {
    return rhs.double_complement();
}
public extension Flector {
    public Flector double_complement() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Horizon operator /(DoubleComplementPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.double_complement();
}
public extension Horizon {
    public Horizon double_complement() {
        return Horizon(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Line operator /(DoubleComplementPrefixOrPostfix lhs, Line rhs) {
    return rhs.double_complement();
}
public extension Line {
    public Line double_complement() {
        return this;
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Motor operator /(DoubleComplementPrefixOrPostfix lhs, Motor rhs) {
    return rhs.double_complement();
}
public extension Motor {
    public Motor double_complement() {
        return this;
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public MultiVector operator /(DoubleComplementPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.double_complement();
}
public extension MultiVector {
    public MultiVector double_complement() {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Origin operator /(DoubleComplementPrefixOrPostfix lhs, Origin rhs) {
    return rhs.double_complement();
}
public extension Origin {
    public Origin double_complement() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Plane operator /(DoubleComplementPrefixOrPostfix lhs, Plane rhs) {
    return rhs.double_complement();
}
public extension Plane {
    public Plane double_complement() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Point operator /(DoubleComplementPrefixOrPostfix lhs, Point rhs) {
    return rhs.double_complement();
}
public extension Point {
    public Point double_complement() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, DoubleComplementPrefixOrPostfix rhs)  {
    return lhs.double_complement();
    }
// Fancy prefix trick
public Scalar operator /(DoubleComplementPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.double_complement();
}
public extension Scalar {
    public Scalar double_complement() {
        return this;
    }
}
