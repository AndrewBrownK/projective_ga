implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public AntiScalar operator /(UnitizePrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.unitize();
}
public extension AntiScalar {
    public AntiScalar unitize() {
        return AntiScalar(
            /* e1234 */
            1.0
        );
    }
}
// Fancy postfix trick
public DualNum operator /(DualNum lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public DualNum operator /(UnitizePrefixOrPostfix lhs, DualNum rhs) {
    return rhs.unitize();
}
public extension DualNum {
    public DualNum unitize() {
        return DualNum(
            /* scalar, e1234 */
            float2(1.0/this.e1234) * this.group0.xy
        );
    }
}
// Fancy postfix trick
public Flector operator /(Flector lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Flector operator /(UnitizePrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitize();
}
public extension Flector {
    public Flector unitize() {
        Flector sub_type = Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            pow(sub_type.e4, 2) + pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)
        );
        return Flector(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
// Fancy postfix trick
public Line operator /(Line lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Line operator /(UnitizePrefixOrPostfix lhs, Line rhs) {
    return rhs.unitize();
}
public extension Line {
    public Line unitize() {
        Line sub_type = Line(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2)
        );
        return Line(
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(Motor lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Motor operator /(UnitizePrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitize();
}
public extension Motor {
    public Motor unitize() {
        Motor sub_type = Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2) + pow(sub_type.e1234, 2)
        );
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product.e1234) * this.group1
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public MultiVector operator /(UnitizePrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitize();
}
public extension MultiVector {
    public MultiVector unitize() {
        MultiVector sub_type = MultiVector(
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
        AntiScalar geometric_anti_product = AntiScalar(
            /* e1234 */
            pow(sub_type.e1234, 2) + pow(sub_type.e4, 2) + pow(sub_type.e41, 2) + pow(sub_type.e42, 2) + pow(sub_type.e43, 2) + pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)
        );
        return MultiVector(
            /* scalar, e1234 */
            float2(geometric_anti_product.e1234) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(geometric_anti_product.e1234) * this.group2.xyz, 
            /* e23, e31, e12 */
            float3(geometric_anti_product.e1234) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product.e1234) * this.group4
        );
    }
}
// Fancy postfix trick
public Origin operator /(Origin lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Origin operator /(UnitizePrefixOrPostfix lhs, Origin rhs) {
    return rhs.unitize();
}
public extension Origin {
    public Origin unitize() {
        return Origin(
            /* e4 */
            1.0
        );
    }
}
// Fancy postfix trick
public Plane operator /(Plane lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Plane operator /(UnitizePrefixOrPostfix lhs, Plane rhs) {
    return rhs.unitize();
}
public extension Plane {
    public Plane unitize() {
        Plane sub_type = Plane(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
        return Plane(
            /* e423, e431, e412, e321 */
            float4(pow(sub_type.e423, 2) + pow(sub_type.e431, 2) + pow(sub_type.e412, 2)) * this.group0
        );
    }
}
// Fancy postfix trick
public Point operator /(Point lhs, UnitizePrefixOrPostfix rhs)  {
    return lhs.unitize();
    }
// Fancy prefix trick
public Point operator /(UnitizePrefixOrPostfix lhs, Point rhs) {
    return rhs.unitize();
}
public extension Point {
    public Point unitize() {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0/this.e4) * this.group0
        );
    }
}
