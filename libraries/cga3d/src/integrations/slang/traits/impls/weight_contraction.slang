implementing cga3d;
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (other.e45 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiDualNum other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g1_w * this.e45) + (right_anti_dual_g1_xyz[0] * this.e15) + (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(AntiLine other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(AntiMotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.scalar * other.scalar))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g0_w * this.e45) + (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane weight_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar weight_contraction(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane weight_contraction(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[1] * this.scalar) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g9_xyz[1] * this.e42) - (right_anti_dual_g9_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group1.zxy), right_anti_dual_g9_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g9_w * this.e45) + (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[1]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g0_w * this.e45) + (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(Scalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.scalar * other.scalar) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group2.xyz), (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435) + (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412) + (other.scalar * this.e4)), 
            /* e1, e2, e3, e5 */
            (float4(other.scalar) * this.group3) + (float4(other.e42, other.e43, other.e41, other.e23) * this.group2.zxyx) + (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) + (other.group1.wwwy * float4(this.group1.xyz, this.e315)) + float4(float3(0.0), (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group1.xyz) + (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0])), 
            /* e15, e25, e35, scalar */
            (this.group1 * float4(float3(right_anti_dual_g3_w), other.e321)) + (this.group2.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g3_w)) + float4(float3(0.0), (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) - (this.group2.zxyx * float4(right_anti_dual_g3_xyz.yzx, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiDualNum other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g1_xyz[0])), 
            /* e15, e25, e35, scalar */
            (this.group1 * float4(float3(right_anti_dual_g1_w), other.e321)) + (this.group2.yzxw * float4(right_anti_dual_g1_xyz.zxy, right_anti_dual_g1_w)) + float4(float3(0.0), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) - float4((right_anti_dual_g1_xyz.yzx * this.group2.zxy), right_anti_dual_g0_xyz[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((right_anti_dual_g0 * float3(this.e321)), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), right_anti_dual_g0[0] * this.e423), 
            /* e5 */
            (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_anti_dual_g0[3]) * this.group2.xyz), (right_anti_dual_g0[3] * this.e4) - (right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g0 * float4(float3(this.e321), this.e5)) + (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx) + float4(float3(0.0), (this.e425 * other.e25) + (this.e435 * other.e35) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), right_anti_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual_g0_w) * float4(this.group1.xyz, this.e4)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)) + float4((right_anti_dual_g0_xyz.zxy * this.group2.yzx), right_anti_dual_g0_xyz[0] * this.e1) - float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((other.group0.yzx * this.group2.zxy), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint weight_contraction(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((other.group0.yzx * this.group2.zxy), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor weight_contraction(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e5 * -1.0) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane weight_contraction(FlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane weight_contraction(Flector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar weight_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual_g1_w) * this.group0.xyz), (right_anti_dual_g1_w * this.e4) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual_g1_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g9_w * this.e4) + (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) + (this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g0[1]) * float4(this.e1, this.e2, this.e3, this.e4)) + (float4(other.e45, other.e45, other.e45, other.e41) * this.group1.xyzx) + float4(float3(0.0), (this.e412 * other.e12) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((right_anti_dual_g8.yzx * this.group0.zxy), this.e423 * other.e23) + float4((other.group4.yzx * this.group2.zxy), this.e431 * other.e31) - float4((float3(this.e321) * other.group5.xyz), 0.0) - float4((right_anti_dual_g8.zxy * this.group0.yzx), 0.0) - float4((other.group4.zxy * this.group2.yzx), 0.0), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g9_xyz[1] * this.e425) - (right_anti_dual_g9_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g9_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group2.zxy), right_anti_dual_g9_xyz[0] * this.e415), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group1.xyz) + (right_anti_dual_g9_xyz.yzx * this.group0.zxy) - (right_anti_dual_g9_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9_w) * this.group0.xyz) - (right_anti_dual_g9_xyz * float3(this.e321)), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35, scalar */
            (float4(right_anti_dual_g0_w) * float4(this.group1.xyz, this.e4)) + float4(float3(0.0), (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)) + float4((right_anti_dual_g0_xyz.zxy * this.group2.yzx), right_anti_dual_g1 * this.e5) - float4((right_anti_dual_g0_xyz.yzx * this.group2.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(Scalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group1.xyz) + (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0])), 
            /* e15, e25, e35, scalar */
            (this.group1 * float4(float3(right_anti_dual_g3_w), other.e321)) + (this.group2.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g3_w)) + float4(float3(0.0), (right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412)) - (this.group2.zxyx * float4(right_anti_dual_g3_xyz.yzx, right_anti_dual_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e423 * other.e23) + (this.e431 * other.e31) + (this.e412 * other.e12) + (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43) + (this.e4 * other.scalar)), 
            /* e1, e2, e3, e5 */
            (float4(other.scalar) * this.group3) + (this.group2.zxyy * float4(other.group0.yzx, other.e31)) + (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e435) * -1.0) + float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), this.e125 * other.e12) - (this.group1.wwwx * float4(other.group1.xyz, right_anti_dual_g2_xyz[0])) - float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), right_anti_dual_g2_xyz[1] * this.e425) - float4((this.group2.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), other.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4((other.group1.xyz * float3(-1.0)), other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(AntiDipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4((other.group3.xyz * float3(-1.0)), other.e4), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiDualNum weight_contraction(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public DualNum weight_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e321, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public FlatPoint weight_contraction(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual_g0[3] * this.scalar), 
            /* e15, e25, e35, e3215 */
            right_anti_dual_g0 * float4(this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint weight_contraction(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiPlane weight_contraction(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiPlane weight_contraction(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiLine weight_contraction(Dipole other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215 * -1.0) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e3215 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e3215 * -1.0) * other.group7.xyz), 0.0), 
            /* e5 */
            this.e3215 * other.e321, 
            /* e15, e25, e35, e45 */
            float4((float3(this.e3215 * -1.0) * other.group5.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215 * -1.0) * other.group4.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e4), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215 * -1.0) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[1] * this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint weight_contraction(RoundPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e4 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightContractionInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiDualNum weight_contraction(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public Scalar weight_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector weight_contraction(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor weight_contraction(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), (this.e3215 * other.e1234) + (this.scalar * other.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4((other.group1.xyz * float3(-1.0)), other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e321 * this.e321) - (other.e412 * this.e125)) - (this.group0.xyzy * float4(float3(other.e4), other.e431)) - (this.group0.wwwx * float4(right_anti_dual_g3_xyz, other.e423)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_contraction(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(AntiFlector other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4((right_anti_dual_g1.xyz * float3(-1.0)), other.e321), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1.zxy * this.group0.yzx) - (right_anti_dual_g1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0 * float3(this.e321)), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(this.e321 * -1.0) * other.group0.xyz), (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiLine weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiLine(
            /* e23, e31, e12 */
            right_anti_dual_g0_xyz * float3(this.e321 * -1.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar weight_contraction(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g6_xyz * float3(this.e321)), 0.0) + float4((right_anti_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g6_xyz[0] * this.e235) - (right_anti_dual_g6_xyz[1] * this.e315) - (right_anti_dual_g6_xyz[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g9.zxy * this.group0.yzx) - (right_anti_dual_g9.yzx * this.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            -(float3(this.e321) * right_anti_dual_g9.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g0[1] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiLine weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiLine(
            /* e23, e31, e12 */
            -(right_anti_dual_g0_xyz * float3(this.e321)) - (float3(other.e4) * this.group0.xyz), 
            /* e15, e25, e35 */
            (right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint weight_contraction(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e321 * other.e321) - (right_anti_dual_g0_xyz[2] * this.e125)) - (this.group0.xyzy * float4(float3(other.e4), right_anti_dual_g0_xyz[1])) - (this.group0.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector weight_contraction(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            (this.group0.zxyx * float4(other.group0.yzx, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiFlector> operator /(AntiFlector lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            (float4(other.scalar) * this.group1) + (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) + (other.e321 * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - (this.group0.wwwx * float4(right_anti_dual_g3_xyz, other.e423)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public Scalar weight_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g1_xyz * float3(this.e321 * -1.0)), (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) + (other.e321 * this.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1_xyz.zxy * this.group0.yzx) - (right_anti_dual_g1_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_anti_dual_g0 * float3(this.e321)), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g0[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g0 * float4(float3(this.e321), this.e5)) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g0_xyz * float3(this.e321 * -1.0)), (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public Scalar weight_contraction(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public Scalar weight_contraction(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (this.group0.yzxx * float4(right_anti_dual_g0.zxy, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) + (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g6_xyz * float3(this.e321)), 0.0) + float4((float3(right_anti_dual_g0[1]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) - (right_anti_dual_g6_xyz[0] * this.e235) - (right_anti_dual_g6_xyz[1] * this.e315) - (right_anti_dual_g6_xyz[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g9_xyz.zxy * this.group0.yzx) - (right_anti_dual_g9_xyz.yzx * this.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g10) * this.group0.xyz) - (right_anti_dual_g9_xyz * float3(this.e321)), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g0[1] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - float4((right_anti_dual_g0_xyz * float3(this.e321)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g0_xyz.zxy * this.group0.yzx) - (right_anti_dual_g0_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_contraction(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) + (this.e321 * other.e321) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125)) - (float4(other.e4) * float4(this.group0.xyz, this.e5)) - (this.group0.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g3_xyz.zxy * this.group0.yzx) - (right_anti_dual_g3_xyz.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector weight_contraction(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            (float4(other.scalar) * this.group1) + (this.group0.zxyx * float4(other.group0.yzx, other.e23)) + float4(float3(0.0), (this.e315 * other.e31) + (this.e125 * other.e12)) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiLine> operator /(AntiLine lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual_g3_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group0.yzx), right_anti_dual_g3_xyz[0] * this.e15) - float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiLine weight_contraction(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35)) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), right_anti_dual_g1_xyz[0] * this.e15) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public Scalar weight_contraction(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor weight_contraction(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public Scalar weight_contraction(Dipole other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public Scalar weight_contraction(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group0.yzx), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35), 
            /* e15, e25, e35, e45 */
            float4((float3(right_anti_dual_g0[1]) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual_g0_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_anti_dual_g0_xyz[1] * this.e25) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiLine weight_contraction(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_anti_dual_g3_xyz[2] * this.e35) + float4((float3(other.e4) * this.group1.xyz), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group0.yzx), right_anti_dual_g3_xyz[0] * this.e15) - float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor weight_contraction(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiMotor> operator /(AntiMotor lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.scalar) * this.group0) + float4(float3(0.0), (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((right_anti_dual_g0 * float3(this.e3215)), other.e23 * this.e23), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.scalar) * this.group1.xyz) - (float3(this.e3215) * other.group1.xyz)), other.scalar * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiDipoleInversion other) {
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(right_anti_dual_g3_xyz, right_anti_dual_g2_w), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) + (other.e321 * this.e3215)) + float4((right_anti_dual_g3_xyz.zxy * this.group0.yzx), right_anti_dual_g3_xyz[0] * this.e15) - float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public DualNum weight_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * this.e3215, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual_g1_xyz * float3(this.e3215 * -1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35) + (other.e321 * this.e3215)) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), right_anti_dual_g1_xyz[0] * this.e15) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((right_anti_dual_g0 * float3(this.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.scalar) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz)), other.scalar * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_anti_dual_g0_xyz * float3(this.e3215 * -1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiPlane weight_contraction(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiPlane weight_contraction(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g0 * float3(this.e3215)), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g0 * float3(this.e3215)), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g9_xyz.zxy * this.group0.yzx), 0.0) - float4((float3(right_anti_dual_g10) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35) + (this.e3215 * other.e321), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g6_xyz * float3(this.e3215)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g10 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_anti_dual_g9_xyz * float3(this.e3215 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g0[1] * this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(right_anti_dual_g0_xyz, right_anti_dual_g1), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((float3(right_anti_dual_g1) * this.group1.xyz), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public Scalar weight_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector weight_contraction(VersorEven other) {
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(right_anti_dual_g3_xyz, right_anti_dual_g2_w), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) + (this.e3215 * other.e321)) + float4((right_anti_dual_g3_xyz.zxy * this.group0.yzx), right_anti_dual_g3_xyz[0] * this.e15) - float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.scalar)) + (this.group0.xyzx * float4(float3(right_anti_dual_g0[3]), other.e23)) + float4(float3(0.0), (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e3215 * other.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) - (float3(this.e3215) * other.group1.xyz)), right_anti_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiPlane> operator /(AntiPlane lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_contraction(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public Scalar weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (this.e5 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g0[1]) * this.group0.xyz), 0.0), 
            /* e5 */
            right_anti_dual_g0[1] * this.e5, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public Scalar weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_contraction(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public Scalar weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane weight_contraction(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<AntiScalar> operator /(AntiScalar lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public CircleRotor weight_contraction(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public DipoleInversion weight_contraction(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public DualNum weight_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Flector weight_contraction(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Line weight_contraction(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Motor weight_contraction(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Plane weight_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Dipole weight_contraction(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Circle weight_contraction(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightContractionInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiDualNum weight_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiFlatPoint weight_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiFlector weight_contraction(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiLine weight_contraction(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiMotor weight_contraction(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public MultiVector weight_contraction(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e12345) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group9.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.e12345 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e12345 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightContractionInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public Sphere weight_contraction(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e12345 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WeightContractionInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public AntiScalar weight_contraction(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public RoundPoint weight_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public VersorOdd weight_contraction(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension AntiScalar {
    public VersorEven weight_contraction(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * float4((other.group2.xyz * float3(-1.0)), other.e3215), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Circle> operator /(Circle lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group2.xyz), (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435) + (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) + (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4(float3(0.0), (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) + float4((other.group0.yzx * this.group2.zxy), other.e31 * this.e315) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group1.xyz) + (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0])), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(right_anti_dual_g3_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group2.zxy), other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Circle weight_contraction(AntiDualNum other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Scalar weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g1_xyz[0])), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) + float4((float3(right_anti_dual_g1_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group2.zxy), other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public RoundPoint weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((right_anti_dual_g0 * float3(this.e321)), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), right_anti_dual_g0[0] * this.e423), 
            /* e5 */
            (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(AntiMotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group2.xyz), (other.e23 * this.e423) + (other.e31 * this.e431) + (other.e12 * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e321) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Dipole weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0_w) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group2.yzx) - (right_anti_dual_g0_xyz.yzx * this.group2.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Scalar weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            (other.e321 * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Scalar weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public RoundPoint weight_contraction(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public RoundPoint weight_contraction(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e423)) + float4(float3(0.0), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((this.group0.yzx * other.group2.zxy), this.e431 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e412 * other.e12) - float4((float3(this.e321) * other.group1.xyz), 0.0) - float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            (this.e415 * other.e15) + (this.e425 * other.e25) + (this.e435 * other.e35) + (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiLine weight_contraction(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e5 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e5 * -1.0) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiPlane weight_contraction(FlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiPlane weight_contraction(Flector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx) + (this.group1.xyzy * other.group0.wwwy) + float4(float3(0.0), this.e435 * other.e35) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Scalar weight_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiMotor weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_anti_dual_g1_w) * this.group0.xyz), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(right_anti_dual_g1_w) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e321 * other.e321) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e45, other.e45, other.e45, other.e41) * this.group1.xyzx) + float4(float3(0.0), (this.e412 * other.e12) + (this.e425 * other.e42) + (this.e435 * other.e43)) + float4((right_anti_dual_g8.yzx * this.group0.zxy), this.e423 * other.e23) + float4((this.group2.zxy * other.group4.yzx), this.e431 * other.e31) - float4((float3(this.e321) * other.group5.xyz), 0.0) - float4((right_anti_dual_g8.zxy * this.group0.yzx), 0.0) - float4((this.group2.yzx * other.group4.zxy), 0.0), 
            /* e5 */
            (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g9_xyz[1] * this.e425) - (right_anti_dual_g9_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g9_w) * this.group1.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group2.zxy), right_anti_dual_g9_xyz[0] * this.e415), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group1.xyz) + (right_anti_dual_g9_xyz.yzx * this.group0.zxy) - (right_anti_dual_g9_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9_w) * this.group0.xyz) - (right_anti_dual_g9_xyz * float3(this.e321)), 
            /* e415, e425, e435, e321 */
            float4(right_anti_dual_g0[1]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Dipole weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * this.group1.xyz) + (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g0_xyz[0])), 
            /* e15, e25, e35 */
            (float3(right_anti_dual_g0_w) * this.group1.xyz) + (right_anti_dual_g0_xyz.zxy * this.group2.yzx) - (right_anti_dual_g0_xyz.yzx * this.group2.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public Circle weight_contraction(Scalar other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group1.xyz) + (right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) - (this.group1.wwwx * float4(right_anti_dual_g3_xyz, right_anti_dual_g3_xyz[0])), 
            /* e15, e25, e35, scalar */
            (this.group1 * float4(float3(right_anti_dual_g3_w), other.e321)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412)) + float4((right_anti_dual_g3_xyz.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group2.zxy), right_anti_dual_g0_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e423 * other.e23) + (this.e431 * other.e31) + (this.e412 * other.e12) + (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)), 
            /* e1, e2, e3, e5 */
            (other.group1.wwwx * float4(this.group1.xyz, this.e235)) + float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), this.e315 * other.e31) + float4((this.group2.zxy * other.group0.yzx), this.e125 * other.e12) - (this.group1.wwwx * float4(other.group1.xyz, right_anti_dual_g2_xyz[0])) - float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), right_anti_dual_g2_xyz[1] * this.e425) - float4((this.group2.yzx * other.group0.zxy), right_anti_dual_g2_xyz[2] * this.e435)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<CircleRotor> operator /(CircleRotor lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(this.e12345)) + (float3(other.scalar) * this.group0.xyz)), other.scalar * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            (other.group2.wwwx * float4(this.group2.xyz, this.e415)) + float4(float3(0.0), (other.e25 * this.e425) + (other.e35 * this.e435) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) - (this.group2.wwwx * float4(other.group2.xyz, right_anti_dual_g1[0])), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group0.zxy * other.group2.yzx), right_anti_dual_g0[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual_g3.yzx), 0.0) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g1_w * this.e321) * float4(this.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (this.group2 * float4(float3(right_anti_dual_g2_w), right_anti_dual_g1_w)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((right_anti_dual_g1_xyz * float3(this.e12345)), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.e12345)) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[1] * this.e235) - (right_anti_dual_g3[0] * this.e315)) + float3((right_anti_dual_g3.zx * this.group2.yz), 0.0) - float3((right_anti_dual_g3.yz * this.group2.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * other.group0.xy.yyyx, 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g0_w * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g0_xyz * float3(this.e12345)), -(right_anti_dual_g0_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) + float4((this.group0.zxy * right_anti_dual_g1.yzx), 0.0) - (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0_w * this.e321) * float4(this.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), right_anti_dual_g0_w * this.e12345) - (right_anti_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g0_xyz * float3(this.e12345)), 0.0) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * float3(this.e12345)), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g1 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((right_anti_dual_g0 * float3(this.e321)), 0.0) + float4((right_anti_dual_g1.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven weight_contraction(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) - (float3(this.e12345) * other.group0.xyz)), other.scalar * this.e321), 
            /* e235, e315, e125, e5 */
            (this.group2 * float4(float3(other.scalar), other.e3215)) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)) - float4((float3(this.e12345) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), (other.e31 * this.e431) + (other.e12 * this.e412)) + float4((this.group0.yzx * other.group1.zxy), other.e23 * this.e423) - float4((float3(this.e321) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public DipoleInversion weight_contraction(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g2_xyz * float3(this.e12345)), -(right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g2[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345 * -1.0) * other.group2.xyz), (this.e12345 * other.e1234) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other.e15) * this.group1.wwwx) + (this.group1.xyzy * float4(float3(right_anti_dual_g1[3]), other.e25)) + (this.group2.yzxw * float4(right_anti_dual_g0.zxy, other.e3215)) + (other.group2.zxyz * float4(this.group0.yzx, this.e435)) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - (this.group2.wwwy * float4(other.group3.xyz, right_anti_dual_g1[1])) - float4((this.group0.zxy * other.group2.yzx), right_anti_dual_g1[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor weight_contraction(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), this.e12345 * other.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e5 * -1.0) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiFlector weight_contraction(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual_g0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * right_anti_dual_g0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiFlector weight_contraction(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual_g0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * right_anti_dual_g0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor weight_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor weight_contraction(Motor other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (right_anti_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e12345)) + (float3(right_anti_dual_g1_w) * this.group1.xyz)), right_anti_dual_g1_w * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * this.e12345) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), right_anti_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            (this.group2.yzxw * float4(right_anti_dual_g7.zxy, other.e1234)) + float4(float3(0.0), -(right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[0] * this.e423) - (right_anti_dual_g6[1] * this.e431) - (right_anti_dual_g6[2] * this.e412)) + float4((float3(right_anti_dual_g6[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g8.yzx * this.group0.zxy), 0.0) - float4((float3(this.e12345) * other.group9.xyz), 0.0) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g7[0] * this.e415) - float4((right_anti_dual_g8.zxy * this.group0.yzx), right_anti_dual_g7[1] * this.e425), 
            /* e5 */
            (this.e12345 * other.e3215) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + float4((float3(right_anti_dual_g9[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g9.zxy * this.group2.yzx), 0.0) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (this.group0.zxy * right_anti_dual_g9.yzx) - (this.group0.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.e12345)) + (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group1), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public DipoleInversion weight_contraction(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_anti_dual_g0.yzx) - (this.group0.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (right_anti_dual_g0.zxy * this.group2.yzx) + float3(float2(0.0), right_anti_dual_g0[0] * this.e315 * -1.0) - float3((right_anti_dual_g0.yz * this.group2.zx), 0.0)), right_anti_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WeightContractionInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public CircleRotor weight_contraction(Scalar other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4(other.scalar) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public RoundPoint weight_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd weight_contraction(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125)) + float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0) + float4((this.group0.zxy * right_anti_dual_g3.yzx), 0.0) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g1_w * this.e321) * float4(this.group0.yzx, 1.0)), 
            /* e23, e31, e12, e45 */
            (this.group2 * float4(float3(right_anti_dual_g2_w), right_anti_dual_g1_w)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((right_anti_dual_g1_xyz * float3(this.e12345)), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.e12345)) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[1] * this.e235) - (right_anti_dual_g3[0] * this.e315)) + float3((right_anti_dual_g3.zx * this.group2.yz), 0.0) - float3((right_anti_dual_g3.yz * this.group2.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz)), this.e12345 * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            (this.group2 * float4(float3(other.scalar), other.e3215)) + float4(float3(0.0), -(right_anti_dual_g2_xyz[0] * this.e415) - (right_anti_dual_g2_xyz[1] * this.e425) - (right_anti_dual_g2_xyz[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((right_anti_dual_g2_xyz * float3(this.e12345)), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other.e41) * this.group1.wwwx) + (this.group1.xyzy * float4(float3(right_anti_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group2.zxy, this.e435)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((right_anti_dual_g2_xyz.yzx * this.group0.zxy), this.e12345 * other.e1234) - float4((float3(this.e12345) * other.group3.xyz), 0.0) - float4((right_anti_dual_g2_xyz.zxy * this.group0.yzx), right_anti_dual_g1[0] * this.e423) - float4((this.group2.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Dipole> operator /(Dipole lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Dipole weight_contraction(AntiDualNum other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g1_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g1_w * this.e45) + (right_anti_dual_g1_xyz[0] * this.e15) + (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Scalar weight_contraction(AntiLine other) {
        return Scalar(
            /* scalar */
            (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(AntiMotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public RoundPoint weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g0_w * this.e45) + (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Scalar weight_contraction(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Scalar weight_contraction(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public AntiPlane weight_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Scalar weight_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Scalar weight_contraction(Flector other) {
        return Scalar(
            /* scalar */
            (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) - (this.e45 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public AntiPlane weight_contraction(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g9_xyz[1] * this.e42) - (right_anti_dual_g9_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g9_xyz.yzx * this.group1.zxy), right_anti_dual_g9_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g9_w * this.e45) + (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35), 
            /* e15, e25, e35, e45 */
            float4(right_anti_dual_g0[1]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g0[1]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0[1]) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public RoundPoint weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g0_w * this.e45) + (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public Dipole weight_contraction(Scalar other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public RoundPoint weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(other.e4) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g3_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g3_xyz[0] * this.e41), 
            /* e5 */
            (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group2.xyz), (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((float3(right_anti_dual_g2[3]) * this.group0.xyz), other.e41 * this.e15) + float4((other.group0.yzx * this.group3.zxy), other.e42 * this.e25) - (other.group1 * float4(float3(this.e1234), this.e45)) - float4((other.group0.zxy * this.group3.yzx), right_anti_dual_g2[0] * this.e41), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((float3(right_anti_dual_g2[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2[3]) * this.group2.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) + float3(float2(0.0), right_anti_dual_g2[1] * this.e4235 * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((right_anti_dual_g2.zx * this.group3.yz), 0.0)), right_anti_dual_g2[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g2[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group3.zxy), right_anti_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), other.e423)) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_anti_dual_g3_xyz[0] * this.e41) - (right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g2_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_anti_dual_g1_xyz.zxy * this.group3.yzx), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), right_anti_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1_w)) - (this.group3.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g2_xyz[1] * this.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            other.group0.xy.yyyx * float4(this.group0.xyz, this.e1234), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g1_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group3.zxy) - (right_anti_dual_g1_xyz.zxy * this.group3.yzx)), right_anti_dual_g1_w * this.e1234), 
            /* e235, e315, e125, e4 */
            float4((float2(right_anti_dual_g1_w) * this.group3.xy), right_anti_dual_g1_w * this.e4125, right_anti_dual_g0_w * this.e1234) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)) - float4((right_anti_dual_g1_xyz * float3(this.e3215)), right_anti_dual_g1_xyz[0] * this.e41), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g1_xyz[0])) + float4(float3(0.0), (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g0_xyz[0] * this.e4235) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125)) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), right_anti_dual_g1_xyz[1] * this.e25) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(this.e1234)), -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((right_anti_dual_g0 * float3(this.e3215)), 0.0) + float4((right_anti_dual_g1.yzx * this.group3.zxy), 0.0) - float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e41, this.e42, this.e43, this.e23) * other.group0.wwwx) + float4(float3(0.0), (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234)) - float4((float3(this.e1234) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (other.e25 * this.e4235) - (other.e15 * this.e4315)) + float3((other.group1.zx * this.group3.yz), 0.0) - (float3(this.e3215) * other.group0.xyz) - float3((other.group1.yz * this.group3.zx), 0.0)), other.scalar * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group3.zxy) - (right_anti_dual_g0_xyz.zxy * this.group3.yzx)), right_anti_dual_g0_w * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, right_anti_dual_g1_w)) + (this.group3.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (this.group3.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_anti_dual_g2 * float3(this.e1234)), other.e23 * this.e4235) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((right_anti_dual_g2.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g2[0] * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx), 
            /* e23, e31, e12, e45 */
            float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235) - (other.e15 * this.e1234), -(other.e42 * this.e3215) - (other.e45 * this.e4315) - (other.e25 * this.e1234), -(other.e43 * this.e3215) - (other.e45 * this.e4125) - (other.e35 * this.e1234), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)), 
            /* e15, e25, e35, scalar */
            (other.group2.zxyx * float4(this.group3.yzx, this.e41)) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group3.zxyx * float4(other.group2.yzx, other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiFlector weight_contraction(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiMotor weight_contraction(FlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35)) - (other.group0 * float4(float3(this.e1234), this.e45)) - float4((float3(other.e45) * this.group3.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiMotor weight_contraction(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (this.e41 * other.e15) + (this.e42 * other.e25) + (this.e43 * other.e35) + (this.e1234 * other.e3215) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group3.xyzx) - (other.group0 * float4(float3(this.e1234), this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiPlane weight_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group3.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiFlector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g1[3]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g3 * float4(this.e1234)) + (this.group3.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4((float3(right_anti_dual_g9_w) * this.group0.xyz), this.e4315 * other.e431) + float4((right_anti_dual_g9_xyz.zxy * this.group1.yzx), this.e4125 * other.e412) - (float4(right_anti_dual_g10) * float4(this.e15, this.e25, this.e35, this.e45)) - float4((float3(this.e3215) * other.group7.xyz), right_anti_dual_g9_xyz[2] * this.e43) - float4((right_anti_dual_g5.yzx * this.group3.zxy), right_anti_dual_g9_xyz[0] * this.e41) - float4((right_anti_dual_g9_xyz.yzx * this.group1.zxy), right_anti_dual_g9_xyz[1] * this.e42), 
            /* e5 */
            (right_anti_dual_g9_w * this.e45) + (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35) - (right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual_g0[1]) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g6_xyz[1] * this.e4315) - (right_anti_dual_g6_xyz[2] * this.e4125)) + float4((right_anti_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_anti_dual_g8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g6_xyz * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz) + (right_anti_dual_g7.zxy * this.group3.yzx) - (right_anti_dual_g7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz) - (float3(other.e45) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g9_xyz.yzx * this.group3.zxy), right_anti_dual_g9_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g9_xyz.zxy, right_anti_dual_g10)), 
            /* e423, e431, e412 */
            (right_anti_dual_g9_xyz * float3(this.e1234)) - (float3(right_anti_dual_g10) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g9_w) * this.group3.xyz) - (right_anti_dual_g9_xyz * float3(this.e3215)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[1]) * this.group3, 
            /* e1234 */
            right_anti_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public Scalar weight_contraction(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g0_xyz * float3(this.e1234)) - (float3(right_anti_dual_g1) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0_xyz.yzx * this.group3.zxy), right_anti_dual_g0_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), right_anti_dual_g1 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public DipoleInversion weight_contraction(Scalar other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public Scalar weight_contraction(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group3.zxy), right_anti_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g0_xyz[0])) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) - (right_anti_dual_g3_xyz[0] * this.e41) - (right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g2_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_anti_dual_g1_xyz.zxy * this.group3.yzx), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), right_anti_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(right_anti_dual_g0_xyz, right_anti_dual_g1_w)) - (this.group3.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g2_xyz[1] * this.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]) * this.group3.yzxx) + float4(float3(0.0), (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12) + (this.e1234 * other.e3215) + (this.e3215 * other.e1234) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), right_anti_dual_g3_xyz[1] * this.e4315) - (other.group1 * float4(float3(this.e1234), this.e45)) - (right_anti_dual_g0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (this.e4315 * other.e31) + (this.e4125 * other.e12)) + float4((right_anti_dual_g2_xyz * float3(this.e1234)), this.e4235 * other.e23) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (right_anti_dual_g2_xyz.yzx * this.group3.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_anti_dual_g2_xyz.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<DualNum> operator /(DualNum lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            right_anti_dual_g2 * float4(float3(this.e12345), this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(this.e12345) * other.group0.xyz), right_anti_dual_g2[3] * this.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            right_anti_dual_g2 * float4(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public DualNum weight_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e3215 * this.e12345) + (other.scalar * this.e5), other.scalar * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Flector weight_contraction(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightContractionInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Line weight_contraction(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Motor weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e235, e315, e125, e5 */
            float4((float2(this.e12345 * -1.0) * other.group1.xy), other.e35 * this.e12345 * -1.0, (right_anti_dual_g0[3] * this.e5) + (other.e3215 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightContractionInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Plane weight_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Dipole weight_contraction(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public Circle weight_contraction(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WeightContractionInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiDualNum weight_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345 * -1.0) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiFlatPoint weight_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiFlector weight_contraction(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiLine weight_contraction(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiMotor weight_contraction(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345), right_anti_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group9.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) + (this.e12345 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group8.xyz, other.e321) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * float4((other.group5.xyz * float3(-1.0)), other.e45), 
            /* e423, e431, e412 */
            float3(this.e12345 * -1.0) * other.group4.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group3.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            right_anti_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public VersorOdd weight_contraction(RoundPoint other) {
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), right_anti_dual_g1 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_anti_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public DualNum weight_contraction(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public RoundPoint weight_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public VersorOdd weight_contraction(VersorEven other) {
        float4 right_anti_dual_g2 = float4(other.group2.xyz, other.e4 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(this.e12345) * other.group0.xyz), (right_anti_dual_g2[3] * this.e5) - (this.e12345 * other.e12345)), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            right_anti_dual_g2 * float4(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension DualNum {
    public VersorEven weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345 * -1.0) * other.group2.xyz), (right_anti_dual_g0[3] * this.e5) + (this.e12345 * other.e3215)), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group3.xyz * float3(-1.0)), other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<FlatPoint> operator /(FlatPoint lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor weight_contraction(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.scalar * this.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4) * this.group0, 
            /* e5 */
            (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) - (other.e5 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint weight_contraction(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public DualNum weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return DualNum(
            /* e5, e12345 */
            float2((right_anti_dual_g1_xyz[0] * this.e15) + (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35) - (other.e5 * this.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint weight_contraction(AntiMotor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public DualNum weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2((right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35) - (other.e5 * this.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public Scalar weight_contraction(Dipole other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public Scalar weight_contraction(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public DualNum weight_contraction(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e45 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public Scalar weight_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public Scalar weight_contraction(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public DualNum weight_contraction(Motor other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e45 * other.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public MultiVector weight_contraction(MultiVector other) {
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4) * this.group0, 
            /* e5 */
            (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35) - (this.e45 * other.e5), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4) * this.group0, 
            /* e5 */
            (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35) - (this.e45 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint weight_contraction(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4) * this.group0, 
            /* e5 */
            (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) - (this.e45 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor weight_contraction(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * other.scalar), 
            /* e15, e25, e35, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Flector> operator /(Flector lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35)) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e15) - float4((other.group0.zxy * this.group1.yzx), other.e45 * this.e45), 
            /* e23, e31, e12, e45 */
            float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235), -(other.e42 * this.e3215) - (other.e45 * this.e4315), -(other.e43 * this.e3215) - (other.e45 * this.e4125), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125) + (other.scalar * this.e45)), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0) + float4((other.group2.zxy * this.group1.yzx), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((other.group2.yzx * this.group1.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group1.zxy) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), right_anti_dual_g2_w * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(float3(right_anti_dual_g3_w), other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (this.group1.yzxw * float4(right_anti_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[2] * this.e4125)) - (this.group1.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - (this.group1.wwwy * float4(other.group0.xyz, right_anti_dual_g2_xyz[1])) - float4((float3(right_anti_dual_g2_w) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Flector weight_contraction(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public DualNum weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2((other.e321 * this.e3215) - (right_anti_dual_g0_xyz[0] * this.e4235) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Motor weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group1.zxy) - (right_anti_dual_g1_xyz.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (this.group1 * float4(float3(right_anti_dual_g1_w), other.e321)) + float4(float3(0.0), (right_anti_dual_g1_w * this.e45) + (right_anti_dual_g1_xyz[0] * this.e15) + (right_anti_dual_g1_xyz[1] * this.e25) + (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125)) - (this.group1.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public FlatPoint weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + float4((right_anti_dual_g0 * float3(this.e3215)), 0.0) + float4((right_anti_dual_g1.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Flector weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((other.group1.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(other.group1.yzx, right_anti_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Motor weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(other.e5 * -1.0) * float4(this.e4235, this.e4315, this.e4125, this.e45)) + float4(float3(0.0), (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public RoundPoint weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public RoundPoint weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            (other.e321 * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiCircleRotor weight_contraction(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e23, e31, e12, e45 */
            float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235), -(other.e42 * this.e3215) - (other.e45 * this.e4315), -(other.e43 * this.e3215) - (other.e45 * this.e4125), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35)) + float4((right_anti_dual_g2.yzx * this.group1.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_anti_dual_g2.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e23, e31, e12, e45 */
            float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235), -(other.e42 * this.e3215) - (other.e45 * this.e4315), -(other.e43 * this.e3215) - (other.e45 * this.e4125), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)), 
            /* e15, e25, e35, scalar */
            (other.group2.zxyw * this.group1.yzxw) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group1.zxyx * float4(other.group2.yzx, other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiFlector weight_contraction(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * this.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e5 * this.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiMotor weight_contraction(FlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e45 * -1.0) * float4(this.group1.xyz, this.e45), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiMotor weight_contraction(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e45 * -1.0) * this.group1.xyz), -(other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiPlane weight_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiFlector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(right_anti_dual_g1_w) * this.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e4315) - (right_anti_dual_g1_xyz[2] * this.e4125)) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g1_w * this.e45) - (this.group1.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g1_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4(float3(0.0), (this.e4315 * other.e431) + (this.e4125 * other.e412)) - (float4(right_anti_dual_g10) * this.group0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_anti_dual_g5.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            (right_anti_dual_g9_w * this.e45) + (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35) + (this.e3215 * other.e321) - (this.e4235 * other.e235) - (this.e4315 * other.e315) - (this.e4125 * other.e125), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual_g0[1]) * this.group0) + float4(float3(0.0), -(right_anti_dual_g6_xyz[1] * this.e4315) - (right_anti_dual_g6_xyz[2] * this.e4125)) + float4((right_anti_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_anti_dual_g8.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g7.zxy * this.group1.yzx) - (right_anti_dual_g7.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) - (float3(other.e45) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g9_xyz.yzx * this.group1.zxy) - (right_anti_dual_g9_xyz.zxy * this.group1.yzx)), right_anti_dual_g10 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_anti_dual_g10 * -1.0) * this.group1.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g9_w) * this.group1.xyz) - (right_anti_dual_g9_xyz * float3(this.e3215)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[1]) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Scalar weight_contraction(Plane other) {
        return Scalar(
            /* scalar */
            -(this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g1 * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group1.zxy) - (right_anti_dual_g0_xyz.zxy * this.group1.yzx)), right_anti_dual_g1 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual_g0_w) * this.group1.xyz) - (right_anti_dual_g0_xyz * float3(this.e3215))), right_anti_dual_g1 * this.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float2(right_anti_dual_g1 * -1.0) * this.group0.xy), right_anti_dual_g1 * this.e35 * -1.0, (right_anti_dual_g0_w * this.e45) + (right_anti_dual_g0_xyz[0] * this.e15) + (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Flector weight_contraction(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public Scalar weight_contraction(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group1.zxy) - (right_anti_dual_g3_xyz.zxy * this.group1.yzx)), right_anti_dual_g2_w * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g0_xyz[0])) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (this.group1.yzxw * float4(right_anti_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_anti_dual_g3_w * this.e45) + (right_anti_dual_g3_xyz[0] * this.e15) + (right_anti_dual_g3_xyz[1] * this.e25) + (right_anti_dual_g3_xyz[2] * this.e35) - (right_anti_dual_g2_xyz[2] * this.e4125)) - (this.group1.zxyy * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[1])) - (this.group1.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g2_w) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Flector {
    public VersorOdd weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]) * this.group1.yzxx) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (this.e45 * other.e45)) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (this.e4235 * other.e23) + (this.e4315 * other.e31) + (this.e4125 * other.e12)) - float4((float3(other.e45) * this.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group1.zxy), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((right_anti_dual_g2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Line> operator /(Line lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiDipoleInversion weight_contraction(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group1.xyz), (other.e41 * this.e415) + (other.e42 * this.e425) + (other.e43 * this.e435)), 
            /* e1, e2, e3, e5 */
            (float4(this.e415, this.e425, this.e435, this.e235) * other.group1.wwwx) + float4(float3(0.0), (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)) + float4((other.group0.yzx * this.group1.zxy), other.e31 * this.e315) - float4((other.group0.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiCircleRotor weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual_g2_w) * this.group1.xyz), -(right_anti_dual_g3_xyz[0] * this.e415) - (right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightContractionInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Line weight_contraction(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public FlatPoint weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g1_xyz[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public DualNum weight_contraction(AntiLine other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Motor weight_contraction(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group1.xyz), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public FlatPoint weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Scalar weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Scalar weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            -(right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public RoundPoint weight_contraction(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e43 * this.e435) + float4((float3(other.e45) * this.group0.xyz), other.e42 * this.e425) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e415) - float4((other.group0.zxy * this.group1.yzx), 0.0), 
            /* e5 */
            (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public RoundPoint weight_contraction(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e43 * this.e435) + float4((float3(other.e45) * this.group0.xyz), other.e42 * this.e425) + float4((other.group0.yzx * this.group1.zxy), other.e41 * this.e415) - float4((other.group0.zxy * this.group1.yzx), 0.0), 
            /* e5 */
            (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public FlatPoint weight_contraction(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Line> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiPlane weight_contraction(FlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e45) * this.group0.xyz), (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiPlane weight_contraction(Flector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e45) * this.group0.xyz), (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Scalar weight_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiMotor weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e435 * other.e43) + float4((float3(other.e45) * this.group0.xyz), this.e415 * other.e41) + float4((this.group1.zxy * other.group4.yzx), this.e425 * other.e42) - float4((this.group1.yzx * other.group4.zxy), 0.0), 
            /* e5 */
            (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), (right_anti_dual_g9_xyz[2] * this.e435) * -1.0) + float4((right_anti_dual_g9_xyz.zxy * this.group1.yzx), 0.0) - (float4(other.e5, other.e5, other.e5, right_anti_dual_g9_xyz[1] * this.e425) * float4(this.group0.xyz, 1.0)) - float4((right_anti_dual_g9_xyz.yzx * this.group1.zxy), right_anti_dual_g9_xyz[0] * this.e415), 
            /* e41, e42, e43 */
            float3(right_anti_dual_g10) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_anti_dual_g10) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(right_anti_dual_g0[1]) * this.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g0[1]) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Dipole weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_anti_dual_g1) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual_g1) * this.group1.xyz), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435)), 
            /* e15, e25, e35 */
            (float3(other.e5 * -1.0) * this.group0.xyz) + (right_anti_dual_g0_xyz.zxy * this.group1.yzx) - (right_anti_dual_g0_xyz.yzx * this.group1.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightContractionInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public Line weight_contraction(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiCircleRotor weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_anti_dual_g2_w) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_anti_dual_g2_w) * this.group1.xyz), -(right_anti_dual_g3_xyz[0] * this.e415) - (right_anti_dual_g3_xyz[1] * this.e425) - (right_anti_dual_g3_xyz[2] * this.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435)) + float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g0_xyz[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Line {
    public AntiDipoleInversion weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.scalar) * this.group1.xyz), (this.e415 * other.e41) + (this.e425 * other.e42) + (this.e435 * other.e43)), 
            /* e1, e2, e3, e5 */
            (float4(this.e415, this.e425, this.e435, this.e235) * other.group1.wwwx) + float4(float3(0.0), (this.e125 * other.e12) - (right_anti_dual_g2_xyz[1] * this.e425) - (right_anti_dual_g2_xyz[2] * this.e435)) + float4((this.group1.zxy * other.group0.yzx), this.e315 * other.e31) - float4((this.group1.yzx * other.group0.zxy), right_anti_dual_g2_xyz[0] * this.e415)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Motor> operator /(Motor lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_anti_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g2[3]) * this.group0.xyz) - (float3(this.e12345) * other.group1.xyz)), other.e45 * this.e12345), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(float3(this.e12345), this.e5)) + (this.group1.xyzx * float4(float3(right_anti_dual_g2[3]), other.e23)) + float4(float3(0.0), (other.e31 * this.e315) + (other.e12 * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            (this.group0.xyzx * float4(float3(other.e45), other.e41)) + float4(float3(0.0), other.e43 * this.e435) + float4((other.group0.yzx * this.group1.zxy), other.e42 * this.e425) - float4((other.group0.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g2_w) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (right_anti_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), right_anti_dual_g3[0] * this.e315 * -1.0) - float3((right_anti_dual_g3.yz * this.group1.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Motor weight_contraction(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group1.xyz), (other.e3215 * this.e12345) + (other.scalar * this.e5))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public FlatPoint weight_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Flector weight_contraction(AntiFlector other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e321 * this.e12345)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((right_anti_dual_g1.zxy * this.group1.yzx), 0.0) - (right_anti_dual_g1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Motor weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_anti_dual_g0 * float3(this.e12345)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((right_anti_dual_g1 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Motor weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz)), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1) + float4(float3(0.0), (other.e23 * this.e235) + (other.e31 * this.e315) + (other.e12 * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Flector weight_contraction(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiCircleRotor weight_contraction(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345 * -1.0) * other.group2.xyz), (other.e1234 * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            (this.group0 * float4(float3(right_anti_dual_g1[3]), other.e3215)) + float4(float3(0.0), (other.e25 * this.e425) + (other.e35 * this.e435) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((right_anti_dual_g0.zxy * this.group1.yzx), other.e15 * this.e415) - (this.group1.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((float3(this.e12345) * other.group3.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiMotor weight_contraction(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e12345 * this.e12345 * -1.0), 
            /* e15, e25, e35, e3215 */
            float4(other.e5 * -1.0) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiFlector weight_contraction(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiFlector weight_contraction(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            (this.group0 * float4(float3(right_anti_dual_g0[3]), other.e3215)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) - (this.group0.wwwx * float4(other.group1.xyz, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiMotor weight_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiMotor weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_anti_dual_g0_xyz * float3(this.e12345)), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (other.e12345 * this.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_anti_dual_g1_w) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), right_anti_dual_g1_w * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), right_anti_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            (this.group0 * float4(float3(other.e45), other.e1234)) + float4(float3(0.0), (right_anti_dual_g7[2] * this.e435) * -1.0) + float4((right_anti_dual_g7.zxy * this.group1.yzx), 0.0) - (this.group0.wwwx * float4(other.group9.xyz, right_anti_dual_g7[0])) - float4((right_anti_dual_g7.yzx * this.group1.zxy), right_anti_dual_g7[1] * this.e425), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) + (this.e12345 * other.e3215) - (right_anti_dual_g6_xyz[0] * this.e235) - (right_anti_dual_g6_xyz[1] * this.e315) - (right_anti_dual_g6_xyz[2] * this.e125) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435) - (this.e12345 * other.e321)) + float4((float3(right_anti_dual_g9[3]) * this.group0.xyz), 0.0) + float4((float3(this.e12345) * other.group8.xyz), 0.0) + float4((right_anti_dual_g9.zxy * this.group1.yzx), 0.0) - (right_anti_dual_g9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.e12345)) + (float3(right_anti_dual_g10) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g6_xyz * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz)), this.e12345 * other.e45), 
            /* e423, e431, e412 */
            right_anti_dual_g7 * float3(this.e12345), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public VersorOdd weight_contraction(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_anti_dual_g1) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4((float2(right_anti_dual_g1) * this.group1.xy), right_anti_dual_g1 * this.e125, -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) + (right_anti_dual_g0.zxy * this.group1.yzx) + float3(float2(0.0), right_anti_dual_g0[0] * this.e315 * -1.0) - float3((right_anti_dual_g0.yz * this.group1.zx), 0.0)), right_anti_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public Motor weight_contraction(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public RoundPoint weight_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public VersorOdd weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0 * float4(float3(right_anti_dual_g2_w), other.e12345 * -1.0)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((right_anti_dual_g0_xyz * float3(this.e12345)), right_anti_dual_g2_w * this.e5), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g3[0] * this.e415) - (right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + (right_anti_dual_g3.zxy * this.group1.yzx) + float3(float2(0.0), right_anti_dual_g3[0] * this.e315 * -1.0) - float3((right_anti_dual_g3.yz * this.group1.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Motor {
    public VersorEven weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group0.xyz) - (float3(this.e12345) * other.group1.xyz)), this.e12345 * other.e45), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group1) + float4(float3(0.0), (this.e235 * other.e23) + (this.e315 * other.e31) + (this.e125 * other.e12) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            (this.group0 * float4(float3(other.e45), other.e1234)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(this.e12345 * -1.0) * other.group3.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group1.yzx), 0.0) - (right_anti_dual_g0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<MultiVector> operator /(MultiVector lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.scalar * this.scalar) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45), other.scalar * this.e12345), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((float3(right_anti_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group8.yzx), 0.0) + float4((this.group7.yzx * other.group2.zxy), 0.0) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group7.zxy * other.group2.yzx), right_anti_dual_g0[1] * this.e425), 
            /* e5 */
            (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.scalar * this.e5) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((other.group2.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group2.yzx, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(this.e3215)) + (float3(other.scalar) * this.group5.xyz) - (float3(right_anti_dual_g1[3]) * this.group9.xyz) - (float3(this.e1234) * other.group2.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group6), 
            /* e423, e431, e412 */
            (right_anti_dual_g0 * float3(this.e12345)) + (float3(other.scalar) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) - (float3(this.e12345) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], other.e431 * this.e4315) * float4(this.group5.yzx, 1.0)) + (this.group9.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4((float3(right_anti_dual_g3[3]) * this.group4.xyz), other.e412 * this.e4125) - (float4(right_anti_dual_g2_w) * this.group3) - (right_anti_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group0.xyz), right_anti_dual_g3[2] * this.e43) - float4((right_anti_dual_g1_xyz.yzx * this.group9.zxy), right_anti_dual_g3[1] * this.e42), 
            /* e5 */
            (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g3[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_anti_dual_g3.zxy), 0.0) - (right_anti_dual_g3.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g2_w) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * right_anti_dual_g3.yzx) - (this.group7.yzx * right_anti_dual_g3.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz * float3(this.e12345)) + (float3(right_anti_dual_g2_w) * this.group8.xyz) + (float3(right_anti_dual_g3[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_w) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2_w) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g2_w * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e3215 * this.e1234) + (other.scalar * this.scalar), other.scalar * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            (other.e3215 * this.e12345) + (other.scalar * this.e5), 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            right_anti_dual_g0 * float4(this.e12345), 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiFlector other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1[0] * this.e1) + (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412) - (right_anti_dual_g0[3] * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(this.e1234)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((float3(right_anti_dual_g1[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_anti_dual_g1.zxy), 0.0) - (right_anti_dual_g1.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (right_anti_dual_g1[0] * this.e15) + (right_anti_dual_g1[1] * this.e25) + (right_anti_dual_g1[2] * this.e35) + (right_anti_dual_g1[3] * this.e45) - (right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g0 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((float3(right_anti_dual_g1[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_anti_dual_g1.zxy), 0.0) - (right_anti_dual_g1.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.zxy * right_anti_dual_g1.yzx) - (this.group7.yzx * right_anti_dual_g1.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g1[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1.yzx * this.group9.zxy) + float3(float2(0.0), right_anti_dual_g1[1] * this.e4235 * -1.0) - float3((right_anti_dual_g1.zx * this.group9.yz), 0.0)), right_anti_dual_g1[3] * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual_g1.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((right_anti_dual_g0 * float3(this.e321)), 0.0) + float4((right_anti_dual_g1.yzx * this.group7.zxy), 0.0) - float4((right_anti_dual_g1.zxy * this.group7.yzx), right_anti_dual_g0[0] * this.e423), 
            /* e5 */
            -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + float4((right_anti_dual_g0 * float3(this.e3215)), 0.0) + float4((right_anti_dual_g1.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0])), 
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12 */
            right_anti_dual_g1 * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * float3(this.e12345)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_anti_dual_g1 * float3(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[3] * this.scalar) + (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12), right_anti_dual_g0[3] * this.e12345), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((this.group7.yzx * other.group1.zxy), 0.0) - float4((this.group7.zxy * other.group1.yzx), right_anti_dual_g0[0] * this.e423), 
            /* e5 */
            (right_anti_dual_g0[3] * this.e5) + (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.e3215 * this.e12345) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) + float4((other.group1.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group1.yzx, right_anti_dual_g0[0])), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual_g0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g0[3]) * this.group5.xyz) - (float3(this.e1234) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group6.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e321), 
            /* e423, e431, e412 */
            float3(right_anti_dual_g0[3]) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * this.group8.xyz) - (float3(this.e12345) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group9, 
            /* e1234 */
            right_anti_dual_g0[3] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) + float4((float3(right_anti_dual_g0[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_anti_dual_g0.zxy), 0.0) - (right_anti_dual_g0.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_anti_dual_g0.zxy), 0.0) - (right_anti_dual_g0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.zxy * right_anti_dual_g0.yzx) - (this.group7.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0.yzx * this.group9.zxy) + float3(float2(0.0), right_anti_dual_g0[1] * this.e4235 * -1.0) - float3((right_anti_dual_g0.zx * this.group9.yz), 0.0)), right_anti_dual_g0[3] * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_anti_dual_g0.xyz, 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, right_anti_dual_g1_w)) + (this.group9.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_anti_dual_g1_xyz * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g1_w * this.e321) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (this.group9.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            right_anti_dual_g1_xyz * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g2[0] * this.e41) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((float3(right_anti_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group8.yzx), 0.0) + float4((right_anti_dual_g2.yzx * this.group7.zxy), 0.0) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((right_anti_dual_g2.zxy * this.group7.yzx), right_anti_dual_g0[1] * this.e425), 
            /* e5 */
            -(right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_anti_dual_g2.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(this.e3215)) + (right_anti_dual_g2 * float3(this.e1234)) - (float3(right_anti_dual_g1[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e235, e315, e125 */
            right_anti_dual_g2 * float3(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e15) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            (other.group2.zxyw * float4(this.group7.yzx, this.e12345)) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((float3(right_anti_dual_g1[3]) * this.group6.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group8.yzx), 0.0) - float4((float2(this.e12345) * other.group3.xy), other.e4125 * this.e12345, right_anti_dual_g0[2] * this.e435) - float4((right_anti_dual_g0.yzx * this.group8.zxy), right_anti_dual_g0[0] * this.e415) - float4((this.group7.zxy * other.group2.yzx), right_anti_dual_g0[1] * this.e425), 
            /* e5 */
            (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.e3215 * this.e12345) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((other.group2.zxy * this.group9.yzx), 0.0) - (this.group9.zxyx * float4(other.group2.yzx, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual_g1.xyz) + (right_anti_dual_g0.zxy * this.group9.yzx) - (right_anti_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g0 * float3(this.e3215)) - (float3(right_anti_dual_g1[3]) * this.group9.xyz) - (float3(this.e1234) * other.group2.xyz), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e5 * this.e4) - (other.e12345 * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e5 * -1.0) * this.group4.xyz), 0.0), 
            /* e5 */
            other.e5 * this.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e5 * -1.0) * this.group6.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e5 * -1.0) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e5 * this.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e5 * -1.0) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e5 * this.e12345 * -1.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) - (other.e45 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e45) * this.group6.xyz), 0.0) + float4((this.group7.yzx * other.group0.zxy), 0.0) - float4((this.group7.zxy * other.group0.yzx), 0.0), 
            /* e5 */
            (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            -(float3(other.e45) * this.group9.xyz) - (float3(this.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e45 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e45 * this.e45) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e45) * this.group6.xyz), 0.0) + float4((this.group7.yzx * other.group0.zxy), 0.0) - float4((float3(this.e12345) * other.group1.xyz), 0.0) - float4((this.group7.zxy * other.group0.yzx), 0.0), 
            /* e5 */
            (other.e15 * this.e415) + (other.e25 * this.e425) + (other.e35 * this.e435) + (other.e3215 * this.e12345), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            -(float3(other.e45) * this.group9.xyz) - (float3(this.e1234) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e45 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345 * -1.0) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1_w * this.e4) - (right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412) - (other.e12345 * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual_g1_xyz * float3(this.e1234)), 0.0) + float4((float3(right_anti_dual_g1_w) * this.group4.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group9.yzx), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (right_anti_dual_g1_w * this.e45) - (right_anti_dual_g1_xyz[0] * this.e4235) - (right_anti_dual_g1_xyz[1] * this.e4315) - (right_anti_dual_g1_xyz[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((right_anti_dual_g1_xyz * float3(this.e12345)) + (float3(right_anti_dual_g1_w) * this.group6.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (right_anti_dual_g0_xyz * float3(this.e12345)) + (float3(right_anti_dual_g1_w) * this.group7.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_anti_dual_g1_w * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_anti_dual_g1_w) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_anti_dual_g1_w * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(MultiVector other) {
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
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1[0] * this.e4235) + (right_anti_dual_g1[1] * this.e4315) + (right_anti_dual_g1[2] * this.e4125) + (right_anti_dual_g1[3] * this.e3215) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (right_anti_dual_g6[0] * this.e23) - (right_anti_dual_g6[1] * this.e31) - (right_anti_dual_g6[2] * this.e12) - (right_anti_dual_g6[3] * this.e45) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), right_anti_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g1 * float4(this.e12345)) + (right_anti_dual_g3 * float4(this.e1234)) + (float4(right_anti_dual_g0[1]) * this.group1) + (this.group9.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4(float3(0.0), -(right_anti_dual_g6[1] * this.e431) - (right_anti_dual_g6[2] * this.e412) - (right_anti_dual_g9[1] * this.e42) - (right_anti_dual_g9[2] * this.e43)) + float4((float3(right_anti_dual_g6[3]) * this.group6.xyz), 0.0) + float4((float3(right_anti_dual_g9[3]) * this.group4.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g7.zxy * this.group8.yzx), other.e431 * this.e4315) + float4((right_anti_dual_g8.yzx * this.group7.zxy), other.e412 * this.e4125) + float4((this.group5.yzx * right_anti_dual_g9.zxy), 0.0) - (float4(right_anti_dual_g10) * this.group3) - (right_anti_dual_g9.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group7.xyz), right_anti_dual_g6[0] * this.e423) - float4((right_anti_dual_g5.yzx * this.group9.zxy), right_anti_dual_g7[0] * this.e415) - float4((right_anti_dual_g7.yzx * this.group8.zxy), right_anti_dual_g7[1] * this.e425) - float4((right_anti_dual_g8.zxy * this.group7.yzx), right_anti_dual_g7[2] * this.e435), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) + (right_anti_dual_g9[0] * this.e15) + (right_anti_dual_g9[1] * this.e25) + (right_anti_dual_g9[2] * this.e35) + (right_anti_dual_g9[3] * this.e45) + (other.e3215 * this.e12345) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group3) + float4(float3(0.0), -(right_anti_dual_g6[1] * this.e4315) - (right_anti_dual_g6[2] * this.e4125) - (right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + float4((float3(right_anti_dual_g9[3]) * this.group6.xyz), 0.0) + float4((float3(this.e3215) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g8.yzx * this.group9.zxy), 0.0) + float4((this.group8.yzx * right_anti_dual_g9.zxy), 0.0) - (right_anti_dual_g9.yzxx * float4(this.group8.zxy, this.e415)) - (this.group9.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group6.xyz) + (float3(right_anti_dual_g0[1]) * this.group4.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * right_anti_dual_g6.xyz) + (right_anti_dual_g7.zxy * this.group9.yzx) + (this.group7.zxy * right_anti_dual_g9.yzx) - (right_anti_dual_g7.yzx * this.group9.zxy) - (this.group7.yzx * right_anti_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.e12345)) + (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g10) * this.group8.xyz) + (float3(right_anti_dual_g0[1]) * this.group5.xyz) + (float3(right_anti_dual_g9[3]) * this.group7.xyz) - (float3(right_anti_dual_g6[3]) * this.group9.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group6) + (right_anti_dual_g9.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_anti_dual_g9[2], right_anti_dual_g9[0], right_anti_dual_g9[1], right_anti_dual_g10) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group7.xyz) + (float3(this.e1234) * right_anti_dual_g9.xyz) - (float3(right_anti_dual_g10) * this.group9.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group8.xyz) + (float3(right_anti_dual_g9[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_anti_dual_g9 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group9), 
            /* e1234 */
            (right_anti_dual_g10 * this.e12345) + (right_anti_dual_g0[1] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1234 * other.e3215) - (this.e4235 * other.e4235) - (this.e4315 * other.e4315) - (this.e4125 * other.e4125), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e12345 * -1.0) * other.group0.xyz), 0.0), 
            /* e5 */
            this.e12345 * other.e3215, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1 * this.e5) + (right_anti_dual_g0[0] * this.e1) + (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e42) - (right_anti_dual_g0[2] * this.e43)) + float4((float3(right_anti_dual_g0[3]) * this.group4.xyz), 0.0) + float4((this.group5.yzx * right_anti_dual_g0.zxy), 0.0) - (float4(right_anti_dual_g1) * this.group3) - (right_anti_dual_g0.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (right_anti_dual_g0[0] * this.e15) + (right_anti_dual_g0[1] * this.e25) + (right_anti_dual_g0[2] * this.e35) + (right_anti_dual_g0[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_anti_dual_g0.zxy), 0.0) - (right_anti_dual_g0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g1) * this.group6.xyz) + (this.group7.zxy * right_anti_dual_g0.yzx) - (this.group7.yzx * right_anti_dual_g0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual_g1) * this.group8.xyz) + (float3(right_anti_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g0.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g1) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual_g0.xyz) - (float3(right_anti_dual_g1) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g1 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Scalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar) * this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group3, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group6, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group7.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(Sphere other) {
        float4 right_anti_dual_g0 = float4((other.group0.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[0] * this.e4235) + (right_anti_dual_g0[1] * this.e4315) + (right_anti_dual_g0[2] * this.e4125) + (right_anti_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e5 */
            this.e12345 * other.e3215, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g2_w * this.e5) + (right_anti_dual_g3[0] * this.e1) + (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e235) - (right_anti_dual_g0_xyz[1] * this.e315) - (right_anti_dual_g0_xyz[2] * this.e125) - (right_anti_dual_g1_xyz[0] * this.e415) - (right_anti_dual_g1_xyz[1] * this.e425) - (right_anti_dual_g1_xyz[2] * this.e435) - (right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (this.e12345 * other.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g0_xyz[1] * this.e4315) * float4(this.group5.yzx, 1.0)) + (this.group9.yzxx * float4(right_anti_dual_g1_xyz.zxy, right_anti_dual_g0_xyz[0])) + float4((float3(right_anti_dual_g3[3]) * this.group4.xyz), right_anti_dual_g0_xyz[2] * this.e4125) - (float4(right_anti_dual_g2_w) * this.group3) - (right_anti_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), right_anti_dual_g3[1] * this.e42) - float4((right_anti_dual_g1_xyz.yzx * this.group9.zxy), right_anti_dual_g3[2] * this.e43), 
            /* e5 */
            (right_anti_dual_g3[0] * this.e15) + (right_anti_dual_g3[1] * this.e25) + (right_anti_dual_g3[2] * this.e35) + (right_anti_dual_g3[3] * this.e45) - (right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g3[3]) * this.group6.xyz), 0.0) + float4((this.group8.yzx * right_anti_dual_g3.zxy), 0.0) - (right_anti_dual_g3.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (right_anti_dual_g0_xyz * float3(this.e12345)) + (float3(right_anti_dual_g2_w) * this.group6.xyz) + (this.group7.zxy * right_anti_dual_g3.yzx) - (this.group7.yzx * right_anti_dual_g3.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g1_xyz * float3(this.e12345)) + (float3(right_anti_dual_g2_w) * this.group8.xyz) + (float3(right_anti_dual_g3[3]) * this.group7.xyz) - (float3(this.e321) * right_anti_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_w) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_anti_dual_g3.xyz) - (float3(right_anti_dual_g2_w) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_anti_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g2_w * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension MultiVector {
    public MultiVector weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4((other.group3.xyz * float3(-1.0)), other.e1234);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g3[0] * this.e4235) + (right_anti_dual_g3[1] * this.e4315) + (right_anti_dual_g3[2] * this.e4125) + (right_anti_dual_g3[3] * this.e3215) + (this.scalar * other.scalar) + (this.e15 * other.e41) + (this.e25 * other.e42) + (this.e35 * other.e43) + (this.e1234 * other.e3215) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g1[0] * this.e23) - (right_anti_dual_g1[1] * this.e31) - (right_anti_dual_g1[2] * this.e12) - (right_anti_dual_g1[3] * this.e45), this.e12345 * other.scalar), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g3 * float4(this.e12345)) + (float4(other.scalar) * this.group1) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other.e41) * this.group6.wwwx) + (this.group6.xyzy * float4(float3(right_anti_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group8.zxy, this.e435)) + float4(float3(0.0), (right_anti_dual_g1[2] * this.e412) * -1.0) + float4((right_anti_dual_g2_xyz.yzx * this.group7.zxy), 0.0) - float4((right_anti_dual_g2_xyz.zxy * this.group7.yzx), right_anti_dual_g1[0] * this.e423) - float4((this.group8.yzx * other.group0.zxy), right_anti_dual_g1[1] * this.e431), 
            /* e5 */
            (this.e12345 * other.e3215) + (this.e5 * other.scalar) - (right_anti_dual_g2_xyz[0] * this.e415) - (right_anti_dual_g2_xyz[1] * this.e425) - (right_anti_dual_g2_xyz[2] * this.e435) - (right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(this.e3215) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g2_xyz.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(right_anti_dual_g2_xyz.zxy, right_anti_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual_g1.xyz) + (float3(other.scalar) * this.group4.xyz) + float3(float2(0.0), (this.e4315 * other.e41) - (this.e4235 * other.e42)) + float3((this.group9.zx * other.group0.yz), 0.0) - float3((this.group9.yz * other.group0.zx), 0.0), 
            /* e23, e31, e12 */
            (right_anti_dual_g2_xyz * float3(this.e1234)) + (float3(other.scalar) * this.group5.xyz) - (float3(right_anti_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group6), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) - (float3(this.e12345) * other.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g2_xyz * float3(this.e12345)) + (float3(other.scalar) * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group9, 
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Plane> operator /(Plane lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public DipoleInversion weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx)), right_anti_dual_g2_w * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e5 * -1.0), other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), 0.0), 
            /* e1, e2, e3, e5 */
            (this.group0.yzxw * float4(right_anti_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) - (this.group0.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - (this.group0.wwwy * float4(other.group0.xyz, right_anti_dual_g2_xyz[1]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightContractionInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Plane weight_contraction(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WeightContractionInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public DualNum weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return DualNum(
            /* e5, e12345 */
            float2((other.e321 * this.e3215) - (right_anti_dual_g0_xyz[0] * this.e4235) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WeightContractionInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Motor weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (this.group0 * float4(float3(other.e5 * -1.0), other.e321)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125)) - (this.group0.wwwx * float4(right_anti_dual_g1_xyz, right_anti_dual_g0_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WeightContractionInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public FlatPoint weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) + float4((right_anti_dual_g0 * float3(this.e3215)), 0.0) + float4((right_anti_dual_g1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g1.zxy, right_anti_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WeightContractionInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Flector weight_contraction(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            (this.group0.yzxx * float4(other.group1.zxy, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WeightContractionInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Line weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Line(
            /* e415, e425, e435 */
            (right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5 * -1.0) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public RoundPoint weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public RoundPoint weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (other.e321 * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Dipole weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g2.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_anti_dual_g2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, scalar */
            (other.group2.zxyw * this.group0.yzxw) + float4(float3(0.0), (other.e4125 * this.e4125) * -1.0) - (this.group0.zxyy * float4(other.group2.yzx, other.e4315)) - (this.group0.wwwx * float4(other.group1.xyz, other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiFlatPoint weight_contraction(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiLine weight_contraction(FlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e45 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e15 * this.e4315) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiMotor weight_contraction(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e45 * -1.0) * this.group0.xyz), -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiPlane weight_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiFlector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e4315) - (right_anti_dual_g1_xyz[2] * this.e4125)) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g1_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public MultiVector weight_contraction(MultiVector other) {
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e1234 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_anti_dual_g5.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g6_xyz[1] * this.e4315) - (right_anti_dual_g6_xyz[2] * this.e4125)) + float4((right_anti_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_anti_dual_g8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g7.zxy * this.group0.yzx) - (right_anti_dual_g7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) - (float3(other.e45) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g9_xyz.yzx * this.group0.zxy) - (right_anti_dual_g9_xyz.zxy * this.group0.yzx)), right_anti_dual_g10 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_anti_dual_g10 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e5 * -1.0) * this.group0.xyz) - (right_anti_dual_g9_xyz * float3(this.e3215)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Scalar weight_contraction(Plane other) {
        return Scalar(
            /* scalar */
            -(other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Circle weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_anti_dual_g1 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), right_anti_dual_g1 * this.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(other.e5 * -1.0) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WeightContractionInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Plane weight_contraction(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public Scalar weight_contraction(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (this.e3215 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_anti_dual_g2_w * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g3_xyz.yzx * this.group0.zxy) - (right_anti_dual_g3_xyz.zxy * this.group0.yzx)), right_anti_dual_g2_w * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e5 * -1.0), right_anti_dual_g0_xyz[0])) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), 0.0), 
            /* e1, e2, e3, e5 */
            (this.group0.yzxw * float4(right_anti_dual_g1_xyz.zxy, other.e321)) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) - (this.group0.zxyy * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[1])) - (this.group0.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Plane {
    public VersorOdd weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.zxyx * float4(other.group0.yzx, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234)) - float4((this.group0.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(-(this.e4235 * other.e45) - (this.e3215 * other.e41), -(this.e4315 * other.e45) - (this.e3215 * other.e42), -(this.e4125 * other.e45) - (this.e3215 * other.e43), (this.e4235 * other.e23) + (this.e4315 * other.e31) + (this.e4125 * other.e12)), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - float4((float3(this.e3215) * other.group1.xyz), 0.0) - float4((right_anti_dual_g2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<RoundPoint> operator /(RoundPoint lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiDualNum other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_contraction(AntiMotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(Motor other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g9_xyz[0] * this.e1) + (right_anti_dual_g9_xyz[1] * this.e2) + (right_anti_dual_g9_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual_g0[1]) * this.group0, 
            /* e5 */
            right_anti_dual_g0[1] * this.e5, 
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
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e1) + (right_anti_dual_g0_xyz[1] * this.e2) + (right_anti_dual_g0_xyz[2] * this.e3) - (other.e4 * this.e5) - (other.e5 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_contraction(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public Scalar weight_contraction(VersorEven other) {
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        return Scalar(
            /* scalar */
            (right_anti_dual_g3_xyz[0] * this.e1) + (right_anti_dual_g3_xyz[1] * this.e2) + (right_anti_dual_g3_xyz[2] * this.e3) - (this.e4 * other.e5) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint weight_contraction(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Scalar> operator /(Scalar lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Scalar {
    public Scalar weight_contraction(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<Sphere> operator /(Sphere lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WeightContractionInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public DipoleInversion weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0) - float4((float3(this.e1234) * other.group2.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((other.group2.zxy * this.group0.yzx) + float3(float2(0.0), other.e15 * this.e4315 * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((other.group2.yz * this.group0.zx), 0.0)), other.scalar * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), right_anti_dual_g3_w * this.e1234) - (this.group0.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(right_anti_dual_g3_w), other.e423)) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)) + float4((right_anti_dual_g2_xyz * float3(this.e1234)), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1_w)) - (this.group0.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public VersorOdd weight_contraction(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.scalar * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g1_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group0.zxy) - (right_anti_dual_g1_xyz.zxy * this.group0.yzx)), right_anti_dual_g1_w * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_anti_dual_g1_w) * this.group0.xyz) - (right_anti_dual_g1_xyz * float3(this.e3215))), right_anti_dual_g0_w * this.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.e1234, this.e1234, this.e1234, -(right_anti_dual_g0_w * this.e3215) - (right_anti_dual_g0_xyz[0] * this.e4235) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125)) * float4(right_anti_dual_g0_xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Dipole weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4(this.e1234, this.e1234, this.e1234, -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) * float4(right_anti_dual_g1, 1.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g0 * float3(this.e3215)) + (right_anti_dual_g1.yzx * this.group0.zxy) - (right_anti_dual_g1.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public VersorOdd weight_contraction(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4((other.group0.xyz * float3(-1.0)), other.e3215), 
            /* e23, e31, e12, e45 */
            float4(this.e1234, this.e1234, this.e1234, (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) * float4((other.group1.xyz * float3(-1.0)), 1.0), 
            /* e15, e25, e35, e1234 */
            float4(((other.group1.zxy * this.group0.yzx) + float3(float2(0.0), other.e15 * this.e4315 * -1.0) - (float3(this.e3215) * other.group0.xyz) - float3((other.group1.yz * this.group0.zx), 0.0)), other.scalar * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Circle weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            right_anti_dual_g0_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group0.zxy) - (right_anti_dual_g0_xyz.zxy * this.group0.yzx)), right_anti_dual_g0_w * this.e1234), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0_w) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public RoundPoint weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, right_anti_dual_g1_w)) + (this.group0.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public RoundPoint weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (this.group0.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WeightContractionInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Dipole weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e1234, this.e1234, this.e1234, other.e31 * this.e4315) * float4(right_anti_dual_g2, 1.0)) + (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), other.e12 * this.e4125) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e15, e25, e35 */
            (right_anti_dual_g2.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_anti_dual_g2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_anti_dual_g0.zxy * this.group0.yzx) - (float3(this.e1234) * other.group1.xyz) - (right_anti_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (this.group0.wwwx * float4(right_anti_dual_g0, other.e23)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) - float4((float3(other.e45) * this.group0.xyz), 0.0) - float4((float3(this.e1234) * other.group2.xyz), 0.0), 
            /* e15, e25, e35, scalar */
            (other.group2.zxyw * this.group0.yzxw) + float4(float3(0.0), (other.e3215 * this.e1234) - (other.e4125 * this.e4125)) - (this.group0.zxyy * float4(other.group2.yzx, other.e4315)) - (this.group0.wwwx * float4(other.group1.xyz, other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WeightContractionInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiFlatPoint weight_contraction(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WeightContractionInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiLine weight_contraction(FlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            -(float3(other.e45) * this.group0.xyz) - (float3(this.e1234) * other.group0.xyz), 
            /* e15, e25, e35 */
            (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e15 * this.e4315) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiMotor weight_contraction(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group0.xyzx) - float4((float3(this.e1234) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiPlane weight_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiFlector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e4315) - (right_anti_dual_g1_xyz[2] * this.e4125)) + float4((right_anti_dual_g1_xyz * float3(this.e1234)), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g1_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g3 * float4(this.e1234)) + (this.group0.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_anti_dual_g5.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(right_anti_dual_g6_xyz[1] * this.e4315) - (right_anti_dual_g6_xyz[2] * this.e4125)) + float4((right_anti_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_anti_dual_g8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g6_xyz * float3(this.e1234)) + (right_anti_dual_g7.zxy * this.group0.yzx) - (right_anti_dual_g7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) - (float3(other.e45) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g9_xyz.yzx * this.group0.zxy), right_anti_dual_g9_w * this.e1234) - (this.group0.yzxw * float4(right_anti_dual_g9_xyz.zxy, right_anti_dual_g10)), 
            /* e423, e431, e412 */
            (right_anti_dual_g9_xyz * float3(this.e1234)) - (float3(right_anti_dual_g10) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g9_w) * this.group0.xyz) - (right_anti_dual_g9_xyz * float3(this.e3215)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[1]) * this.group0, 
            /* e1234 */
            right_anti_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Scalar weight_contraction(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WeightContractionInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Circle weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (right_anti_dual_g0_xyz * float3(this.e1234)) - (float3(right_anti_dual_g1) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0_xyz.yzx * this.group0.zxy), right_anti_dual_g0_w * this.e1234) - (this.group0.yzxw * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1)), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g0_w) * this.group0.xyz) - (right_anti_dual_g0_xyz * float3(this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WeightContractionInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Sphere weight_contraction(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public Scalar weight_contraction(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group0.zxy), right_anti_dual_g3_w * this.e1234) - (this.group0.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g0_xyz[0])) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)) + float4((right_anti_dual_g2_xyz * float3(this.e1234)), 0.0) + float4((right_anti_dual_g1_xyz.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(right_anti_dual_g0_xyz, right_anti_dual_g1_w)) - (this.group0.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension Sphere {
    public VersorOdd weight_contraction(VersorOdd other) {
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.zxyx * float4(other.group0.yzx, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (this.e3215 * other.e1234) + (this.e1234 * other.e3215)) - float4((float3(this.e1234) * other.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(this.e1234, this.e1234, this.e1234, this.e4235 * other.e23) * float4(right_anti_dual_g2_xyz, 1.0)) + float4(float3(0.0), (this.e4315 * other.e31) + (this.e4125 * other.e12)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz.yzx * this.group0.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_anti_dual_g2_xyz.zxy * this.group0.yzx)), this.e1234 * other.scalar), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<VersorEven> operator /(VersorEven lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiCircleRotor other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_anti_dual_g0 * float3(this.e12345)) + (float3(right_anti_dual_g2[3]) * this.group0.xyz)), right_anti_dual_g2[3] * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g2[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(float3(this.e12345), this.e5)) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)) + float4((float3(right_anti_dual_g2[3]) * this.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g2[3]) * this.group3) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - (float4(right_anti_dual_g2[2], right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g1[0]) * this.group0.yzxx) - float4((right_anti_dual_g0.yzx * this.group2.zxy), right_anti_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiDipoleInversion other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g2_w) * float4(this.group1.xyz, this.e5)) + (right_anti_dual_g3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((float3(this.e12345) * other.group0.xyz), right_anti_dual_g3[1] * this.e2) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_xyz[0]) * this.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.e12345)) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[1] * this.e235) - (right_anti_dual_g3[0] * this.e315)) + float3((right_anti_dual_g3.zx * this.group2.yz), 0.0) - float3((right_anti_dual_g3.yz * this.group2.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group2.xyz), (other.e3215 * this.e12345) + (other.scalar * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(AntiFlatPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_anti_dual_g0_w * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g0_xyz * float3(this.e12345)), -(right_anti_dual_g0_w * this.e321) - (right_anti_dual_g0_xyz[0] * this.e423) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g1[1] * this.e2) + (right_anti_dual_g1[2] * this.e3) + (right_anti_dual_g1[3] * this.e4) - (right_anti_dual_g0_w * this.e321) - (right_anti_dual_g0_xyz[1] * this.e431) - (right_anti_dual_g0_xyz[2] * this.e412)) - (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0_xyz[0]) * this.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(float3(right_anti_dual_g1[3]), right_anti_dual_g0_w)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) - (right_anti_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((right_anti_dual_g0_xyz * float3(this.e12345)), 0.0) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0 * float3(this.e12345)), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g1 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e423) - (right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)) + float4((right_anti_dual_g0 * float3(this.e321)), 0.0) + float4((right_anti_dual_g1.yzx * this.group0.zxy), 0.0) - float4((right_anti_dual_g1.zxy * this.group0.yzx), right_anti_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorEven weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_anti_dual_g0[3]) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_anti_dual_g0.xyz)), right_anti_dual_g0[3] * this.e321), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(right_anti_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e431) - (right_anti_dual_g0[2] * this.e412)) + float4((float3(right_anti_dual_g0[3]) * this.group3.xyz), 0.0) + float4((right_anti_dual_g1.yzx * this.group0.zxy), 0.0) - (float4(right_anti_dual_g1[2], right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g0[0]) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd weight_contraction(AntiPlane other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)) - float4((right_anti_dual_g0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_anti_dual_g0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public Scalar weight_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(Circle other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor weight_contraction(CircleRotor other) {
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((right_anti_dual_g2_xyz * float3(this.e12345)), -(right_anti_dual_g2_xyz[0] * this.e423) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e12345 * this.e12345))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(Dipole other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_anti_dual_g2 * float3(this.e12345)), -(right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((float3(right_anti_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g1.xyz), 0.0) + float4((right_anti_dual_g0.zxy * this.group2.yzx), 0.0) + float4((right_anti_dual_g2.yzx * this.group0.zxy), 0.0) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((right_anti_dual_g2.zxy * this.group0.yzx), right_anti_dual_g2[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion weight_contraction(DipoleInversion other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_anti_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345 * -1.0) * other.group2.xyz), (other.e1234 * this.e12345) - (right_anti_dual_g0[0] * this.e415) - (right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435) - (right_anti_dual_g1[0] * this.e423) - (right_anti_dual_g1[1] * this.e431) - (right_anti_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other.e15) * this.group1.wwwx) + (other.group2.zxyz * float4(this.group0.yzx, this.e435)) + (this.group1.xyzy * float4(float3(right_anti_dual_g1[3]), other.e25)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125)) + float4((right_anti_dual_g0.zxy * this.group2.yzx), other.e3215 * this.e12345) - (this.group2.zxyx * float4(right_anti_dual_g0.yzx, right_anti_dual_g1[0])) - float4((float3(this.e12345) * other.group3.xyz), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor weight_contraction(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e5 * -1.0) * this.group0.xyz), -(other.e5 * this.e4) - (other.e12345 * this.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(other.e5 * -1.0) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiFlector weight_contraction(FlatPoint other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiFlector weight_contraction(Flector other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_anti_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)) + float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g0.yzx * this.group0.zxy), 0.0) - (right_anti_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor weight_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (this.group0 * float4(float3(right_anti_dual_g1_w), other.e12345 * -1.0)) + float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e415) - (right_anti_dual_g0_xyz[1] * this.e425) - (right_anti_dual_g0_xyz[2] * this.e435) - (right_anti_dual_g1_xyz[0] * this.e423) - (right_anti_dual_g1_xyz[1] * this.e431) - (right_anti_dual_g1_xyz[2] * this.e412)) + float4((right_anti_dual_g0_xyz * float3(this.e12345)), right_anti_dual_g1_w * this.e4), 
            /* e15, e25, e35, e3215 */
            float4(((right_anti_dual_g1_xyz * float3(this.e12345)) + (float3(right_anti_dual_g1_w) * this.group1.xyz)), right_anti_dual_g1_w * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float4 right_anti_dual_g6 = float4((other.group5.xyz * float3(-1.0)), other.e45);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_anti_dual_g9 = float4(other.group1.xyz, other.e5 * -1.0);
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g10 * this.e5) + (right_anti_dual_g0[0] * this.e12345) + (right_anti_dual_g9[0] * this.e1) + (right_anti_dual_g9[1] * this.e2) + (right_anti_dual_g9[2] * this.e3) + (right_anti_dual_g9[3] * this.e4) - (right_anti_dual_g5[0] * this.e415) - (right_anti_dual_g5[1] * this.e425) - (right_anti_dual_g5[2] * this.e435) - (right_anti_dual_g3[0] * this.e423) - (right_anti_dual_g3[1] * this.e431) - (right_anti_dual_g3[2] * this.e412) - (right_anti_dual_g3[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), right_anti_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g0[1]) * this.group3) + (this.group0.zxyw * float4(right_anti_dual_g8.yzx, other.e1234)) + float4(float3(0.0), -(right_anti_dual_g7[1] * this.e425) - (right_anti_dual_g7[2] * this.e435) - (right_anti_dual_g6[2] * this.e412)) + float4((float3(right_anti_dual_g6[3]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_anti_dual_g6.xyz), 0.0) + float4((right_anti_dual_g7.zxy * this.group2.yzx), 0.0) - (this.group0.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6[0])) - (this.group0.wwwy * float4(other.group9.xyz, right_anti_dual_g6[1])) - float4((right_anti_dual_g7.yzx * this.group2.zxy), right_anti_dual_g7[0] * this.e415), 
            /* e5 */
            (right_anti_dual_g0[1] * this.e5) + (other.e3215 * this.e12345) - (right_anti_dual_g8[0] * this.e415) - (right_anti_dual_g8[1] * this.e425) - (right_anti_dual_g8[2] * this.e435) - (right_anti_dual_g6[0] * this.e235) - (right_anti_dual_g6[1] * this.e315) - (right_anti_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            (right_anti_dual_g3 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g9[1] * this.e425) - (right_anti_dual_g9[2] * this.e435)) + float4((float3(right_anti_dual_g9[3]) * this.group1.xyz), 0.0) + float4((right_anti_dual_g9.zxy * this.group2.yzx), 0.0) - (right_anti_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + float3(float2(0.0), (right_anti_dual_g9[0] * this.e431) - (right_anti_dual_g9[1] * this.e423)) + float3((right_anti_dual_g9.yz * this.group0.zx), 0.0) - float3((right_anti_dual_g9.zx * this.group0.yz), 0.0), 
            /* e23, e31, e12 */
            (right_anti_dual_g5 * float3(this.e12345)) + (float3(right_anti_dual_g10) * this.group2.xyz) + (float3(right_anti_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_anti_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g6 * float4(this.e12345)) + (float4(right_anti_dual_g0[1]) * this.group1), 
            /* e423, e431, e412 */
            (right_anti_dual_g7 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual_g8 * float3(this.e12345)) + (float3(right_anti_dual_g0[1]) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_anti_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public AntiPlane weight_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd weight_contraction(RoundPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e5 * -1.0);
        float right_anti_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g1) * float4(this.group1.xyz, this.e5)) + (right_anti_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g0[1] * this.e2) + (right_anti_dual_g0[2] * this.e3) + (right_anti_dual_g0[3] * this.e4)) - float4((right_anti_dual_g0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e425) - (right_anti_dual_g0[2] * this.e435)) + float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g0[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group1.xyz) + (right_anti_dual_g0.zxy * this.group2.yzx) + float3(float2(0.0), right_anti_dual_g0[0] * this.e315 * -1.0) - float3((right_anti_dual_g0.yz * this.group2.zx), 0.0)), right_anti_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorEven weight_contraction(Scalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group2, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public RoundPoint weight_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e5 */
            other.e3215 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd weight_contraction(VersorEven other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e12345 * -1.0);
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float4 right_anti_dual_g3 = float4(other.group3.xyz, other.e5 * -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0 * float4(this.e12345)) + (float4(right_anti_dual_g2_w) * float4(this.group1.xyz, this.e5)) + (right_anti_dual_g3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_anti_dual_g3[1] * this.e2) + (right_anti_dual_g3[2] * this.e3) + (right_anti_dual_g3[3] * this.e4) - (right_anti_dual_g2_xyz[1] * this.e431) - (right_anti_dual_g2_xyz[2] * this.e412) - (right_anti_dual_g0[0] * this.e235) - (right_anti_dual_g0[1] * this.e315) - (right_anti_dual_g0[2] * this.e125) - (right_anti_dual_g1[0] * this.e415) - (right_anti_dual_g1[1] * this.e425) - (right_anti_dual_g1[2] * this.e435) - (right_anti_dual_g1[3] * this.e321)) - (float4(right_anti_dual_g3[2], right_anti_dual_g3[0], right_anti_dual_g3[1], right_anti_dual_g2_xyz[0]) * this.group0.yzxx), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g1 * float4(this.e12345)) + float4(float3(0.0), -(right_anti_dual_g3[1] * this.e425) - (right_anti_dual_g3[2] * this.e435)) + float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) + float4((float3(right_anti_dual_g3[3]) * this.group0.xyz), 0.0) - (right_anti_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((right_anti_dual_g2_xyz * float3(this.e12345)) + (float3(right_anti_dual_g3[3]) * this.group1.xyz) + float3(float2(0.0), (right_anti_dual_g3[1] * this.e235) - (right_anti_dual_g3[0] * this.e315)) + float3((right_anti_dual_g3.zx * this.group2.yz), 0.0) - float3((right_anti_dual_g3.yz * this.group2.zx), 0.0)), right_anti_dual_g2_w * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_anti_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WeightContractionInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorEven {
    public VersorEven weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_anti_dual_g2 = float4((other.group2.xyz * float3(-1.0)), other.e3215);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.scalar) * this.group0.xyz) - (float3(this.e12345) * other.group0.xyz)), this.e12345 * other.scalar), 
            /* e415, e425, e435, e321 */
            (right_anti_dual_g1 * float4(this.e12345)) + (float4(other.scalar) * this.group1), 
            /* e235, e315, e125, e5 */
            (right_anti_dual_g2 * float4(this.e12345)) + (float4(other.scalar) * this.group2) + float4(float3(0.0), -(right_anti_dual_g1[0] * this.e235) - (right_anti_dual_g1[1] * this.e315) - (right_anti_dual_g1[2] * this.e125) - (right_anti_dual_g2[0] * this.e415) - (right_anti_dual_g2[1] * this.e425) - (right_anti_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group3) + (float4(right_anti_dual_g1[0], right_anti_dual_g1[1], right_anti_dual_g1[2], other.e41) * this.group1.wwwx) + (float4(right_anti_dual_g2[1], right_anti_dual_g2[2], right_anti_dual_g2[0], other.e1234) * this.group0.zxyw) + (this.group1.xyzy * float4(float3(right_anti_dual_g1[3]), other.e42)) + (other.group0.yzxz * float4(this.group2.zxy, this.e435)) + float4(float3(0.0), (right_anti_dual_g1[2] * this.e412) * -1.0) - (float4(right_anti_dual_g2[2], right_anti_dual_g2[0], right_anti_dual_g2[1], right_anti_dual_g1[0]) * this.group0.yzxx) - (this.group0.wwwy * float4(other.group3.xyz, right_anti_dual_g1[1])) - float4((this.group2.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WeightContractionInfixPartial<VersorOdd> operator /(VersorOdd lhs, WeightContractionInfix rhs) {
    return WeightContractionInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiCircleRotor other) {
        float4 right_anti_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g2[3]) * this.group0) + float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((other.group0.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e1234), this.e45)) - float4((other.group0.zxy * this.group3.yzx), right_anti_dual_g2[0] * this.e41), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((float3(right_anti_dual_g2[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g2[3]) * this.group2.xyz) + (right_anti_dual_g2.yzx * this.group3.zxy) + float3(float2(0.0), right_anti_dual_g2[1] * this.e4235 * -1.0) - (float3(this.e3215) * other.group1.xyz) - float3((right_anti_dual_g2.zx * this.group3.yz), 0.0)), right_anti_dual_g2[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g2[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiDipoleInversion other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group3.zxy), right_anti_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), other.e423)) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_anti_dual_g3_xyz[0] * this.e41) - (right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g2_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_anti_dual_g1_xyz.zxy * this.group3.yzx), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), right_anti_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(other.group0.xyz, right_anti_dual_g1_w)) - (this.group3.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g2_xyz[1] * this.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(other.scalar) * this.group0.xyz), (other.e3215 * this.e1234) + (other.scalar * this.scalar)), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint weight_contraction(AntiFlatPoint other) {
        float4 right_anti_dual_g0 = float4(other.group0.xyz, other.e321 * -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_anti_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125) - (right_anti_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiFlector other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g1_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g1_xyz.yzx * this.group3.zxy) - (right_anti_dual_g1_xyz.zxy * this.group3.yzx)), right_anti_dual_g1_w * this.e1234), 
            /* e235, e315, e125, e4 */
            float4((float2(right_anti_dual_g1_w) * this.group3.xy), right_anti_dual_g1_w * this.e4125, right_anti_dual_g0_w * this.e1234) + float4(float3(0.0), -(right_anti_dual_g1_xyz[1] * this.e42) - (right_anti_dual_g1_xyz[2] * this.e43)) - float4((right_anti_dual_g1_xyz * float3(this.e3215)), right_anti_dual_g1_xyz[0] * this.e41), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g1_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g0_xyz, right_anti_dual_g1_xyz[0])) + float4(float3(0.0), (right_anti_dual_g1_xyz[2] * this.e35) - (right_anti_dual_g0_xyz[0] * this.e4235) - (right_anti_dual_g0_xyz[1] * this.e4315) - (right_anti_dual_g0_xyz[2] * this.e4125)) + float4((right_anti_dual_g1_xyz.zxy * this.group1.yzx), right_anti_dual_g1_xyz[1] * this.e25) - float4((right_anti_dual_g1_xyz.yzx * this.group1.zxy), right_anti_dual_g0_w * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(AntiLine other) {
        float3 right_anti_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_anti_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_anti_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual_g1 * float3(this.e1234)), -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12) - (right_anti_dual_g1[0] * this.e41) - (right_anti_dual_g1[1] * this.e42) - (right_anti_dual_g1[2] * this.e43)) + float4((right_anti_dual_g0 * float3(this.e3215)), 0.0) + float4((right_anti_dual_g1.yzx * this.group3.zxy), 0.0) - float4((right_anti_dual_g1.zxy * this.group3.yzx), right_anti_dual_g0[0] * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_contraction(AntiMotor other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_anti_dual_g0 * float4(float3(this.e1234), this.scalar)) + (this.group0.xyzx * float4(float3(right_anti_dual_g0[3]), other.e15)) + float4(float3(0.0), (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12)), 
            /* e23, e31, e12, e45 */
            (float4(right_anti_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_anti_dual_g0[0] * this.e4235) - (right_anti_dual_g0[1] * this.e4315) - (right_anti_dual_g0[2] * this.e4125)) - float4((float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_anti_dual_g0.xyz) + float3(float2(0.0), (other.e25 * this.e4235) - (other.e15 * this.e4315)) + float3((other.group1.zx * this.group3.yz), 0.0) - float3((other.group1.yz * this.group3.zx), 0.0)), right_anti_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(AntiPlane other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_anti_dual_g0_xyz * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4(((right_anti_dual_g0_xyz.yzx * this.group3.zxy) - (right_anti_dual_g0_xyz.zxy * this.group3.yzx)), right_anti_dual_g0_w * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), right_anti_dual_g0_xyz[0] * this.e41), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint weight_contraction(Circle other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.group2.xyz, right_anti_dual_g1_w)) + (this.group3.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WeightContractionInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint weight_contraction(CircleRotor other) {
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_anti_dual_g2_xyz, right_anti_dual_g1_w)) + (this.group3.yzxx * float4(right_anti_dual_g1_xyz.zxy, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((right_anti_dual_g1_xyz.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(right_anti_dual_g1_w * this.e3215) - (right_anti_dual_g2_xyz[0] * this.e4235) - (right_anti_dual_g2_xyz[1] * this.e4315) - (right_anti_dual_g2_xyz[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(Dipole other) {
        float3 right_anti_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_anti_dual_g2 * float3(this.e1234)), other.e23 * this.e4235) - float4((float3(other.e45) * this.group3.xyz), 0.0) - float4((float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) - (right_anti_dual_g2[1] * this.e42) - (right_anti_dual_g2[2] * this.e43)) + float4((right_anti_dual_g2.yzx * this.group3.zxy), other.e41 * this.e15) - (other.group1 * float4(float3(this.e3215), this.e45)) - float4((right_anti_dual_g2.zxy * this.group3.yzx), right_anti_dual_g2[0] * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WeightContractionInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor weight_contraction(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.yzx * this.group3.zxy) - (float3(this.e1234) * other.group1.xyz) - (other.group0.zxy * this.group3.yzx), 
            /* e23, e31, e12, e45 */
            float4(-(other.e41 * this.e3215) - (other.e45 * this.e4235) - (other.e15 * this.e1234), -(other.e42 * this.e3215) - (other.e45 * this.e4315) - (other.e25 * this.e1234), -(other.e43 * this.e3215) - (other.e45 * this.e4125) - (other.e35 * this.e1234), (other.e23 * this.e4235) + (other.e31 * this.e4315) + (other.e12 * this.e4125)), 
            /* e15, e25, e35, scalar */
            (other.group2.zxyx * float4(this.group3.yzx, this.e41)) + float4(float3(0.0), (other.e41 * this.e15) + (other.e42 * this.e25) + (other.e43 * this.e35) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (other.group1 * float4(float3(this.e3215), this.e45)) - (this.group3.zxyx * float4(other.group2.yzx, other.e4235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiFlector weight_contraction(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiMotor weight_contraction(FlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43)) - (other.group0 * float4(float3(this.e1234), this.e45)) - float4((float3(other.e45) * this.group3.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WeightContractionInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiMotor weight_contraction(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (other.e15 * this.e41) + (other.e25 * this.e42) + (other.e35 * this.e43) + (other.e3215 * this.e1234) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)) - (float4(other.e45, other.e45, other.e45, other.e4235) * this.group3.xyzx) - (other.group0 * float4(float3(this.e1234), this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WeightContractionInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiPlane weight_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e425, other.e435, other.e415, other.e235) * this.group3.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WeightContractionInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiFlector weight_contraction(Motor other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 right_anti_dual_g1 = float4(other.group1.xyz, other.e5 * -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_anti_dual_g1[3]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            (right_anti_dual_g1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(right_anti_dual_g1[1] * this.e4315) - (right_anti_dual_g1[2] * this.e4125)) + float4((float3(right_anti_dual_g1[3]) * this.group0.xyz), 0.0) + float4((right_anti_dual_g0_xyz.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(right_anti_dual_g0_xyz.yzx, right_anti_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WeightContractionInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public MultiVector weight_contraction(MultiVector other) {
        float2 right_anti_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float3 right_anti_dual_g1_xyz = other.group9.xyz * float3(-1.0);
        float4 right_anti_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float3 right_anti_dual_g5 = other.group6.xyz;
        float3 right_anti_dual_g6_xyz = other.group5.xyz * float3(-1.0);
        float3 right_anti_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_anti_dual_g8 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g9_xyz = other.group1.xyz;
        float right_anti_dual_g9_w = other.e5 * -1.0;
        float right_anti_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_anti_dual_g0[1] * this.scalar) + (right_anti_dual_g1_xyz[0] * this.e4235) + (right_anti_dual_g1_xyz[1] * this.e4315) + (right_anti_dual_g1_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215) - (right_anti_dual_g6_xyz[0] * this.e23) - (right_anti_dual_g6_xyz[1] * this.e31) - (right_anti_dual_g6_xyz[2] * this.e12) - (right_anti_dual_g7[0] * this.e15) - (right_anti_dual_g7[1] * this.e25) - (right_anti_dual_g7[2] * this.e35) - (right_anti_dual_g8[0] * this.e41) - (right_anti_dual_g8[1] * this.e42) - (right_anti_dual_g8[2] * this.e43) - (other.e45 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (right_anti_dual_g3 * float4(this.e1234)) + (this.group3.yzxx * float4(right_anti_dual_g5.zxy, other.e423)) + float4(float3(0.0), other.e412 * this.e4125) + float4((float3(right_anti_dual_g9_w) * this.group0.xyz), 0.0) + float4((right_anti_dual_g9_xyz.zxy * this.group1.yzx), other.e431 * this.e4315) - (float4(right_anti_dual_g10) * float4(this.e15, this.e25, this.e35, this.e45)) - float4((float3(this.e3215) * other.group7.xyz), right_anti_dual_g9_xyz[2] * this.e43) - float4((right_anti_dual_g5.yzx * this.group3.zxy), right_anti_dual_g9_xyz[0] * this.e41) - float4((right_anti_dual_g9_xyz.yzx * this.group1.zxy), right_anti_dual_g9_xyz[1] * this.e42), 
            /* e5 */
            (right_anti_dual_g9_w * this.e45) + (right_anti_dual_g9_xyz[0] * this.e15) + (right_anti_dual_g9_xyz[1] * this.e25) + (right_anti_dual_g9_xyz[2] * this.e35) - (right_anti_dual_g3[0] * this.e4235) - (right_anti_dual_g3[1] * this.e4315) - (right_anti_dual_g3[2] * this.e4125) - (right_anti_dual_g3[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual_g0[1]) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(right_anti_dual_g6_xyz[1] * this.e4315) - (right_anti_dual_g6_xyz[2] * this.e4125)) + float4((right_anti_dual_g6_xyz * float3(this.e3215)), 0.0) + float4((right_anti_dual_g8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(right_anti_dual_g8.zxy, right_anti_dual_g6_xyz[0])), 
            /* e41, e42, e43 */
            (right_anti_dual_g6_xyz * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group0.xyz) + (right_anti_dual_g7.zxy * this.group3.yzx) - (right_anti_dual_g7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (right_anti_dual_g7 * float3(this.e3215)) + (right_anti_dual_g8 * float3(this.e1234)) + (float3(right_anti_dual_g0[1]) * this.group1.xyz) - (float3(other.e45) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g9_xyz.yzx * this.group3.zxy), right_anti_dual_g9_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g9_xyz.zxy, right_anti_dual_g10)), 
            /* e423, e431, e412 */
            (right_anti_dual_g9_xyz * float3(this.e1234)) - (float3(right_anti_dual_g10) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_anti_dual_g9_w) * this.group3.xyz) - (right_anti_dual_g9_xyz * float3(this.e3215)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[1]) * this.group3, 
            /* e1234 */
            right_anti_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public Scalar weight_contraction(Plane other) {
        return Scalar(
            /* scalar */
            (other.e3215 * this.e1234) - (other.e4235 * this.e4235) - (other.e4315 * this.e4315) - (other.e4125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(RoundPoint other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float right_anti_dual_g0_w = other.e5 * -1.0;
        float right_anti_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g0_xyz * float3(this.e1234)) - (float3(right_anti_dual_g1) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g0_xyz.yzx * this.group3.zxy), right_anti_dual_g0_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g0_xyz.zxy, right_anti_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_anti_dual_g0_xyz[0] * this.e41) - (right_anti_dual_g0_xyz[1] * this.e42) - (right_anti_dual_g0_xyz[2] * this.e43)) + float4((float3(right_anti_dual_g0_w) * this.group3.xyz), 0.0) - float4((right_anti_dual_g0_xyz * float3(this.e3215)), right_anti_dual_g1 * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g0_w) * float4(this.group0.xyz, this.e45)) + float4(float3(0.0), (right_anti_dual_g0_xyz[1] * this.e25) + (right_anti_dual_g0_xyz[2] * this.e35)) + float4((right_anti_dual_g0_xyz.zxy * this.group1.yzx), right_anti_dual_g0_xyz[0] * this.e15) - float4((float3(right_anti_dual_g1) * this.group2.xyz), 0.0) - float4((right_anti_dual_g0_xyz.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_contraction(Scalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WeightContractionInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public Scalar weight_contraction(Sphere other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            (right_anti_dual_g0_xyz[0] * this.e4235) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) + (other.e3215 * this.e1234) + (other.e1234 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WeightContractionInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion weight_contraction(VersorEven other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float3 right_anti_dual_g1_xyz = other.group1.xyz;
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2_xyz = other.group2.xyz;
        float right_anti_dual_g2_w = other.e4 * -1.0;
        float3 right_anti_dual_g3_xyz = other.group3.xyz;
        float right_anti_dual_g3_w = other.e5 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (right_anti_dual_g3_xyz * float3(this.e1234)) - (float3(right_anti_dual_g2_w) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual_g3_xyz.yzx * this.group3.zxy), right_anti_dual_g3_w * this.e1234) - (this.group3.yzxw * float4(right_anti_dual_g3_xyz.zxy, right_anti_dual_g2_w)), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(right_anti_dual_g3_w), right_anti_dual_g0_xyz[0])) + float4(float3(0.0), (right_anti_dual_g1_w * this.e1234) + (right_anti_dual_g0_xyz[1] * this.e4315) + (right_anti_dual_g0_xyz[2] * this.e4125) - (right_anti_dual_g3_xyz[0] * this.e41) - (right_anti_dual_g3_xyz[1] * this.e42) - (right_anti_dual_g3_xyz[2] * this.e43)) - float4((right_anti_dual_g3_xyz * float3(this.e3215)), right_anti_dual_g2_w * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_anti_dual_g3_w) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwx * float4(right_anti_dual_g2_xyz, right_anti_dual_g3_xyz[0])) + float4(float3(0.0), (right_anti_dual_g2_xyz[2] * this.e4125) * -1.0) + float4((right_anti_dual_g1_xyz.zxy * this.group3.yzx), right_anti_dual_g3_xyz[1] * this.e25) + float4((right_anti_dual_g3_xyz.zxy * this.group1.yzx), right_anti_dual_g3_xyz[2] * this.e35) - (float4(this.e3215) * float4(right_anti_dual_g0_xyz, right_anti_dual_g1_w)) - (this.group3.zxyx * float4(right_anti_dual_g1_xyz.yzx, right_anti_dual_g2_xyz[0])) - float4((float3(right_anti_dual_g2_w) * this.group2.xyz), 0.0) - float4((right_anti_dual_g3_xyz.yzx * this.group1.zxy), right_anti_dual_g2_xyz[1] * this.e4315)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WeightContractionInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.weight_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd weight_contraction(VersorOdd other) {
        float4 right_anti_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_anti_dual_g2_xyz = other.group2.xyz * float3(-1.0);
        float3 right_anti_dual_g3_xyz = other.group3.xyz * float3(-1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(right_anti_dual_g0[3]) * this.group0) + (float4(right_anti_dual_g0[2], right_anti_dual_g0[0], right_anti_dual_g0[1], right_anti_dual_g3_xyz[0]) * this.group3.yzxx) + float4(float3(0.0), (right_anti_dual_g3_xyz[1] * this.e4315) + (right_anti_dual_g3_xyz[2] * this.e4125) + (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.e1234 * this.e3215) + (other.e3215 * this.e1234) - (right_anti_dual_g2_xyz[0] * this.e41) - (right_anti_dual_g2_xyz[1] * this.e42) - (right_anti_dual_g2_xyz[2] * this.e43) - (right_anti_dual_g0[1] * this.e25) - (right_anti_dual_g0[2] * this.e35)) - (other.group1 * float4(float3(this.e1234), this.e45)) - (right_anti_dual_g0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_anti_dual_g0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), (other.e31 * this.e4315) + (other.e12 * this.e4125)) + float4((right_anti_dual_g2_xyz * float3(this.e1234)), other.e23 * this.e4235) + float4((float3(right_anti_dual_g0[3]) * this.group1.xyz), 0.0) - float4((float3(other.e45) * this.group3.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual_g0[3]) * this.group2.xyz) + (right_anti_dual_g2_xyz.yzx * this.group3.zxy) - (float3(this.e3215) * other.group1.xyz) - (right_anti_dual_g2_xyz.zxy * this.group3.yzx)), right_anti_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_anti_dual_g0[3]) * this.group3
        );
    }
}
