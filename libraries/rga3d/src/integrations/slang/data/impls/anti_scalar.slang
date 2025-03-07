implementing rga3d;
using traits;
public extension AntiScalar {
    public AntiScalar operator +(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            other.e1234 + this.e1234
        );
    }
}
public extension AntiScalar {
    public DualNum operator +(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(0.0, this.e1234) + other.group0.xy
        );
    }
}
public extension AntiScalar {
    public MultiVector operator +(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator +(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public Motor operator +(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, 0.0)
        );
    }
}
public extension AntiScalar {
    public Motor operator +(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            other.group0 + float4(float3(0.0), this.e1234), 
            /* e23, e31, e12, scalar */
            other.group1
        );
    }
}
public extension AntiScalar {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234) + other.group0.xy, 
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
public extension AntiScalar {
    public MultiVector operator +(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator +(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator +(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public DualNum operator +(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar, this.e1234)
        );
    }
}
public extension AntiScalar {
    public AntiScalar operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Flector operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Origin operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Line operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Motor operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Origin operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public Plane operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiScalar operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension AntiScalar {
    public AntiScalar neg() {
        return AntiScalar(
            /* e1234 */
            this.e1234 * -1.0
        );
    }
}
public extension AntiScalar {
    public AntiScalar operator -(AntiScalar other) {
        return AntiScalar(
            /* e1234 */
            this.e1234 - other.e1234
        );
    }
}
public extension AntiScalar {
    public DualNum operator -(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar, this.e1234 - other.e1234) * float2(-1.0, 1.0)
        );
    }
}
public extension AntiScalar {
    public MultiVector operator -(Flector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator -(Horizon other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
public extension AntiScalar {
    public Motor operator -(Motor other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 - other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0)
        );
    }
}
public extension AntiScalar {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, this.e1234 - other.e1234) * float2(-1.0, 1.0), 
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
public extension AntiScalar {
    public MultiVector operator -(Origin other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator -(Plane other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public MultiVector operator -(Point other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
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
public extension AntiScalar {
    public DualNum operator -(Scalar other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar, this.e1234) * float2(-1.0, 1.0)
        );
    }
}
