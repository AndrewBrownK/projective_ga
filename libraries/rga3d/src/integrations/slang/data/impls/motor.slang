implementing rga3d;
public extension Motor {
    public Motor operator +(AntiScalar other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234), 
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
            float4((other.group0.xyz + this.group0.xyz), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz + this.group1.xyz), this.scalar)
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
            this.group1 + float4(float3(0.0), other.scalar)
        );
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
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
public extension Motor {
    public Motor operator -(DualNum other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
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
            float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
public extension Motor {
    public Motor operator -(Line other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1
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
            float4(float3(0.0), other.e4) * float4(0.0, 0.0, 0.0, -1.0), 
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
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
