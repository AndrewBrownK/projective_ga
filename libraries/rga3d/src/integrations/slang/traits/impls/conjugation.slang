implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public AntiScalar operator /(ConjugationPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.conjugation();
}
public extension AntiScalar {
    public AntiScalar conjugation() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public DualNum operator /(ConjugationPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.conjugation();
}
public extension DualNum {
    public DualNum conjugation() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Flector operator /(ConjugationPrefixOrPostfix lhs, Flector rhs) {
    return rhs.conjugation();
}
public extension Flector {
    public Flector conjugation() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Horizon operator /(ConjugationPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.conjugation();
}
public extension Horizon {
    public Horizon conjugation() {
        return this;
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Line operator /(ConjugationPrefixOrPostfix lhs, Line rhs) {
    return rhs.conjugation();
}
public extension Line {
    public Line conjugation() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Motor operator /(ConjugationPrefixOrPostfix lhs, Motor rhs) {
    return rhs.conjugation();
}
public extension Motor {
    public Motor conjugation() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public MultiVector operator /(ConjugationPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.conjugation();
}
public extension MultiVector {
    public MultiVector conjugation() {
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
public Origin operator /(Origin lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Origin operator /(ConjugationPrefixOrPostfix lhs, Origin rhs) {
    return rhs.conjugation();
}
public extension Origin {
    public Origin conjugation() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Plane operator /(ConjugationPrefixOrPostfix lhs, Plane rhs) {
    return rhs.conjugation();
}
public extension Plane {
    public Plane conjugation() {
        return this;
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Point operator /(ConjugationPrefixOrPostfix lhs, Point rhs) {
    return rhs.conjugation();
}
public extension Point {
    public Point conjugation() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, ConjugationPrefixOrPostfix rhs)  {
    return lhs.conjugation();
    }
// Fancy prefix trick
public Scalar operator /(ConjugationPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.conjugation();
}
public extension Scalar {
    public Scalar conjugation() {
        return this;
    }
}
