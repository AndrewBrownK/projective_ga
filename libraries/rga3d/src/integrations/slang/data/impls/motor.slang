implementing rga3d;
public extension Motor {
    public Motor operator +(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor {
    public Motor operator +(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar + this.scalar)
        );
    }
}
public extension Motor {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Motor {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Motor {
    public Motor operator +(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(other.group1.xyz, 0.0)
        );
    }
}
public extension Motor {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            other.group0 + this.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + this.group1
        );
    }
}
public extension Motor {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234) + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group0.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Motor {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Motor {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public Motor operator +(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, this.scalar + other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public BitXorInfixPartial<Motor> operator /(Motor lhs, BitXorInfix rhs) {
    return BitXorInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BitXorInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public AntiScalar bit_xor(AntiScalar other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Motor bit_xor(DualNum other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Flector bit_xor(Flector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Horizon operator /(BitXorInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Horizon bit_xor(Horizon other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Motor bit_xor(Line other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Motor bit_xor(Motor other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BitXorInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public MultiVector bit_xor(MultiVector other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Motor> lhs, Origin rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Flector bit_xor(Origin other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Plane operator /(BitXorInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Plane bit_xor(Plane other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Flector operator /(BitXorInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Flector bit_xor(Point other) {
        return this.wedge(other);
    }
}
// Fancy infix trick (second half)
public Motor operator /(BitXorInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.bit_xor(rhs);
}
public extension Motor {
    public Motor bit_xor(Scalar other) {
        return this.wedge(other);
    }
}
public extension Motor {
    public Motor operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Motor operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Flector operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Motor operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Flector operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Flector operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Motor operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Motor {
    public Motor neg() {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Motor {
    public Motor not() {
        return this.right_dual();
    }
}
public extension Motor {
    public Motor operator -(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, this.e1234 - other.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor {
    public Motor operator -(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, this.e1234 - other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, this.scalar - other.scalar)
        );
    }
}
public extension Motor {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Motor {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Motor {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 - float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            this.group1 - float4(other.group1.xyz, 0.0)
        );
    }
}
public extension Motor {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 - other.group0, 
            /* e23, e31, e12, scalar */
            this.group1 - other.group1
        );
    }
}
public extension Motor {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234) - other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Motor {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Motor {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(this.scalar, this.e1234), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            this.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor {
    public Motor operator -(Scalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, this.scalar - other.scalar)
        );
    }
}
