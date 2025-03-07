implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiAutoMorphismPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_auto_morphism();
}
public extension AntiScalar {
    public AntiScalar anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public DualNum operator /(AntiAutoMorphismPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_auto_morphism();
}
public extension DualNum {
    public DualNum anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Flector operator /(AntiAutoMorphismPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_auto_morphism();
}
public extension Flector {
    public Flector anti_auto_morphism() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Horizon operator /(AntiAutoMorphismPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.anti_auto_morphism();
}
public extension Horizon {
    public Horizon anti_auto_morphism() {
        return Horizon(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Line operator /(AntiAutoMorphismPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_auto_morphism();
}
public extension Line {
    public Line anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Motor operator /(AntiAutoMorphismPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_auto_morphism();
}
public extension Motor {
    public Motor anti_auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public MultiVector operator /(AntiAutoMorphismPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_auto_morphism();
}
public extension MultiVector {
    public MultiVector anti_auto_morphism() {
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
public Origin operator /(Origin lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Origin operator /(AntiAutoMorphismPrefixOrPostfix lhs, Origin rhs) {
    return rhs.anti_auto_morphism();
}
public extension Origin {
    public Origin anti_auto_morphism() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Plane operator /(AntiAutoMorphismPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_auto_morphism();
}
public extension Plane {
    public Plane anti_auto_morphism() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Point operator /(AntiAutoMorphismPrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_auto_morphism();
}
public extension Point {
    public Point anti_auto_morphism() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AntiAutoMorphismPrefixOrPostfix rhs)  {
    return lhs.anti_auto_morphism();
    }
// Fancy prefix trick
public Scalar operator /(AntiAutoMorphismPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_auto_morphism();
}
public extension Scalar {
    public Scalar anti_auto_morphism() {
        return this;
    }
}
