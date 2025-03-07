implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public AntiScalar operator /(AutoMorphismPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.auto_morphism();
}
public extension AntiScalar {
    public AntiScalar auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public DualNum operator /(AutoMorphismPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.auto_morphism();
}
public extension DualNum {
    public DualNum auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Flector operator /(AutoMorphismPrefixOrPostfix lhs, Flector rhs) {
    return rhs.auto_morphism();
}
public extension Flector {
    public Flector auto_morphism() {
        return Flector(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Horizon operator /(Horizon lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Horizon operator /(AutoMorphismPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.auto_morphism();
}
public extension Horizon {
    public Horizon auto_morphism() {
        return Horizon(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Line operator /(AutoMorphismPrefixOrPostfix lhs, Line rhs) {
    return rhs.auto_morphism();
}
public extension Line {
    public Line auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Motor operator /(AutoMorphismPrefixOrPostfix lhs, Motor rhs) {
    return rhs.auto_morphism();
}
public extension Motor {
    public Motor auto_morphism() {
        return this;
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public MultiVector operator /(AutoMorphismPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.auto_morphism();
}
public extension MultiVector {
    public MultiVector auto_morphism() {
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
public Origin operator /(Origin lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Origin operator /(AutoMorphismPrefixOrPostfix lhs, Origin rhs) {
    return rhs.auto_morphism();
}
public extension Origin {
    public Origin auto_morphism() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Plane operator /(AutoMorphismPrefixOrPostfix lhs, Plane rhs) {
    return rhs.auto_morphism();
}
public extension Plane {
    public Plane auto_morphism() {
        return Plane(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Point operator /(AutoMorphismPrefixOrPostfix lhs, Point rhs) {
    return rhs.auto_morphism();
}
public extension Point {
    public Point auto_morphism() {
        return Point(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, AutoMorphismPrefixOrPostfix rhs)  {
    return lhs.auto_morphism();
    }
// Fancy prefix trick
public Scalar operator /(AutoMorphismPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.auto_morphism();
}
public extension Scalar {
    public Scalar auto_morphism() {
        return this;
    }
}
