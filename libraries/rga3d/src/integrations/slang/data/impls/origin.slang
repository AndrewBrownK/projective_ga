implementing rga3d;
using traits;
public extension Origin {
    public MultiVector operator +(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Flector operator +(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Origin {
    public Flector operator +(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator +(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Origin {
    public Origin operator +(Origin other) {
        return Origin(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
public extension Origin {
    public Flector operator +(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Origin {
    public Point operator +(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4)
        );
    }
}
public extension Origin {
    public MultiVector operator +(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Origin operator *(DualNum other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Motor operator *(Flector other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public AntiScalar operator *(Horizon other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Plane operator *(Line other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Flector operator *(Motor other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public MultiVector operator *(MultiVector other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public AntiScalar operator *(Plane other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Line operator *(Point other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Origin operator *(Scalar other) {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public Origin neg() {
        return Origin(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Origin {
    public MultiVector operator -(AntiScalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(DualNum other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public Flector operator -(Flector other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Flector operator -(Horizon other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Line other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Motor other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(MultiVector other) {
        return MultiVector(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Origin operator -(Origin other) {
        return Origin(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
public extension Origin {
    public Flector operator -(Plane other) {
        return Flector(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Origin {
    public Point operator -(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Origin {
    public MultiVector operator -(Scalar other) {
        return MultiVector(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
