implementing cga3d;
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float4(this.scalar).xyz * other.group2.xyz * float3(-1.0)), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2_xyz * float4(this.scalar).xyz), -(this.e41 * right_anti_dual_g1[0]) - (this.e42 * right_anti_dual_g1[1]) - (this.e43 * right_anti_dual_g1[2]) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (this.scalar * other.e4)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (float3(this.scalar) * other.group3.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) + (other.group0.yzx * this.group2.zxy) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (other.group0.zxy * this.group2.yzx)), this.scalar * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * float3(float2(other.e3215), other.e3215)), other.scalar * this.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector weight_expansion(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (this.group0.yzx * right_anti_dual_g0.zxy) - (this.group0.zxy * right_anti_dual_g0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Flector weight_expansion(AntiFlector other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz) + (this.group0.yzx * right_anti_dual_g0.zxy) - (this.group0.zxy * right_anti_dual_g0.yzx)), this.scalar * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float4(this.scalar).xyz), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual_g1 * float4(this.scalar).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group0.xyz)), this.scalar * other.scalar), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(right_anti_dual_g0) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float4(this.scalar).xyz), -(this.e41 * right_anti_dual_g1[0]) - (this.e42 * right_anti_dual_g1[1]) - (this.e43 * right_anti_dual_g1[2]) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (this.group0.yzx * other.group2.zxy) + (other.group0.yzx * this.group2.zxy) - (this.group0.zxy * other.group2.yzx) - (other.group0.zxy * this.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual_g2_w) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_anti_dual_g2_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g2_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(right_anti_dual_g2_w) * this.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) + (other.group0.yzx * this.group2.zxy) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (other.group0.zxy * this.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2 * float4(this.scalar).xyz), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(DipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (float3(this.scalar) * other.group0.xyz) - (this.group0.yzx * other.group3.zxy)), this.e45 * other.e45 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group3.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((this.e25 * other.e4125) - (this.e35 * other.e4315), (this.e35 * other.e4235) - (this.e15 * other.e4125), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + (float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group2.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345 * -1.0) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(1.0, 1.0, other.e12345, 0.0) * float4((this.group2.xyz * float3(float2(other.e12345 * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e5 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor weight_expansion(FlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * other.e45), 
            /* e235, e315, e125, e12345 */
            float4((float4(this.scalar).xyz * other.group0.xyz * float3(-1.0)), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(Flector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy)), this.e45 * other.e45 * -1.0), 
            /* e415, e425, e435, e321 */
            (this.group1.wwwy * other.group1.xyzy) + float4((float3(other.e3215) * this.group0.xyz), (this.e23 * other.e4235) + (this.e12 * other.e4125) + (this.scalar * other.e45)), 
            /* e235, e315, e125, e5 */
            float4((float3((this.e25 * other.e4125) - (this.e35 * other.e4315), (this.e35 * other.e4235) - (this.e15 * other.e4125), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + (float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group0.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4((float4(this.scalar).xyz * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion weight_expansion(Line other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float4(this.scalar).xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float4(this.scalar).xyz), -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group0.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz * float3(this.e45)) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), this.scalar * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[0] * this.scalar, (right_anti_dual_g0[1] * this.scalar) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g6_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e45) * right_anti_dual_g1.xyz)), this.scalar * other.e45), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz) + (this.group0.yzx * right_anti_dual_g1.zxy) - (this.group0.zxy * right_anti_dual_g1.yzx), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g1[1] * this.e35) - (right_anti_dual_g1[2] * this.e25), (right_anti_dual_g1[2] * this.e15) - (right_anti_dual_g1[0] * this.e35), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + (right_anti_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g5 * float3(this.e45)) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz) + (this.group0.yzx * right_anti_dual_g3.zxy) + (other.group7.yzx * this.group2.zxy) - (this.group0.zxy * right_anti_dual_g3.yzx) - (other.group7.zxy * this.group2.yzx)), this.scalar * other.e5 * -1.0), 
            /* e1234 */
            -(right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion weight_expansion(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.yzx * right_anti_dual_g0.zxy) - (this.group0.zxy * right_anti_dual_g0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e45) * right_anti_dual_g0.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0, 0.0, (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15), 0.0) + float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + float3(((right_anti_dual_g0.yz * this.group2.zx) - (right_anti_dual_g0.zx * this.group2.yz)), 0.0)), 0.0), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public Sphere weight_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz * float3(this.e45))), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx)), this.scalar * other.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(right_anti_dual_g0_xyz, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), right_anti_dual_g0_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group2.xyz)), this.scalar * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (float3(this.scalar) * other.group3.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), this.scalar * other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiCircleRotor {
    public VersorEven weight_expansion(VersorOdd other) {
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_anti_dual_g3[3]) * this.group1.xyz) + (this.group0.yzx * right_anti_dual_g3.zxy) - (float3(this.scalar) * other.group0.xyz) - (this.group0.zxy * right_anti_dual_g3.yzx)), this.scalar * other.scalar), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g3[3]) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e45) * right_anti_dual_g3.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((right_anti_dual_g3[1] * this.e35) - (right_anti_dual_g3[2] * this.e25), (right_anti_dual_g3[2] * this.e15) - (right_anti_dual_g3[0] * this.e35), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + (float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group2.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group3.yz) - (other.group1.yz * this.group3.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(other.e321) * this.group3.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), (other.e1 * this.e1) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e5 * this.e4))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector weight_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0_xyz * float3(this.e4)) - (float3(right_anti_dual_g0_w) * this.group3.xyz)), right_anti_dual_g0_w * this.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g0_xyz.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Motor weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0_xyz * float3(this.e4)) + (float3(other.e321) * this.group3.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g0_xyz.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Plane weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0.yzx * this.group3.zxy) - (float3(this.e4) * other.group1.xyz) - (right_anti_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Flector weight_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((this.e2 * other.e12) - (this.e3 * other.e31), (this.e3 * other.e23) - (this.e1 * other.e12), (this.e1 * other.e31) - (this.e2 * other.e23)) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e4) * other.group1.xyz)), this.e321 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar weight_expansion(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(right_anti_dual_g0) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group2.xyz) + (float3(this.e5) * other.group0.xyz) + (float3(other.e321) * this.group3.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group0.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), right_anti_dual_g1_w * this.e321 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group3.xyz)), right_anti_dual_g2_w * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g2_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2_w) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e1 * other.e31) - (this.e2 * other.e23), 0.0) + float4((float3(((this.group3.yz * other.group1.zx) - (this.group3.zx * other.group1.yz)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (this.e4 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(this.e4) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.yzxw * float4(this.group3.zxy, this.e4)) - (this.group3.yzxw * float4(other.group3.zxy, other.e1234)), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(float3(other.e3215), right_anti_dual_g0[0])) + float4((float3(this.e5) * other.group3.xyz), (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) - (this.e321 * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            (this.group3.yzxy * float4(other.group1.zxy, other.e25)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group3.zx * other.group1.yz)), (this.e415 * other.e4315) - (this.e2 * other.e23)) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), (this.e321 * other.e3215) + (this.e1 * other.e15) - (this.e235 * other.e4235) - (this.e5 * other.e45)) - (this.group2.xyzz * float4(float3(other.e1234), other.e4125)) - (other.group3.yzxy * float4(this.group1.zxy, this.e315))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), other.e5 * this.e4 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * -1.0) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * -1.0) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public Sphere weight_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.e4).xyz * other.group0.xyz * float3(-1.0)), (this.e1 * other.e15) + (this.e2 * other.e25) + (this.e3 * other.e35) - (this.e5 * other.e45)), 
            /* e1234 */
            this.e4 * other.e45
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.zx * other.group1.yz) - (this.group3.yz * other.group1.zx)), (this.e2 * other.e4235) - (this.e1 * other.e4315), this.e4 * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.e3215) * this.group3.xyz)), (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) + (this.e4 * other.e45)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + float3((this.group1.yz * other.group1.zx), this.e415 * other.e4315)), (this.e321 * other.e3215) + (this.e1 * other.e15) - (this.e5 * other.e45)) - (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) - (other.group1.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor weight_expansion(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float4(this.e4).xyz), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), this.e4 * other.e5 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group2.xyz) + (right_anti_dual_g1_xyz.zxy * this.group3.yzx) - (right_anti_dual_g1_xyz.yzx * this.group3.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (this.e4 * other.e5) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[0]) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * float4(this.group3.xyz, this.e4)) - (right_anti_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g1[2] * this.e2) - (right_anti_dual_g1[1] * this.e3), (right_anti_dual_g1[0] * this.e3) - (right_anti_dual_g1[2] * this.e1), (right_anti_dual_g1[1] * this.e1) - (right_anti_dual_g1[0] * this.e2)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[0]) * this.group1.xyz) + (float3(this.e4) * other.group8.xyz) + (float3(this.e5) * other.group7.xyz) - (float3(right_anti_dual_g3_w) * this.group3.xyz)), right_anti_dual_g0[0] * this.e321), 
            /* e423, e431, e412 */
            (right_anti_dual_g5 * float3(this.e4)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group2.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e5) * other.group4.xyz) + (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g6_xyz.yzx * this.group3.zxy) + float3(((right_anti_dual_g1.yz * this.group1.zx) - (right_anti_dual_g1.zx * this.group1.yz)), (right_anti_dual_g1[0] * this.e425) - (right_anti_dual_g1[1] * this.e415)) - (float3(right_anti_dual_g1[3]) * this.group2.xyz) - (float3(this.e4) * other.group3.xyz)), (right_anti_dual_g1[0] * this.e235) + (right_anti_dual_g1[1] * this.e315) + (right_anti_dual_g1[2] * this.e125) + (this.e321 * other.e3215)) - (this.group3.yzxw * float4(right_anti_dual_g6_xyz.zxy, other.e45)), 
            /* e1234 */
            (this.e4 * other.e45) - (this.e423 * right_anti_dual_g1[0]) - (this.e431 * right_anti_dual_g1[1]) - (this.e412 * right_anti_dual_g1[2]) - (other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (right_anti_dual_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.zx * other.group0.yz) - (this.group3.yz * other.group0.zx)), (this.e2 * other.e4235) - (this.e1 * other.e4315), this.e4 * other.e3215), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group3.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), (this.e415 * other.e4315) - (this.e425 * other.e4235))), this.e321 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar weight_expansion(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g0_xyz.zxy * this.group3.yzx), this.e4 * other.e3215) - (this.group3.zxyw * float4(right_anti_dual_g0_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group3.xyz) - (right_anti_dual_g0_xyz * float3(this.e5))), this.e321 * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), (right_anti_dual_g0_xyz[0] * this.e235) + (this.e321 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group3.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g3_xyz[0])) + float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group0.xyz) - (right_anti_dual_g0_xyz.zxy * this.group3.yzx)), (right_anti_dual_g3_xyz[1] * this.e2) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e4 * other.e5)), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group1.xyz) - (float3(right_anti_dual_g1_w) * this.group3.xyz)), right_anti_dual_g0_w * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g3_xyz.zxy * this.group3.yzx), this.e4 * other.e3215) - (this.group3.zxyw * float4(right_anti_dual_g3_xyz.yzx, other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group3.xyz) - (right_anti_dual_g3_xyz * float3(this.e5))), (this.e4 * other.e45) - (right_anti_dual_g3_xyz[0] * this.e423) - (right_anti_dual_g3_xyz[1] * this.e431) - (right_anti_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3_xyz.yzx * this.group1.zxy) + float3(((this.group3.yz * other.group1.zx) - (this.group3.zx * other.group1.yz)), (this.e1 * other.e31) - (this.e2 * other.e23)) - (float3(this.e4) * other.group2.xyz) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), (right_anti_dual_g3_xyz[0] * this.e235) + (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215))
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
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
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
public DualNum operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public DualNum weight_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public FlatPoint weight_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Flector weight_expansion(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Line weight_expansion(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum weight_expansion(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Dipole weight_expansion(Circle other) {
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
public VersorOdd operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, right_anti_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group2.xyz * float3(float2(this.scalar), 1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g2_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Circle weight_expansion(Dipole other) {
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
public VersorEven operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven weight_expansion(DipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(float2(this.scalar * -1.0), this.scalar) * float3(1.0, 1.0, -1.0)), this.e3215 * other.e1234), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiDualNum weight_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(this.e3215 * other.e12345) - (this.scalar * other.e5), this.scalar * other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiFlector weight_expansion(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiLine weight_expansion(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiMotor weight_expansion(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * float3(float2(this.scalar), this.scalar)), (this.e3215 * right_anti_dual_g0[3]) - (this.scalar * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[0] * this.scalar, (right_anti_dual_g0[1] * this.scalar) + (this.e3215 * right_anti_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g1 * float4(this.scalar), 
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
            float4((other.group1.xyz * float3(float2(this.scalar), this.scalar)), (right_anti_dual_g0[0] * this.e3215) - (this.scalar * other.e5)), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiPlane weight_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public Sphere weight_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven weight_expansion(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * right_anti_dual_g0[3]), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public VersorOdd weight_expansion(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((other.group3.xyz * float3(float2(this.scalar), this.scalar)), (this.e3215 * right_anti_dual_g0[3]) - (this.scalar * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiDualNum {
    public VersorEven weight_expansion(VersorOdd other) {
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((other.group0.xyz * float3(float2(this.scalar * -1.0), this.scalar) * float3(1.0, 1.0, -1.0)), (this.e3215 * right_anti_dual_g3[3]) + (this.scalar * other.scalar)), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e3215 * this.e321, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(AntiMotor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar weight_expansion(Circle other) {
        return AntiScalar(
            /* e12345 */
            (this.e321 * other.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g2_w * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g2_w) * this.group0.xyz), (this.e321 * other.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(other.e1234).xyz * this.group0.xyz * float3(-1.0)), (this.e321 * other.e3215) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125)), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(Flector other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e321 * other.e3215) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_expansion(Motor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e321 * other.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
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
            float4(float3(0.0), right_anti_dual_g0[0] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e1234 * -1.0) * this.group0.xyz), (right_anti_dual_g1_xyz[0] * this.e235) + (right_anti_dual_g1_xyz[1] * this.e315) + (right_anti_dual_g1_xyz[2] * this.e125) + (this.e321 * other.e3215)), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum weight_expansion(Plane other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e321 * other.e3215) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e1234 * -1.0) * this.group0.xyz), (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215)), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g0_w * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlatPoint {
    public Sphere weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(other.e1234).xyz * this.group0.xyz * float3(-1.0)), (right_anti_dual_g3_xyz[0] * this.e235) + (right_anti_dual_g3_xyz[1] * this.e315) + (right_anti_dual_g3_xyz[2] * this.e125) + (this.e321 * other.e3215)), 
            /* e1234 */
            this.e321 * other.e1234 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiFlector> operator /(AntiFlector lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Sphere weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((other.e12 * this.e2) - (other.e31 * this.e3), (other.e23 * this.e3) - (other.e12 * this.e1), (other.e31 * this.e1) - (other.e23 * this.e2)) - (right_anti_dual_g0 * float3(this.e5))), other.e45 * this.e5 * -1.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group1.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            (this.group1.yzxx * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[0])) + float4(((float3(this.e5) * other.group1.xyz) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy)), (right_anti_dual_g3_xyz[1] * this.e2) - (right_anti_dual_g1_w * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Flector weight_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215, 0.0) * float4((this.group1.xyz * float3(float2(other.e3215), 1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Motor weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e321) * float4(this.group1.xyz, this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group1.yzx) - (right_anti_dual_g0_xyz.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Motor weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0_w * -1.0) * this.group1.xyz), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_w * this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group1.yzx) - (right_anti_dual_g0_xyz.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Plane weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0.yzx * this.group1.zxy) - (right_anti_dual_g0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Flector weight_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float4(other.e3215).xyz * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), (this.e1 * other.e31) - (this.e2 * other.e23), this.e321 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar weight_expansion(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public CircleRotor weight_expansion(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e321) * this.group1.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), right_anti_dual_g1_w * this.e321 * -1.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e5) * other.group0.xyz), (right_anti_dual_g2_w * this.e321) - (right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0])), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group0.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy)), right_anti_dual_g2_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public Sphere weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((this.e2 * other.e12) - (this.e3 * other.e31), (this.e3 * other.e23) - (this.e1 * other.e12), (this.e1 * other.e31) - (this.e2 * other.e23)) - (right_anti_dual_g0 * float3(this.e5))), this.e5 * other.e45 * -1.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group1.zx * other.group3.yz) - (this.group1.yz * other.group3.zx)), (this.e2 * other.e4235) - (this.e1 * other.e4315), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(float3(other.e3215), right_anti_dual_g0[0])) + float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group1.yzxx * float4(other.group1.zxy, other.e15)) + float4((float3(-(this.group1.zx * other.group1.yz), this.e2 * other.e23 * -1.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(other.e1234) * this.group0.xyz)), -(this.e235 * other.e4235) - (this.e5 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_expansion(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e1 * other.e15) + (this.e2 * other.e25) + (this.e3 * other.e35) - (this.e5 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_expansion(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (this.e2 * other.e4235) - (this.e1 * other.e4315), 0.0) + float4(((this.group1.zx * other.group1.yz) - (this.group1.yz * other.group1.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e3215 */
            (this.group1.xyzx * float4(float3(other.e3215), other.e15)) + (this.group1.wwwy * float4(other.group1.xyz, other.e25)) + float4(float3(0.0), (this.e3 * other.e35) - (this.e235 * other.e4235) - (this.e5 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint weight_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_expansion(Motor other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(right_anti_dual_g0_w) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group1.yzx) - (right_anti_dual_g1_xyz.yzx * this.group1.zxy)), right_anti_dual_g0_w * this.e321), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g0_w) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, right_anti_dual_g0[0], 0.0) * float4((this.group1.xyz * float3(float2(right_anti_dual_g0[0]), 1.0)), 0.0), 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) - (right_anti_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz.zxy * this.group1.yzx) - (right_anti_dual_g1_xyz.yzx * this.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e5) * other.group7.xyz), (right_anti_dual_g0[0] * this.e321) - (right_anti_dual_g5[1] * this.e2) - (right_anti_dual_g5[2] * this.e3)) - (this.group1.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g5[0])), 
            /* e423, e431, e412 */
            (other.group7.yzx * this.group1.zxy) - (other.group7.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group8.zxy * this.group1.yzx) - (other.group8.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e5) * other.group4.xyz) + (right_anti_dual_g6_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group0.xyz)), right_anti_dual_g1_xyz[0] * this.e235) - (this.group1.yzxw * float4(right_anti_dual_g6_xyz.zxy, other.e45)), 
            /* e1234 */
            -(other.e41 * this.e1) - (other.e42 * this.e2) - (other.e43 * this.e3) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_expansion(Plane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (this.e2 * other.e4235) - (this.e1 * other.e4315), 0.0) + float4(((this.group1.zx * other.group0.yz) - (this.group1.yz * other.group0.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group1.xyz)), this.e235 * other.e4235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public AntiScalar weight_expansion(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group1.yzx) - (right_anti_dual_g0_xyz.yzx * this.group1.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) - (right_anti_dual_g0_xyz * float3(this.e5))), this.e321 * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e1234 * -1.0) * this.group0.xyz), (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315) + (right_anti_dual_g0_xyz[2] * this.e125) + (this.e321 * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public VersorEven weight_expansion(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0.yz * this.group1.zx) - (right_anti_dual_g0.zx * this.group1.yz)), (right_anti_dual_g0[0] * this.e2) - (right_anti_dual_g0[1] * this.e1), (this.e1 * other.e1) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e2) - (right_anti_dual_g1_xyz[2] * this.e3)) - (this.group1.xyzx * float4(float3(right_anti_dual_g1_w), right_anti_dual_g1_xyz[0])), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0[3]) * this.group0.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy)), right_anti_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float4(right_anti_dual_g0[3]).xyz * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiFlector {
    public DipoleInversion weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group1.yzx) - (right_anti_dual_g3_xyz.yzx * this.group1.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) - (right_anti_dual_g3_xyz * float3(this.e5))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e5) * other.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (this.group1.zx * other.group1.yz)), (this.e1 * other.e31) - (this.e2 * other.e23)) - (float3(other.e1234) * this.group0.xyz)), right_anti_dual_g3_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiLine> operator /(AntiLine lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Sphere weight_expansion(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (float3(other.e321) * this.group0.xyz) - (other.group0.zxy * this.group1.yzx)), 0.0), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(1.0, 1.0, other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Plane weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float4(other.e321).xyz * float3(-1.0)), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Plane weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((this.group0.xyz * float4(other.e321).xyz * float3(-1.0)), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar weight_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)), 
            /* e235, e315, e125, e5 */
            float4((this.group0.xyz * float4(other.e3215).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightExpansionInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiLine weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Sphere weight_expansion(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e1234 */
            -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual_g2_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g2_w) * this.group1.xyz), -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiScalar weight_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor weight_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.group1.xyz * float4(other.e1234).xyz), (this.e23 * other.e4235) + (this.e31 * other.e4315) + (this.e12 * other.e4125)), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group3.zxy) - (this.group1.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightExpansionInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiLine weight_expansion(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(Flector other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group1.zxy) - (this.group1.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiDualNum weight_expansion(Line other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(this.e23 * other.e235) - (this.e31 * other.e315) - (this.e12 * other.e125) - (this.e15 * other.e415) - (this.e25 * other.e425) - (this.e35 * other.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiMotor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual_g0_w) * this.group1.xyz), -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, right_anti_dual_g0[0], 0.0) * float4((this.group1.xyz * float3(float2(right_anti_dual_g0[0]), 1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e1234) * this.group1.xyz), -(right_anti_dual_g1_xyz[0] * this.e23) - (right_anti_dual_g1_xyz[1] * this.e31) - (right_anti_dual_g1_xyz[2] * this.e12)), 
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g1_xyz.yzx * this.group1.zxy) - (right_anti_dual_g1_xyz.zxy * this.group1.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group1.zxy * other.group7.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group7.zxy)), 0.0), 
            /* e1234 */
            -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public AntiFlatPoint weight_expansion(Plane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public Circle weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e1234) * this.group1.xyz), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public DipoleInversion weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0_w) * this.group1.xyz), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (float3(other.e321) * this.group0.xyz) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiLine {
    public CircleRotor weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e1234) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.group1.xyz * float4(other.e1234).xyz), -(right_anti_dual_g3_xyz[0] * this.e23) - (right_anti_dual_g3_xyz[1] * this.e31) - (right_anti_dual_g3_xyz[2] * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3_xyz.yzx * this.group1.zxy) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiMotor> operator /(AntiMotor lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float4(this.scalar).xyz * other.group2.xyz * float3(-1.0)), (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((float4(this.scalar).xyz * other.group2.xyz), -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group0.xyz) + (float3(this.scalar) * other.group3.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), other.e5 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Motor weight_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * this.scalar), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Flector weight_expansion(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(right_anti_dual_g0[3]).xyz * this.group0.xyz), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Flector weight_expansion(AntiFlector other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e5 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Motor weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float4(this.scalar).xyz), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)), 
            /* e235, e315, e125, e5 */
            float4((other.group1.xyz * float4(this.scalar).xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(this.scalar).xyz * other.group0.xyz * float3(-1.0)), (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group1.xyz)), other.e3215 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_anti_dual_g0) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(right_anti_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public DipoleInversion weight_expansion(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float4(this.scalar).xyz), -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group0.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd weight_expansion(CircleRotor other) {
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, right_anti_dual_g2_w), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual_g2_w) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), right_anti_dual_g1_w * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2_w) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1_w) * this.group0.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), right_anti_dual_g2_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public CircleRotor weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((other.group2.xyz * float4(this.scalar).xyz * float3(-1.0)), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven weight_expansion(DipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) - (float3(this.scalar) * other.group0.xyz)), this.e3215 * other.e1234), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group1.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((this.e25 * other.e4125) - (this.e35 * other.e4315), (this.e35 * other.e4235) - (this.e15 * other.e4125), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + (float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group2.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_expansion(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * this.scalar) - (other.e12345 * this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_expansion(Flector other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3((right_anti_dual_g1[1] * this.e35) - (right_anti_dual_g1[2] * this.e25), (right_anti_dual_g1[2] * this.e15) - (right_anti_dual_g1[0] * this.e35), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + (float3(right_anti_dual_g1[3]) * this.group0.xyz) - (float3(this.scalar) * other.group0.xyz)), this.scalar * other.e45), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g1 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_expansion(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((other.group0.xyz * float4(this.scalar).xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * float4(this.scalar).xyz), -(other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35) - (other.e235 * this.e23) - (other.e315 * this.e31) - (other.e125 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), right_anti_dual_g0_w * this.scalar), 
            /* e15, e25, e35, e3215 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[0] * this.scalar, (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[3] * this.e3215) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(right_anti_dual_g0[0]) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz)), right_anti_dual_g3_w * this.scalar), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group7.xyz, 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group6.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g6_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz)), this.scalar * other.e45), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group0.xyz), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g1[1] * this.e35) - (right_anti_dual_g1[2] * this.e25), (right_anti_dual_g1[2] * this.e15) - (right_anti_dual_g1[0] * this.e35), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + (float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g3_w) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz) + (other.group7.yzx * this.group1.zxy) - (other.group7.zxy * this.group1.yzx)), right_anti_dual_g0[0] * this.e3215), 
            /* e1234 */
            -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (this.scalar * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_expansion(Plane other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(0.0, 0.0, (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15), 0.0) + float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + float3(((right_anti_dual_g0.yz * this.group1.zx) - (right_anti_dual_g0.zx * this.group1.yz)), 0.0)), 0.0), 
            /* e1, e2, e3, e5 */
            right_anti_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public Sphere weight_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.scalar * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven weight_expansion(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0[3]) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float4(right_anti_dual_g0[3]).xyz * this.group1.xyz), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)), 
            /* e235, e315, e125, e5 */
            float4((float3((right_anti_dual_g0[1] * this.e35) - (right_anti_dual_g0[2] * this.e25), (right_anti_dual_g0[2] * this.e15) - (right_anti_dual_g0[0] * this.e35), (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15)) + (float3(other.e3215) * this.group0.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public VersorOdd weight_expansion(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), right_anti_dual_g1_w * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz)), this.scalar * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g0[1] * this.e35) - (right_anti_dual_g0[2] * this.e25), (right_anti_dual_g0[2] * this.e15) - (right_anti_dual_g0[0] * this.e35), (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15)) + (float3(right_anti_dual_g1_w) * this.group0.xyz) + (float3(this.scalar) * other.group3.xyz)), right_anti_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiMotor {
    public VersorEven weight_expansion(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(this.scalar)) + (float4(right_anti_dual_g3[3]) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g3[3]) * this.group1.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e45), 
            /* e235, e315, e125, e5 */
            float4((float3((right_anti_dual_g3[1] * this.e35) - (right_anti_dual_g3[2] * this.e25), (right_anti_dual_g3[2] * this.e15) - (right_anti_dual_g3[0] * this.e35), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + (float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group2.xyz)), this.scalar * other.e3215), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiPlane> operator /(AntiPlane lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Sphere weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((other.e12 * this.e2) - (other.e31 * this.e3), (other.e23 * this.e3) - (other.e12 * this.e1), (other.e31 * this.e1) - (other.e23 * this.e2)) - (right_anti_dual_g0 * float3(this.e5))), other.e45 * this.e5 * -1.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e321) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            (this.group0.yzxx * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g3_xyz[0])) + float4(((float3(this.e5) * other.group1.xyz) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g3_xyz[1] * this.e2)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public FlatPoint weight_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, other.e3215, 0.0) * float4((this.group0.xyz * float3(float2(other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Line weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            float3(other.e321) * this.group0.xyz, 
            /* e235, e315, e125 */
            (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Motor weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(other.e321).xyz * this.group0.xyz), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Plane weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Flector weight_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float4(other.e3215).xyz * this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar weight_expansion(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightExpansionInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_expansion(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Circle weight_expansion(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e321) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.e321) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * other.group1.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Sphere weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3((this.e2 * other.e12) - (this.e3 * other.e31), (this.e3 * other.e23) - (this.e1 * other.e12), (this.e1 * other.e31) - (this.e2 * other.e23)) - (right_anti_dual_g0 * float3(this.e5))), this.e5 * other.e45 * -1.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group0.zx * other.group3.yz) - (this.group0.yz * other.group3.zx)), (this.e2 * other.e4235) - (this.e1 * other.e4315), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e5) * other.group3.xyz) + (float3(other.e3215) * this.group0.xyz)), right_anti_dual_g0[0] * this.e1), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((this.e2 * other.e12) - (this.e3 * other.e31), (this.e3 * other.e23) - (this.e1 * other.e12), (this.e1 * other.e31) - (this.e2 * other.e23)) - (right_anti_dual_g0 * float3(this.e5))), this.e3 * other.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightExpansionInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_expansion(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e1 * other.e15) + (this.e2 * other.e25) + (this.e3 * other.e35) - (this.e5 * other.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiMotor weight_expansion(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(0.0, 0.0, (this.e2 * other.e4235) - (this.e1 * other.e4315), 0.0) + float4(((this.group0.zx * other.group1.yz) - (this.group0.yz * other.group1.zx)), 0.0, 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.e3215) * this.group0.xyz)), this.e5 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlatPoint weight_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiFlector weight_expansion(Motor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((float3(this.e5) * other.group0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, right_anti_dual_g0[0], 0.0) * float4((this.group0.xyz * float3(float2(right_anti_dual_g0[0]), 1.0)), 0.0), 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e5))), this.e5 * other.e1234 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group7.xyz) + (float3(other.e321) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group6.xyz) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g6_xyz.yzx * this.group0.zxy) - (right_anti_dual_g7 * float3(this.e5)) - (right_anti_dual_g6_xyz.zxy * this.group0.yzx)), this.e5 * other.e45 * -1.0), 
            /* e1234 */
            (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiLine weight_expansion(Plane other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3((this.e3 * other.e4315) - (this.e2 * other.e4125), (this.e1 * other.e4125) - (this.e3 * other.e4235), (this.e2 * other.e4235) - (this.e1 * other.e4315)), 
            /* e15, e25, e35 */
            (float3(this.e5) * other.group0.xyz) + (float3(other.e3215) * this.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public AntiScalar weight_expansion(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public Dipole weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public VersorEven weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), this.e1 * other.e1), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (float3(other.e321) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiPlane {
    public DipoleInversion weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy)), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e5))), this.e1 * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((this.e2 * other.e12) - (this.e3 * other.e31), (this.e3 * other.e23) - (this.e1 * other.e12), (this.e1 * other.e31) - (this.e2 * other.e23)) + (float3(this.e5) * other.group0.xyz)), this.e5 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<AntiScalar> operator /(AntiScalar lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(Motor other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_expansion(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Circle> operator /(Circle lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Plane weight_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public AntiScalar weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Plane weight_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Circle weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public AntiScalar weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public CircleRotor weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g2_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g2_w) * this.group2.xyz), (this.e321 * other.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Sphere weight_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Circle weight_expansion(DualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125 */
            float3(other.e12345 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Sphere weight_expansion(Flector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (this.group1.zx * other.group1.yz)), 0.0)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public AntiScalar weight_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public CircleRotor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
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
            float4(right_anti_dual_g0[0]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g1_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g1_xyz.zxy * this.group1.yzx)), (right_anti_dual_g1_xyz[0] * this.e235) + (right_anti_dual_g1_xyz[1] * this.e315)), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Sphere weight_expansion(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Sphere weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315)), 
            /* e1234 */
            -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public CircleRotor weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Circle {
    public Sphere weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), (right_anti_dual_g3_xyz[0] * this.e235) + (right_anti_dual_g3_xyz[1] * this.e315)), 
            /* e1234 */
            -(right_anti_dual_g3_xyz[0] * this.e423) - (right_anti_dual_g3_xyz[1] * this.e431) - (right_anti_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<CircleRotor> operator /(CircleRotor lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Plane weight_expansion(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Plane weight_expansion(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual_g0) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g2_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g2_w) * this.group2.xyz), (right_anti_dual_g2_w * this.e12345) + (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Sphere weight_expansion(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group3.zx) - (this.group1.zx * other.group3.yz)), 0.0) - (float3(other.e1234) * this.group2.xyz)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_expansion(DualNum other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.e12345 * -1.0) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Sphere weight_expansion(Flector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (this.group1.zx * other.group1.yz)), 0.0)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public AntiScalar weight_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (right_anti_dual_g0_w * this.e12345) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e12345) + (this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
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
            float4(right_anti_dual_g0[0]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g1_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g1_xyz.zxy * this.group1.yzx)), (right_anti_dual_g1_xyz[0] * this.e235) + (this.e321 * other.e3215)), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Sphere weight_expansion(Plane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((this.group1.yz * other.group0.zx) - (this.group1.zx * other.group0.yz)), 0.0)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Sphere weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), (right_anti_dual_g0_xyz[0] * this.e235) + (this.e321 * other.e3215)), 
            /* e1234 */
            -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (right_anti_dual_g0_w * this.e12345) + (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension CircleRotor {
    public Sphere weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), (right_anti_dual_g3_xyz[0] * this.e235) + (this.e321 * other.e3215)), 
            /* e1234 */
            -(right_anti_dual_g3_xyz[0] * this.e423) - (right_anti_dual_g3_xyz[1] * this.e431) - (right_anti_dual_g3_xyz[2] * this.e412) - (this.e321 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Dipole> operator /(Dipole lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Sphere weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (other.group0.yzx * this.group2.zxy) + (this.group0.yzx * right_anti_dual_g2.zxy) - (float3(other.e321) * this.group1.xyz) - (other.group0.zxy * this.group2.yzx) - (this.group0.zxy * right_anti_dual_g2.yzx)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Line weight_expansion(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e3215) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Plane weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Plane weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public AntiScalar weight_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * float4(other.e3215).xyz), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)), 
            /* e235, e315, e125, e5 */
            float4((float4(other.e3215).xyz * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Dipole weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e15, e25, e35 */
            float3(right_anti_dual_g0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Sphere weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (other.group0.yzx * this.group2.zxy) + (other.group2.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (other.group0.zxy * this.group2.yzx) - (other.group2.yzx * this.group0.zxy)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g2_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g2_w) * this.group2.xyz), -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) + (other.group0.yzx * this.group2.zxy) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (other.group0.zxy * this.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public AntiScalar weight_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public CircleRotor weight_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group3.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group3.zxy) - (this.group2.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Dipole weight_expansion(DualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35 */
            float3(other.e12345 * -1.0) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public CircleRotor weight_expansion(Flector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group1.xyz) + (float3(other.e3215) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group1.zxy) - (this.group2.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Sphere weight_expansion(Line other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group0.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), -(right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz * float3(this.e45)) + (this.group0.yzx * right_anti_dual_g1.zxy) - (this.group0.zxy * right_anti_dual_g1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[0]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g1_xyz.yzx * this.group2.zxy) - (right_anti_dual_g1_xyz.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g5 * float3(this.e45)) + (this.group0.yzx * other.group8.zxy) + (this.group2.zxy * other.group7.yzx) - (float3(other.e321) * this.group1.xyz) - (this.group0.zxy * other.group8.yzx) - (this.group2.yzx * other.group7.zxy)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Circle weight_expansion(Plane other) {
        return Circle(
            /* e423, e431, e412 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group0.xyz) + (float3(other.e3215) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public Circle weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz * float3(this.e45))), 0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public DipoleInversion weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) + (this.group0.yzx * right_anti_dual_g2.zxy) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx) - (this.group0.zxy * right_anti_dual_g2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Dipole {
    public CircleRotor weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e45))), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g3_xyz.yzx * this.group2.zxy) - (right_anti_dual_g3_xyz.zxy * this.group2.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (other.group0.yzx * this.group2.zxy) + (this.group0.yzx * right_anti_dual_g2.zxy) - (float3(other.e321) * this.group1.xyz) - (other.group0.zxy * this.group2.yzx) - (this.group0.zxy * right_anti_dual_g2.yzx)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Motor weight_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(1.0, 1.0, other.e3215, 0.0) * float4((this.group1.xyz * float3(float2(other.e3215), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Plane weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Plane weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * float4(other.e3215).xyz), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e3215 * this.e1234)), 
            /* e235, e315, e125, e5 */
            float4((float4(other.e3215).xyz * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (other.group0.yzx * this.group2.zxy) + (other.group2.zxy * this.group0.yzx) - (float3(other.e321) * this.group1.xyz) - (other.group0.zxy * this.group2.yzx) - (other.group2.yzx * this.group0.zxy)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g2_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g2_w) * this.group2.xyz), (right_anti_dual_g2_w * this.e1234) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (float3(right_anti_dual_g2_w) * this.group3.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) + (other.group0.yzx * this.group2.zxy) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (other.group0.zxy * this.group2.yzx)), right_anti_dual_g2_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e45) * other.group3.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group1.xyz) + float3(((other.group3.zx * this.group2.yz) - (other.group3.yz * this.group2.zx)), (other.e4315 * this.e15) - (other.e4235 * this.e25))), (other.e41 * this.e15) + (other.e42 * this.e25) + (this.e43 * other.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e1234 * this.e3215) - (other.e45 * this.e45) - (other.e4235 * this.e4235))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(DualNum other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345 * -1.0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Flector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group1.xyz) + (float3(other.e3215) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((float3(other.e3215) * this.group1.xyz) + float3(((this.group2.yz * other.group1.zx) - (this.group2.zx * other.group1.yz)), (this.e15 * other.e4315) - (this.e25 * other.e4235))), (this.e41 * other.e15) + (this.e42 * other.e25) - (this.e45 * other.e45) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public Sphere weight_expansion(Line other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group0.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0), 
            /* e1234 */
            -(this.e41 * other.e415) - (this.e42 * other.e425) - (this.e43 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (right_anti_dual_g0_w * this.e1234) - (right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz * float3(this.e45)) + (float3(right_anti_dual_g0_w) * this.group3.xyz) + (this.group0.yzx * right_anti_dual_g1.zxy) - (this.group0.zxy * right_anti_dual_g1.yzx)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[0]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[0]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) + (right_anti_dual_g1_xyz.yzx * this.group2.zxy) - (right_anti_dual_g1_xyz.zxy * this.group2.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g5 * float3(this.e45)) + (float3(right_anti_dual_g0[0]) * this.group3.xyz) + (this.group0.yzx * other.group8.zxy) + (other.group7.yzx * this.group2.zxy) - (float3(other.e321) * this.group1.xyz) - (this.group0.zxy * other.group8.yzx) - (other.group7.zxy * this.group2.yzx)), right_anti_dual_g0[0] * this.e3215), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1234) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Plane other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group0.xyz) + (float3(other.e3215) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(0.0, 0.0, (this.e15 * other.e4315) - (this.e25 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group1.xyz) + float3(((this.group2.yz * other.group0.zx) - (this.group2.zx * other.group0.yz)), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz * float3(this.e45))), 0.0), 
            /* e235, e315, e125, e12345 */
            (float4(other.e3215) * float4(this.group1.xyz, this.e1234)) + float4(((right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx)), right_anti_dual_g0_xyz[0] * this.e4235)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(right_anti_dual_g0_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0_w) * this.group2.xyz), (right_anti_dual_g0_w * this.e1234) - (right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (float3(right_anti_dual_g0_w) * this.group3.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) + (this.group0.yzx * right_anti_dual_g2.zxy) - (float3(other.e321) * this.group1.xyz) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx) - (this.group0.zxy * right_anti_dual_g2.yzx)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DipoleInversion {
    public CircleRotor weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e1234) * this.group1.xyz) + (right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e45))), 0.0), 
            /* e235, e315, e125, e12345 */
            (this.group1.xyzx * float4(float3(other.e3215), other.e23)) + float4(((right_anti_dual_g3_xyz.yzx * this.group2.zxy) - (right_anti_dual_g3_xyz.zxy * this.group2.yzx)), (right_anti_dual_g3_xyz[0] * this.e4235) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<DualNum> operator /(DualNum lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Plane weight_expansion(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Motor weight_expansion(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(other.group0.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e235, e315, e125, e5 */
            float4(1.0, 1.0, this.e5, 0.0) * float4((other.group1.xyz * float3(float2(this.e5), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightExpansionInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public DualNum weight_expansion(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Line weight_expansion(Circle other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Motor weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(float2(this.e5), this.e5)), right_anti_dual_g2_w * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, right_anti_dual_g2_w)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Plane weight_expansion(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Flector weight_expansion(DipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightExpansionInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public DualNum weight_expansion(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345 * -1.0) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e5 * other.e45 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Flector weight_expansion(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, this.e5, 0.0) * float4((other.group1.xyz * float3(float2(this.e5), 1.0)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e5 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public AntiFlatPoint weight_expansion(Line other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(1.0, 1.0, this.e5, 0.0) * float4((other.group0.xyz * float3(float2(this.e5), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Motor weight_expansion(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.e12345 * right_anti_dual_g0[3]), 
            /* e235, e315, e125, e5 */
            right_anti_dual_g0 * float4(this.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e12345) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group9.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(1.0, 1.0, this.e5, 0.0) * float4((other.group7.xyz * float3(float2(this.e5), 1.0)), 0.0), 
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
public FlatPoint operator /(WeightExpansionInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public FlatPoint weight_expansion(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, this.e5, 0.0) * float4((other.group0.xyz * float3(float2(this.e5), 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public AntiScalar weight_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public FlatPoint weight_expansion(Sphere other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group0.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Motor weight_expansion(VersorEven other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float3(float2(this.e5), this.e5)), (right_anti_dual_g0_w * this.e12345) - (this.e5 * other.e4)), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, right_anti_dual_g0_w)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension DualNum {
    public Flector weight_expansion(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5) * float4(other.group0.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<FlatPoint> operator /(FlatPoint lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Plane weight_expansion(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint weight_expansion(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Plane weight_expansion(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Flector weight_expansion(CircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar weight_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Motor weight_expansion(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e45) * other.group3.xyz)), other.e45 * this.e45 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (other.e4315 * this.e15) - (other.e4235 * this.e25), 0.0) + float4(((other.group3.zx * this.group0.yz) - (other.group3.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public FlatPoint weight_expansion(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Motor weight_expansion(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45) * float4(other.group1.xyz, other.e45) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (this.e15 * other.e4315) - (this.e25 * other.e4235), 0.0) + float4(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Plane weight_expansion(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float4(this.e45).xyz), -(other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Flector weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual_g0_xyz * float4(this.e45).xyz), -(right_anti_dual_g0_xyz[0] * this.e15) - (right_anti_dual_g0_xyz[1] * this.e25) - (right_anti_dual_g0_xyz[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public MultiVector weight_expansion(MultiVector other) {
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group6.xyz) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Line weight_expansion(Plane other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e45) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3((this.e25 * other.e4125) - (this.e35 * other.e4315), (this.e35 * other.e4235) - (this.e15 * other.e4125), (this.e15 * other.e4315) - (this.e25 * other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Line weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Line(
            /* e415, e425, e435 */
            (float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e45)), 
            /* e235, e315, e125 */
            (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Flector weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension FlatPoint {
    public Motor weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e45))), this.e45 * other.e45 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Flector> operator /(Flector lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public AntiScalar weight_expansion(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Plane weight_expansion(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Flector weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Plane weight_expansion(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Flector weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g2_w) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g2_w) * this.group1.xyz) + (float3(this.e45) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), right_anti_dual_g2_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public AntiScalar weight_expansion(Dipole other) {
        return AntiScalar(
            /* e12345 */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Motor weight_expansion(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) + (float3(this.e45) * other.group3.xyz)), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e1234 * this.e3215) - (other.e45 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (other.e4315 * this.e15) - (other.e4235 * this.e25), 0.0) + float4(((other.group3.zx * this.group0.yz) - (other.group3.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Flector weight_expansion(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public AntiScalar weight_expansion(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Motor weight_expansion(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(this.e45).xyz * other.group1.xyz), -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (other.e4315 * this.e15) - (other.e4235 * this.e25), 0.0) + float4(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Plane weight_expansion(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float4(this.e45).xyz), -(other.e415 * this.e15) - (other.e425 * this.e25) - (other.e435 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Flector weight_expansion(Motor other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0_w) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0_w) * this.group1.xyz) + (float3(this.e45) * other.group0.xyz)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (this.e3215 * other.e1234) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[0]) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e45))), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0[0]) * this.group1.xyz) + (float3(this.e45) * other.group6.xyz) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx)), right_anti_dual_g0[0] * this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Motor weight_expansion(Plane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float4(this.e45).xyz * other.group0.xyz), -(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(0.0, 0.0, (this.e15 * other.e4315) - (this.e25 * other.e4235), 0.0) + float4(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Motor weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e45))), right_anti_dual_g0_xyz[0] * this.e4235), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Flector weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0_w) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0_w) * this.group1.xyz) + (float3(this.e45) * other.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Flector {
    public Motor weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e1234) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e45))), (right_anti_dual_g3_xyz[0] * this.e4235) - (this.e45 * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Line> operator /(Line lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public AntiScalar weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Line weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Line(
            /* e415, e425, e435 */
            float3(right_anti_dual_g0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public AntiScalar weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiScalar(
            /* e12345 */
            -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Motor weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g2_w) * this.group0.xyz), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Plane weight_expansion(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (float3(other.e1234) * this.group1.xyz) - (this.group0.zxy * other.group3.yzx)), this.e235 * other.e4235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Line weight_expansion(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Plane weight_expansion(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), this.e235 * other.e4235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public AntiScalar weight_expansion(Line other) {
        return AntiScalar(
            /* e12345 */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Motor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)), 
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
            float4(1.0, 1.0, right_anti_dual_g0[0], 0.0) * float4((this.group0.xyz * float3(float2(right_anti_dual_g0[0]), 1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[0]) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (float3(other.e1234) * this.group1.xyz) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx)), right_anti_dual_g1_xyz[0] * this.e235), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Plane weight_expansion(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), this.e235 * other.e4235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Plane weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (float3(other.e1234) * this.group1.xyz) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), right_anti_dual_g0_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Motor weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), -(right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(right_anti_dual_g0_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Line {
    public Plane weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (float3(other.e1234) * this.group1.xyz) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx)), right_anti_dual_g3_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Motor> operator /(Motor lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Plane weight_expansion(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float4(this.e5).xyz), other.e45 * this.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float4(this.e5).xyz), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e4 * this.e5)), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual_g1_xyz * float4(this.e5).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((other.group0.xyz * float4(this.e5).xyz), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual_g1_xyz * float4(this.e5).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_anti_dual_g2_w) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz)), right_anti_dual_g2_w * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual_g2_w) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), right_anti_dual_g2_w * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Plane weight_expansion(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((other.group0.xyz * float4(this.e5).xyz), other.e45 * this.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Flector weight_expansion(DipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5) * float4(other.group3.xyz, other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((other.e4125 * this.e425) - (other.e4315 * this.e435), (other.e4235 * this.e435) - (other.e4125 * this.e415), (other.e4315 * this.e415) - (other.e4235 * this.e425)) + (float3(this.e5) * other.group0.xyz) - (float3(other.e1234) * this.group1.xyz)), other.e4235 * this.e235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * -1.0) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightExpansionInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public AntiDualNum weight_expansion(FlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e45 * this.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Flector weight_expansion(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float4(this.e5).xyz * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e4315 * this.e415) - (other.e4235 * this.e425), 0.0) + float4(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((other.group0.xyz * float4(this.e5).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(right_anti_dual_g0_w) * this.group0.xyz), (right_anti_dual_g0_w * this.e12345) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float3 right_anti_dual_g5 = other.group6.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e12345) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (this.e5 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            right_anti_dual_g1 * float4(this.e5 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.e5) * other.group7.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e5) * other.group4.xyz) + float3(((right_anti_dual_g1.yz * this.group0.zx) - (right_anti_dual_g1.zx * this.group0.yz)), (right_anti_dual_g1[0] * this.e425) - (right_anti_dual_g1[1] * this.e415)) - (float3(right_anti_dual_g1[3]) * this.group1.xyz)), right_anti_dual_g1[0] * this.e235), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Flector weight_expansion(Plane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float4(this.e5).xyz * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((this.group0.yz * other.group0.zx) - (this.group0.zx * other.group0.yz)), (this.e415 * other.e4315) - (this.e425 * other.e4235), this.e235 * other.e4235 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public AntiScalar weight_expansion(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Flector weight_expansion(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.e5 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g0[1] * this.e435) - (right_anti_dual_g0[2] * this.e425), (right_anti_dual_g0[2] * this.e415) - (right_anti_dual_g0[0] * this.e435), (right_anti_dual_g0[0] * this.e425) - (right_anti_dual_g0[1] * this.e415)) - (float3(right_anti_dual_g0[3]) * this.group1.xyz)), right_anti_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Motor weight_expansion(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            (right_anti_dual_g0 * float4(float3(this.e5), this.e12345)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), right_anti_dual_g0[3] * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Motor {
    public Flector weight_expansion(VersorOdd other) {
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g3 * float4(this.e5 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, right_anti_dual_g3[1] * this.e415 * -1.0, 0.0) + (right_anti_dual_g3.yzxx * float4(this.group0.zxy, this.e235)) + float4(((float3(this.e5) * other.group0.xyz) + float3(-(right_anti_dual_g3.zx * this.group0.yz), 0.0) - (float3(right_anti_dual_g3[3]) * this.group1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<MultiVector> operator /(MultiVector lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * other.scalar) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (this.e23 * right_anti_dual_g1[0]) - (this.e31 * right_anti_dual_g1[1]) - (this.e12 * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * this.e45)), 
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
            right_anti_dual_g1 * float4(this.scalar), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (right_anti_dual_g1[0] * this.e2) - (right_anti_dual_g1[1] * this.e1), 0.0) + float4((float3(((right_anti_dual_g1.yz * this.group1.zx) - (right_anti_dual_g1.zx * this.group1.yz)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3_w * this.e4) + (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_anti_dual_g1_xyz * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (right_anti_dual_g3_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1_w) * this.group5.xyz) + (right_anti_dual_g2_xyz.zxy * this.group4.yzx) + (other.group0.yzx * this.group3.zxy) - (right_anti_dual_g2_xyz.yzx * this.group4.zxy) - (other.group0.zxy * this.group3.yzx)), right_anti_dual_g3_w * this.scalar), 
            /* e1234 */
            (right_anti_dual_g2_w * this.scalar) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiDualNum other) {
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
            float4(1.0, 1.0, other.e3215, 0.0) * float4((this.group4.xyz * float3(float2(other.e3215), 1.0)), 0.0), 
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
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e423 * right_anti_dual_g0[0]) - (this.e431 * right_anti_dual_g0[1]) - (this.e412 * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g0[2] * this.e2) - (right_anti_dual_g0[1] * this.e3), (right_anti_dual_g0[0] * this.e3) - (right_anti_dual_g0[2] * this.e1), (right_anti_dual_g0[1] * this.e1) - (right_anti_dual_g0[0] * this.e2)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g0.zxy) - (this.group4.zxy * right_anti_dual_g0.yzx)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiFlector other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1_w * this.e4) + (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (this.e423 * right_anti_dual_g0[0]) - (this.e431 * right_anti_dual_g0[1]) - (this.e412 * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g0[2] * this.e2) - (right_anti_dual_g0[1] * this.e3), (right_anti_dual_g0[0] * this.e3) - (right_anti_dual_g0[2] * this.e1), (right_anti_dual_g0[1] * this.e1) - (right_anti_dual_g0[0] * this.e2)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g0.zxy) - (this.group4.zxy * right_anti_dual_g0.yzx)), right_anti_dual_g1_w * this.scalar), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)), 
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
            float4(1.0, 1.0, this.scalar, 0.0) * float4((right_anti_dual_g0 * float3(float2(this.scalar), 1.0)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_anti_dual_g1 * float3(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1 * float3(this.e4)) + (right_anti_dual_g0.yzx * this.group1.zxy) - (right_anti_dual_g0.zxy * this.group1.yzx)), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * other.scalar) + (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (other.e3215 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group4.xyz) - (float3(this.scalar) * other.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group5.xyz) - (float3(this.scalar) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((other.e12 * this.e2) - (other.e31 * this.e3), (other.e23 * this.e3) - (other.e12 * this.e1), (other.e31 * this.e1) - (other.e23 * this.e2)) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e4) * other.group1.xyz)), other.e3215 * this.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)), 
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
            right_anti_dual_g0 * float4(this.scalar), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e5 */
            right_anti_dual_g0 * this.e5, 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0) * this.group3, 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * this.group6, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * this.group9, 
            /* e1234 */
            right_anti_dual_g0 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_anti_dual_g1_xyz * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group2.xyz) + (float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (float3(right_anti_dual_g1_w) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) + (other.group2.zxy * this.group4.yzx) - (other.group0.zxy * this.group3.yzx) - (other.group2.yzx * this.group4.zxy)), 0.0), 
            /* e1234 */
            -(right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g2_w * this.scalar, (right_anti_dual_g2_w * this.e12345) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2_w) * this.group1, 
            /* e5 */
            right_anti_dual_g2_w * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(right_anti_dual_g2_w) * this.group3.xyz)), (right_anti_dual_g1_w * this.scalar) + (right_anti_dual_g2_w * this.e45)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g2_w) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group6.xyz) + (float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group1.xyz)), right_anti_dual_g2_w * this.e321), 
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group7.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group8.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (float3(right_anti_dual_g1_w) * this.group5.xyz) + (float3(right_anti_dual_g2_w) * this.group9.xyz) + (right_anti_dual_g2_xyz.zxy * this.group4.yzx) + (other.group0.yzx * this.group3.zxy) - (right_anti_dual_g2_xyz.yzx * this.group4.zxy) - (other.group0.zxy * this.group3.yzx)), right_anti_dual_g2_w * this.e3215), 
            /* e1234 */
            (right_anti_dual_g2_w * this.e1234) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (this.e23 * right_anti_dual_g1[0]) - (this.e31 * right_anti_dual_g1[1]) - (this.e12 * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * this.e45)), 
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
            right_anti_dual_g1 * float4(this.scalar), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e235, e315, e125 */
            right_anti_dual_g2 * float3(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (right_anti_dual_g1[0] * this.e2) - (right_anti_dual_g1[1] * this.e1), 0.0) + float4(((right_anti_dual_g2 * float3(this.e4)) + float3(((right_anti_dual_g1.yz * this.group1.zx) - (right_anti_dual_g1.zx * this.group1.yz)), 0.0) - (right_anti_dual_g0 * float3(this.e5))), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (other.group3 * float4(float3(this.e5), this.e4)) + float4((float3(other.e3215) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            -(float3(other.e1234) * this.group1.xyz) - (float3(this.e4) * other.group3.xyz), 
            /* e23, e31, e12 */
            float3((other.e4315 * this.e3) - (other.e4125 * this.e2), (other.e4125 * this.e1) - (other.e4235 * this.e3), (other.e4235 * this.e2) - (other.e4315 * this.e1)), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group3.xyz) + (float3(other.e3215) * this.group4.xyz) + (float3(this.e45) * other.group3.xyz) - (float3(this.scalar) * other.group1.xyz)), this.scalar * other.e45), 
            /* e423, e431, e412 */
            (right_anti_dual_g0 * float3(this.scalar)) + (float3(other.e1234) * this.group5.xyz) + (this.group4.zxy * other.group3.yzx) - (this.group4.yzx * other.group3.zxy), 
            /* e235, e315, e125 */
            float3((other.e4125 * this.e25) - (other.e4315 * this.e35), (other.e4235 * this.e35) - (other.e4125 * this.e15), (other.e4315 * this.e15) - (other.e4235 * this.e25)) + (float3(other.e3215) * this.group5.xyz) - (float3(this.scalar) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (this.group1.yzxy * float4(other.group1.zxy, other.e25)) + float4(((float3(other.e3215) * this.group7.xyz) + float3(((other.group3.zx * this.group6.yz) - (other.group1.yz * this.group1.zx) - (other.group3.yz * this.group6.zx)), (other.e4315 * this.e415) - (other.e23 * this.e2) - (other.e4235 * this.e425)) - (right_anti_dual_g0 * float3(this.e5)) - (float3(other.e1234) * this.group8.xyz) - (float3(this.e4) * other.group2.xyz)), (other.e15 * this.e1) + (other.e35 * this.e3) - (this.e235 * other.e4235) - (this.e315 * other.e4315) - (this.e125 * other.e4125) - (other.e45 * this.e5)), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) + (other.e45 * this.e4) - (other.e1234 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e12345 * this.scalar * -1.0, -(other.e5 * this.e4) - (other.e12345 * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e5 */
            other.e12345 * this.e5 * -1.0, 
            /* e15, e25, e35, e45 */
            float4(other.e12345 * -1.0) * this.group3, 
            /* e41, e42, e43 */
            float3(other.e12345 * -1.0) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345 * -1.0) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * -1.0) * this.group6, 
            /* e423, e431, e412 */
            float3(other.e12345 * -1.0) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345 * -1.0) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group9.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * this.scalar) - (other.e12345 * this.e3215)), 
            /* e1234 */
            other.e12345 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (other.e45 * this.e45)), 
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
            float4(float3(0.0), this.scalar * other.e45), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.e4).xyz * other.group0.xyz * float3(-1.0)), (other.e15 * this.e1) + (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5)), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)), 
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group1.xyz * float3(float2(this.scalar * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), other.e3215 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3((other.e4315 * this.e3) - (other.e4125 * this.e2), (other.e4125 * this.e1) - (other.e4235 * this.e3), (other.e4235 * this.e2) - (other.e4315 * this.e1)), 
            /* e415, e425, e435, e321 */
            (other.group1.xyzx * float4(float3(this.e45), this.e23)) + float4((float3(other.e3215) * this.group4.xyz), (this.scalar * other.e45) + (this.e31 * other.e4315) + (this.e12 * other.e4125)), 
            /* e423, e431, e412 */
            (this.group4.zxy * other.group1.yzx) - (this.group4.yzx * other.group1.zxy), 
            /* e235, e315, e125 */
            float3((other.e4125 * this.e25) - (other.e4315 * this.e35), (other.e4235 * this.e35) - (other.e4125 * this.e15), (other.e4315 * this.e15) - (other.e4235 * this.e25)) + (float3(other.e3215) * this.group5.xyz) - (float3(this.scalar) * other.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group7.xyz) + float3((other.group1.zx * this.group6.yz), other.e4315 * this.e415) - (float3(this.e4) * other.group0.xyz)), (other.e15 * this.e1) + (other.e25 * this.e2) - (this.e235 * other.e4235) - (this.e125 * other.e4125) - (other.e45 * this.e5)) - (other.group1.yzxy * float4(this.group6.zxy, this.e315)), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group1.xyz * float3(float2(this.scalar), 1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float4(this.e4).xyz), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group0.xyz) + (other.group1.zxy * this.group4.yzx) - (other.group1.yzx * this.group4.zxy)), 0.0), 
            /* e1234 */
            -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * right_anti_dual_g0[3], (right_anti_dual_g1_w * this.e4) + (this.e12345 * right_anti_dual_g0[3]) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * this.group1, 
            /* e5 */
            right_anti_dual_g0[3] * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0[3]) * this.group3.xyz)), right_anti_dual_g0[3] * this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[3]) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * right_anti_dual_g0.xyz) + (float3(right_anti_dual_g0[3]) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0[3]) * this.group6.xyz)), right_anti_dual_g0[3] * this.e321), 
            /* e423, e431, e412 */
            (float3(right_anti_dual_g0[3]) * this.group7.xyz) + (float3(this.e4) * right_anti_dual_g0.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * this.group8.xyz) + (float3(this.e5) * right_anti_dual_g0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group1.yzx) - (right_anti_dual_g1_xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0 * float4(float3(this.e45), this.e3215)) + float4(((float3(right_anti_dual_g0[3]) * this.group9.xyz) + (right_anti_dual_g1_xyz.zxy * this.group4.yzx) - (right_anti_dual_g1_xyz.yzx * this.group4.zxy)), right_anti_dual_g1_w * this.scalar), 
            /* e1234 */
            (right_anti_dual_g0[3] * this.e1234) - (this.e41 * right_anti_dual_g0[0]) - (this.e42 * right_anti_dual_g0[1]) - (this.e43 * right_anti_dual_g0[2])
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[0] * this.scalar, (right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (this.e23 * right_anti_dual_g6[0]) - (this.e31 * right_anti_dual_g6[1]) - (this.e12 * right_anti_dual_g6[2]) - (this.e423 * right_anti_dual_g3[0]) - (this.e431 * right_anti_dual_g3[1]) - (this.e412 * right_anti_dual_g3[2]) - (right_anti_dual_g3[3] * this.e321) - (right_anti_dual_g6[3] * this.e45)), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group1), 
            /* e5 */
            (right_anti_dual_g0[0] * this.e5) + (this.scalar * other.e3215), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group3) + (float4(other.e3215) * this.group1) - (right_anti_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g0[0]) * this.group4.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g1[2] * this.e2) - (right_anti_dual_g1[1] * this.e3), (right_anti_dual_g1[0] * this.e3) - (right_anti_dual_g1[2] * this.e1), (right_anti_dual_g1[1] * this.e1) - (right_anti_dual_g1[0] * this.e2)) + (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group6) + float4(((float3(right_anti_dual_g1[3]) * this.group3.xyz) + (float3(other.e3215) * this.group4.xyz) + (float3(this.e4) * right_anti_dual_g3.xyz) + (float3(this.e5) * other.group7.xyz) - (float3(right_anti_dual_g3[3]) * this.group1.xyz) - (float3(this.e45) * right_anti_dual_g1.xyz)), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g5 * float3(this.e4)) + (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group7.xyz) + (float3(right_anti_dual_g1[3]) * this.group5.xyz) + (other.group7.yzx * this.group1.zxy) + (this.group4.yzx * right_anti_dual_g1.zxy) - (other.group7.zxy * this.group1.yzx) - (this.group4.zxy * right_anti_dual_g1.yzx), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g1[1] * this.e35) + (right_anti_dual_g3[2] * this.e2) - (right_anti_dual_g1[2] * this.e25) - (right_anti_dual_g3[1] * this.e3), (right_anti_dual_g1[2] * this.e15) + (right_anti_dual_g3[0] * this.e3) - (right_anti_dual_g1[0] * this.e35) - (right_anti_dual_g3[2] * this.e1), (right_anti_dual_g1[0] * this.e25) + (right_anti_dual_g3[1] * this.e1) - (right_anti_dual_g1[1] * this.e15) - (right_anti_dual_g3[0] * this.e2)) + (right_anti_dual_g5 * float3(this.e5)) + (right_anti_dual_g8 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group8.xyz) + (float3(other.e3215) * this.group5.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * this.group9) + float4(((right_anti_dual_g5 * float3(this.e45)) + (right_anti_dual_g8 * float3(this.e4)) + (float3(right_anti_dual_g3[3]) * this.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (other.group7.yzx * this.group3.zxy) + (this.group4.yzx * right_anti_dual_g3.zxy) + float3(((right_anti_dual_g1.yz * this.group6.zx) + (right_anti_dual_g6.yz * this.group1.zx) - (right_anti_dual_g1.zx * this.group6.yz) - (right_anti_dual_g6.zx * this.group1.yz)), (right_anti_dual_g1[0] * this.e425) + (right_anti_dual_g6[0] * this.e2) - (right_anti_dual_g1[1] * this.e415) - (right_anti_dual_g6[1] * this.e1)) - (right_anti_dual_g7 * float3(this.e5)) - (float3(right_anti_dual_g1[3]) * this.group8.xyz) - (other.group7.zxy * this.group3.yzx) - (this.group4.zxy * right_anti_dual_g3.yzx)), (this.e235 * right_anti_dual_g1[0]) + (this.e315 * right_anti_dual_g1[1]) + (this.e125 * right_anti_dual_g1[2]) + (other.e3215 * this.e321)), 
            /* e1234 */
            (right_anti_dual_g10 * this.scalar) + (right_anti_dual_g0[0] * this.e1234) + (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3) + (right_anti_dual_g6[3] * this.e4) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (this.e423 * right_anti_dual_g1[0]) - (this.e431 * right_anti_dual_g1[1]) - (this.e412 * right_anti_dual_g1[2]) - (right_anti_dual_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e3215 * this.e1234) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)), 
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group0.xyz * float3(float2(this.scalar * -1.0), 1.0) * float3(1.0, 1.0, -1.0)), 0.0), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), this.e4 * other.e3215), 
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3((this.e3 * other.e4315) - (this.e2 * other.e4125), (this.e1 * other.e4125) - (this.e3 * other.e4235), (this.e2 * other.e4235) - (this.e1 * other.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e45) * other.group0.xyz) + (float3(other.e3215) * this.group4.xyz)), 0.0), 
            /* e423, e431, e412 */
            (this.group4.zxy * other.group0.yzx) - (this.group4.yzx * other.group0.zxy), 
            /* e235, e315, e125 */
            float3((this.e25 * other.e4125) - (this.e35 * other.e4315), (this.e35 * other.e4235) - (this.e15 * other.e4125), (this.e15 * other.e4315) - (this.e25 * other.e4235)) + (float3(other.e3215) * this.group5.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e415 * other.e4315) - (this.e425 * other.e4235), 0.0) + float4(((float3(other.e3215) * this.group7.xyz) + float3(((this.group6.yz * other.group0.zx) - (this.group6.zx * other.group0.yz)), 0.0)), 0.0), 
            /* e1234 */
            (this.e423 * other.e4235) + (this.e431 * other.e4315) + (this.e412 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)), 
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
            right_anti_dual_g0 * float4(this.scalar), 
            /* e1234 */
            right_anti_dual_g1 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group1) - (right_anti_dual_g0 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g0[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g0[2] * this.e2) - (right_anti_dual_g0[1] * this.e3), (right_anti_dual_g0[0] * this.e3) - (right_anti_dual_g0[2] * this.e1), (right_anti_dual_g0[1] * this.e1) - (right_anti_dual_g0[0] * this.e2)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group3.xyz) + (float3(other.e3215) * this.group4.xyz) - (float3(this.e45) * right_anti_dual_g0.xyz)), 0.0), 
            /* e423, e431, e412 */
            (float3(right_anti_dual_g0[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g0.zxy) - (this.group4.zxy * right_anti_dual_g0.yzx), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g0[1] * this.e35) - (right_anti_dual_g0[2] * this.e25), (right_anti_dual_g0[2] * this.e15) - (right_anti_dual_g0[0] * this.e35), (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15)) + (float3(other.e3215) * this.group5.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g0.yzxy * float4(this.group6.zxy, this.e315)) + float4(((float3(other.e3215) * this.group7.xyz) + float3(-(right_anti_dual_g0.zx * this.group6.yz), right_anti_dual_g0[1] * this.e415 * -1.0) - (float3(right_anti_dual_g0[3]) * this.group8.xyz)), this.e235 * right_anti_dual_g0[0]), 
            /* e1234 */
            -(this.e423 * right_anti_dual_g0[0]) - (this.e431 * right_anti_dual_g0[1]) - (this.e412 * right_anti_dual_g0[2]) - (right_anti_dual_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0_w * this.scalar, (right_anti_dual_g0_w * this.e12345) + (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3_w * this.e4) + (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * this.group1, 
            /* e5 */
            right_anti_dual_g0_w * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual_g0_w) * this.group3) + (float4(this.scalar) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)), 
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group4.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group6.xyz) - (float3(right_anti_dual_g1_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e321), 
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group7.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group8.xyz) + (right_anti_dual_g2_xyz.zxy * this.group1.yzx) - (right_anti_dual_g2_xyz.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(right_anti_dual_g0_w) * this.group9) + float4(((right_anti_dual_g1_xyz * float3(this.e45)) + (right_anti_dual_g3_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1_w) * this.group5.xyz) + (right_anti_dual_g0_xyz.yzx * this.group3.zxy) + (right_anti_dual_g2_xyz.zxy * this.group4.yzx) - (right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group4.zxy)), right_anti_dual_g3_w * this.scalar), 
            /* e1234 */
            (right_anti_dual_g0_w * this.e1234) + (right_anti_dual_g2_w * this.scalar) - (right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1_xyz[0] * this.e41) - (right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension MultiVector {
    public MultiVector weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * other.scalar) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (right_anti_dual_g3[0] * this.e4235) + (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (other.e3215 * this.e1234) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(this.scalar), 
            /* e5 */
            this.scalar * other.e3215, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group1) - (right_anti_dual_g3 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g3[3]) * this.group1.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g3[2] * this.e2) - (right_anti_dual_g3[1] * this.e3), (right_anti_dual_g3[0] * this.e3) - (right_anti_dual_g3[2] * this.e1), (right_anti_dual_g3[1] * this.e1) - (right_anti_dual_g3[0] * this.e2)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g3[3]) * this.group3.xyz) + (float3(other.e3215) * this.group4.xyz) - (float3(this.scalar) * other.group1.xyz) - (float3(this.e45) * right_anti_dual_g3.xyz)), this.scalar * other.e45), 
            /* e423, e431, e412 */
            (float3(right_anti_dual_g3[3]) * this.group5.xyz) + (this.group4.yzx * right_anti_dual_g3.zxy) - (float3(this.scalar) * other.group0.xyz) - (this.group4.zxy * right_anti_dual_g3.yzx), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g3[1] * this.e35) - (right_anti_dual_g3[2] * this.e25), (right_anti_dual_g3[2] * this.e15) - (right_anti_dual_g3[0] * this.e35), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + (right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group5.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g3.yzxz * float4(this.group6.zxy, this.e125)) + float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e5) * other.group0.xyz) + float3(((this.group1.yz * other.group1.zx) - (right_anti_dual_g3.zx * this.group6.yz) - (this.group1.zx * other.group1.yz)), (this.e1 * other.e31) - (right_anti_dual_g3[1] * this.e415) - (this.e2 * other.e23)) - (float3(right_anti_dual_g3[3]) * this.group8.xyz)), (this.e235 * right_anti_dual_g3[0]) + (this.e315 * right_anti_dual_g3[1]) + (this.e321 * other.e3215) - (other.e45 * this.e5)), 
            /* e1234 */
            (this.e4 * other.e45) - (this.e423 * right_anti_dual_g3[0]) - (this.e431 * right_anti_dual_g3[1]) - (this.e412 * right_anti_dual_g3[2]) - (right_anti_dual_g3[3] * this.e321) - (this.e1 * other.e41) - (this.e2 * other.e42) - (this.e3 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Plane> operator /(Plane lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public Plane weight_expansion(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public Plane weight_expansion(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public AntiScalar weight_expansion(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e1234 * this.e3215) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public Plane weight_expansion(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public AntiScalar weight_expansion(Flector other) {
        return AntiScalar(
            /* e12345 */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public Plane weight_expansion(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public MultiVector weight_expansion(MultiVector other) {
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (this.e3215 * other.e1234)), 
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
            float4(other.e12345 * -1.0) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public AntiScalar weight_expansion(Plane other) {
        return AntiScalar(
            /* e12345 */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public AntiScalar weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public Plane weight_expansion(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Plane {
    public AntiScalar weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<RoundPoint> operator /(RoundPoint lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Sphere weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(other.e321) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), other.e1 * this.e1)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public FlatPoint weight_expansion(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Line weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            (right_anti_dual_g0_xyz * float3(this.e4)) + (float3(other.e321) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Motor weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0_xyz * float3(this.e4)) + (float3(other.e321) * this.group0.xyz)), other.e1 * this.e1), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Plane weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0.yzx * this.group0.zxy) - (float3(this.e4) * other.group1.xyz) - (right_anti_dual_g0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Flector weight_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group0.zx * this.group0.yz) - (other.group0.yz * this.group0.zx)), 0.0) - (float3(this.e4) * other.group1.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar weight_expansion(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightExpansionInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e5 */
            right_anti_dual_g0 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Circle weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Circle(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e321) * this.group0.xyz) + (float3(this.e4) * other.group2.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125 */
            (right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(other.e321) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e4 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g2_w * this.e4), 
            /* e1, e2, e3, e5 */
            float4(right_anti_dual_g2_w) * float4(this.group0.xyz, this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Sphere weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(other.e1234) * this.group0.xyz) - (float3(this.e4) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.yzxw * this.group0.zxyw) + float4(-(other.group3.zx * this.group0.yz), other.e4315 * this.e1 * -1.0, other.e1234 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e5) * other.group3.xyz)), right_anti_dual_g0[0] * this.e1), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven weight_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e5 * this.e4 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.e12345 * this.e5 * -1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Sphere weight_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.e4).xyz * other.group0.xyz * float3(-1.0)), (other.e15 * this.e1) + (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5)), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.yz * this.group0.zx) - (other.group1.zx * this.group0.yz)), (other.e4235 * this.e2) - (other.e4315 * this.e1), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz)), other.e45 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.e4).xyz * other.group0.xyz * float3(-1.0)), (other.e15 * this.e1) + (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Circle weight_expansion(Line other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float4(this.e4).xyz), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((right_anti_dual_g0_xyz * float4(this.e4).xyz), other.e5 * this.e4 * -1.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g1_xyz * float4(this.e4).xyz), -(right_anti_dual_g0_xyz[0] * this.e1) - (right_anti_dual_g0_xyz[1] * this.e2) - (right_anti_dual_g0_xyz[2] * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (other.e4 * this.e5) - (this.e4 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[0]) * this.group0, 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group0) - (right_anti_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group0.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g1[2] * this.e2) - (right_anti_dual_g1[1] * this.e3), (right_anti_dual_g1[0] * this.e3) - (right_anti_dual_g1[2] * this.e1), (right_anti_dual_g1[1] * this.e1) - (right_anti_dual_g1[0] * this.e2)), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e321) * this.group0.xyz) + (float3(this.e4) * other.group8.xyz) + (float3(this.e5) * other.group7.xyz)), 0.0), 
            /* e423, e431, e412 */
            (right_anti_dual_g5 * float3(this.e4)) + (other.group7.yzx * this.group0.zxy) - (other.group7.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g5 * float3(this.e5)) + (other.group8.zxy * this.group0.yzx) - (other.group8.yzx * this.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g6_xyz.yzx * this.group0.zxy) - (right_anti_dual_g7 * float3(this.e5)) - (float3(this.e4) * other.group3.xyz) - (right_anti_dual_g6_xyz.zxy * this.group0.yzx)), 0.0), 
            /* e1234 */
            (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Dipole weight_expansion(Plane other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), (other.e4235 * this.e2) - (other.e4315 * this.e1), other.e3215 * this.e4), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) + (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public AntiScalar weight_expansion(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public Dipole weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_anti_dual_g0_xyz.zxy, other.e3215)) + float4(-(right_anti_dual_g0_xyz.yzx * this.group0.zxy), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e5))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public VersorEven weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g3_xyz[0])) + (this.group0.wwwy * float4(right_anti_dual_g1_xyz, right_anti_dual_g3_xyz[1])) + float4(-(right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz * float3(this.e4)) + (float3(other.e321) * this.group0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension RoundPoint {
    public DipoleInversion weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(right_anti_dual_g3_xyz.zxy, other.e3215)) + float4(-(right_anti_dual_g3_xyz.yzx * this.group0.zxy), other.e1234 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) - (right_anti_dual_g3_xyz * float3(this.e5))), this.e1 * other.e41 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (this.e1 * other.e31) - (this.e2 * other.e23), 0.0) + float4(((float3(this.e5) * other.group0.xyz) + float3(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0) - (float3(this.e4) * other.group2.xyz)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<Scalar> operator /(Scalar lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
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
public DipoleInversion operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
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
public DualNum operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public DualNum weight_expansion(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public FlatPoint weight_expansion(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Flector weight_expansion(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Line weight_expansion(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Motor weight_expansion(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightExpansionInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Scalar weight_expansion(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightExpansionInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Dipole weight_expansion(Circle other) {
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
public AntiCircleRotor operator /(WeightExpansionInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiCircleRotor weight_expansion(CircleRotor other) {
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
public Circle operator /(WeightExpansionInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Circle weight_expansion(Dipole other) {
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
public AntiDipoleInversion operator /(WeightExpansionInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiDipoleInversion weight_expansion(DipoleInversion other) {
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
public AntiDualNum operator /(WeightExpansionInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiDualNum weight_expansion(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightExpansionInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiFlatPoint weight_expansion(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightExpansionInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiFlector weight_expansion(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightExpansionInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiLine weight_expansion(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightExpansionInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiMotor weight_expansion(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public MultiVector weight_expansion(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar * -1.0) * other.group9.xyz), other.e1234 * this.scalar), 
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
            float4((float3(this.scalar) * other.group1.xyz), other.e5 * this.scalar * -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightExpansionInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiPlane weight_expansion(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public Sphere weight_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.scalar) * other.group0.xyz), other.e5 * this.scalar * -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightExpansionInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public RoundPoint weight_expansion(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            other.e3215 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public VersorOdd weight_expansion(VersorEven other) {
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
public VersorEven operator /(WeightExpansionInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Scalar {
    public VersorEven weight_expansion(VersorOdd other) {
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
public WeightExpansionInfixPartial<Sphere> operator /(Sphere lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.e3215 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public Sphere weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e1234 */
            right_anti_dual_g0 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public Sphere weight_expansion(CircleRotor other) {
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g2_w) * this.group0, 
            /* e1234 */
            right_anti_dual_g2_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public Sphere weight_expansion(DualNum other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(Flector other) {
        return AntiScalar(
            /* e12345 */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public Sphere weight_expansion(Motor other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0_w) * this.group0, 
            /* e1234 */
            right_anti_dual_g0_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (this.e3215 * other.e1234)), 
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
            float4(right_anti_dual_g0[0]) * this.group0, 
            /* e1234 */
            right_anti_dual_g0[0] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public Sphere weight_expansion(VersorEven other) {
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0_w) * this.group0, 
            /* e1234 */
            right_anti_dual_g0_w * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension Sphere {
    public AntiScalar weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g3_xyz[0] * this.e4235) + (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<VersorEven> operator /(VersorEven lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Sphere weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group3.yz) - (other.group1.yz * this.group3.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor weight_expansion(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(this.e5) * other.group0.xyz) - (float3(right_anti_dual_g1_w) * this.group3.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(right_anti_dual_g1_xyz, other.e4 * -1.0)) + float4(((right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), (other.e1 * this.e1) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Flector weight_expansion(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e3215) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Motor weight_expansion(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0_xyz * float3(this.e4)) - (float3(right_anti_dual_g0_w) * this.group3.xyz)), right_anti_dual_g0_w * this.e321 * -1.0), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g0_xyz.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Motor weight_expansion(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            (this.group3.xyzx * float4(float3(other.e321), other.e1)) + float4((right_anti_dual_g0_xyz * float3(this.e4)), (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g0_xyz.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Plane weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0.yzx * this.group3.zxy) - (float3(this.e4) * other.group1.xyz) - (right_anti_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Flector weight_expansion(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e3215) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((other.e12 * this.e2) - (other.e31 * this.e3), (other.e23 * this.e3) - (other.e12 * this.e1), (other.e31 * this.e1) - (other.e23 * this.e2)) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e4) * other.group1.xyz)), other.e3215 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar weight_expansion(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public VersorEven weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(right_anti_dual_g0) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor weight_expansion(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return CircleRotor(
            /* e423, e431, e412 */
            (right_anti_dual_g1_xyz * float3(this.e4)) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e321) * this.group3.xyz) + (float3(this.e5) * other.group0.xyz) + (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e235, e315, e125, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (other.group2.zxy * this.group3.yzx) - (other.group2.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public VersorEven weight_expansion(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e12345 * -1.0;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group0.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), right_anti_dual_g2_w * this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g2_w) * this.group1.xyz) + (float3(other.e321) * this.group3.xyz) + (float3(this.e5) * other.group0.xyz)), right_anti_dual_g2_w * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g2_w) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g2_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g2_w) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Sphere weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4((float3(((other.group1.zx * this.group3.yz) - (other.group1.yz * this.group3.zx)), 0.0) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), 0.0), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (other.e45 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            -(float3(other.e1234) * this.group3.xyz) - (float3(this.e4) * other.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.yzxw * this.group3.zxyw) + float4(-(other.group3.zx * this.group3.yz), other.e4315 * this.e1 * -1.0, other.e1234 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (other.group3.wwwx * float4(this.group3.xyz, this.e423)) + float4((float3(this.e5) * other.group3.xyz), (right_anti_dual_g0[0] * this.e1) + (other.e4315 * this.e431) + (other.e4125 * this.e412) - (other.e1234 * this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (this.group3.yzxx * float4(other.group1.zxy, other.e15)) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((other.group3.zx * this.group1.yz) - (other.group1.yz * this.group3.zx) - (other.group3.yz * this.group1.zx)), (other.e4315 * this.e415) - (other.e23 * this.e2) - (other.e4235 * this.e425)) - (right_anti_dual_g0 * float3(this.e5)) - (float3(this.e4) * other.group2.xyz)), (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5) - (other.e4235 * this.e235) - (other.e4315 * this.e315)) - (this.group2.xyzz * float4(float3(other.e1234), other.e4125))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public VersorEven weight_expansion(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((this.group0.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * this.e4) - (other.e12345 * this.e12345)), 
            /* e415, e425, e435, e321 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.e12345 * -1.0) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.e12345 * -1.0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public Sphere weight_expansion(FlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.e4).xyz * other.group0.xyz * float3(-1.0)), (other.e15 * this.e1) + (other.e25 * this.e2) + (other.e35 * this.e3) - (other.e45 * this.e5)), 
            /* e1234 */
            other.e45 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Flector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.yz * this.group3.zx) - (other.group1.zx * this.group3.yz)), (other.e4235 * this.e2) - (other.e4315 * this.e1), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            (other.group1.wwwx * float4(this.group3.xyz, this.e423)) + float4((float3(this.e5) * other.group1.xyz), (other.e45 * this.e4) + (other.e4315 * this.e431) + (other.e4125 * this.e412)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + float3((other.group1.zx * this.group1.yz), other.e4315 * this.e415)), (other.e15 * this.e1) + (other.e25 * this.e2) - (other.e4315 * this.e315) - (other.e4125 * this.e125)) - (other.group0 * float4(float3(this.e4), this.e5)) - (other.group1.yzxx * float4(this.group1.zxy, this.e235))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public CircleRotor weight_expansion(Line other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((other.group1.xyz * float4(this.e4).xyz), -(other.e415 * this.e1) - (other.e425 * this.e2) - (other.e435 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4(((float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group3.yzx) - (other.group1.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public VersorEven weight_expansion(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(float3(this.e4), this.e12345)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (other.e5 * this.e4)), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (float3(right_anti_dual_g0[3]) * this.group1.xyz)), right_anti_dual_g0[3] * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e5) * right_anti_dual_g0.xyz) + (right_anti_dual_g1_xyz.zxy * this.group3.yzx) - (right_anti_dual_g1_xyz.yzx * this.group3.zxy)), right_anti_dual_g0[3] * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float right_anti_dual_g3_w = other.e321 * -1.0;
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (right_anti_dual_g3_w * this.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412) - (other.e4 * this.e5) - (this.e4 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[0]) * this.group3, 
            /* e5 */
            right_anti_dual_g0[0] * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e3215) * this.group3) - (right_anti_dual_g1 * float4(this.e5)), 
            /* e41, e42, e43 */
            (float3(this.e4) * right_anti_dual_g1.xyz) - (float3(right_anti_dual_g1[3]) * this.group3.xyz), 
            /* e23, e31, e12 */
            float3((right_anti_dual_g1[2] * this.e2) - (right_anti_dual_g1[1] * this.e3), (right_anti_dual_g1[0] * this.e3) - (right_anti_dual_g1[2] * this.e1), (right_anti_dual_g1[1] * this.e1) - (right_anti_dual_g1[0] * this.e2)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[0]) * this.group1.xyz) + (float3(this.e5) * other.group7.xyz) + (float3(this.e4) * other.group8.xyz) - (float3(right_anti_dual_g3_w) * this.group3.xyz)), right_anti_dual_g0[0] * this.e321), 
            /* e423, e431, e412 */
            (right_anti_dual_g5 * float3(this.e4)) + (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (other.group7.yzx * this.group3.zxy) - (other.group7.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (right_anti_dual_g5 * float3(this.e5)) + (float3(right_anti_dual_g0[0]) * this.group2.xyz) + (other.group8.zxy * this.group3.yzx) - (other.group8.yzx * this.group3.zxy), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e321)) + (right_anti_dual_g1.yzxz * float4(this.group1.zxy, this.e125)) + float4(((right_anti_dual_g8 * float3(this.e4)) + (right_anti_dual_g6_xyz.yzx * this.group3.zxy) + float3(-(right_anti_dual_g1.zx * this.group1.yz), right_anti_dual_g1[1] * this.e415 * -1.0) - (right_anti_dual_g7 * float3(this.e5))), (right_anti_dual_g1[0] * this.e235) + (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g8[1] * this.e2) - (right_anti_dual_g8[2] * this.e3)) - (this.group2 * float4(float3(right_anti_dual_g1[3]), other.e45)) - (this.group3.yzxx * float4(right_anti_dual_g6_xyz.zxy, right_anti_dual_g8[0])), 
            /* e1234 */
            (right_anti_dual_g7[0] * this.e1) + (right_anti_dual_g7[1] * this.e2) + (right_anti_dual_g7[2] * this.e3) + (other.e45 * this.e4) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412) - (right_anti_dual_g1[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4 * -1.0) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.yz * this.group3.zx) - (other.group0.zx * this.group3.yz)), (other.e4235 * this.e2) - (other.e4315 * this.e1), other.e3215 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group3.xyz) + (float3(this.e5) * other.group0.xyz)), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (other.e4315 * this.e415) - (other.e4235 * this.e425), 0.0) + float4(((float3(other.e3215) * this.group0.xyz) + float3(((other.group0.zx * this.group1.yz) - (other.group0.yz * this.group1.zx)), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public AntiScalar weight_expansion(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiScalar(
            /* e12345 */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (this.e4 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion weight_expansion(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group3.yzx) - (right_anti_dual_g0_xyz.yzx * this.group3.zxy)), (other.e3215 * this.e4) - (this.e5 * other.e1234)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group3.xyz) - (right_anti_dual_g0_xyz * float3(this.e5))), this.e321 * other.e1234 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), (right_anti_dual_g0_xyz[0] * this.e235) + (right_anti_dual_g0_xyz[1] * this.e315))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public VersorEven weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(right_anti_dual_g0_w) * this.group0) + float4(((right_anti_dual_g1_xyz * float3(this.e4)) + (right_anti_dual_g0_xyz.yzx * this.group3.zxy) - (right_anti_dual_g0_xyz.zxy * this.group3.yzx)), (other.e1 * this.e1) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e4 * this.e5)), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz * float3(this.e5)) + (right_anti_dual_g2_xyz * float3(this.e4)) + (float3(right_anti_dual_g0_w) * this.group1.xyz) + (float3(other.e321) * this.group3.xyz)), right_anti_dual_g0_w * this.e321), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual_g1_xyz * float3(this.e5)) + (float3(right_anti_dual_g0_w) * this.group2.xyz) + (right_anti_dual_g2_xyz.zxy * this.group3.yzx) - (right_anti_dual_g2_xyz.yzx * this.group3.zxy)), right_anti_dual_g0_w * this.e5), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0_w) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorEven {
    public DipoleInversion weight_expansion(VersorOdd other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g3_xyz * float3(this.e4)) - (float3(other.e1234) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(right_anti_dual_g3_xyz.zxy, other.e3215)) + float4(-(right_anti_dual_g3_xyz.yzx * this.group3.zxy), this.e5 * other.e1234 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group3.xyz) - (right_anti_dual_g3_xyz * float3(this.e5))), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group2.wwwz * float4(other.group0.xyz, right_anti_dual_g3_xyz[2])) + float4(((float3(other.e3215) * this.group0.xyz) + (right_anti_dual_g3_xyz.yzx * this.group1.zxy) + float3(((this.group3.yz * other.group1.zx) - (this.group3.zx * other.group1.yz)), (this.e1 * other.e31) - (this.e2 * other.e23)) - (float3(this.e4) * other.group2.xyz) - (float3(other.e1234) * this.group2.xyz) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), (right_anti_dual_g3_xyz[0] * this.e235) + (right_anti_dual_g3_xyz[1] * this.e315) + (this.e321 * other.e3215) - (this.e5 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WeightExpansionInfixPartial<VersorOdd> operator /(VersorOdd lhs, WeightExpansionInfix rhs) {
    return WeightExpansionInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor weight_expansion(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float4(this.scalar).xyz * other.group2.xyz * float3(-1.0)), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2_xyz * float4(this.scalar).xyz), -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43) - (other.e4 * this.scalar)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.scalar) * other.group3.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (right_anti_dual_g2_xyz.zxy * this.group0.yzx) + (other.group0.yzx * this.group2.zxy) - (right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (other.group0.zxy * this.group2.yzx)), other.e5 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Motor weight_expansion(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * float3(float2(other.e3215), other.e3215)), (other.e3215 * this.e1234) + (other.scalar * this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(other.e3215) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Flector weight_expansion(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0, 0.0, (right_anti_dual_g0[1] * this.e41) - (right_anti_dual_g0[0] * this.e42), 0.0) + float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + float3(((right_anti_dual_g0.zx * this.group0.yz) - (right_anti_dual_g0.yz * this.group0.zx)), 0.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Flector weight_expansion(AntiFlector other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g0[2] * this.e42) - (right_anti_dual_g0[1] * this.e43), (right_anti_dual_g0[0] * this.e43) - (right_anti_dual_g0[2] * this.e41), (right_anti_dual_g0[1] * this.e41) - (right_anti_dual_g0[0] * this.e42)) + (float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e5 * this.scalar * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Motor weight_expansion(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float4(this.scalar).xyz), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual_g1 * float4(this.scalar).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Motor weight_expansion(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            (right_anti_dual_g0 * float4(this.scalar)) + (other.group1.wwwx * this.group0.xyzx) + float4(float3(0.0), (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group1.xyz)), other.e3215 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Plane weight_expansion(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_expansion(AntiScalar other) {
        float right_anti_dual_g0 = other.e12345 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(right_anti_dual_g0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual_g0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.xyz * float4(this.scalar).xyz), -(other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (other.group0.yzx * this.group2.zxy) + (other.group2.zxy * this.group0.yzx) - (other.group0.zxy * this.group2.yzx) - (other.group2.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_expansion(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e12345 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(right_anti_dual_g2[3]) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), right_anti_dual_g2[3] * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g2[3]) * this.group1), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(float3(this.scalar), this.e1234)) + float4((float3(right_anti_dual_g2[3]) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g2[2] * this.e42) - (right_anti_dual_g2[1] * this.e43), (right_anti_dual_g2[0] * this.e43) - (right_anti_dual_g2[2] * this.e41), (right_anti_dual_g2[1] * this.e41) - (right_anti_dual_g2[0] * this.e42)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(right_anti_dual_g2[3]) * this.group3.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (other.group0.yzx * this.group2.zxy) - (other.group0.zxy * this.group2.yzx)), right_anti_dual_g2[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor weight_expansion(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return CircleRotor(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.scalar), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((right_anti_dual_g2 * float4(this.scalar).xyz), -(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven weight_expansion(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.xyzy * float4(float3(other.e1234), other.e31)) + float4(((right_anti_dual_g0 * float3(this.scalar)) + float3(((other.group3.yz * this.group0.zx) - (other.group3.zx * this.group0.yz)), (other.e4235 * this.e42) - (other.e4315 * this.e41))), (other.e23 * this.e23) + (other.e12 * this.e12) + (other.e1234 * this.e3215) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315)), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e1234) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(this.e45) * other.group3.xyz) - (float3(this.scalar) * other.group1.xyz)), other.e45 * this.scalar), 
            /* e235, e315, e125, e5 */
            float4((float3((other.e4125 * this.e25) - (other.e4315 * this.e35), (other.e4235 * this.e35) - (other.e4125 * this.e15), (other.e4315 * this.e15) - (other.e4235 * this.e25)) + (float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group2.xyz)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_expansion(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345 * -1.0) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.e12345 * -1.0) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.e12345 * -1.0) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((this.group3.xyz * float3(float2(other.e12345 * -1.0), other.e12345) * float3(1.0, 1.0, -1.0)), -(other.e5 * this.scalar) - (other.e12345 * this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightExpansionInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public CircleRotor weight_expansion(FlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e45 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float4(this.scalar).xyz * other.group0.xyz * float3(-1.0)), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven weight_expansion(Flector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0.zxyx * float4(other.group1.yzx, other.e15)) + float4(-(other.group1.zx * this.group0.yz), other.e4315 * this.e41 * -1.0, -(other.e45 * this.e45) - (other.e4235 * this.e4235)), 
            /* e415, e425, e435, e321 */
            (other.group1.wwwx * float4(this.group0.xyz, this.e23)) + float4((float3(this.e45) * other.group1.xyz), (other.e45 * this.scalar) + (other.e4315 * this.e31) + (other.e4125 * this.e12)), 
            /* e235, e315, e125, e5 */
            float4((float3((other.e4125 * this.e25) - (other.e4315 * this.e35), (other.e4235 * this.e35) - (other.e4125 * this.e15), (other.e4315 * this.e15) - (other.e4235 * this.e25)) + (float3(other.e3215) * this.group1.xyz) - (float3(this.scalar) * other.group0.xyz)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float4(this.scalar).xyz * other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public DipoleInversion weight_expansion(Line other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((other.group0.xyz * float4(this.scalar).xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group1.xyz * float4(this.scalar).xyz), -(other.e415 * this.e41) - (other.e425 * this.e42) - (other.e435 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(((float3(this.e45) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_expansion(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g0_w) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group1.xyz)), right_anti_dual_g0_w * this.e45), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group2.xyz)), right_anti_dual_g0_w * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(((right_anti_dual_g0_xyz * float3(this.e45)) + (float3(right_anti_dual_g0_w) * this.group3.xyz) + (right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightExpansionInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public MultiVector weight_expansion(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g1 = float4((other.group9.xyz * float3(-1.0)), other.e1234);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(right_anti_dual_g0[0] * this.scalar, (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (other.e45 * this.e45)), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g1 * float4(this.scalar), 
            /* e5 */
            other.e3215 * this.scalar, 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.scalar)) + (float4(right_anti_dual_g0[0]) * float4(this.group2.xyz, this.e45)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g0[0]) * this.group0.xyz) + (float3(this.scalar) * other.group7.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.scalar)) + (float3(right_anti_dual_g0[0]) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g6_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e45) * right_anti_dual_g1.xyz)), other.e45 * this.scalar), 
            /* e423, e431, e412 */
            float3((right_anti_dual_g1[2] * this.e42) - (right_anti_dual_g1[1] * this.e43), (right_anti_dual_g1[0] * this.e43) - (right_anti_dual_g1[2] * this.e41), (right_anti_dual_g1[1] * this.e41) - (right_anti_dual_g1[0] * this.e42)) + (right_anti_dual_g7 * float3(this.scalar)) + (float3(right_anti_dual_g1[3]) * this.group1.xyz), 
            /* e235, e315, e125 */
            float3((right_anti_dual_g1[1] * this.e35) - (right_anti_dual_g1[2] * this.e25), (right_anti_dual_g1[2] * this.e15) - (right_anti_dual_g1[0] * this.e35), (right_anti_dual_g1[0] * this.e25) - (right_anti_dual_g1[1] * this.e15)) + (right_anti_dual_g8 * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g3[2] * this.e42) - (right_anti_dual_g3[1] * this.e43), (right_anti_dual_g3[0] * this.e43) - (right_anti_dual_g3[2] * this.e41), (right_anti_dual_g3[1] * this.e41) - (right_anti_dual_g3[0] * this.e42)) + (right_anti_dual_g5 * float3(this.e45)) + (float3(right_anti_dual_g0[0]) * this.group3.xyz) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz) + (other.group7.yzx * this.group2.zxy) - (other.group7.zxy * this.group2.yzx)), right_anti_dual_g0[0] * this.e3215), 
            /* e1234 */
            (right_anti_dual_g0[0] * this.e1234) - (right_anti_dual_g5[0] * this.e41) - (right_anti_dual_g5[1] * this.e42) - (right_anti_dual_g5[2] * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e4 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven weight_expansion(Plane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((other.group0.yz * this.group0.zx) - (other.group0.zx * this.group0.yz)), (other.e4235 * this.e42) - (other.e4315 * this.e41), other.e4235 * this.e4235 * -1.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e45) * other.group0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3((other.e4125 * this.e25) - (other.e4315 * this.e35), (other.e4235 * this.e35) - (other.e4125 * this.e15), (other.e4315 * this.e15) - (other.e4235 * this.e25)) + (float3(other.e3215) * this.group1.xyz)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float4(this.scalar).xyz * other.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightExpansionInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public Sphere weight_expansion(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float4(this.scalar).xyz * other.group0.xyz), this.scalar * other.e5 * -1.0), 
            /* e1234 */
            other.e4 * this.scalar * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public AntiScalar weight_expansion(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven weight_expansion(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(0.0, 0.0, right_anti_dual_g0[0] * this.e42 * -1.0, 0.0) + (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e4235)) + (right_anti_dual_g0.wwwy * float4(this.group1.xyz, this.e4315)) + float4(-(right_anti_dual_g0.yz * this.group0.zx), 0.0, 0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.e45) * right_anti_dual_g0.xyz)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3((right_anti_dual_g0[1] * this.e35) - (right_anti_dual_g0[2] * this.e25), (right_anti_dual_g0[2] * this.e15) - (right_anti_dual_g0[0] * this.e35), (right_anti_dual_g0[0] * this.e25) - (right_anti_dual_g0[1] * this.e15)) + (float3(other.e3215) * this.group1.xyz)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightExpansionInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_expansion(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e12345 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_anti_dual_g0_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group0.xyz)), right_anti_dual_g0_w * this.scalar), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group1), 
            /* e15, e25, e35, e1234 */
            (right_anti_dual_g2 * float4(this.scalar)) + (float4(right_anti_dual_g0_w) * this.group2) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e23) - (right_anti_dual_g0_xyz[1] * this.e31) - (right_anti_dual_g0_xyz[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3((right_anti_dual_g2[2] * this.e42) - (right_anti_dual_g2[1] * this.e43), (right_anti_dual_g2[0] * this.e43) - (right_anti_dual_g2[2] * this.e41), (right_anti_dual_g2[1] * this.e41) - (right_anti_dual_g2[0] * this.e42)) + (float3(right_anti_dual_g0_w) * this.group3.xyz) + (float3(right_anti_dual_g1[3]) * this.group1.xyz) + (float3(this.scalar) * other.group3.xyz) + (float3(this.e45) * right_anti_dual_g1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group2.zxy) - (right_anti_dual_g0_xyz.zxy * this.group2.yzx)), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightExpansionInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.weight_expansion(rhs);
}
public extension VersorOdd {
    public VersorEven weight_expansion(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (right_anti_dual_g0 * float4(this.scalar)) + (right_anti_dual_g3.zxyx * float4(this.group0.yzx, this.e4235)) + (this.group1.xyzx * float4(float3(right_anti_dual_g3[3]), other.e23)) + float4(-(right_anti_dual_g3.yz * this.group0.zx), right_anti_dual_g3[0] * this.e42 * -1.0, -(right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (other.e45 * this.e45)), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g3[3]) * this.group2.xyz) + (float3(other.e3215) * this.group0.xyz) - (float3(this.scalar) * other.group1.xyz) - (float3(this.e45) * right_anti_dual_g3.xyz)), other.e45 * this.scalar), 
            /* e235, e315, e125, e5 */
            float4((float3((right_anti_dual_g3[1] * this.e35) - (right_anti_dual_g3[2] * this.e25), (right_anti_dual_g3[2] * this.e15) - (right_anti_dual_g3[0] * this.e35), (right_anti_dual_g3[0] * this.e25) - (right_anti_dual_g3[1] * this.e15)) + (right_anti_dual_g2_xyz * float3(this.scalar)) + (float3(other.e3215) * this.group1.xyz)), other.e3215 * this.scalar), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g3 * float4(this.scalar)
        );
    }
}
