implementing rga3d;
public extension DualNum {
    public DualNum operator +(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension DualNum {
    public Motor operator +(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
public extension DualNum {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 + other.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar + other.scalar)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension DualNum {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator +(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public AntiScalar operator *(AntiScalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Line operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Origin operator *(Origin other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public Flector operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public DualNum neg() {
        return DualNum(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension DualNum {
    public AntiScalar not() {
        return this.right_dual();
    }
}
public extension DualNum {
    public DualNum operator -(AntiScalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
public extension DualNum {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension DualNum {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 - other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public DualNum operator -(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy
        );
    }
}
