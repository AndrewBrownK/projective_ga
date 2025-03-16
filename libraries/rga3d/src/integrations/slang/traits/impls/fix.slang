implementing rga3d;
// Fancy postfix trick
public Horizon operator /(Horizon lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Horizon operator /(FixPrefixOrPostfix lhs, Horizon rhs) {
    return rhs.fix();
}
public extension Horizon {
    public Horizon fix() {
        return Horizon(
            /* e321 */
            1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Plane operator /(FixPrefixOrPostfix lhs, Plane rhs) {
    return rhs.fix();
}
public extension Plane {
    public Plane fix() {
        return Plane(
            /* e423, e431, e412, e321 */
            float4((this.group0.xyz / this.group0.www), 1.0)
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Point operator /(FixPrefixOrPostfix lhs, Point rhs) {
    return rhs.fix();
}
public extension Point {
    public Point fix() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 3), pow(this.e2, 3), pow(this.e3, 3), this.e3 * this.e3 * this.e4) + (float4_pow(this.group0.yxxx, 2) * this.group0) + (float4_pow(this.group0.zzyy, 2) * this.group0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Scalar lhs, FixPrefixOrPostfix rhs)  {
    return lhs.fix();
    }
// Fancy prefix trick
public Scalar operator /(FixPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.fix();
}
public extension Scalar {
    public Scalar fix() {
        return Scalar(
            /* scalar */
            1.0
        );
    }
}
