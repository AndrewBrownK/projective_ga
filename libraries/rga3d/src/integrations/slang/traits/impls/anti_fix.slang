implementing rga3d;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, AntiFixPrefixOrPostfix rhs)  {
    return lhs.anti_fix();
    }
// Fancy prefix trick
public AntiScalar operator /(AntiFixPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.anti_fix();
}
public extension AntiScalar {
    public AntiScalar anti_fix() {
        return AntiScalar(
            /* e1234 */
            1.0
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, AntiFixPrefixOrPostfix rhs)  {
    return lhs.anti_fix();
    }
// Fancy prefix trick
public Origin operator /(AntiFixPrefixOrPostfix lhs, Origin rhs) {
    return rhs.anti_fix();
}
public extension Origin {
    public Origin anti_fix() {
        return Origin(
            /* e4 */
            1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, AntiFixPrefixOrPostfix rhs)  {
    return lhs.anti_fix();
    }
// Fancy prefix trick
public Plane operator /(AntiFixPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_fix();
}
public extension Plane {
    public Plane anti_fix() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e423 * this.e423 + this.e431 * this.e431 + this.e412 * this.e412) * this.group0
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, AntiFixPrefixOrPostfix rhs)  {
    return lhs.anti_fix();
    }
// Fancy prefix trick
public Point operator /(AntiFixPrefixOrPostfix lhs, Point rhs) {
    return rhs.anti_fix();
}
public extension Point {
    public Point anti_fix() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0/this.e4) * this.group0
        );
    }
}
