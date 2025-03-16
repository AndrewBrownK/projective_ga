implementing cga3d;
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, (other.scalar * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)) * float4(other.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * this.group2.www), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (this.e42 * other.e125) + (this.e35 * other.e431) + (this.scalar * other.e1), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (this.e43 * other.e235) + (this.e15 * other.e412) + (this.scalar * other.e2), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (this.e41 * other.e315) + (this.e25 * other.e423) + (this.scalar * other.e3), -(right_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5)) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xx, other.e3215, this.scalar * other.scalar) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector bulk_expansion(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector bulk_expansion(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23) + (this.scalar * other.e1), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31) + (this.scalar * other.e2), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12) + (this.scalar * other.e3), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (this.scalar * other.e5)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiLine other) {
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * this.group2.www), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)), 
            /* e235, e315, e125, e5 */
            float4((right_dual_g1 * float3(this.scalar)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor bulk_expansion(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35)) + (right_dual_g0 * float4(this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e3215) * this.group1.xyz)), this.scalar * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_dual_g0) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * this.group2.www), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (this.e42 * other.e125) + (this.e35 * other.e431), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (this.e43 * other.e235) + (this.e15 * other.e412), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (this.e41 * other.e315) + (this.e25 * other.e423), -(right_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(CircleRotor other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual_g2_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g2_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g2_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_w * this.e15) + (this.scalar * other.e235), (right_dual_g2_w * this.e25) + (this.scalar * other.e315), (right_dual_g2_w * this.e35) + (this.scalar * other.e125), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (this.e42 * other.e125) + (this.e35 * other.e431), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (this.e43 * other.e235) + (this.e15 * other.e412), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (this.e41 * other.e315) + (this.e25 * other.e423), -(right_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * this.group2.www), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(DipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.e42 * other.e4125) + (this.e23 * other.e1234) + (this.scalar * other.e41), (this.e43 * other.e4235) + (this.e31 * other.e1234) + (this.scalar * other.e42), (this.e41 * other.e4315) + (this.e12 * other.e1234) + (this.scalar * other.e43), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) - float4((this.group0.zxy * other.group3.yzx), right_dual_g1[0] * this.e23), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(this.e31 * other.e4315) - (this.e12 * other.e4125)) + (right_dual_g1 * float4(this.scalar)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.zxy * other.group3.yzx) - (this.group2.yzx * other.group3.zxy)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor bulk_expansion(FlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * other.e45), 
            /* e235, e315, e125, e12345 */
            float4((other.group0.xyz * this.group2.www), -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(Flector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e42 * other.e4125, this.e43 * other.e4235, this.e41 * other.e4315, -(this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45)) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e15), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e4315) - (this.e12 * other.e4125)) + float4((this.group0.xyz * other.group1.www), this.scalar * other.e45) - (this.group1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.e3215) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion bulk_expansion(Line other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * this.group2.www), -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0_w) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual_g0_w) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g0_w * this.e45), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g0_w * this.e15) + (this.scalar * other.e235), (right_dual_g0_w * this.e25) + (this.scalar * other.e315), (right_dual_g0_w * this.e35) + (this.scalar * other.e125), -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435) - (this.scalar * other.e5)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[0] * this.scalar, (right_dual_g0[1] * this.scalar) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            right_dual_g1 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group6.xyz), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.scalar * other.e23), (this.e42 * other.e3215) + (this.scalar * other.e31), (this.e43 * other.e3215) + (this.scalar * other.e12), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + (this.group2 * float4(right_dual_g1.www, other.e45)) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_dual_g1.zxy) - (this.group0.zxy * right_dual_g1.yzx), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g3[2] * this.e42) + (right_dual_g3[3] * this.e23) + (this.e45 * other.e415) + (this.e35 * other.e431) + (this.scalar * other.e1), (right_dual_g3[0] * this.e43) + (right_dual_g3[3] * this.e31) + (this.e45 * other.e425) + (this.e15 * other.e412) + (this.scalar * other.e2), (right_dual_g3[1] * this.e41) + (right_dual_g3[3] * this.e12) + (this.e45 * other.e435) + (this.e25 * other.e423) + (this.scalar * other.e3), -(right_dual_g3[2] * this.e12) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435) - (this.scalar * other.e5)) - (right_dual_g3.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group7.zxy * this.group2.yzx), right_dual_g3[1] * this.e31), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion bulk_expansion(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.yzx * right_dual_g0.zxy) - (this.group0.zxy * right_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[3] * this.e41, right_dual_g0[3] * this.e42, right_dual_g0[3] * this.e43, -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.yzx * this.group2.zxy), 0.0) - float4((right_dual_g0.zxy * this.group2.yzx), 0.0), 
            /* e1, e2, e3, e5 */
            right_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public Sphere bulk_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g0_xyz[1] * this.e31) - (right_dual_g0_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0])), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group1.xyz) + (right_dual_g0_xyz.yzx * this.group2.zxy) - (right_dual_g0_xyz.zxy * this.group2.yzx)), this.scalar * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_dual_g0_xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual_g0_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g0_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g0_w * this.e15) + (this.scalar * other.e235), (right_dual_g0_w * this.e25) + (this.scalar * other.e315), (right_dual_g0_w * this.e35) + (this.scalar * other.e125), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (this.e42 * other.e125) + (this.e35 * other.e431) + (this.scalar * other.e1), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (this.e43 * other.e235) + (this.e15 * other.e412) + (this.scalar * other.e2), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (this.e41 * other.e315) + (this.e25 * other.e423) + (this.scalar * other.e3), -(right_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5)) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven bulk_expansion(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_dual_g3[2] * this.e42) + (right_dual_g3[3] * this.e23), (right_dual_g3[0] * this.e43) + (right_dual_g3[3] * this.e31), (right_dual_g3[1] * this.e41) + (right_dual_g3[3] * this.e12), -(right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)) + (right_dual_g0 * float4(this.scalar)) - float4((this.group0.zxy * right_dual_g3.yzx), right_dual_g2_xyz[0] * this.e41), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g3[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g3[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group3.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e4 * -1.0)) + (this.group3.yzxx * float4(other.group2.zxy, other.e1)) + float4(float3(0.0), (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)) - float4((other.group2.yzx * this.group3.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector bulk_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor bulk_expansion(AntiFlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e1 * other.e321, this.e2 * other.e321, this.e3 * other.e321, -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125)) + (other.group0 * float4(this.group2.www, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor bulk_expansion(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (other.group0 * float4(this.group2.www, this.e321)) + (this.group3.xyzx * float4(other.group0.www, other.e1)) + float4(float3(0.0), (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e4 * other.e5)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Plane bulk_expansion(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e15) + (this.e3 * other.e31), (this.e4 * other.e25) + (this.e1 * other.e12), (this.e4 * other.e35) + (this.e2 * other.e23), -(this.e2 * other.e25) - (this.e3 * other.e35)) - (this.group3.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector bulk_expansion(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e423 * other.e3215) + (this.e4 * other.e15), (this.e431 * other.e3215) + (this.e4 * other.e25), (this.e412 * other.e3215) + (this.e4 * other.e35), -(this.e2 * other.e25) - (this.e3 * other.e35)) + float4((right_dual_g0.yzx * this.group3.zxy), this.e321 * other.e3215) - (this.group3.yzxx * float4(right_dual_g0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bulk_expansion(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_dual_g0) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e235) + (this.e5 * other.e423), (this.e4 * other.e315) + (this.e5 * other.e431), (this.e4 * other.e125) + (this.e5 * other.e412), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group2.yzx * this.group3.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float4 right_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_dual_g2[3] * this.e423) + (this.e4 * other.e415) + (this.e3 * other.e431), (right_dual_g2[3] * this.e431) + (this.e4 * other.e425) + (this.e1 * other.e412), (right_dual_g2[3] * this.e412) + (this.e4 * other.e435) + (this.e2 * other.e423), -(right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g2[3] * this.e415) + (this.e5 * other.e423), (right_dual_g2[3] * this.e425) + (this.e5 * other.e431), (right_dual_g2[3] * this.e435) + (this.e5 * other.e412), -(this.e2 * other.e425) - (this.e3 * other.e435)) + (right_dual_g2 * float4(this.group2.www, this.e321)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy)), right_dual_g2[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g2[3]) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e4 * other.e15) + (this.e3 * other.e31), (this.e4 * other.e25) + (this.e1 * other.e12), (this.e4 * other.e35) + (this.e2 * other.e23), -(this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group3.yzxx * float4(other.group1.zxy, other.e15)) - (this.group3.wwwy * float4(other.group0.xyz, other.e25)), 
            /* e1234 */
            (this.e4 * other.e45) + (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * float4(this.group3.yzx, this.e4)) - (this.group3.zxyw * float4(other.group3.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43) - (this.e431 * other.e4315) - (this.e412 * other.e4125) - (this.e321 * other.e1234)) + float4((this.group3.xyz * other.group3.www), this.e4 * other.e45) - (other.group3.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4((this.group3.zxy * other.group1.yzx), this.e125 * other.e4125) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group3.yzxz * float4(other.group1.zxy, other.e35)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - float4((this.group1.yzx * other.group3.zxy), this.e1 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.yy, other.e12345, this.e4 * other.e5) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere bulk_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * this.group2.www), -(this.e1 * other.e15) - (this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45)), 
            /* e1234 */
            this.e4 * other.e45
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), this.e4 * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * other.e4315) - (this.e412 * other.e4125)) + float4((this.group3.xyz * other.group1.www), this.e4 * other.e45) - (other.group1.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e4125) - (this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45)) - float4((this.group1.yzx * other.group1.zxy), this.e1 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor bulk_expansion(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * this.group2.www), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_dual_g0_w * this.e423) + (this.e4 * other.e415), (right_dual_g0_w * this.e431) + (this.e4 * other.e425), (right_dual_g0_w * this.e412) + (this.e4 * other.e435), -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e4 * other.e5)), 
            /* e415, e425, e435, e321 */
            float4(this.e4 * other.e235, this.e4 * other.e315, this.e4 * other.e125, -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)) + (float4(right_dual_g0_w) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0_w) * this.group2.xyz) + (float3(this.e5) * other.group0.xyz) + (this.group3.yzx * other.group1.zxy) - (this.group3.zxy * other.group1.yzx)), right_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0_w) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g3_w * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e4 * other.e5) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[0]) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * float4(this.group3.xyz, this.e4)) - (right_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group3.xyz), 
            /* e23, e31, e12 */
            (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e4 * other.e235) + (this.e5 * other.e423), (this.e4 * other.e315) + (this.e5 * other.e431), (this.e4 * other.e125) + (this.e5 * other.e412), -(this.e2 * other.e425) - (this.e3 * other.e435)) + (float4(right_dual_g0[0]) * this.group1) - (this.group3.xyzx * float4(float3(right_dual_g3_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[0]) * this.group2.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxz * float4(this.group1.zxy, this.e125)) + (this.group2.wwwy * float4(right_dual_g8, right_dual_g1[1])) + float4((right_dual_g6_xyz.yzx * this.group3.zxy), right_dual_g1[0] * this.e235) - (float4(this.e5) * float4(other.group4.xyz, other.e45)) - (this.group3.yzxx * float4(right_dual_g6_xyz.zxy, right_dual_g8[0])) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g8[1] * this.e2) - float4((this.group2.xyz * right_dual_g1.www), right_dual_g8[2] * this.e3), 
            /* e1234 */
            (this.e4 * other.e45) + (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), this.e4 * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e3215, this.e2 * other.e3215, this.e3 * other.e3215, -(this.e431 * other.e4315) - (this.e412 * other.e4125)) - (other.group0.xyzx * float4(this.group3.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0_xyz.zxy * this.group3.yzx), this.e4 * other.e3215) - (this.group3.zxyw * float4(right_dual_g0_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(this.e1 * other.e3215, this.e2 * other.e3215, this.e3 * other.e3215, -(right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234)) - float4((right_dual_g0_xyz * this.group3.www), right_dual_g0_xyz[0] * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g0_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g0_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group0.www), right_dual_g0_xyz[1] * this.e315) + float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyx * float4(right_dual_g0.yzx, other.e1)) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e4 * other.e5) - (this.e5 * other.e4)) + float4((this.group0.xyz * right_dual_g0.www), this.e2 * other.e2) + float4((other.group1.xyz * this.group2.www), this.e3 * other.e3) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0[3] * this.e415) + (this.e4 * other.e235), (right_dual_g0[3] * this.e425) + (this.e4 * other.e315), (right_dual_g0[3] * this.e435) + (this.e4 * other.e125), -(this.e2 * other.e425) - (this.e3 * other.e435)) + (right_dual_g0 * float4(this.group3.www, this.e321)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (this.group3.yzx * other.group2.zxy) - (this.group3.zxy * other.group2.yzx)), right_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g3_xyz.zxy * this.group3.yzx), this.e4 * other.e3215) - (this.group3.zxyw * float4(right_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43) - (right_dual_g3_xyz[1] * this.e431) - (right_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)) + float4((this.group3.xyz * other.group3.www), this.e4 * other.e45) - float4((right_dual_g3_xyz * this.group3.www), right_dual_g3_xyz[0] * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            (this.group2.wwwx * float4(right_dual_g2_xyz, right_dual_g3_xyz[0])) + float4((this.group0.xyz * other.group3.www), right_dual_g3_xyz[2] * this.e125) + float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[1] * this.e315) + float4((this.group3.zxy * other.group1.yzx), this.e321 * other.e3215) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group3.yzxz * float4(other.group1.zxy, right_dual_g2_xyz[2])) - float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g2_xyz[0] * this.e1) - float4((this.group2.xyz * other.group2.www), right_dual_g2_xyz[1] * this.e2)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public DualNum bulk_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public FlatPoint bulk_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Flector bulk_expansion(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Line bulk_expansion(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_expansion(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Dipole bulk_expansion(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g2_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Circle bulk_expansion(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(DipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.yy, this.scalar, this.e3215 * other.e1234) * float4((other.group0.xyz * float3(-1.0)), 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(other.group2.xyz, other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e3215 * other.e12345) + (this.scalar * other.e5), this.scalar * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlector bulk_expansion(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiLine bulk_expansion(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            right_dual_g0 * float4(this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.group0.yy, this.scalar, (right_dual_g0[3] * this.e3215) - (this.scalar * other.e5)) * float4(other.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[0] * this.scalar, (right_dual_g0[1] * this.scalar) + (right_dual_g1[3] * this.e3215)), 
            /* e1, e2, e3, e4 */
            right_dual_g1 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.yy, this.scalar, (right_dual_g0[0] * this.e3215) - (this.scalar * other.e5)) * float4(other.group1.xyz, 1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiPlane bulk_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public Sphere bulk_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_dual_g0[3] * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.yy, this.scalar, (right_dual_g0[3] * this.e3215) - (this.scalar * other.e5)) * float4(other.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven bulk_expansion(VersorOdd other) {
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.group0.yy, this.scalar, (right_dual_g3[3] * this.e3215) + (this.scalar * other.scalar)) * float4(other.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e321, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(AntiMotor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar bulk_expansion(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g2_w * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g2_w) * this.group0.xy), right_dual_g2_w * this.e125, (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e1234, other.e1234, other.e1234, (this.e235 * other.e4235) + (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(Flector other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e4235) + (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_expansion(Motor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[0] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e1234, other.e1234, other.e1234, (right_dual_g1_xyz[0] * this.e235) + (right_dual_g1_xyz[1] * this.e315) + (right_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum bulk_expansion(Plane other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e4235) + (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e1234, other.e1234, other.e1234, (right_dual_g0_xyz[0] * this.e235) + (right_dual_g0_xyz[1] * this.e315) + (right_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0_w * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g0_w) * this.group0.xy), right_dual_g0_w * this.e125, (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e1234, other.e1234, other.e1234, (right_dual_g3_xyz[0] * this.e235) + (right_dual_g3_xyz[1] * this.e315) + (right_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiFlector> operator /(AntiFlector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group1.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(other.e423 * this.e5, other.e431 * this.e5, other.e412 * this.e5, -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxy * float4(other.group2.zxy, other.e2)) + (this.group1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5)) - float4((other.group2.yzx * this.group1.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Flector bulk_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Motor bulk_expansion(AntiFlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e321) * float4(this.group1.xyz, this.e321), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Motor bulk_expansion(AntiFlector other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(right_dual_g0_w * -1.0) * this.group1.xy), right_dual_g0_w * this.e3 * -1.0, (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0_w * this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Plane bulk_expansion(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(this.e2 * other.e25) - (this.e3 * other.e35)) - (this.group1.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Flector bulk_expansion(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e25) - (this.e3 * other.e35)) + float4((right_dual_g0.yzx * this.group1.zxy), this.e321 * other.e3215) - (this.group1.yzxx * float4(right_dual_g0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar bulk_expansion(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e423, this.e5 * other.e431, this.e5 * other.e412, -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group2.yzx * this.group1.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e3 * other.e431, this.e1 * other.e412, this.e2 * other.e423, -(this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)) - float4((other.group0.zxy * this.group1.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) + float4((other.group0.xyz * this.group1.www), right_dual_g2_w * this.e321) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx)), right_dual_g2_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public Sphere bulk_expansion(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group1.yzxx * float4(other.group1.zxy, other.e15)) - (this.group1.wwwy * float4(other.group0.xyz, other.e25)), 
            /* e1234 */
            (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e2 * other.e42) + (this.e3 * other.e43)) - float4((other.group3.xyz * this.group1.www), this.e321 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215) - (this.e5 * other.e45)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e4235) - (this.group1.yzxy * float4(other.group1.zxy, other.e25)) - (this.group1.wwwz * float4(other.group0.xyz, other.e35)) - (other.group2.wwwx * float4(this.group0.xyz, this.e1))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(this.e1 * other.e15) - (this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_expansion(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215) - (this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group1.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint bulk_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group1.zxyx * float4(other.group1.yzx, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual_g0[3] * this.e235) + (this.e2 * other.e125), (right_dual_g0[3] * this.e315) + (this.e3 * other.e235), (right_dual_g0[3] * this.e125) + (this.e1 * other.e315), -(right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)) + (right_dual_g0 * float4(this.group1.www, this.e321)) - (this.group1.zxyx * float4(other.group1.yzx, right_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[3]) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g3_w * this.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[0]) * this.group1.xyz), 0.0), 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) - (right_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12 */
            (right_dual_g1_xyz.zxy * this.group1.yzx) - (right_dual_g1_xyz.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) + float4((other.group7.xyz * this.group1.www), right_dual_g0[0] * this.e321) - (this.group1.xyzx * float4(float3(right_dual_g3_w), other.e415)), 
            /* e423, e431, e412 */
            (other.group7.yzx * this.group1.zxy) - (other.group7.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group1.yzx) - (other.group8.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_dual_g1_xyz[1] * this.e315) + (right_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215) - (this.e5 * other.e45)) + float4((right_dual_g6_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e235) - (float4(other.e1234, other.e1234, other.e1234, right_dual_g8[1] * this.e2) * float4(this.group0.xyz, 1.0)) - (this.group1.yzxx * float4(right_dual_g6_xyz.zxy, right_dual_g8[0])) - (this.group1.wwwz * float4(other.group4.xyz, right_dual_g8[2])), 
            /* e1234 */
            (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_expansion(Plane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e5 * other.e4235 * -1.0, this.e5 * other.e4315 * -1.0, this.e5 * other.e4125 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125) + (this.e321 * other.e3215)) + (other.group0.wwwx * float4(this.group1.xyz, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz.zxy * this.group1.yzx) - (right_dual_g0_xyz.yzx * this.group1.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) - (right_dual_g0_xyz * float3(this.e5))), this.e321 * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e1234, other.e1234, other.e1234, (right_dual_g0_xyz[0] * this.e235) + (right_dual_g0_xyz[1] * this.e315) + (right_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(right_dual_g0.yzx, other.e1)) + float4(float3(0.0), (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (this.e5 * other.e4)) - float4((right_dual_g0.zxy * this.group1.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            (right_dual_g0 * float4(this.group1.www, this.e321)) + float4(float3(0.0), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx)), right_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g3[3] * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy)), right_dual_g3[3] * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e2 * other.e42) + (this.e3 * other.e43)) - (right_dual_g3 * float4(this.group1.www, this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125) + (this.e321 * other.e3215) - (this.e5 * other.e45)) + float4((this.group1.zxy * other.group1.yzx), right_dual_g3[0] * this.e235) - (this.group1.yzxy * float4(other.group1.zxy, right_dual_g2_xyz[1])) - (this.group1.wwwz * float4(other.group0.xyz, right_dual_g2_xyz[2])) - float4((this.group0.xyz * right_dual_g3.www), right_dual_g2_xyz[0] * this.e1)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiLine> operator /(AntiLine lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35), (right_dual_g1_w * this.e31) + (other.e412 * this.e15), (right_dual_g1_w * this.e12) + (other.e423 * this.e25), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group1.yzx), other.e415 * this.e15), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Plane bulk_expansion(AntiFlatPoint other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float2(right_dual_g0_w) * this.group0.xyz.xy), right_dual_g0_w * this.e12, -(other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Plane bulk_expansion(AntiFlector other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float2(right_dual_g0_w) * this.group0.xyz.xy), right_dual_g0_w * this.e12, -(other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiLine bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Sphere bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (this.e35 * other.e431), (right_dual_g1_w * this.e31) + (this.e15 * other.e412), (right_dual_g1_w * this.e12) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e235), 
            /* e1234 */
            -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g2_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g2_w) * this.group1.xyz.xy), right_dual_g2_w * this.e35, -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (this.e35 * other.e431), (right_dual_g1_w * this.e31) + (this.e15 * other.e412), (right_dual_g1_w * this.e12) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar bulk_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor bulk_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e1234, other.e1234, other.e1234, -(this.e23 * other.e4235) - (this.e31 * other.e4315) - (this.e12 * other.e4125)) * float4(this.group1.xyz, 1.0), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e3215) + (this.e35 * other.e4315), (this.e31 * other.e3215) + (this.e15 * other.e4125), (this.e12 * other.e3215) + (this.e25 * other.e4235), -(this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) - float4((this.group1.yzx * other.group3.zxy), this.e23 * other.e23)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiLine bulk_expansion(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(Flector other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e3215) + (this.e35 * other.e4315), (this.e31 * other.e3215) + (this.e15 * other.e4125), (this.e12 * other.e3215) + (this.e25 * other.e4235), -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiDualNum bulk_expansion(Line other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiMotor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float2(right_dual_g0_w) * this.group1.xyz.xy), right_dual_g0_w * this.e35, -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((float3(right_dual_g0[0]) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e1234, other.e1234, other.e1234, -(right_dual_g1_xyz[0] * this.e23) - (right_dual_g1_xyz[1] * this.e31) - (right_dual_g1_xyz[2] * this.e12)) * float4(this.group1.xyz, 1.0), 
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) + (right_dual_g1_xyz.yzx * this.group1.zxy) - (right_dual_g1_xyz.zxy * this.group1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g3_w * this.e23) + (this.e35 * other.e431), (right_dual_g3_w * this.e31) + (this.e15 * other.e412), (right_dual_g3_w * this.e12) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group1.yzx * other.group7.zxy), this.e23 * other.e235), 
            /* e1234 */
            -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint bulk_expansion(Plane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e23 * other.e3215) + (this.e35 * other.e4315), (this.e31 * other.e3215) + (this.e15 * other.e4125), (this.e12 * other.e3215) + (this.e25 * other.e4235), -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (other.group0.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public Circle bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e1234, other.e1234, other.e1234, -(right_dual_g0_xyz[0] * this.e23) - (right_dual_g0_xyz[1] * this.e31) - (right_dual_g0_xyz[2] * this.e12)) * float4(this.group1.xyz, 1.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) + (right_dual_g0_xyz.yzx * this.group1.zxy) - (right_dual_g0_xyz.zxy * this.group1.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float right_dual_g1_w = other.e321 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g0_w) * this.group1.xyz.xy), right_dual_g0_w * this.e35, -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (this.e35 * other.e431), (right_dual_g1_w * this.e31) + (this.e15 * other.e412), (right_dual_g1_w * this.e12) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e1234, other.e1234, other.e1234, -(right_dual_g3_xyz[0] * this.e23) - (right_dual_g3_xyz[1] * this.e31) - (right_dual_g3_xyz[2] * this.e12)) * float4(this.group1.xyz, 1.0), 
            /* e235, e315, e125, e12345 */
            float4((right_dual_g3_xyz[1] * this.e35) + (this.e23 * other.e3215), (right_dual_g3_xyz[2] * this.e15) + (this.e31 * other.e3215), (right_dual_g3_xyz[0] * this.e25) + (this.e12 * other.e3215), -(this.e31 * other.e31) - (this.e12 * other.e12) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) - float4((right_dual_g3_xyz.zxy * this.group1.yzx), this.e23 * other.e23)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiMotor> operator /(AntiMotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, (other.scalar * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) * float4(other.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar, this.scalar, this.scalar, -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar)) * float4(other.group2.xyz, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e1 * this.scalar), (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e2 * this.scalar), (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e3 * this.scalar), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12) - (other.e5 * this.scalar)) - (this.group1.yzxx * float4(other.group0.zxy, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * this.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Flector bulk_expansion(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Flector bulk_expansion(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[3] * this.e23) + (other.e1 * this.scalar), (right_dual_g0[3] * this.e31) + (other.e2 * this.scalar), (right_dual_g0[3] * this.e12) + (other.e3 * this.scalar), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (other.e5 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiLine other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar, this.scalar, this.scalar, -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.scalar * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar, this.scalar, this.scalar, (other.scalar * this.scalar) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e3215 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion bulk_expansion(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * this.group0.www), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[3] * this.e23) + (this.e35 * other.e431), (right_dual_g1[3] * this.e31) + (this.e15 * other.e412), (right_dual_g1[3] * this.e12) + (this.e25 * other.e423), -(right_dual_g1[1] * this.e25) - (right_dual_g1[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) - (this.group1.yzxx * float4(other.group0.zxy, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, right_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual_g2_w) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), right_dual_g1_w * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g2_w * this.e15) + (this.scalar * other.e235), (right_dual_g2_w * this.e25) + (this.scalar * other.e315), (right_dual_g2_w * this.e35) + (this.scalar * other.e125), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e35 * other.e431, this.e15 * other.e412, this.e25 * other.e423, -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float2(right_dual_g1_w) * this.group0.xy), right_dual_g1_w * this.e12, right_dual_g2_w * this.e3215) - float4((other.group0.zxy * this.group1.yzx), this.e23 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor bulk_expansion(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) * float4((other.group2.xyz * float3(-1.0)), 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e23 * other.e1234, this.e31 * other.e1234, this.e12 * other.e1234, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((right_dual_g0 * this.group0.www), this.e3215 * other.e1234), 
            /* e415, e425, e435, e321 */
            float4(this.e15 * other.e1234, this.e25 * other.e1234, this.e35 * other.e1234, -(this.e23 * other.e4235) - (this.e31 * other.e4315) - (this.e12 * other.e4125)) + (right_dual_g1 * float4(this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345, other.e12345, other.e12345, (this.scalar * other.e5) + (this.e3215 * other.e12345)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_expansion(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g1[3] * this.e23, right_dual_g1[3] * this.e31, right_dual_g1[3] * this.e12, right_dual_g1[2] * this.e12 * -1.0) + float4((right_dual_g1.yzx * this.group1.zxy), this.scalar * other.e45) - (right_dual_g1.zxyx * float4(this.group1.yzx, this.e23)) - (this.group0.wwwy * float4(other.group0.xyz, right_dual_g1[1])), 
            /* e1, e2, e3, e5 */
            right_dual_g1 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * this.group0.www), -(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(right_dual_g0_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g0_w * this.scalar), 
            /* e15, e25, e35, e3215 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0_w) * this.group1) + float4(float3(0.0), -(right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[0] * this.scalar, (right_dual_g0[1] * this.scalar) + (right_dual_g1[3] * this.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12)), 
            /* e1, e2, e3, e4 */
            right_dual_g1 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz)), right_dual_g3_w * this.scalar), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group7.xyz, 
            /* e23, e31, e12 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group6.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.scalar * other.e23, this.scalar * other.e31, this.scalar * other.e12, -(right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group1.xyz * right_dual_g1.www), this.scalar * other.e45), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) + (right_dual_g1.yzx * this.group1.zxy) - (float3(this.scalar) * other.group3.xyz) - (right_dual_g1.zxy * this.group1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.scalar * other.e1) + (this.e35 * other.e431), (this.scalar * other.e2) + (this.e15 * other.e412), (this.scalar * other.e3) + (this.e25 * other.e423), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float2(right_dual_g3_w) * this.group0.xy), right_dual_g3_w * this.e12, right_dual_g0[0] * this.e3215) - float4((other.group7.zxy * this.group1.yzx), this.e23 * other.e235), 
            /* e1234 */
            -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412) - (this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_expansion(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual_g0[1] * this.e35) + (right_dual_g0[3] * this.e23), (right_dual_g0[2] * this.e15) + (right_dual_g0[3] * this.e31), (right_dual_g0[0] * this.e25) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            right_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public Sphere bulk_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((this.group1.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_dual_g0 * float4(this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), right_dual_g1_w * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_dual_g0[3] * this.e15) + (this.scalar * other.e235), (right_dual_g0[3] * this.e25) + (this.scalar * other.e315), (right_dual_g0[3] * this.e35) + (this.scalar * other.e125), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (this.scalar * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[1] * this.e35) + (this.scalar * other.e1), (right_dual_g0[2] * this.e15) + (this.scalar * other.e2), (right_dual_g0[0] * this.e25) + (this.scalar * other.e3), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.scalar * other.e5) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float2(right_dual_g1_w) * this.group0.xy), right_dual_g1_w * this.e12, right_dual_g0[3] * this.e3215) - float4((right_dual_g0.zxy * this.group1.yzx), this.e23 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven bulk_expansion(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g0 * float4(this.scalar)) + (float4(right_dual_g3[3]) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g3[3] * this.e15, right_dual_g3[3] * this.e25, right_dual_g3[3] * this.e35, -(right_dual_g3[0] * this.e23) - (right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_dual_g3.yzx * this.group1.zxy) - (float3(this.scalar) * other.group2.xyz) - (right_dual_g3.zxy * this.group1.yzx)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiPlane> operator /(AntiPlane lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e321 * this.e1), (other.e431 * this.e5) + (other.e321 * this.e2), (other.e412 * this.e5) + (other.e321 * this.e3), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125, e12345 */
            (this.group0.yzxy * float4(other.group2.zxy, other.e2)) + (this.group0.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), other.e3 * this.e3) - (other.group2.yzxw * this.group0.zxyw)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public FlatPoint bulk_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Line bulk_expansion(AntiFlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e321) * this.group0.xyz, 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Motor bulk_expansion(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * right_dual_g0.www * float3(-1.0)), (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Plane bulk_expansion(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e25 * this.e2) - (other.e35 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Flector bulk_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e25 * this.e2) - (other.e35 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar bulk_expansion(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_expansion(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Circle bulk_expansion(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e1 * other.e321) + (this.e5 * other.e423), (this.e2 * other.e321) + (this.e5 * other.e431), (this.e3 * other.e321) + (this.e5 * other.e412), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion bulk_expansion(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e1 * other.e321) + (this.e5 * other.e423), (this.e2 * other.e321) + (this.e5 * other.e431), (this.e3 * other.e321) + (this.e5 * other.e412), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Sphere bulk_expansion(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25)), 
            /* e1234 */
            (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e5 * other.e4235 * -1.0, this.e5 * other.e4315 * -1.0, this.e5 * other.e4125 * -1.0, (this.e2 * other.e42) + (this.e3 * other.e43)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)) - (this.group0.wwwy * float4(other.group0.xyz, other.e25))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_expansion(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(this.e1 * other.e15) - (this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiMotor bulk_expansion(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e1 * other.e3215, this.e2 * other.e3215, this.e3 * other.e3215, -(this.e2 * other.e25) - (this.e3 * other.e35) - (this.e5 * other.e45)) - (this.group0.wwwx * float4(other.group1.xyz, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlatPoint bulk_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group0.zxyx * float4(other.group1.yzx, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlector bulk_expansion(Motor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.e2 * other.e125) + (this.e5 * other.e415), (this.e3 * other.e235) + (this.e5 * other.e425), (this.e1 * other.e315) + (this.e5 * other.e435), -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group0.zxyx * float4(other.group1.yzx, other.e415)), 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[0]) * this.group0.xyz), 0.0), 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group0.xyz) - (right_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            (right_dual_g1_xyz.zxy * this.group0.yzx) - (right_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(this.e5 * other.e423, this.e5 * other.e431, this.e5 * other.e412, -(this.e2 * other.e425) - (this.e3 * other.e435)) - (this.group0.xyzx * float4(right_dual_g3.www, other.e415)), 
            /* e423, e431, e412 */
            (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group6.xyz) + (right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(right_dual_g8[2] * this.e3) - (this.e5 * other.e45)) - (this.group0.yzxx * float4(other.group5.zxy, right_dual_g8[0])) - (this.group0.wwwy * float4(other.group4.xyz, right_dual_g8[1])), 
            /* e1234 */
            (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiLine bulk_expansion(Plane other) {
        return AntiLine(
            /* e23, e31, e12 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public Dipole bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (right_dual_g0_xyz * float3(this.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public VersorEven bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0.zxyx * float4(other.group0.yzx, other.e1)) + float4(float3(0.0), (this.e2 * other.e2) + (this.e3 * other.e3)) - (this.group0.yzxw * float4(other.group0.zxy, other.e4)), 
            /* e415, e425, e435, e321 */
            float4((this.e1 * other.e321) + (this.e5 * other.e423), (this.e2 * other.e321) + (this.e5 * other.e431), (this.e3 * other.e321) + (this.e5 * other.e412), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), right_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0_w) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g3_xyz[0] * this.e5 * -1.0, right_dual_g3_xyz[1] * this.e5 * -1.0, right_dual_g3_xyz[2] * this.e5 * -1.0, (this.e2 * other.e42) + (this.e3 * other.e43)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e2 * other.e23, -(right_dual_g2_xyz[2] * this.e3) - (this.e5 * other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, right_dual_g2_xyz[0])) - (this.group0.wwwy * float4(other.group0.xyz, right_dual_g2_xyz[1]))
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<AntiScalar> operator /(AntiScalar lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(DualNum other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(Motor other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_expansion(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.e12345 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Circle> operator /(Circle lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Plane bulk_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Plane bulk_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Circle bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * this.group1, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g2_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g2_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g2_w) * this.group2.xyz.xy), right_dual_g2_w * this.e125, (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e4125) - (this.e235 * other.e1234), -(this.e435 * other.e4235) - (this.e315 * other.e1234), -(this.e415 * other.e4315) - (this.e125 * other.e1234), (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Circle bulk_expansion(DualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Flector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public AntiScalar bulk_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g0_w) * this.group2.xyz.xy), right_dual_g0_w * this.e125, -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[0]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[0]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g1_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g1_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g1_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group9.www), right_dual_g1_xyz[1] * this.e315) + float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g0_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g0_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group0.www), right_dual_g0_xyz[1] * this.e315) + float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public CircleRotor bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g0_w) * this.group2.xyz.xy), right_dual_g0_w * this.e125, (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Circle {
    public Sphere bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g3_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g3_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g3_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group3.www), right_dual_g3_xyz[1] * this.e315) + float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g3_xyz[0] * this.e423) - (right_dual_g3_xyz[1] * this.e431) - (right_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<CircleRotor> operator /(CircleRotor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Plane bulk_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Plane bulk_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_dual_g0) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(Circle other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g2_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g2_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g2_w) * this.group2.xy), right_dual_g2_w * this.e125, (right_dual_g2_w * this.e12345) + (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(this.e425 * other.e4125) - (this.e235 * other.e1234), -(this.e435 * other.e4235) - (this.e315 * other.e1234), -(this.e415 * other.e4315) - (this.e125 * other.e1234), (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(DualNum other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Flector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar bulk_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g0_w) * this.group2.xy), right_dual_g0_w * this.e125, (right_dual_g0_w * this.e12345) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e12345) + (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[0]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[0]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g1_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g1_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g1_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group9.www), right_dual_g1_xyz[1] * this.e315) + float4((right_dual_g1_xyz.yzx * this.group1.zxy), right_dual_g1_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g1_xyz[0] * this.e423) - (right_dual_g1_xyz[1] * this.e431) - (right_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g0_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g0_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group0.www), right_dual_g0_xyz[1] * this.e315) + float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g0_xyz[0] * this.e423) - (right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float2(right_dual_g0_w) * this.group2.xy), right_dual_g0_w * this.e125, (right_dual_g0_w * this.e12345) + (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension CircleRotor {
    public Sphere bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g3_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g3_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g3_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215)) + float4((this.group0.xyz * other.group3.www), right_dual_g3_xyz[1] * this.e315) + float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[0] * this.e235), 
            /* e1234 */
            -(right_dual_g3_xyz[0] * this.e423) - (right_dual_g3_xyz[1] * this.e431) - (right_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Dipole> operator /(Dipole lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e415 * this.e15) - float4((this.group0.zxy * other.group2.yzx), other.e425 * this.e25), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Line bulk_expansion(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e3215) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Plane bulk_expansion(AntiFlatPoint other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0_w * this.e23) + (other.e125 * this.e42), (right_dual_g0_w * this.e31) + (other.e235 * this.e43), (right_dual_g0_w * this.e12) + (other.e315 * this.e41), -(other.e315 * this.e31) - (other.e125 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Plane bulk_expansion(AntiFlector other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0_w * this.e23) + (other.e125 * this.e42), (right_dual_g0_w * this.e31) + (other.e235 * this.e43), (right_dual_g0_w * this.e12) + (other.e315 * this.e41), -(other.e315 * this.e31) - (other.e125 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * other.group1.www), -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Dipole bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * this.group1, 
            /* e15, e25, e35 */
            float3(right_dual_g0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e415 * this.e15) - float4((other.group2.yzx * this.group0.zxy), other.e425 * this.e25), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g2_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g2_w) * this.group2.xyz.xy), right_dual_g2_w * this.e35, -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group0.zxy * this.group2.yzx), other.e415 * this.e15) - float4((this.group0.zxy * other.group2.yzx), other.e425 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e3215) + (this.e35 * other.e4315), (this.e31 * other.e3215) + (this.e15 * other.e4125), (this.e12 * other.e3215) + (this.e25 * other.e4235), -(this.e42 * other.e25) - (this.e43 * other.e35) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Dipole bulk_expansion(DualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(Flector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e12345 */
            float4((this.e23 * other.e3215) + (this.e35 * other.e4315), (this.e31 * other.e3215) + (this.e15 * other.e4125), (this.e12 * other.e3215) + (this.e25 * other.e4235), -(this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45)) - float4((this.group2.yzx * other.group1.zxy), this.e41 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Sphere bulk_expansion(Line other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g0_w) * this.group2.xyz.xy), right_dual_g0_w * this.e35, -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0[0]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g1_xyz[1] * this.e31) - (right_dual_g1_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g1_xyz, right_dual_g1_xyz[0])), 
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g1_xyz.zxy * this.group0.yzx) - (right_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_dual_g1_xyz.yzx * this.group2.zxy) - (right_dual_g1_xyz.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g3_w * this.e23) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g3_w * this.e31) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g3_w * this.e12) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group8.yzx), this.e23 * other.e235) - float4((this.group2.yzx * other.group7.zxy), this.e31 * other.e315), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Circle bulk_expansion(Plane other) {
        return Circle(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public Circle bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g0_xyz[1] * this.e31) - (right_dual_g0_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0])), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_dual_g0_xyz.yzx * this.group2.zxy) - (right_dual_g0_xyz.zxy * this.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float right_dual_g1_w = other.e321 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g0_w) * this.group2.xyz.xy), right_dual_g0_w * this.e35, -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g1_w * this.e31) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g1_w * this.e12) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Dipole {
    public CircleRotor bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g3_xyz[1] * this.e31) - (right_dual_g3_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0])), 
            /* e235, e315, e125, e12345 */
            float4((right_dual_g3_xyz[1] * this.e35) + (this.e23 * other.e3215), (right_dual_g3_xyz[2] * this.e15) + (this.e31 * other.e3215), (right_dual_g3_xyz[0] * this.e25) + (this.e12 * other.e3215), -(right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) - float4((right_dual_g3_xyz.zxy * this.group2.yzx), right_dual_g2_xyz[0] * this.e41)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e415)) - float4((this.group0.zxy * other.group2.yzx), other.e425 * this.e25), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Motor bulk_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Plane bulk_expansion(AntiFlatPoint other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0_w * this.e23) + (other.e125 * this.e42), (right_dual_g0_w * this.e31) + (other.e235 * this.e43), (right_dual_g0_w * this.e12) + (other.e315 * this.e41), -(other.e315 * this.e31) - (other.e125 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Plane bulk_expansion(AntiFlector other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0_w * this.e23) + (other.e125 * this.e42), (right_dual_g0_w * this.e31) + (other.e235 * this.e43), (right_dual_g0_w * this.e12) + (other.e315 * this.e41), -(other.e315 * this.e31) - (other.e125 * this.e12)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * other.group1.www), (other.e3215 * this.e1234) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, other.e415)) - float4((other.group2.yzx * this.group0.zxy), other.e425 * this.e25), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g2_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g2_w) * this.group2.xy), right_dual_g2_w * this.e35, (right_dual_g2_w * this.e1234) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g2_w * this.e4235) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g2_w * this.e4315) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g2_w * this.e4125) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((float2(right_dual_g1_w) * this.group1.xy), right_dual_g1_w * this.e12, right_dual_g2_w * this.e3215) - (this.group2.yzxx * float4(other.group0.zxy, other.e415)) - float4((this.group0.zxy * other.group2.yzx), other.e425 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e1234 * this.e15) + (other.e3215 * this.e41), (other.e1234 * this.e25) + (other.e3215 * this.e42), (other.e1234 * this.e35) + (other.e3215 * this.e43), -(other.e4315 * this.e31) - (other.e4125 * this.e12)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (other.group3.wwwx * float4(this.group1.xyz, this.e4235)) + float4(float3(0.0), (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e45 * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)) + float4((other.group3.yzx * this.group2.zxy), other.e1234 * this.e3215) - (this.group2.yzxx * float4(other.group3.zxy, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(DualNum other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e15) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Flector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (this.group1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e4315) + (this.e4125 * other.e4125) - (this.e42 * other.e25) - (this.e43 * other.e35) - (this.e45 * other.e45)) - float4((this.group2.yzx * other.group1.zxy), this.e41 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere bulk_expansion(Line other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((this.e42 * other.e125) + (this.e45 * other.e415), (this.e43 * other.e235) + (this.e45 * other.e425), (this.e41 * other.e315) + (this.e45 * other.e435), -(this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e235), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((this.group2.xyz * right_dual_g0.www), (right_dual_g0[3] * this.e1234) - (right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[3] * this.e4235) + (this.e42 * other.e125), (right_dual_g0[3] * this.e4315) + (this.e43 * other.e235), (right_dual_g0[3] * this.e4125) + (this.e41 * other.e315), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + (right_dual_g0 * float4(this.group1.www, this.e3215)) - float4((this.group0.zxy * other.group1.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0[0]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g1_xyz[1] * this.e31) - (right_dual_g1_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g1_xyz, right_dual_g1_xyz[0])), 
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g1_xyz.zxy * this.group0.yzx) - (right_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_dual_g1_xyz.yzx * this.group2.zxy) - (right_dual_g1_xyz.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[0] * this.e4235) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g0[0] * this.e4315) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g0[0] * this.e4125) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + float4((float2(right_dual_g3_w) * this.group1.xy), right_dual_g3_w * this.e12, right_dual_g0[0] * this.e3215) - float4((this.group0.zxy * other.group8.yzx), this.e23 * other.e235) - float4((other.group7.zxy * this.group2.yzx), this.e31 * other.e315), 
            /* e1234 */
            (right_dual_g0[0] * this.e1234) - (this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Plane other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(this.e25 * other.e4125 * -1.0, this.e35 * other.e4235 * -1.0, this.e15 * other.e4315 * -1.0, (this.e4315 * other.e4315) + (this.e4125 * other.e4125)) + (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g0_xyz.zxy * this.group0.yzx) - (right_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g0_xyz[1] * this.e31) - (right_dual_g0_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g0_xyz, right_dual_g0_xyz[0])), 
            /* e235, e315, e125, e12345 */
            float4(right_dual_g0_xyz[2] * this.e25 * -1.0, right_dual_g0_xyz[0] * this.e35 * -1.0, right_dual_g0_xyz[1] * this.e15 * -1.0, (right_dual_g0_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234)) + float4((right_dual_g0_xyz.yzx * this.group2.zxy), right_dual_g0_xyz[0] * this.e4235) + float4((this.group1.xyz * other.group0.www), right_dual_g0_xyz[1] * this.e4315)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float right_dual_g1_w = other.e321 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float2(right_dual_g0_w) * this.group2.xy), right_dual_g0_w * this.e35, (right_dual_g0_w * this.e1234) - (this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (this.e42 * other.e125) + (this.e45 * other.e415) + (this.e35 * other.e431), (right_dual_g1_w * this.e31) + (this.e43 * other.e235) + (this.e45 * other.e425) + (this.e15 * other.e412), (right_dual_g1_w * this.e12) + (this.e41 * other.e315) + (this.e45 * other.e435) + (this.e25 * other.e423), -(this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435)) + (float4(right_dual_g0_w) * this.group3) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_dual_g3_xyz.zxy * this.group0.yzx) - (right_dual_g3_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((this.e41 * other.e3215) + (this.e15 * other.e1234), (this.e42 * other.e3215) + (this.e25 * other.e1234), (this.e43 * other.e3215) + (this.e35 * other.e1234), -(right_dual_g3_xyz[1] * this.e31) - (right_dual_g3_xyz[2] * this.e12)) - (this.group1.wwwx * float4(right_dual_g3_xyz, right_dual_g3_xyz[0])), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), (right_dual_g3_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (this.e23 * other.e23) - (this.e31 * other.e31) - (this.e12 * other.e12) - (this.e45 * other.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)) + float4((right_dual_g3_xyz.yzx * this.group2.zxy), right_dual_g3_xyz[0] * this.e4235) + float4((this.group1.xyz * other.group3.www), right_dual_g3_xyz[1] * this.e4315) - float4((right_dual_g3_xyz.zxy * this.group2.yzx), right_dual_g2_xyz[0] * this.e41)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<DualNum> operator /(DualNum lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Plane bulk_expansion(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(other.group0.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public DualNum bulk_expansion(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Line bulk_expansion(Circle other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xx, this.e5, right_dual_g2_w * this.e12345) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, right_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Plane bulk_expansion(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(DipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public DualNum bulk_expansion(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * other.e45, 0.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e5 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiFlatPoint bulk_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), right_dual_g0[3] * this.e12345), 
            /* e235, e315, e125, e5 */
            right_dual_g0 * float4(this.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e12345) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group9.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e5) * other.group7.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e5) * other.group6.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group4.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public FlatPoint bulk_expansion(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public FlatPoint bulk_expansion(Sphere other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group0.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Motor bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xx, this.e5, (right_dual_g0_w * this.e12345) - (this.e5 * other.e4)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, right_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension DualNum {
    public Flector bulk_expansion(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<FlatPoint> operator /(FlatPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_expansion(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(CircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(DipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e1234 * this.e15, other.e1234 * this.e25, other.e1234 * this.e35, -(other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)) - (this.group0.wwwx * float4(right_dual_g3.xyz, other.e41)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_expansion(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(Flector other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * -1.0) * float4(right_dual_g1.xyz, other.e45), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Plane bulk_expansion(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * this.group0.www), -(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * this.group0.www), -(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public MultiVector bulk_expansion(MultiVector other) {
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.group0.yx[0] * -1.0) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) - (this.group0.yzxx * float4(other.group7.zxy, other.e415)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Line bulk_expansion(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Line(
            /* e415, e425, e435 */
            float3(this.e45 * -1.0) * right_dual_g0.xyz, 
            /* e235, e315, e125 */
            (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Line bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Line(
            /* e415, e425, e435 */
            (float3(other.e1234) * this.group0.xyz) - (right_dual_g0_xyz * float3(this.e45)), 
            /* e235, e315, e125 */
            (right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Flector bulk_expansion(VersorEven other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension FlatPoint {
    public Motor bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e15 * other.e1234, this.e25 * other.e1234, this.e35 * other.e1234, -(this.e25 * other.e42) - (this.e35 * other.e43) - (this.e45 * other.e45)) - (this.group0.wwwx * float4(right_dual_g3_xyz, other.e41)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Flector> operator /(Flector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g2_w) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45), (other.e412 * this.e15) + (other.e425 * this.e45), (other.e423 * this.e25) + (other.e435 * this.e45), -(other.e425 * this.e25) - (other.e435 * this.e35)) + (float4(right_dual_g2_w) * this.group1) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e15) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e1234) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e42 * this.e25) - (other.e43 * this.e35) - (other.e45 * this.e45)) - (this.group0.wwwx * float4(other.group3.xyz, other.e41)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public AntiScalar bulk_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45, this.e45, this.e45, (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) - (other.e45 * this.e45)) * float4((other.group1.xyz * float3(-1.0)), 1.0), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Plane bulk_expansion(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * this.group0.www), -(this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3] * this.e4235, right_dual_g0[3] * this.e4315, right_dual_g0[3] * this.e4125, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35)) + (right_dual_g0 * float4(this.group0.www, this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[0]) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual_g1_xyz.yzx * this.group0.zxy) - (right_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) + (float4(right_dual_g0[0]) * this.group1) - (this.group0.yzxx * float4(other.group7.zxy, other.e415)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Plane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * this.group0.www * float3(-1.0)), (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0_xyz[0] * this.e45 * -1.0, right_dual_g0_xyz[1] * this.e45 * -1.0, right_dual_g0_xyz[2] * this.e45 * -1.0, (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)) + (float4(other.e1234, other.e1234, other.e1234, right_dual_g0_xyz[0] * this.e4235) * float4(this.group0.xyz, 1.0)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g0_xyz.yzx * this.group0.zxy) - (right_dual_g0_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Flector bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_dual_g0_w) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.e35 * other.e431) + (this.e45 * other.e415), (this.e15 * other.e412) + (this.e45 * other.e425), (this.e25 * other.e423) + (this.e45 * other.e435), -(this.e25 * other.e425) - (this.e35 * other.e435)) + (float4(right_dual_g0_w) * this.group1) - (this.group0.yzxx * float4(other.group0.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Flector {
    public Motor bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (this.e25 * other.e42) - (this.e35 * other.e43) - (this.e45 * other.e45)) + float4((this.group0.xyz * other.group2.www), right_dual_g3_xyz[0] * this.e4235) - (this.group0.wwwx * float4(right_dual_g3_xyz, other.e41)), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g3_xyz.yzx * this.group0.zxy) - (right_dual_g3_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Line> operator /(Line lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Line bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_dual_g0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(right_dual_g2_w) * this.group0.xyz.xy), right_dual_g2_w * this.e435, -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual_g2_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(other.e1234 * this.e235) - (other.e4125 * this.e425), -(other.e1234 * this.e315) - (other.e4235 * this.e435), -(other.e1234 * this.e125) - (other.e4315 * this.e415), (other.e4315 * this.e315) + (other.e4125 * this.e125)) + (other.group3.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Line bulk_expansion(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e4125 * this.e425 * -1.0, other.e4235 * this.e435 * -1.0, other.e4315 * this.e415 * -1.0, (other.e4315 * this.e315) + (other.e4125 * this.e125)) + (other.group1.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public AntiScalar bulk_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(right_dual_g0_w) * this.group0.xyz.xy), right_dual_g0_w * this.e435, -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual_g0_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(right_dual_g0[0]) * this.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[0]) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g1_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g1_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g1_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g1_xyz[1] * this.e315) + (right_dual_g1_xyz[2] * this.e125)) + float4((right_dual_g1_xyz.yzx * this.group0.zxy), right_dual_g1_xyz[0] * this.e235), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g0_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g0_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g0_xyz[1] * this.e315) + (right_dual_g0_xyz[2] * this.e125)) + float4((right_dual_g0_xyz.yzx * this.group0.zxy), right_dual_g0_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Motor bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(right_dual_g0_w) * this.group0.xyz.xy), right_dual_g0_w * this.e435, -(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual_g0_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Line {
    public Plane bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g3_xyz[2] * this.e425) - (this.e235 * other.e1234), -(right_dual_g3_xyz[0] * this.e435) - (this.e315 * other.e1234), -(right_dual_g3_xyz[1] * this.e415) - (this.e125 * other.e1234), (right_dual_g3_xyz[1] * this.e315) + (right_dual_g3_xyz[2] * this.e125)) + float4((right_dual_g3_xyz.yzx * this.group0.zxy), right_dual_g3_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Motor> operator /(Motor lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Plane bulk_expansion(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5, this.e5, this.e5, -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e4 * this.e5)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Circle other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5, this.e5, this.e5, -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e423 * this.e5, other.e431 * this.e5, other.e412 * this.e5, -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)) + (float4(right_dual_g2_w) * this.group0), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2_w) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), right_dual_g2_w * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Plane bulk_expansion(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4((other.group0.xyz * float3(-1.0)), other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(DipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * float4(other.group3.xyz, other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e4125 * this.e425 * -1.0, other.e4235 * this.e435 * -1.0, other.e4315 * this.e415 * -1.0, other.e4125 * this.e125) + (other.group3.yzxy * float4(this.group0.zxy, this.e315)) + (this.group1.wwwx * float4(other.group0.xyz, other.e4235)) - (this.group1 * float4(other.group2.www, other.e45))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public AntiDualNum bulk_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * this.e5, 0.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e4315 * this.e315) + (other.e4125 * this.e125)) - float4((other.group1.zxy * this.group0.yzx), other.e45 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float2(right_dual_g0_w) * this.group0.xy), right_dual_g0_w * this.e435, (right_dual_g0_w * this.e12345) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0_w) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), right_dual_g0_w * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e12345) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            right_dual_g1 * float4(this.e5 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group7.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.e5) * other.group6.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g1[3] * this.e235 * -1.0, right_dual_g1[3] * this.e315 * -1.0, right_dual_g1[3] * this.e125 * -1.0, right_dual_g1[2] * this.e125) + (right_dual_g1.yzxx * float4(this.group0.zxy, this.e235)) + (this.group1.wwwy * float4(other.group4.xyz, right_dual_g1[1])) - float4((right_dual_g1.zxy * this.group0.yzx), this.e5 * other.e45), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Plane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (other.group0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.e5 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[1] * this.e315) + (right_dual_g0[2] * this.e125)) + (right_dual_g0.yzxx * float4(this.group0.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Motor bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0[3] * this.e415, right_dual_g0[3] * this.e425, right_dual_g0[3] * this.e435, -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e5 * other.e4)) + (right_dual_g0 * float4(this.group1.www, this.e12345)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), right_dual_g0[3] * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Motor {
    public Flector bulk_expansion(VersorOdd other) {
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g3 * float4(this.e5 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g3[2] * this.e425) - (right_dual_g3[3] * this.e235), -(right_dual_g3[0] * this.e435) - (right_dual_g3[3] * this.e315), -(right_dual_g3[1] * this.e415) - (right_dual_g3[3] * this.e125), (right_dual_g3[1] * this.e315) + (right_dual_g3[2] * this.e125)) + (right_dual_g3.yzxx * float4(this.group0.zxy, this.e235)) - (float4(this.e5) * float4((other.group0 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, other.e45))
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<MultiVector> operator /(MultiVector lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.scalar * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[1] * this.e3) + (other.e15 * this.e4), (right_dual_g1[2] * this.e1) + (other.e25 * this.e4), (right_dual_g1[0] * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (this.group1.yzxx * float4(right_dual_g1.zxy, other.e15)), 
            /* e1234 */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g2_w * this.e5) + (right_dual_g3_w * this.e4) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g1_w * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, right_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42) + (other.e1 * this.scalar), (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43) + (other.e2 * this.scalar), (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41) + (other.e3 * this.scalar), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + float4((float2(right_dual_g1_w) * this.group5.xyz.xy), right_dual_g1_w * this.e12, right_dual_g3_w * this.scalar) - (this.group3.yzxx * float4(other.group0.zxy, other.e415)) - float4((this.group4.zxy * other.group2.yzx), other.e425 * this.e25), 
            /* e1234 */
            (right_dual_g2_w * this.scalar) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e3215 * this.e1234) + (other.scalar * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e3215) * this.group4.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group4.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float right_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1_w * this.e4) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[3] * this.e23) + (other.e1 * this.scalar), (right_dual_g0[3] * this.e31) + (other.e2 * this.scalar), (right_dual_g0[3] * this.e12) + (other.e3 * this.scalar), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) + float4((this.group4.yzx * right_dual_g0.zxy), right_dual_g1_w * this.scalar) - (right_dual_g0.yzxx * float4(this.group4.zxy, this.e23)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * float3(this.scalar)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_dual_g1 * float3(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[1] * this.e3) + (right_dual_g1[0] * this.e4), (right_dual_g0[2] * this.e1) + (right_dual_g1[1] * this.e4), (right_dual_g0[0] * this.e2) + (right_dual_g1[2] * this.e4), -(right_dual_g1[1] * this.e2) - (right_dual_g1[2] * this.e3)) - (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g1[0])), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.scalar * this.scalar) + (other.e3215 * this.e1234) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e15 * this.e4) + (other.e3215 * this.e423), (other.e25 * this.e4) + (other.e3215 * this.e431), (other.e35 * this.e4) + (other.e3215 * this.e412), -(other.e25 * this.e2) - (other.e35 * this.e3)) + float4((other.group0.yzx * this.group1.zxy), other.e3215 * this.e321) - (this.group1.yzxx * float4(other.group0.zxy, other.e15)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiPlane other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.scalar), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * this.group1, 
            /* e5 */
            right_dual_g0 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0) * this.group3, 
            /* e41, e42, e43 */
            float3(right_dual_g0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * this.group6, 
            /* e423, e431, e412 */
            float3(right_dual_g0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * this.group9, 
            /* e1234 */
            right_dual_g0 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g1_w * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, right_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1_w * this.e23) + (other.e431 * this.e35) + (other.e415 * this.e45) + (other.e125 * this.e42), (right_dual_g1_w * this.e31) + (other.e412 * this.e15) + (other.e425 * this.e45) + (other.e235 * this.e43), (right_dual_g1_w * this.e12) + (other.e423 * this.e25) + (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - (this.group3.yzxx * float4(other.group0.zxy, other.e415)) - float4((other.group2.yzx * this.group4.zxy), other.e425 * this.e25), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float4 right_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g2[3] * this.scalar, (right_dual_g2[3] * this.e12345) - (right_dual_g1_w * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g2[3]) * this.group1, 
            /* e5 */
            right_dual_g2[3] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_dual_g2[3]) * this.group3) + (float4(this.scalar) * float4(right_dual_g2.xyz, right_dual_g1_w)), 
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g2[3]) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g2[3] * this.e415) + (other.e423 * this.e5), (right_dual_g2[3] * this.e425) + (other.e431 * this.e5), (right_dual_g2[3] * this.e435) + (other.e412 * this.e5), -(other.e425 * this.e2) - (other.e435 * this.e3)) + (right_dual_g2 * float4(this.group1.www, this.e321)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(right_dual_g2[3]) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g2[3]) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (right_dual_g2.zxy * this.group1.yzx) - (right_dual_g2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g2[2] * this.e42) + (right_dual_g2[3] * this.e4235) + (other.e431 * this.e35) + (other.e415 * this.e45), (right_dual_g2[0] * this.e43) + (right_dual_g2[3] * this.e4315) + (other.e412 * this.e15) + (other.e425 * this.e45), (right_dual_g2[1] * this.e41) + (right_dual_g2[3] * this.e4125) + (other.e423 * this.e25) + (other.e435 * this.e45), -(right_dual_g2[2] * this.e12) - (other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35)) + float4((float2(right_dual_g1_w) * this.group5.xyz.xy), right_dual_g1_w * this.e12, right_dual_g2[3] * this.e3215) - (right_dual_g2.yzxx * float4(this.group4.zxy, this.e23)) - float4((other.group0.zxy * this.group3.yzx), right_dual_g2[1] * this.e31), 
            /* e1234 */
            (right_dual_g2[3] * this.e1234) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e235, e315, e125 */
            right_dual_g2 * float3(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g2[0] * this.e4) + (right_dual_g1[1] * this.e3), (right_dual_g2[1] * this.e4) + (right_dual_g1[2] * this.e1), (right_dual_g2[2] * this.e4) + (right_dual_g1[0] * this.e2), -(right_dual_g2[2] * this.e3) - (right_dual_g1[3] * this.e5)) - (float4(this.e5, this.e5, this.e5, right_dual_g2[0] * this.e1) * float4(right_dual_g0, 1.0)) - (this.group1.yzxy * float4(right_dual_g1.zxy, right_dual_g2[1])), 
            /* e1234 */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1234 * this.e3215) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e1234), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group1) - (float4(this.e5) * float4(other.group3.xyz, other.e1234)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group3.zxy * this.group1.yzx) - (other.group3.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e1234 * this.e15) + (other.e3215 * this.e41), (other.e1234 * this.e25) + (other.e3215 * this.e42), (other.e1234 * this.e35) + (other.e3215 * this.e43), -(other.e4315 * this.e31) - (other.e4125 * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (other.group3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (right_dual_g0 * float3(this.scalar)) + (float3(other.e1234) * this.group5.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group3.zxy) - (other.group3.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group7.xyz, this.e321)) + (other.group3.yzxz * float4(this.group6.zxy, this.e125)) + float4((right_dual_g1.yzx * this.group1.zxy), other.e4235 * this.e235) + float4((other.group2.xyz * this.group1.www), other.e4315 * this.e315) - (float4(this.e5) * float4(right_dual_g0, right_dual_g1[3])) - (other.group2.wwwy * float4(this.group8.xyz, this.e2)) - (this.group1.yzxx * float4(right_dual_g1.zxy, other.e15)) - float4((other.group3.zxy * this.group6.yzx), other.e35 * this.e3), 
            /* e1234 */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g1[3] * this.e4) - (other.e1234 * this.e321) - (other.e4235 * this.e423) - (other.e4315 * this.e431) - (other.e4125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e12345 * this.scalar, (other.e5 * this.e4) + (other.e12345 * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5 */
            other.e12345 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group3, 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group6, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yy, other.e12345, (other.e5 * this.scalar) + (other.e12345 * this.e3215)) * float4(this.group9.xyz, 1.0), 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e45 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e45 * this.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e4, this.e4, this.e4, -(other.e15 * this.e1) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) * float4(other.group0.xyz, 1.0), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e45 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), other.e3215 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e4315 * this.e31) - (other.e4125 * this.e12)) + float4((this.group4.xyz * other.group1.www), other.e45 * this.scalar) - (other.group1.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (this.group4.yzx * other.group1.zxy) - (this.group4.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxy * float4(this.group6.zxy, this.e315)) + (other.group1.wwwz * float4(this.group7.xyz, this.e125)) + float4(float3(0.0), (other.e3215 * this.e321) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) + float4((other.group0.xyz * this.group1.www), other.e4235 * this.e235) - float4((other.group1.zxy * this.group6.yzx), other.e15 * this.e1), 
            /* e1234 */
            (other.e45 * this.e4) - (other.e4235 * this.e423) - (other.e4315 * this.e431) - (other.e4125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) * float4(other.group1.xyz, 1.0), 
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e415 * this.e45) + (other.e125 * this.e42), (other.e425 * this.e45) + (other.e235 * this.e43), (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group1.yzx * this.group4.zxy), other.e415 * this.e15), 
            /* e1234 */
            -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float right_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0_w * this.scalar, (right_dual_g0_w * this.e12345) + (right_dual_g1_w * this.e4) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0_w) * this.group1, 
            /* e5 */
            right_dual_g0_w * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual_g0_w) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz)), right_dual_g0_w * this.e45), 
            /* e41, e42, e43 */
            float3(right_dual_g0_w) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(right_dual_g0_w) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e235 * this.e4, other.e315 * this.e4, other.e125 * this.e4, -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) + (float4(right_dual_g0_w) * this.group6), 
            /* e423, e431, e412 */
            (float3(right_dual_g0_w) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g0_w) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e125 * this.e42, other.e235 * this.e43, other.e315 * this.e41, -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) + (float4(right_dual_g0_w) * this.group9) + float4((other.group0.xyz * this.group3.www), right_dual_g1_w * this.scalar) - float4((this.group4.zxy * other.group1.yzx), other.e415 * this.e15), 
            /* e1234 */
            (right_dual_g0_w * this.e1234) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float right_dual_g9_w = other.e5 * -1.0;
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[0] * this.scalar, (right_dual_g10 * this.e5) + (right_dual_g9_w * this.e4) + (right_dual_g0[0] * this.e12345) + (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (other.e45 * this.e45) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e1, e2, e3, e4 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group1), 
            /* e5 */
            (right_dual_g0[0] * this.e5) + (other.e3215 * this.scalar), 
            /* e15, e25, e35, e45 */
            (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * this.group3) + (float4(other.e3215) * this.group1) - (right_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[0]) * this.group4.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[0]) * this.group5.xyz) + (float3(this.scalar) * other.group6.xyz) + (right_dual_g1.zxy * this.group1.yzx) - (right_dual_g1.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3[0] * this.e4) + (other.e23 * this.scalar) + (other.e423 * this.e5) + (other.e3215 * this.e41), (right_dual_g3[1] * this.e4) + (other.e31 * this.scalar) + (other.e431 * this.e5) + (other.e3215 * this.e42), (right_dual_g3[2] * this.e4) + (other.e12 * this.scalar) + (other.e412 * this.e5) + (other.e3215 * this.e43), -(right_dual_g1[2] * this.e12) - (other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) + (float4(right_dual_g0[0]) * this.group6) + float4((this.group3.xyz * right_dual_g1.www), other.e45 * this.scalar) - (right_dual_g1.xyzx * float4(this.group3.www, this.e23)) - float4((this.group1.xyz * right_dual_g3.www), right_dual_g1[1] * this.e31), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g0[0]) * this.group7.xyz) + (float3(right_dual_g1[3]) * this.group5.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group1.zxy) + (this.group4.yzx * right_dual_g1.zxy) - (other.group7.zxy * this.group1.yzx) - (this.group4.zxy * right_dual_g1.yzx), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.scalar)) + (float3(right_dual_g0[0]) * this.group8.xyz) + (float3(other.e3215) * this.group5.xyz) + (float3(this.e5) * other.group6.xyz) + (right_dual_g1.yzx * this.group3.zxy) + (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g3.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e415 * this.e45) + (other.e431 * this.e35) + (other.e3215 * this.e423), (other.e12 * this.e1) + (other.e425 * this.e45) + (other.e412 * this.e15) + (other.e3215 * this.e431), (other.e23 * this.e2) + (other.e435 * this.e45) + (other.e423 * this.e25) + (other.e3215 * this.e412), -(other.e45 * this.e5) - (other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35)) + (right_dual_g1.yzxx * float4(this.group6.zxy, this.e235)) + (float4(right_dual_g0.xx, right_dual_g0[0], right_dual_g9_w * this.scalar) * float4(this.group9.xyz, 1.0)) + (float4(this.group0.xx, this.scalar, other.e3215 * this.e321) * float4(other.group1.xyz, 1.0)) + float4((right_dual_g8 * this.group1.www), right_dual_g0[0] * this.e3215) + float4((this.group5.xyz * right_dual_g3.www), right_dual_g1[2] * this.e125) + float4((this.group4.yzx * right_dual_g3.zxy), right_dual_g1[1] * this.e315) - (float4(this.e5, this.e5, this.e5, right_dual_g8[0] * this.e1) * float4(right_dual_g7, 1.0)) - (right_dual_g3.yzxx * float4(this.group4.zxy, this.e23)) - float4((this.group8.xyz * right_dual_g1.www), right_dual_g8[2] * this.e3) - float4((other.group5.zxy * this.group1.yzx), right_dual_g3[1] * this.e31) - float4((other.group7.zxy * this.group3.yzx), right_dual_g3[2] * this.e12) - float4((right_dual_g1.zxy * this.group6.yzx), right_dual_g8[1] * this.e2), 
            /* e1234 */
            (right_dual_g10 * this.scalar) + (right_dual_g0[0] * this.e1234) + (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (other.e45 * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * other.e4235) + (this.e4315 * other.e4315) + (this.e4125 * other.e4125) + (this.e1234 * other.e3215)), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), this.e4 * other.e3215), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(this.e41 * other.e3215, this.e42 * other.e3215, this.e43 * other.e3215, -(this.e31 * other.e4315) - (this.e12 * other.e4125)) - (other.group0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) - (this.group3.yzx * other.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e425 * other.e4125 * -1.0, this.e435 * other.e4235 * -1.0, this.e415 * other.e4315 * -1.0, (this.e315 * other.e4315) + (this.e125 * other.e4125)) + (this.group6.zxyw * other.group0.yzxw) + (other.group0.wwwx * float4(this.group7.xyz, this.e235)), 
            /* e1234 */
            -(this.e423 * other.e4235) - (this.e431 * other.e4315) - (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1 * this.e5) + (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.scalar), 
            /* e1234 */
            right_dual_g1 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215)), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group1) - (right_dual_g0 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g0[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g0[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(right_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_dual_g0.zxy) - (this.group4.zxy * right_dual_g0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) + (right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0[2] * this.e425) - (right_dual_g0[3] * this.e235), -(right_dual_g0[0] * this.e435) - (right_dual_g0[3] * this.e315), -(right_dual_g0[1] * this.e415) - (right_dual_g0[3] * this.e125), (right_dual_g0[2] * this.e125) + (this.e321 * other.e3215)) + (right_dual_g0.yzxx * float4(this.group6.zxy, this.e235)) + float4((this.group7.xyz * other.group0.www), right_dual_g0[1] * this.e315), 
            /* e1234 */
            -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        float right_dual_g2_w = other.e4 * -1.0;
        float right_dual_g3_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[3] * this.scalar, (right_dual_g2_w * this.e5) + (right_dual_g3_w * this.e4) + (right_dual_g0[3] * this.e12345) + (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g1_w * this.e321) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e5 */
            right_dual_g0[3] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(this.group0.xx, this.scalar, right_dual_g0[3] * this.e45) * float4(other.group2.xyz, 1.0)) + float4((this.group3.xyz * right_dual_g0.www), right_dual_g1_w * this.scalar), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[3]) * this.group4.xyz) + (float3(this.scalar) * right_dual_g0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[3]) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0[3] * this.e415) + (this.e4 * other.e235), (right_dual_g0[3] * this.e425) + (this.e4 * other.e315), (right_dual_g0[3] * this.e435) + (this.e4 * other.e125), -(this.e2 * other.e425) - (this.e3 * other.e435)) + (right_dual_g0 * float4(this.e5, this.e5, this.e5, this.e321)) - (this.group1.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(right_dual_g0[3]) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + (this.group1.yzx * other.group2.zxy) - (this.group1.zxy * other.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[3] * this.e4235) + (this.scalar * other.e1) + (this.e45 * other.e415) + (this.e42 * other.e125), (right_dual_g0[3] * this.e4315) + (this.scalar * other.e2) + (this.e45 * other.e425) + (this.e43 * other.e235), (right_dual_g0[3] * this.e4125) + (this.scalar * other.e3) + (this.e45 * other.e435) + (this.e41 * other.e315), -(this.e35 * other.e435) - (this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125)) + (right_dual_g0.yzxw * float4(this.group3.zxy, this.e3215)) + float4((float2(right_dual_g1_w) * this.group5.xyz.xy), right_dual_g1_w * this.e12, right_dual_g3_w * this.scalar) - (this.group3.yzxx * float4(right_dual_g0.zxy, other.e415)) - float4((this.group4.zxy * other.group2.yzx), this.e25 * other.e425), 
            /* e1234 */
            (right_dual_g2_w * this.scalar) + (right_dual_g0[3] * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension MultiVector {
    public MultiVector bulk_expansion(VersorOdd other) {
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) + (this.scalar * other.scalar) + (this.e1234 * other.e3215) - (right_dual_g2_xyz[0] * this.e41) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (this.e15 * other.e41) - (this.e25 * other.e42) - (this.e35 * other.e43)), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group1) - (right_dual_g3 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual_g3.xyz) - (float3(right_dual_g3[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3[3] * this.e15) + (this.e41 * other.e3215), (right_dual_g3[3] * this.e25) + (this.e42 * other.e3215), (right_dual_g3[3] * this.e35) + (this.e43 * other.e3215), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * float4(this.group3.www, this.e23)), 
            /* e423, e431, e412 */
            (float3(right_dual_g3[3]) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.yzx * right_dual_g3.zxy) - (this.group4.zxy * right_dual_g3.yzx), 
            /* e235, e315, e125 */
            (right_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group5.xyz) + (right_dual_g3.yzx * this.group3.zxy) - (right_dual_g3.zxy * this.group3.yzx), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group7.xyz, this.e321)) + (right_dual_g3.yzxz * float4(this.group6.zxy, this.e125)) + float4((right_dual_g2_xyz * this.group1.www), right_dual_g3[0] * this.e235) + float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g3[1] * this.e315) - (float4(this.e5) * float4(other.group0.xyz, right_dual_g1[3])) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g2_xyz[0])) - float4((this.group8.xyz * right_dual_g3.www), right_dual_g2_xyz[2] * this.e3) - float4((right_dual_g3.zxy * this.group6.yzx), right_dual_g2_xyz[1] * this.e2), 
            /* e1234 */
            (right_dual_g1[3] * this.e4) + (this.e1 * other.e41) + (this.e2 * other.e42) + (this.e3 * other.e43) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Plane> operator /(Plane lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e1234 * this.e3215) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public MultiVector bulk_expansion(MultiVector other) {
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (other.e1234 * this.e3215)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yx[0] * -1.0) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public Plane bulk_expansion(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Plane {
    public AntiScalar bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_dual_g3_xyz[0] * this.e4235) + (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<RoundPoint> operator /(RoundPoint lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e321 * this.e1) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e321 * this.e2) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e321 * this.e3) + (other.e125 * this.e4), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5, this.e5, this.e5, other.e1 * this.e1) * float4(other.group1.xyz, 1.0)) + (this.group0.yzxy * float4(other.group2.zxy, other.e2)) + float4(float3(0.0), (other.e3 * this.e3) - (other.e5 * this.e4)) - (other.group2.yzxw * float4(this.group0.zxy, this.e5))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public FlatPoint bulk_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Line bulk_expansion(AntiFlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            (float3(other.e321) * this.group0.xyz) + (float3(this.e4) * other.group0.xyz), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Motor bulk_expansion(AntiFlector other) {
        float right_dual_g0_w = other.e321 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_dual_g0_w * this.e1 * -1.0, right_dual_g0_w * this.e2 * -1.0, right_dual_g0_w * this.e3 * -1.0, (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)) + (float4(this.e4) * float4(other.group0.xyz, other.e5 * -1.0)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Plane bulk_expansion(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Flector bulk_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar bulk_expansion(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkExpansionInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * this.group0, 
            /* e5 */
            right_dual_g0 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Circle bulk_expansion(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e321 * this.e1) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e321 * this.e2) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e321 * this.e3) + (other.e125 * this.e4), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e321 * this.e1) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e321 * this.e2) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e321 * this.e3) + (other.e125 * this.e4), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)), right_dual_g2_w * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g2_w) * float4(this.group0.xyz, this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group0.yzxw) - float4((other.group3.yzx * this.group0.zxy), other.e1234 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(other.e4235 * this.e5 * -1.0, other.e4315 * this.e5 * -1.0, other.e4125 * this.e5 * -1.0, (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e5 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e12345 * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Sphere bulk_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e4, this.e4, this.e4, -(other.e15 * this.e1) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) * float4(other.group0.xyz, 1.0), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), other.e45 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e4, this.e4, this.e4, -(other.e15 * this.e1) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) * float4(other.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Circle bulk_expansion(Line other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) * float4(other.group1.xyz, 1.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) * float4(other.group1.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), right_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0_w) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[0]) * this.group0, 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group0) - (right_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group0.xyz), 
            /* e23, e31, e12 */
            (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e321 * this.e1) + (other.e423 * this.e5) + (other.e235 * this.e4), (other.e321 * this.e2) + (other.e431 * this.e5) + (other.e315 * this.e4), (other.e321 * this.e3) + (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g8[0] * this.e4) + (other.e31 * this.e3), (right_dual_g8[1] * this.e4) + (other.e12 * this.e1), (right_dual_g8[2] * this.e4) + (other.e23 * this.e2), -(right_dual_g8[2] * this.e3) - (other.e45 * this.e5)) - (float4(this.e5, this.e5, this.e5, right_dual_g8[0] * this.e1) * float4(other.group4.xyz, 1.0)) - (this.group0.yzxy * float4(other.group5.zxy, right_dual_g8[1])), 
            /* e1234 */
            (other.e45 * this.e4) + (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Dipole bulk_expansion(Plane other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), other.e3215 * this.e4), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public Dipole bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_dual_g0_xyz.zxy, other.e3215)) - float4((right_dual_g0_xyz.yzx * this.group0.zxy), this.e5 * other.e1234), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (right_dual_g0_xyz * float3(this.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0.zxyx * float4(other.group0.yzx, other.e1)) + (this.group0.wwwy * float4(other.group1.xyz, other.e2)) + float4(float3(0.0), (this.e3 * other.e3) - (this.e5 * other.e4)) - (this.group0.yzxw * float4(other.group0.zxy, other.e5)), 
            /* e415, e425, e435, e321 */
            float4((this.e1 * other.e321) + (this.e4 * other.e235) + (this.e5 * other.e423), (this.e2 * other.e321) + (this.e4 * other.e315) + (this.e5 * other.e431), (this.e3 * other.e321) + (this.e4 * other.e125) + (this.e5 * other.e412), -(this.e1 * other.e415) - (this.e2 * other.e425) - (this.e3 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) - (this.group0.zxy * other.group2.yzx)), right_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0_w) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_dual_g3_xyz.zxy, other.e3215)) - float4((right_dual_g3_xyz.yzx * this.group0.zxy), this.e5 * other.e1234), 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g3_xyz[0] * this.e5 * -1.0, right_dual_g3_xyz[1] * this.e5 * -1.0, right_dual_g3_xyz[2] * this.e5 * -1.0, (this.e2 * other.e42) + (this.e3 * other.e43) + (this.e4 * other.e45)) + (this.group0.xyzx * float4(other.group3.www, other.e41)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g2_xyz[0] * this.e4) + (this.e3 * other.e31), (right_dual_g2_xyz[1] * this.e4) + (this.e1 * other.e12), (right_dual_g2_xyz[2] * this.e4) + (this.e2 * other.e23), -(right_dual_g2_xyz[2] * this.e3) - (this.e5 * other.e45)) - (float4(this.e5, this.e5, this.e5 * other.e43, right_dual_g2_xyz[1] * this.e2) * float4(other.group0.xy, 1.0, 1.0)) - (this.group0.yzxx * float4(other.group1.zxy, right_dual_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Scalar> operator /(Scalar lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public DualNum bulk_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public FlatPoint bulk_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Flector bulk_expansion(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Line bulk_expansion(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Motor bulk_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkExpansionInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Scalar bulk_expansion(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkExpansionInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Dipole bulk_expansion(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkExpansionInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiCircleRotor bulk_expansion(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkExpansionInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Circle bulk_expansion(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkExpansionInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiDipoleInversion bulk_expansion(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkExpansionInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiDualNum bulk_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiFlatPoint bulk_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkExpansionInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiFlector bulk_expansion(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkExpansionInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiLine bulk_expansion(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkExpansionInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiMotor bulk_expansion(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public MultiVector bulk_expansion(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group9.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(this.scalar * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkExpansionInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiPlane bulk_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public Sphere bulk_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkExpansionInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public RoundPoint bulk_expansion(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.scalar * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public VersorOdd bulk_expansion(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Scalar {
    public VersorEven bulk_expansion(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<Sphere> operator /(Sphere lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * this.group0, 
            /* e1234 */
            right_dual_g0 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(CircleRotor other) {
        float right_dual_g2_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2_w) * this.group0, 
            /* e1234 */
            right_dual_g2_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e1234 * this.e3215) + (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(DualNum other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(Motor other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0_w) * this.group0, 
            /* e1234 */
            right_dual_g0_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g1_xyz[0] * this.e4235) + (right_dual_g1_xyz[1] * this.e4315) + (right_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[0]) * this.group0, 
            /* e1234 */
            right_dual_g0[0] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e4235) + (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_dual_g0_xyz[0] * this.e4235) + (right_dual_g0_xyz[1] * this.e4315) + (right_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public Sphere bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0_w) * this.group0, 
            /* e1234 */
            right_dual_g0_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension Sphere {
    public AntiScalar bulk_expansion(VersorOdd other) {
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_dual_g3_xyz[0] * this.e4235) + (right_dual_g3_xyz[1] * this.e4315) + (right_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<VersorEven> operator /(VersorEven lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group3.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(AntiDipoleInversion other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e4 * -1.0)) + (this.group3.yzxx * float4(other.group2.zxy, other.e1)) + float4(float3(0.0), (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)) - float4((other.group2.yzx * this.group3.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Flector bulk_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Motor bulk_expansion(AntiFlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e321 * this.e1, other.e321 * this.e2, other.e321 * this.e3, -(other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) + (other.group0 * float4(this.group3.www, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Motor bulk_expansion(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (other.group0 * float4(this.group3.www, this.e321)) + (this.group3.xyzx * float4(other.group0.www, other.e1)) + float4(float3(0.0), (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Plane bulk_expansion(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (this.group3.yzxx * float4(other.group0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Flector bulk_expansion(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e15 * this.e4) + (other.e3215 * this.e423), (other.e25 * this.e4) + (other.e3215 * this.e431), (other.e35 * this.e4) + (other.e3215 * this.e412), -(other.e25 * this.e2) - (other.e35 * this.e3)) + float4((right_dual_g0.yzx * this.group3.zxy), other.e3215 * this.e321) - (this.group3.yzxx * float4(right_dual_g0.zxy, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar bulk_expansion(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_dual_g0) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(Circle other) {
        float right_dual_g1_w = other.e321 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e12345 */
            float4((other.e415 * this.e5) + (other.e125 * this.e2), (other.e425 * this.e5) + (other.e235 * this.e3), (other.e435 * this.e5) + (other.e315 * this.e1), -(other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) - float4((other.group2.yzx * this.group3.zxy), right_dual_g1_w * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(CircleRotor other) {
        float right_dual_g1_w = other.e321 * -1.0;
        float4 right_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.e431 * this.e3) + (other.e415 * this.e4), (other.e412 * this.e1) + (other.e425 * this.e4), (other.e423 * this.e2) + (other.e435 * this.e4), -(right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)) + (float4(right_dual_g2[3]) * this.group0) - float4((other.group0.zxy * this.group3.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g2[3] * this.e415) + (other.e423 * this.e5), (right_dual_g2[3] * this.e425) + (other.e431 * this.e5), (right_dual_g2[3] * this.e435) + (other.e412 * this.e5), -(other.e425 * this.e2) - (other.e435 * this.e3)) + (right_dual_g2 * float4(this.group3.www, this.e321)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (right_dual_g2.zxy * this.group3.yzx) - (right_dual_g2.yzx * this.group3.zxy)), right_dual_g2[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g2[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((other.e31 * this.e3) + (other.e15 * this.e4), (other.e12 * this.e1) + (other.e25 * this.e4), (other.e23 * this.e2) + (other.e35 * this.e4), -(other.e25 * this.e2) - (other.e35 * this.e3)) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (this.group3.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e1234 */
            (other.e41 * this.e1) + (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group3.yzxw) - float4((other.group3.yzx * this.group3.zxy), other.e1234 * this.e5), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (other.e42 * this.e2) + (other.e43 * this.e3) + (other.e45 * this.e4) - (other.e4235 * this.e423) - (other.e4315 * this.e431) - (other.e4125 * this.e412)) - float4((other.group3.xyz * this.group2.www), other.e1234 * this.e321), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxz * float4(this.group1.zxy, this.e125)) + float4((other.group1.yzx * this.group3.zxy), other.e4235 * this.e235) + float4((other.group2.xyz * this.group3.www), other.e4315 * this.e315) - (float4(this.e5) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e15)) - float4((other.group3.zxy * this.group1.yzx), other.e35 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.group0.yy, other.e12345, (other.e5 * this.e4) + (other.e12345 * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public Sphere bulk_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e4, this.e4, this.e4, -(other.e15 * this.e1) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) * float4(other.group0.xyz, 1.0), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * this.e431) - (other.e4125 * this.e412)) - (other.group1.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxy * float4(this.group1.zxy, this.e315)) + (other.group1.wwwz * float4(this.group0.xyz, this.e125)) + float4(float3(0.0), (other.e3215 * this.e321) - (other.e25 * this.e2) - (other.e35 * this.e3) - (other.e45 * this.e5)) + float4((other.group0.xyz * this.group3.www), other.e4235 * this.e235) - float4((other.group1.zxy * this.group1.yzx), other.e15 * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor bulk_expansion(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e4, this.e4, this.e4, -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)) * float4(other.group1.xyz, 1.0), 
            /* e235, e315, e125, e12345 */
            float4((other.e415 * this.e5) + (other.e125 * this.e2), (other.e425 * this.e5) + (other.e235 * this.e3), (other.e435 * this.e5) + (other.e315 * this.e1), -(other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)) - float4((other.group1.yzx * this.group3.zxy), other.e415 * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0[3] * this.e423, right_dual_g0[3] * this.e431, right_dual_g0[3] * this.e412, -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4)) + (right_dual_g0 * float4(this.group3.www, this.e12345)), 
            /* e415, e425, e435, e321 */
            float4(other.e235 * this.e4, other.e315 * this.e4, other.e125 * this.e4, -(right_dual_g0[0] * this.e1) - (right_dual_g0[1] * this.e2) - (right_dual_g0[2] * this.e3)) + (float4(right_dual_g0[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_dual_g0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_dual_g3_w = other.e321 * -1.0;
        float3 right_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_dual_g0[0] * this.e12345) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g3_w * this.e321) - (other.e4 * this.e5) - (other.e5 * this.e4) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[0]) * this.group3, 
            /* e5 */
            right_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group3) - (right_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group3.xyz), 
            /* e23, e31, e12 */
            (right_dual_g1.zxy * this.group3.yzx) - (right_dual_g1.yzx * this.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4((other.e423 * this.e5) + (other.e235 * this.e4), (other.e431 * this.e5) + (other.e315 * this.e4), (other.e412 * this.e5) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) + (float4(right_dual_g0[0]) * this.group1) - (this.group3.xyzx * float4(float3(right_dual_g3_w), other.e415)), 
            /* e423, e431, e412 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.e4) * other.group6.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[0]) * this.group2.xyz) + (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_dual_g1.yzxz * float4(this.group1.zxy, this.e125)) + float4((right_dual_g8 * this.group3.www), right_dual_g1[1] * this.e315) + float4((right_dual_g6_xyz.yzx * this.group3.zxy), right_dual_g1[0] * this.e235) - (this.group2 * float4(right_dual_g1.www, other.e45)) - (this.group3.yzxx * float4(right_dual_g6_xyz.zxy, right_dual_g8[0])) - float4((right_dual_g7 * this.group2.www), right_dual_g8[1] * this.e2) - float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g8[2] * this.e3), 
            /* e1234 */
            (right_dual_g7[0] * this.e1) + (right_dual_g7[1] * this.e2) + (right_dual_g7[2] * this.e3) + (other.e45 * this.e4) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(other.e3215 * this.e1, other.e3215 * this.e2, other.e3215 * this.e3, -(other.e4315 * this.e431) - (other.e4125 * this.e412)) - (other.group0.xyzx * float4(this.group2.www, this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e4125 * this.e425 * -1.0, other.e4235 * this.e435 * -1.0, other.e4315 * this.e415 * -1.0, (other.e4125 * this.e125) + (other.e3215 * this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + (other.group0.wwwy * float4(this.group0.xyz, this.e315))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar bulk_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(Sphere other) {
        float3 right_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_dual_g0_xyz.zxy, other.e3215)) - float4((right_dual_g0_xyz.yzx * this.group3.zxy), other.e1234 * this.e5), 
            /* e15, e25, e35, e1234 */
            float4(other.e3215 * this.e1, other.e3215 * this.e2, other.e3215 * this.e3, -(right_dual_g0_xyz[1] * this.e431) - (right_dual_g0_xyz[2] * this.e412) - (other.e1234 * this.e321)) - float4((right_dual_g0_xyz * this.group2.www), right_dual_g0_xyz[0] * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            float4(-(right_dual_g0_xyz[2] * this.e425) - (other.e1234 * this.e235), -(right_dual_g0_xyz[0] * this.e435) - (other.e1234 * this.e315), -(right_dual_g0_xyz[1] * this.e415) - (other.e1234 * this.e125), (right_dual_g0_xyz[2] * this.e125) + (other.e3215 * this.e321)) + float4((right_dual_g0_xyz.yzx * this.group1.zxy), right_dual_g0_xyz[0] * this.e235) + float4((this.group0.xyz * other.group0.www), right_dual_g0_xyz[1] * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public VersorEven bulk_expansion(VersorEven other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_dual_g1_w = other.e321 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g0.yzxw * float4(this.group3.zxy, this.e12345)) + (this.group3.wwwy * float4(other.group1.xyz, other.e2)) + float4(float3(0.0), (other.e3 * this.e3) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e5 * this.e4) - (other.e4 * this.e5)) + float4((this.group0.xyz * right_dual_g0.www), other.e1 * this.e1) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g1_w * this.e321), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0[3] * this.e415) + (other.e235 * this.e4), (right_dual_g0[3] * this.e425) + (other.e315 * this.e4), (right_dual_g0[3] * this.e435) + (other.e125 * this.e4), -(other.e425 * this.e2) - (other.e435 * this.e3)) + (right_dual_g0 * float4(this.group2.www, this.e321)) - (this.group3.xyzx * float4(float3(right_dual_g1_w), other.e415)), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy)), right_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion bulk_expansion(VersorOdd other) {
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_dual_g3_xyz.zxy, other.e3215)) - float4((right_dual_g3_xyz.yzx * this.group3.zxy), this.e5 * other.e1234), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(other.group3.www, other.e41)) + float4(float3(0.0), (this.e2 * other.e42) + (this.e3 * other.e43) + (this.e4 * other.e45) - (right_dual_g3_xyz[1] * this.e431) - (right_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)) - float4((right_dual_g3_xyz * this.group2.www), right_dual_g3_xyz[0] * this.e423), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g2_xyz * this.group3.www), right_dual_g3_xyz[0] * this.e235) + float4((right_dual_g3_xyz.yzx * this.group1.zxy), right_dual_g3_xyz[1] * this.e315) + float4((this.group3.zxy * other.group1.yzx), this.e321 * other.e3215) + float4((this.group0.xyz * other.group3.www), right_dual_g3_xyz[2] * this.e125) - (other.group1.zxyw * float4(this.group3.yzx, this.e5)) - float4((right_dual_g3_xyz.zxy * this.group1.yzx), right_dual_g2_xyz[0] * this.e1) - float4((this.group2.xyz * other.group2.www), right_dual_g2_xyz[1] * this.e2) - float4((other.group0.xyz * this.group2.www), right_dual_g2_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (first half)
public BulkExpansionInfixPartial<VersorOdd> operator /(VersorOdd lhs, BulkExpansionInfix rhs) {
    return BulkExpansionInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, (other.scalar * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)) * float4(other.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(AntiDipoleInversion other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar, this.scalar, this.scalar, -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar)) * float4(other.group2.xyz, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42) + (other.e1 * this.scalar), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43) + (other.e2 * this.scalar), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41) + (other.e3 * this.scalar), -(right_dual_g1[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12) - (other.e5 * this.scalar)) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((other.group2.yzx * this.group0.zxy), right_dual_g1[1] * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.group0.xx, other.e3215, (other.e3215 * this.e1234) + (other.scalar * this.scalar)) * float4(this.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Flector bulk_expansion(AntiFlatPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Flector bulk_expansion(AntiFlector other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[2] * this.e42) + (right_dual_g0[3] * this.e23) + (other.e1 * this.scalar), (right_dual_g0[0] * this.e43) + (right_dual_g0[3] * this.e31) + (other.e2 * this.scalar), (right_dual_g0[1] * this.e41) + (right_dual_g0[3] * this.e12) + (other.e3 * this.scalar), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (other.e5 * this.scalar)) - (right_dual_g0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiLine other) {
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar, this.scalar, this.scalar, -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)) * float4(other.group0.xyz, 1.0), 
            /* e235, e315, e125, e5 */
            float4((right_dual_g1 * float3(this.scalar)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Motor bulk_expansion(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            (right_dual_g0 * float4(this.scalar)) + (float4(other.e3215) * float4(this.group0.xyz, this.e1234)) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e3215 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Plane bulk_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(AntiScalar other) {
        float right_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(Circle other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar, this.scalar, this.scalar, -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) * float4(other.group2.xyz, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (other.e431 * this.e35) + (other.e125 * this.e42), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (other.e412 * this.e15) + (other.e235 * this.e43), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (other.e423 * this.e25) + (other.e315 * this.e41), -(right_dual_g1[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - (this.group2.yzxx * float4(other.group0.zxy, right_dual_g1[0])) - float4((other.group2.yzx * this.group0.zxy), right_dual_g1[1] * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(CircleRotor other) {
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g2[3] * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g2[3]) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(right_dual_g2[3] * this.e15, right_dual_g2[3] * this.e25, right_dual_g2[3] * this.e35, -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) + (right_dual_g2 * float4(this.group0.www, this.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (right_dual_g2[3] * this.e4235) + (other.e431 * this.e35), (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (right_dual_g2[3] * this.e4315) + (other.e412 * this.e15), (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (right_dual_g2[3] * this.e4125) + (other.e423 * this.e25), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12)) + float4((right_dual_g1.xyz * this.group1.www), right_dual_g2[3] * this.e3215) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((right_dual_g2.yzx * this.group0.zxy), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)) * float4(other.group2.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group3.zxyy * float4(this.group0.yzx, this.e4315)) + float4(float3(0.0), (other.e4125 * this.e4125) + (other.e3215 * this.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43)) + float4((right_dual_g0 * this.group0.www), other.e1234 * this.e3215) + float4((this.group1.xyz * other.group2.www), other.e4235 * this.e4235) - float4((other.group3.yzx * this.group0.zxy), right_dual_g0[0] * this.e15), 
            /* e415, e425, e435, e321 */
            float4((other.e1234 * this.e15) + (other.e3215 * this.e41), (other.e1234 * this.e25) + (other.e3215 * this.e42), (other.e1234 * this.e35) + (other.e3215 * this.e43), -(other.e4315 * this.e31) - (other.e4125 * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + (other.group3.yzx * this.group2.zxy) - (other.group3.zxy * this.group2.yzx)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yy, other.e12345, (other.e5 * this.scalar) + (other.e12345 * this.e3215)) * float4(this.group3.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkExpansionInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor bulk_expansion(FlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e45 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4(this.scalar, this.scalar, this.scalar, -(other.e15 * this.e41) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e45 * this.e45)) * float4(other.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Flector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234) - (other.e25 * this.e42) - (other.e35 * this.e43) - (other.e45 * this.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(other.group1.www, other.e45)) + float4(float3(0.0), -(other.e4315 * this.e31) - (other.e4125 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz) + (other.group1.yzx * this.group2.zxy) - (other.group1.zxy * this.group2.yzx)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion bulk_expansion(Line other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar, this.scalar, this.scalar, -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)) * float4(other.group1.xyz, 1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.e415 * this.e45) + (other.e125 * this.e42), (other.e425 * this.e45) + (other.e235 * this.e43), (other.e435 * this.e45) + (other.e315 * this.e41), -(other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12)) - float4((other.group1.yzx * this.group0.zxy), other.e415 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(Motor other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g0[3]) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * right_dual_g0.xyz)), right_dual_g0[3] * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(other.e235 * this.scalar, other.e315 * this.scalar, other.e125 * this.scalar, -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) + (float4(right_dual_g0[3]) * this.group2), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g0[3] * this.e4235) + (other.e125 * this.e42), (right_dual_g0[3] * this.e4315) + (other.e235 * this.e43), (right_dual_g0[3] * this.e4125) + (other.e315 * this.e41), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12) - (other.e5 * this.scalar)) + (right_dual_g0 * float4(this.group1.www, this.e3215)) - float4((other.group1.yzx * this.group0.zxy), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkExpansionInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public MultiVector bulk_expansion(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_dual_g0[0] * this.scalar, (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (other.e45 * this.e45) - (other.e23 * this.e23) - (other.e31 * this.e31) - (other.e12 * this.e12)), 
            /* e1, e2, e3, e4 */
            right_dual_g1 * float4(this.scalar), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            (right_dual_g3 * float4(this.scalar)) + (float4(right_dual_g0[0]) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group6.xyz), 
            /* e415, e425, e435, e321 */
            float4((other.e23 * this.scalar) + (other.e3215 * this.e41), (other.e31 * this.scalar) + (other.e3215 * this.e42), (other.e12 * this.scalar) + (other.e3215 * this.e43), -(right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)) + float4((this.group2.xyz * right_dual_g1.www), other.e45 * this.scalar) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.scalar)) + (float3(right_dual_g1[3]) * this.group1.xyz) + (right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g1.yzx * this.group2.zxy) - (right_dual_g1.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g3[2] * this.e42) + (right_dual_g3[3] * this.e23) + (other.e1 * this.scalar) + (other.e415 * this.e45) + (other.e431 * this.e35), (right_dual_g3[0] * this.e43) + (right_dual_g3[3] * this.e31) + (other.e2 * this.scalar) + (other.e425 * this.e45) + (other.e412 * this.e15), (right_dual_g3[1] * this.e41) + (right_dual_g3[3] * this.e12) + (other.e3 * this.scalar) + (other.e435 * this.e45) + (other.e423 * this.e25), -(right_dual_g3[2] * this.e12) - (other.e5 * this.scalar) - (other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35)) + (float4(right_dual_g0[0]) * this.group3) - (right_dual_g3.yzxx * float4(this.group0.zxy, this.e23)) - float4((other.group7.zxy * this.group2.yzx), right_dual_g3[1] * this.e31), 
            /* e1234 */
            (right_dual_g0[0] * this.e1234) - (other.e4 * this.scalar) - (other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Plane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e4315 * this.e43 * -1.0, other.e4125 * this.e41 * -1.0, other.e4235 * this.e42 * -1.0, (other.e4315 * this.e4315) + (other.e4125 * this.e4125) + (other.e3215 * this.e1234)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)), 
            /* e415, e425, e435, e321 */
            float4(other.e3215 * this.e41, other.e3215 * this.e42, other.e3215 * this.e43, -(other.e4315 * this.e31) - (other.e4125 * this.e12)) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkExpansionInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public Sphere bulk_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public AntiScalar bulk_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(Sphere other) {
        float4 right_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0[1] * this.e43 * -1.0, right_dual_g0[2] * this.e41 * -1.0, right_dual_g0[0] * this.e42 * -1.0, (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234)) + (right_dual_g0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_dual_g0.wwwy * float4(this.group1.xyz, this.e4315)), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0[3] * this.e15) + (other.e3215 * this.e41), (right_dual_g0[3] * this.e25) + (other.e3215 * this.e42), (right_dual_g0[3] * this.e35) + (other.e3215 * this.e43), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) + (right_dual_g0.yzx * this.group2.zxy) - (right_dual_g0.zxy * this.group2.yzx)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkExpansionInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_expansion(VersorEven other) {
        float right_dual_g0_w = other.e12345 * -1.0;
        float4 right_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_dual_g0_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_dual_g0_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.scalar)) + (float4(right_dual_g0_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            (right_dual_g2 * float4(this.scalar)) + (float4(right_dual_g0_w) * this.group2) + float4(float3(0.0), -(right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_dual_g1[0] * this.e45) + (right_dual_g1[3] * this.e23) + (right_dual_g2[2] * this.e42) + (other.e431 * this.e35) + (other.e1 * this.scalar), (right_dual_g1[1] * this.e45) + (right_dual_g1[3] * this.e31) + (right_dual_g2[0] * this.e43) + (other.e412 * this.e15) + (other.e2 * this.scalar), (right_dual_g1[2] * this.e45) + (right_dual_g1[3] * this.e12) + (right_dual_g2[1] * this.e41) + (other.e423 * this.e25) + (other.e3 * this.scalar), -(right_dual_g1[2] * this.e35) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12) - (other.e5 * this.scalar)) + (float4(right_dual_g0_w) * this.group3) - (this.group2.yzxy * float4(other.group0.zxy, right_dual_g1[1])) - float4((right_dual_g2.yzx * this.group0.zxy), right_dual_g1[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkExpansionInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.bulk_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven bulk_expansion(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_dual_g0 * float4(this.scalar)) + (right_dual_g3.zxyx * float4(this.group0.yzx, this.e4235)) + (right_dual_g3.wwwy * float4(this.group1.xyz, this.e4315)) + float4(float3(0.0), (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g2_xyz[1] * this.e42) - (right_dual_g2_xyz[2] * this.e43) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)) - (this.group0.zxyx * float4(right_dual_g3.yzx, right_dual_g2_xyz[0])), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3[3] * this.e15) + (other.e3215 * this.e41), (right_dual_g3[3] * this.e25) + (other.e3215 * this.e42), (right_dual_g3[3] * this.e35) + (other.e3215 * this.e43), -(right_dual_g3[1] * this.e31) - (right_dual_g3[2] * this.e12)) + (right_dual_g1 * float4(this.scalar)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((right_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz) + (right_dual_g3.yzx * this.group2.zxy) - (right_dual_g3.zxy * this.group2.yzx)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            right_dual_g3 * float4(this.scalar)
        );
    }
}
