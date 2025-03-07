implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.weight();
}
public extension AntiScalar {
    public AntiScalar weight() {
        return this;
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public AntiScalar operator /(WeightPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.weight();
}
public extension DualNum {
    public AntiScalar weight() {
        return AntiScalar(
            /* e1234 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Flector operator /(WeightPrefixOrPostfix lhs, Flector rhs) {
    return rhs.weight();
}
public extension Flector {
    public Flector weight() {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Line operator /(WeightPrefixOrPostfix lhs, Line rhs) {
    return rhs.weight();
}
public extension Line {
    public Line weight() {
        return Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Motor operator /(WeightPrefixOrPostfix lhs, Motor rhs) {
    return rhs.weight();
}
public extension Motor {
    public Motor weight() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public MultiVector operator /(WeightPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.weight();
}
public extension MultiVector {
    public MultiVector weight() {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Origin operator /(WeightPrefixOrPostfix lhs, Origin rhs) {
    return rhs.weight();
}
public extension Origin {
    public Origin weight() {
        return this;
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Plane operator /(WeightPrefixOrPostfix lhs, Plane rhs) {
    return rhs.weight();
}
public extension Plane {
    public Plane weight() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Origin operator /(Point lhs, WeightPrefixOrPostfix rhs)  {
    return lhs.weight();
    }
// Fancy prefix trick
public Origin operator /(WeightPrefixOrPostfix lhs, Point rhs) {
    return rhs.weight();
}
public extension Point {
    public Origin weight() {
        return Origin(
            /* e4 */
            this.e4
        );
    }
}
