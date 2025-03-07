implementing cga3d;
using traits;
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g2[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g2[3]) * this.group2.xyz), (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g3[2] * this.e31) + (right_dual_g3[3] * this.e41), (right_dual_g3[0] * this.e12) + (right_dual_g3[3] * this.e42), (right_dual_g3[1] * this.e23) + (right_dual_g3[3] * this.e43), -(right_dual_g2[3] * this.e45) - (right_dual_g3[2] * this.e43)) - (right_dual_g3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * right_dual_g2.www), right_dual_g3[1] * this.e42), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiDualNum other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[2] * this.e31) + (right_dual_g1[3] * this.e41), (right_dual_g1[0] * this.e12) + (right_dual_g1[3] * this.e42), (right_dual_g1[1] * this.e23) + (right_dual_g1[3] * this.e43), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) - (right_dual_g1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (right_dual_g1[0] * this.e15) + (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane bulk_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public Scalar bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane bulk_contraction(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[1] * this.scalar) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g9[2] * this.e31) + (right_dual_g9[3] * this.e41), (right_dual_g9[0] * this.e12) + (right_dual_g9[3] * this.e42), (right_dual_g9[1] * this.e23) + (right_dual_g9[3] * this.e43), -(right_dual_g10 * this.e45) - (right_dual_g9[2] * this.e43)) - (right_dual_g9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual_g10) * this.group2.xyz), right_dual_g9[1] * this.e42), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0[1]) * this.group1.xyz, 
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
public RoundPoint operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g1 * this.e45) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(right_dual_g1) * this.group2.xyz), right_dual_g0[1] * this.e42), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(Scalar other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint bulk_contraction(VersorEven other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g3[2] * this.e31) + (right_dual_g3[3] * this.e41), (right_dual_g3[0] * this.e12) + (right_dual_g3[3] * this.e42), (right_dual_g3[1] * this.e23) + (right_dual_g3[3] * this.e43), -(right_dual_g2[3] * this.e45) - (right_dual_g3[2] * this.e43)) - (right_dual_g3.yzxx * float4(this.group1.zxy, this.e41)) - float4((this.group2.xyz * right_dual_g2.www), right_dual_g3[1] * this.e42), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g2[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g2[3]) * this.group2.xyz), (right_dual_g2[3] * this.e4) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[3] * this.e1), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[3] * this.e2), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[3] * this.e3), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[2] * this.e435)) + (right_dual_g2.yzxw * float4(this.group0.zxy, this.e5)) - (right_dual_g2.zxyx * float4(this.group0.yzx, this.e415)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g2[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_dual_g3.yzx) - (this.group0.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[3] * this.e235) + (right_dual_g3[3] * this.e423), (right_dual_g2[3] * this.e315) + (right_dual_g3[3] * this.e431), (right_dual_g2[3] * this.e125) + (right_dual_g3[3] * this.e412), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual_g3[3]) * float4(this.group1.xyz, this.e4)) + (right_dual_g3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((right_dual_g3.yzx * this.group2.zxy), right_dual_g2[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiDualNum other) {
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
public Scalar operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * right_dual_g1.yzx) - (this.group0.yzx * right_dual_g1.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual_g1[3]) * float4(this.group1.xyz, this.e4)) + (right_dual_g1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - float4((right_dual_g1.yzx * this.group2.zxy), right_dual_g0[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - float4((right_dual_g1.zxy * this.group0.yzx), right_dual_g0[0] * this.e423), 
            /* e5 */
            -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), (right_dual_g0[3] * this.e4) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g0[3] * this.e1), (right_dual_g0[1] * this.e321) + (right_dual_g0[3] * this.e2), (right_dual_g0[2] * this.e321) + (right_dual_g0[3] * this.e3), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((this.group0.zxy * right_dual_g1.yzx), right_dual_g0[3] * this.e5) - (right_dual_g1.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual_g0[3]) * float4(this.group1.xyz, this.e4)) + (right_dual_g0.zxyx * float4(this.group2.yzx, this.e1)) + float4((right_dual_g0.yzx * this.group2.zxy * float3(-1.0)), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g2[1] * this.e412) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g2[2] * this.e423) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g2[0] * this.e431) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g1[1] * this.e431) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g1[0] * this.e423), 
            /* e5 */
            -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g0[1] * this.e425), 
            /* e5 */
            -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0[0]) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public Scalar bulk_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g1[3]) * this.group0.xyz), (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * right_dual_g1.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) + (this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g7[2] * this.e315) + (right_dual_g8[1] * this.e412) + (right_dual_g6[0] * this.e321) + (right_dual_g6[3] * this.e415), (right_dual_g7[0] * this.e125) + (right_dual_g8[2] * this.e423) + (right_dual_g6[1] * this.e321) + (right_dual_g6[3] * this.e425), (right_dual_g7[1] * this.e235) + (right_dual_g8[0] * this.e431) + (right_dual_g6[2] * this.e321) + (right_dual_g6[3] * this.e435), -(right_dual_g7[0] * this.e415) - (right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435) - (right_dual_g6[2] * this.e412)) + (float4(right_dual_g0[1]) * float4(this.group3.xyz, this.e4)) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g6[1] * this.e431) - float4((right_dual_g8.zxy * this.group0.yzx), right_dual_g6[0] * this.e423), 
            /* e5 */
            (right_dual_g0[1] * this.e5) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (right_dual_g9[3] * this.e415), (right_dual_g9[0] * this.e125) + (right_dual_g9[3] * this.e425), (right_dual_g9[1] * this.e235) + (right_dual_g9[3] * this.e435), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group1.xyz) + (this.group0.zxy * right_dual_g9.yzx) - (this.group0.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * this.e235) + (right_dual_g0[3] * this.e423), (right_dual_g1 * this.e315) + (right_dual_g0[3] * this.e431), (right_dual_g1 * this.e125) + (right_dual_g0[3] * this.e412), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual_g0[3]) * float4(this.group1.xyz, this.e4)) + (right_dual_g0.zxyx * float4(this.group2.yzx, this.e1)) + float4((right_dual_g0.yzx * this.group2.zxy * float3(-1.0)), (right_dual_g1 * this.e5) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(Scalar other) {
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
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_dual_g3.yzx) - (this.group0.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[3] * this.e235) + (right_dual_g3[3] * this.e423), (right_dual_g2[3] * this.e315) + (right_dual_g3[3] * this.e431), (right_dual_g2[3] * this.e125) + (right_dual_g3[3] * this.e412), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            (float4(right_dual_g3[3]) * float4(this.group1.xyz, this.e4)) + (right_dual_g3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) - float4((right_dual_g3.yzx * this.group2.zxy), right_dual_g2[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), (right_dual_g0[3] * this.e4) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e1) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e2) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e3) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[2] * this.e435)) + float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g0[3] * this.e5) - (right_dual_g2.zxyx * float4(this.group0.yzx, this.e415)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g2[1] * this.e425)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xx, this.e3215, this.scalar) * float4((other.group0.xyz * float3(-1.0)), right_dual_g2[3]), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, right_dual_g2[3])
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(AntiDipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4((other.group3 * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_contraction(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e321 * -1.0, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(((other.group1 * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(this.group0.xx, this.e3215) * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * other.e321 * -1.0) * float4(0.0, 0.0, 0.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public FlatPoint bulk_contraction(AntiLine other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0) * float3(this.group0.xx, this.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_dual_g0[3] * this.scalar), 
            /* e15, e25, e35, e3215 */
            right_dual_g0 * float4(this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_contraction(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((other.group0.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiPlane bulk_contraction(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiPlane bulk_contraction(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiLine bulk_contraction(Dipole other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215) * other.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            float3(this.e3215) * (other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4((other.group0.xyz * float3(-1.0)), other.e1234), 
            /* e15, e25, e35, e3215 */
            float4(((other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(this.group0.xx, this.e3215)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other.group7.xyz * float3(this.group0.xx, this.e3215) * float3(-1.0)), 0.0), 
            /* e5 */
            this.e3215 * other.e321, 
            /* e15, e25, e35, e45 */
            float4(((float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0)).xyz * float3(this.group0.xx, this.e3215)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215) * other.group4.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e4 * -1.0) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * (float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g0[1] * this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint bulk_contraction(RoundPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4((float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(BulkContractionInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiDualNum bulk_contraction(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public Scalar bulk_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiFlector bulk_contraction(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4((float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e4 * -1.0) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4((other.group0 * float4(1.0, 1.0, 1.0, -1.0)).xyz, other.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiDualNum {
    public AntiMotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.group0.xx, this.e3215, (right_dual_g0[3] * this.scalar) + (this.e3215 * other.e1234)) * float4(right_dual_g0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4((other.group1 * float4(-1.0, -1.0, -1.0, 1.0)).xyz, right_dual_g0[3])
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group0.zxy), right_dual_g1[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group0.xyz * (other.group2 * float4(1.0, 1.0, 1.0, -1.0)).www) - (right_dual_g3.xyz * this.group0.www)), (other.e321 * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_contraction(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_dual_g1.xyz, other.e321 * -1.0) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0 * float3(this.e321)), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0.xyz * this.group0.www), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiLine bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e321) * right_dual_g0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public Scalar bulk_contraction(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group0.zxy), right_dual_g1[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * right_dual_g6.xyz), 0.0) + float4((right_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_dual_g7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(((right_dual_g9.zxy * this.group0.yzx) - (right_dual_g9.yzx * this.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e4 * -1.0) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[1] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiLine bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiLine(
            /* e23, e31, e12 */
            (float3(other.e4 * -1.0) * this.group0.xyz) - (float3(this.e321) * right_dual_g0.xyz), 
            /* e15, e25, e35 */
            (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint bulk_contraction(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * (float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0)).www), (this.e321 * other.e321) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0])), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiFlector> operator /(AntiFlector lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g2[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g2[3] * this.e1), (right_dual_g1[1] * this.e321) + (right_dual_g2[3] * this.e2), (right_dual_g1[2] * this.e321) + (right_dual_g2[3] * this.e3), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + float4((right_dual_g0.zxy * this.group0.yzx), right_dual_g2[3] * this.e5) - float4((right_dual_g0.yzx * this.group0.zxy), right_dual_g1[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (other.e321 * this.e321) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((this.group0.xyz * right_dual_g2.www), right_dual_g3[0] * this.e1) - float4((right_dual_g3.xyz * this.group0.www), other.e423 * this.e235), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual_g1.xyz * this.group0.www * float3(-1.0)), (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (other.e321 * this.e321)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g1.zxy * this.group0.yzx) - (right_dual_g1.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0 * float3(this.e321)), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            (right_dual_g0 * float4(this.group0.www, this.e5)) + float4((this.group1.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual_g0.xyz * this.group0.www * float3(-1.0)), (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public Scalar bulk_contraction(Circle other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public Scalar bulk_contraction(CircleRotor other) {
        return Scalar(
            /* scalar */
            (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group0.zxy), right_dual_g1[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiPlane bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (this.e321 * other.e321) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[1]) * this.group1.xyz), 0.0) + float4((float3(this.e321) * right_dual_g6.xyz), 0.0) + float4((right_dual_g7.zxy * this.group0.yzx), 0.0) - float4((right_dual_g7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (right_dual_g0[1] * this.e5) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4(((right_dual_g9.zxy * this.group0.yzx) - (right_dual_g9.yzx * this.group0.zxy)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[1] * this.e321), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e321, this.e321, this.e321, 1.0) * float4(right_dual_g0.xyz, (right_dual_g1 * this.e5) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)) * float4(-1.0, -1.0, -1.0, 1.0)) + float4((float3(right_dual_g1) * this.group0.xyz), right_dual_g0[0] * this.e1), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiMotor bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual_g2[3] * this.e5) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (this.e321 * other.e321) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125)) + float4((this.group0.xyz * right_dual_g2.www), right_dual_g3[0] * this.e1) - (this.group0.wwwx * float4(right_dual_g3.xyz, right_dual_g0[0])), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g3.zxy * this.group0.yzx) - (right_dual_g3.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiFlector {
    public AntiFlector bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[3] * this.e1) + (right_dual_g1[0] * this.e321), (right_dual_g0[3] * this.e2) + (right_dual_g1[1] * this.e321), (right_dual_g0[3] * this.e3) + (right_dual_g1[2] * this.e321), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) + (right_dual_g0.zxyw * float4(this.group0.yzx, this.e5)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiLine> operator /(AntiLine lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g2[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * right_dual_g2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g2[3] * this.e15) - (right_dual_g3[1] * this.e12), -(right_dual_g2[3] * this.e25) - (right_dual_g3[2] * this.e23), -(right_dual_g2[3] * this.e35) - (right_dual_g3[0] * this.e31), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35)) + (right_dual_g3.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiLine bulk_contraction(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (right_dual_g1.zxyx * float4(this.group0.yzx, this.e15)) + float4((this.group0.zxy * right_dual_g1.yzx * float3(-1.0)), (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public Scalar bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * right_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (right_dual_g0.zxyx * float4(this.group0.yzx, this.e15)) + float4((this.group0.zxy * right_dual_g0.yzx * float3(-1.0)), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public Scalar bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public Scalar bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * right_dual_g9.zxy), 0.0) - float4((float3(other.e4 * -1.0) * this.group1.xyz), 0.0) - float4((this.group0.zxy * right_dual_g9.yzx), 0.0), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35), 
            /* e15, e25, e35, e45 */
            float4((this.group1.xyz * float3(right_dual_g0.yy, right_dual_g0[1])), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
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
public AntiPlane operator /(BulkContractionInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * this.e15) - (right_dual_g0[1] * this.e12), -(right_dual_g1 * this.e25) - (right_dual_g0[2] * this.e23), -(right_dual_g1 * this.e35) - (right_dual_g0[0] * this.e31), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35)) + (right_dual_g0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiLine bulk_contraction(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiPlane bulk_contraction(VersorEven other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g2[3] * this.e15) - (right_dual_g3[1] * this.e12), -(right_dual_g2[3] * this.e25) - (right_dual_g3[2] * this.e23), -(right_dual_g2[3] * this.e35) - (right_dual_g3[0] * this.e31), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35)) + (right_dual_g3.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiLine {
    public AntiMotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * right_dual_g0.www), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiMotor> operator /(AntiMotor lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((right_dual_g0 * this.group1.www) + (this.group0.xyz * right_dual_g2.www)), (right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g2[3]) * this.group1.xyz) + (float3(this.e3215) * right_dual_g1.xyz)), right_dual_g2[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual_g3.xyz, right_dual_g2[3]) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g2[3] * this.e15) - (right_dual_g3[1] * this.e12), -(right_dual_g2[3] * this.e25) - (right_dual_g3[2] * this.e23), -(right_dual_g2[3] * this.e35) - (right_dual_g3[0] * this.e31), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35)) + (right_dual_g3.zxyx * float4(this.group0.yzx, this.e15)) - float4((other.group0.xyz * this.group1.www), other.e321 * this.e3215 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e321 * this.e3215 * -1.0, 1.0) * float2(-1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual_g1.xyz * this.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_dual_g1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35)) - float4((right_dual_g1.yzx * this.group0.zxy), other.e321 * this.e3215 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((right_dual_g0 * this.group1.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e3215) * right_dual_g0.xyz)), right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((right_dual_g0.xyz * this.group1.www * float3(-1.0)), 0.0), 
            /* e1, e2, e3, e5 */
            (right_dual_g0.zxyx * float4(this.group0.yzx, this.e15)) + float4((right_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiPlane bulk_contraction(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.group0.xyz * this.group1.www * float3(-1.0)), this.e3215 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiPlane bulk_contraction(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215) * float4(other.group0.xyz, other.e321 * -1.0) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual_g0 * float3(this.e3215)), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((right_dual_g1.xyz * this.group1.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual_g0 * float3(this.e3215)), (this.e3215 * other.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((right_dual_g1.xyz * this.group1.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[1] * this.scalar) + (this.e3215 * other.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g9.zxy * this.group0.yzx), 0.0) - float4((float3(right_dual_g10) * this.group1.xyz), 0.0) - float4((float3(this.e3215) * other.group7.xyz), 0.0) - float4((right_dual_g9.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (this.e3215 * other.e321), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual_g0[1]) * this.group1.xyz) + (float3(this.e3215) * right_dual_g6.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) + (float3(right_dual_g0[1]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g10 * this.e3215) * float4(0.0, 0.0, 0.0, -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215) * right_dual_g9.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g0[1] * this.e3215), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual_g0.xyz, right_dual_g1) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * this.e15) - (right_dual_g0[1] * this.e12), -(right_dual_g1 * this.e25) - (right_dual_g0[2] * this.e23), -(right_dual_g1 * this.e35) - (right_dual_g0[0] * this.e31), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35)) + (right_dual_g0.zxyx * float4(this.group0.yzx, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public Scalar bulk_contraction(Sphere other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiFlector bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215) * float4(right_dual_g3.xyz, right_dual_g2[3]) * float4(-1.0), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g0[0] * this.e3215) - (right_dual_g2[3] * this.e15), -(right_dual_g0[1] * this.e3215) - (right_dual_g2[3] * this.e25), -(right_dual_g0[2] * this.e3215) - (right_dual_g2[3] * this.e35), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35)) + (right_dual_g3.zxyx * float4(this.group0.yzx, this.e15)) - float4((right_dual_g3.yzx * this.group0.zxy), this.e3215 * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiMotor {
    public AntiMotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(right_dual_g0[3]) * this.group0) + (float4(this.e3215) * float4(right_dual_g0.xyz, other.e1234)) + float4(float3(0.0), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e3215) * right_dual_g1.xyz)), right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiPlane> operator /(AntiPlane lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public Scalar bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) - (this.e5 * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.xyz * float3(right_dual_g0.yy, right_dual_g0[1])), 0.0), 
            /* e5 */
            right_dual_g0[1] * this.e5, 
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
public Scalar operator /(BulkContractionInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public Scalar bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public Scalar bulk_contraction(VersorEven other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiPlane {
    public AntiPlane bulk_contraction(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<AntiScalar> operator /(AntiScalar lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public CircleRotor bulk_contraction(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * other.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public DipoleInversion bulk_contraction(AntiDipoleInversion other) {
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
public DualNum operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public DualNum bulk_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Flector bulk_contraction(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Line bulk_contraction(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Motor bulk_contraction(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Plane bulk_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Dipole bulk_contraction(Circle other) {
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
public AntiCircleRotor operator /(BulkContractionInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
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
public Circle operator /(BulkContractionInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Circle bulk_contraction(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
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
public AntiDualNum operator /(BulkContractionInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiDualNum bulk_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiFlatPoint bulk_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiFlector bulk_contraction(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiLine bulk_contraction(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiMotor bulk_contraction(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public MultiVector bulk_contraction(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e12345) * other.group0.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e12345 * other.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e12345 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkContractionInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public Sphere bulk_contraction(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e12345 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(BulkContractionInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public AntiScalar bulk_contraction(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public RoundPoint bulk_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public VersorOdd bulk_contraction(VersorEven other) {
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
public VersorEven operator /(BulkContractionInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension AntiScalar {
    public VersorEven bulk_contraction(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Circle> operator /(Circle lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g2[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g2[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g1[1] * this.e315) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g1[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_dual_g3.yzx) - (this.group0.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[3] * this.e235) + (right_dual_g3[3] * this.e423), (right_dual_g2[3] * this.e315) + (right_dual_g3[3] * this.e431), (right_dual_g2[3] * this.e125) + (right_dual_g3[3] * this.e412), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group2.zxy * right_dual_g3.yzx), right_dual_g2[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkContractionInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Circle bulk_contraction(AntiDualNum other) {
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
public Scalar operator /(BulkContractionInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Scalar bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * right_dual_g1.yzx) - (this.group0.yzx * right_dual_g1.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g1[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g1[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g1[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - float4((this.group2.zxy * right_dual_g1.yzx), right_dual_g0[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public RoundPoint bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - float4((right_dual_g1.zxy * this.group0.yzx), right_dual_g0[0] * this.e423), 
            /* e5 */
            -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - float4((this.group0.yzx * right_dual_g1.zxy), right_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Dipole bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Dipole(
            /* e41, e42, e43 */
            (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group2.yzx * right_dual_g0.zxy) - (this.group2.zxy * right_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Scalar bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Scalar bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public RoundPoint bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g2[1] * this.e412) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g2[2] * this.e423) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g2[0] * this.e431) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g0[1] * this.e425), 
            /* e5 */
            -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g0[1] * this.e425), 
            /* e5 */
            -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiLine bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0[0]) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(right_dual_g0[0]) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiPlane bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_dual_g0.zxy), right_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiPlane bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_dual_g0.zxy), right_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Scalar bulk_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(right_dual_g1[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((this.group1.xyz * right_dual_g1.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e321 * other.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g7[2] * this.e315) + (right_dual_g8[1] * this.e412) + (right_dual_g6[0] * this.e321) + (right_dual_g6[3] * this.e415), (right_dual_g7[0] * this.e125) + (right_dual_g8[2] * this.e423) + (right_dual_g6[1] * this.e321) + (right_dual_g6[3] * this.e425), (right_dual_g7[1] * this.e235) + (right_dual_g8[0] * this.e431) + (right_dual_g6[2] * this.e321) + (right_dual_g6[3] * this.e435), -(right_dual_g7[0] * this.e415) - (right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435) - (right_dual_g6[2] * this.e412)) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g6[1] * this.e431) - float4((right_dual_g8.zxy * this.group0.yzx), right_dual_g6[0] * this.e423), 
            /* e5 */
            -(right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (right_dual_g9[3] * this.e415), (right_dual_g9[0] * this.e125) + (right_dual_g9[3] * this.e425), (right_dual_g9[1] * this.e235) + (right_dual_g9[3] * this.e435), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group1.xyz) + (this.group0.zxy * right_dual_g9.yzx) - (this.group0.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[1]) * this.group1, 
            /* e423, e431, e412 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Dipole bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * this.e235) + (right_dual_g0[3] * this.e423), (right_dual_g1 * this.e315) + (right_dual_g0[3] * this.e431), (right_dual_g1 * this.e125) + (right_dual_g0[3] * this.e412), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35 */
            (float3(right_dual_g0[3]) * this.group1.xyz) + (this.group2.yzx * right_dual_g0.zxy) - (this.group2.zxy * right_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkContractionInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public Circle bulk_contraction(Scalar other) {
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
public AntiCircleRotor operator /(BulkContractionInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group1.xyz) + (this.group0.zxy * right_dual_g3.yzx) - (this.group0.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[3] * this.e235) + (right_dual_g3[3] * this.e423), (right_dual_g2[3] * this.e315) + (right_dual_g3[3] * this.e431), (right_dual_g2[3] * this.e125) + (right_dual_g3[3] * this.e412), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) - float4((this.group2.zxy * right_dual_g3.yzx), right_dual_g2[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Circle {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g1[0] * this.e235) - float4((this.group2.zxy * right_dual_g0.yzx), right_dual_g1[1] * this.e315)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<CircleRotor> operator /(CircleRotor lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(this.e12345)) + (float3(right_dual_g2[3]) * this.group0.xyz)), right_dual_g2[3] * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g2[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            float4((right_dual_g2[0] * this.e12345) + (right_dual_g2[3] * this.e235), (right_dual_g2[1] * this.e12345) + (right_dual_g2[3] * this.e315), (right_dual_g2[2] * this.e12345) + (right_dual_g2[3] * this.e125), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g0[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g2[3] * this.e415) + (right_dual_g3[1] * this.e412) + (other.e423 * this.e12345), (right_dual_g2[3] * this.e425) + (right_dual_g3[2] * this.e423) + (other.e431 * this.e12345), (right_dual_g2[3] * this.e435) + (right_dual_g3[0] * this.e431) + (other.e412 * this.e12345), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group0.yzx * right_dual_g3.zxy), right_dual_g2[0] * this.e423), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1[0] * this.e12345) + (right_dual_g2[3] * this.e235), (right_dual_g1[1] * this.e12345) + (right_dual_g2[3] * this.e315), (right_dual_g1[2] * this.e12345) + (right_dual_g2[3] * this.e125), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g1[3] * this.e12345) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group2.yzx) - (right_dual_g3.yzx * this.group2.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(other.group0.yy, other.scalar, other.e3215), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g0[3] * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g0.xyz * this.group2.www), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((this.group0.zxy * right_dual_g1.yzx) - (this.group0.yzx * right_dual_g1.zxy)), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g0[3] * this.e12345) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_dual_g0.xyz), 0.0) + float4((right_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_dual_g1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * this.group2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g1 * float3(this.e12345)), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - float4((right_dual_g1.zxy * this.group0.yzx), right_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e321), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435))) + (this.group2 * float4(right_dual_g0.www, right_dual_g1[3])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - float4((this.group0.yzx * right_dual_g1.zxy), right_dual_g0[0] * this.e423)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public DipoleInversion bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_dual_g0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(this.e12345)), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g2[1] * this.e412) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g2[2] * this.e423) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g2[0] * this.e431) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g2[0] * this.e415) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g2[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412) + (right_dual_g3[0] * this.e12345), (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423) + (right_dual_g3[1] * this.e12345), (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431) + (right_dual_g3[2] * this.e12345), -(right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) + (this.group2.yzxw * float4(right_dual_g0.zxy, right_dual_g3[3])) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g1[1] * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0.xx, right_dual_g0[0], right_dual_g0[1]) * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0[0]) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiFlector bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((this.group0.yzx * right_dual_g0.zxy), right_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiFlector bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[3] * this.e415) + (right_dual_g1[0] * this.e12345), (right_dual_g0[3] * this.e425) + (right_dual_g1[1] * this.e12345), (right_dual_g0[3] * this.e435) + (right_dual_g1[2] * this.e12345), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + float4((this.group0.zxy * right_dual_g0.yzx), right_dual_g1[3] * this.e12345) - float4((this.group0.yzx * right_dual_g0.zxy), right_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * this.group2.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412))) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g0[3] * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), right_dual_g1[3] * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * this.e12345) - (right_dual_g3_w * this.e321) - (this.e423 * other.e235) - (this.e431 * other.e315) - (this.e412 * other.e125) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), right_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g7[2] * this.e315) + (right_dual_g1[0] * this.e12345) + (right_dual_g6[0] * this.e321) + (right_dual_g6[3] * this.e415), (right_dual_g7[0] * this.e125) + (right_dual_g1[1] * this.e12345) + (right_dual_g6[1] * this.e321) + (right_dual_g6[3] * this.e425), (right_dual_g7[1] * this.e235) + (right_dual_g1[2] * this.e12345) + (right_dual_g6[2] * this.e321) + (right_dual_g6[3] * this.e435), -(right_dual_g7[0] * this.e415) - (right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435) - (right_dual_g6[2] * this.e412)) + float4((right_dual_g8.yzx * this.group0.zxy), right_dual_g1[3] * this.e12345) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g6[1] * this.e431) - float4((right_dual_g8.zxy * this.group0.yzx), right_dual_g6[0] * this.e423), 
            /* e5 */
            (this.e12345 * other.e3215) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (this.e12345 * other.e235), (right_dual_g9[0] * this.e125) + (this.e12345 * other.e315), (right_dual_g9[1] * this.e235) + (this.e12345 * other.e125), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + float4((this.group1.xyz * right_dual_g9.www), right_dual_g3_w * this.e12345) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (this.group0.zxy * right_dual_g9.yzx) - (this.group0.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group1), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public DipoleInversion bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(right_dual_g1) * this.group1.xyz) + (this.group0.zxy * right_dual_g0.yzx) - (this.group0.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * this.e235) + (right_dual_g0[3] * this.e423), (right_dual_g1 * this.e315) + (right_dual_g0[3] * this.e431), (right_dual_g1 * this.e125) + (right_dual_g0[3] * this.e412), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (right_dual_g0.zxy * this.group2.yzx) - (right_dual_g0.yzx * this.group2.zxy)), right_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(BulkContractionInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public CircleRotor bulk_contraction(Scalar other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public RoundPoint bulk_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorOdd bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g0[0] * this.e12345) + (right_dual_g2[3] * this.e415), (right_dual_g0[1] * this.e12345) + (right_dual_g2[3] * this.e425), (right_dual_g0[2] * this.e12345) + (right_dual_g2[3] * this.e435), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) + float4((this.group0.zxy * right_dual_g3.yzx), right_dual_g0[3] * this.e12345) - float4((this.group0.yzx * right_dual_g3.zxy), right_dual_g2[0] * this.e423), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1[0] * this.e12345) + (right_dual_g2[3] * this.e235), (right_dual_g1[1] * this.e12345) + (right_dual_g2[3] * this.e315), (right_dual_g1[2] * this.e12345) + (right_dual_g2[3] * this.e125), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g1[3] * this.e12345) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group2.yzx) - (right_dual_g3.yzx * this.group2.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension CircleRotor {
    public VersorEven bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_dual_g2.xyz, -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435))) + (this.group2 * float4(right_dual_g0.www, right_dual_g2[3])), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g3[0] * this.e12345), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g3[1] * this.e12345), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g3[2] * this.e12345), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[2] * this.e412)) + float4((this.group0.zxy * right_dual_g2.yzx), right_dual_g3[3] * this.e12345) - float4((this.group0.yzx * right_dual_g2.zxy), right_dual_g1[0] * this.e423) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g1[1] * this.e431)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Dipole> operator /(Dipole lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g2[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g2[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g3[2] * this.e31) + (right_dual_g3[3] * this.e41), (right_dual_g3[0] * this.e12) + (right_dual_g3[3] * this.e42), (right_dual_g3[1] * this.e23) + (right_dual_g3[3] * this.e43), -(right_dual_g2[3] * this.e45) - (right_dual_g3[2] * this.e43)) - (right_dual_g3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual_g2.www), right_dual_g3[0] * this.e41), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Dipole bulk_contraction(AntiDualNum other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[2] * this.e31) + (right_dual_g1[3] * this.e41), (right_dual_g1[0] * this.e12) + (right_dual_g1[3] * this.e42), (right_dual_g1[1] * this.e23) + (right_dual_g1[3] * this.e43), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) - (right_dual_g1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (right_dual_g1[0] * this.e15) + (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Scalar bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public RoundPoint bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Scalar bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Scalar bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public AntiPlane bulk_contraction(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Scalar bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Scalar bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public AntiPlane bulk_contraction(Motor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_dual_g1.www), right_dual_g1[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g9[2] * this.e31) + (right_dual_g9[3] * this.e41), (right_dual_g9[0] * this.e12) + (right_dual_g9[3] * this.e42), (right_dual_g9[1] * this.e23) + (right_dual_g9[3] * this.e43), -(right_dual_g10 * this.e45) - (right_dual_g9[2] * this.e43)) - (right_dual_g9.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual_g10) * this.group2.xyz), right_dual_g9[0] * this.e41), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(right_dual_g0[1]) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g0[1]) * this.group1.xyz, 
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
public RoundPoint operator /(BulkContractionInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public RoundPoint bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g1 * this.e45) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxy * float4(this.group1.zxy, this.e42)) - float4((float3(right_dual_g1) * this.group2.xyz), right_dual_g0[0] * this.e41), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public Dipole bulk_contraction(Scalar other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public RoundPoint bulk_contraction(VersorEven other) {
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g3[2] * this.e31) + (right_dual_g3[3] * this.e41), (right_dual_g3[0] * this.e12) + (right_dual_g3[3] * this.e42), (right_dual_g3[1] * this.e23) + (right_dual_g3[3] * this.e43), -(right_dual_g2[3] * this.e45) - (right_dual_g3[2] * this.e43)) - (right_dual_g3.yzxy * float4(this.group1.zxy, this.e42)) - float4((this.group2.xyz * right_dual_g2.www), right_dual_g3[0] * this.e41), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Dipole {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g0[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(right_dual_g0[3]) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g0[3]) * this.group2.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g0[2] * this.e4315) + (right_dual_g1[0] * this.e1234) + (right_dual_g2[3] * this.e41), (right_dual_g0[0] * this.e4125) + (right_dual_g1[1] * this.e1234) + (right_dual_g2[3] * this.e42), (right_dual_g0[1] * this.e4235) + (right_dual_g1[2] * this.e1234) + (right_dual_g2[3] * this.e43), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((right_dual_g0.yzx * this.group3.zxy), right_dual_g0[0] * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[0] * this.e1234) + (right_dual_g2[3] * this.e23), (right_dual_g2[1] * this.e1234) + (right_dual_g2[3] * this.e31), (right_dual_g2[2] * this.e1234) + (right_dual_g2[3] * this.e12), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g0 * this.group3.www), right_dual_g2[3] * this.e45) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group3.zxy) - (right_dual_g2.zxy * this.group3.yzx)), right_dual_g2[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual_g3.www, other.e423)) + float4(float3(0.0), (right_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_dual_g2[3] * this.e45) - (right_dual_g3[1] * this.e42) - (right_dual_g3[2] * this.e43)) - float4((right_dual_g3.xyz * this.group3.www), right_dual_g3[0] * this.e41), 
            /* e1, e2, e3, e5 */
            (right_dual_g3.zxyw * this.group1.yzxw) + (this.group2.wwwz * float4(right_dual_g2.xyz, right_dual_g3[2])) + float4((this.group0.xyz * right_dual_g3.www), right_dual_g3[0] * this.e15) + float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g3[1] * this.e25) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0])) - float4((right_dual_g3.yzx * this.group1.zxy), right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.scalar, other.e3215) * float4(this.group0.xyz, this.e1234), 
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
public RoundPoint operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1.yzx * this.group3.zxy) - (right_dual_g1.zxy * this.group3.yzx)), right_dual_g1[3] * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((this.group3.xyz * right_dual_g1.www), right_dual_g0[3] * this.e1234) - (right_dual_g1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g1.zxyx * float4(this.group1.yzx, this.e15)) + (this.group2.wwwy * float4(right_dual_g0.xyz, right_dual_g1[1])) + float4(float3(0.0), (right_dual_g1[2] * this.e35) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)) - float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g0[0] * this.e4235)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * float3(this.e1234)), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) - float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g1[0] * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43))) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g1[3] * this.e1234), 
            /* e23, e31, e12, e45 */
            (float4(right_dual_g0[3]) * this.group1) + (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group3.zxy) - (right_dual_g1.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e235, e315, e125, e4 */
            float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + float4((right_dual_g0.yzx * this.group1.zxy * float3(-1.0)), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxy * float4(right_dual_g1.zxy, other.e431)) + float4((other.group2.xyz * this.group2.www), other.e423 * this.e4235), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public RoundPoint bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxx * float4(right_dual_g1.zxy, other.e423)) + float4((right_dual_g2.xyz * this.group2.www), other.e431 * this.e4315), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g2[1] * this.e4125) + (right_dual_g1[0] * this.e3215), (right_dual_g2[2] * this.e4235) + (right_dual_g1[1] * this.e3215), (right_dual_g2[0] * this.e4315) + (right_dual_g1[2] * this.e3215), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group3.zxyx * float4(right_dual_g2.yzx, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiFlector bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[0]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[0]) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiMotor bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45))) - float4((this.group3.xyz * right_dual_g0.www), right_dual_g0[0] * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiMotor bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) + float4((right_dual_g0.xyz * this.group2.www), right_dual_g1[0] * this.e4235) - float4((this.group3.xyz * right_dual_g0.www), right_dual_g0[0] * this.e41), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiPlane bulk_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((this.e1234 * other.e235) + (this.e4315 * other.e435), (this.e1234 * other.e315) + (this.e4125 * other.e415), (this.e1234 * other.e125) + (this.e4235 * other.e425), -(this.e4315 * other.e315) - (this.e4125 * other.e125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiFlector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g1[3]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e4315) + (right_dual_g1[0] * this.e1234), (right_dual_g0[0] * this.e4125) + (right_dual_g1[1] * this.e1234), (right_dual_g0[1] * this.e4235) + (right_dual_g1[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((this.group0.xyz * right_dual_g1.www), right_dual_g1[3] * this.e45) - (this.group3.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (this.e1234 * other.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (right_dual_g3 * float4(this.e1234)) + (this.group3.yzxy * float4(other.group6.zxy, other.e431)) + float4((this.group0.xyz * right_dual_g9.www), this.e4235 * other.e423) + float4((right_dual_g9.zxy * this.group1.yzx), this.e4125 * other.e412) - (float4(right_dual_g10) * float4(this.group2.xyz, this.e45)) - (right_dual_g9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group7.xyz * this.group3.www), right_dual_g9[0] * this.e41) - float4((this.group3.zxy * other.group6.yzx), right_dual_g9[1] * this.e42), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45) - (right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125)) + (float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45)) - float4((right_dual_g8.zxy * this.group3.yzx), right_dual_g6[0] * this.e4235), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[1]) * this.group0.xyz) + (float3(this.e1234) * right_dual_g6.xyz) + (right_dual_g7.zxy * this.group3.yzx) - (right_dual_g7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group1.xyz) - (float3(right_dual_g6[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g9.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g9.zxy, right_dual_g10)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g9.xyz) - (float3(right_dual_g10) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g9[3]) * this.group3.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * this.group3, 
            /* e1234 */
            right_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public Scalar bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g1) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g0.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g0.zxy, right_dual_g1)), 
            /* e235, e315, e125, e4 */
            float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g1 * this.e45) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * this.e15) - (right_dual_g0[1] * this.e12), -(right_dual_g1 * this.e25) - (right_dual_g0[2] * this.e23), -(right_dual_g1 * this.e35) - (right_dual_g0[0] * this.e31), (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35)) + (float4(right_dual_g0[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g0.zxyx * float4(this.group1.yzx, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public DipoleInversion bulk_contraction(Scalar other) {
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
public Scalar operator /(BulkContractionInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public Scalar bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) - (right_dual_g2[3] * this.e45) - (right_dual_g3[1] * this.e42) - (right_dual_g3[2] * this.e43)) + float4((this.group3.xyz * right_dual_g3.www), right_dual_g1[3] * this.e1234) - (right_dual_g3.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g3.zxyx * float4(this.group1.yzx, this.e15)) + (this.group2.wwwy * float4(right_dual_g2.xyz, right_dual_g3[1])) + float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g3[2] * this.e35) - (float4(this.e3215) * float4(right_dual_g0.xyz, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyz * float4(right_dual_g1.yzx, right_dual_g2[2])) - float4((right_dual_g3.yzx * this.group1.zxy), right_dual_g2[0] * this.e4235)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DipoleInversion {
    public VersorOdd bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group3.yzxy * float4(right_dual_g0.zxy, right_dual_g3[1])) + float4(float3(0.0), (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g2[3] * this.e1234) + float4((right_dual_g1.xyz * this.group2.www), right_dual_g3[0] * this.e4235) - float4((right_dual_g0.yzx * this.group3.zxy), right_dual_g2[0] * this.e41), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + (float4(right_dual_g0[3]) * this.group1) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group3.zxy) - (right_dual_g2.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<DualNum> operator /(DualNum lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4((other.group0.xyz * float3(-1.0)), right_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            right_dual_g2 * float4(this.group0.yy, this.e12345, this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.yy, this.e12345, this.e5) * float4(other.group0.xyz, right_dual_g2[3]), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            right_dual_g2 * float4(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public DualNum bulk_contraction(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e3215 * this.e12345) + (other.scalar * this.e5), other.scalar * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Flector bulk_contraction(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkContractionInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Line bulk_contraction(AntiLine other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Motor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            right_dual_g0 * float4(this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.group0.yy, this.e12345, (right_dual_g0[3] * this.e5) + (right_dual_g1[3] * this.e12345)) * float4(right_dual_g1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkContractionInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Plane bulk_contraction(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Dipole bulk_contraction(Circle other) {
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
public AntiCircleRotor operator /(BulkContractionInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
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
public Circle operator /(BulkContractionInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public Circle bulk_contraction(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz * float3(-1.0), 
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
public AntiDualNum operator /(BulkContractionInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiDualNum bulk_contraction(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * float2(other.e5 * -1.0, other.e12345 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiFlatPoint bulk_contraction(FlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * float4(other.e15 * -1.0, other.e25 * -1.0, other.e35 * -1.0, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiFlector bulk_contraction(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiLine bulk_contraction(Line other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiMotor bulk_contraction(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345), right_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            (right_dual_g0[1] * this.e5) + (this.e12345 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group8.xyz, other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group7.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            right_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public VersorOdd bulk_contraction(RoundPoint other) {
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), right_dual_g1 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), right_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public DualNum bulk_contraction(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public RoundPoint bulk_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public VersorOdd bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.group0.yy, this.e12345, (right_dual_g0[3] * this.e12345) + (right_dual_g2[3] * this.e5)) * float4(right_dual_g0.xyz, 1.0), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            right_dual_g2 * float4(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension DualNum {
    public VersorEven bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group0.yy, this.e12345, (right_dual_g0[3] * this.e5) + (right_dual_g2[3] * this.e12345)) * float4(right_dual_g2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<FlatPoint> operator /(FlatPoint lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g2[3] * this.e45), 
            /* e15, e25, e35, scalar */
            float4((this.group0.xyz * right_dual_g2.www), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4 * -1.0) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public DualNum bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return DualNum(
            /* e5, e12345 */
            float2((right_dual_g1[0] * this.e15) + (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(AntiMotor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public DualNum bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DualNum(
            /* e5, e12345 */
            float2((right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public Scalar bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public Scalar bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (other.e45 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public DualNum bulk_contraction(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e5 * this.e45 * -1.0, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<FlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public Scalar bulk_contraction(FlatPoint other) {
        return Scalar(
            /* scalar */
            other.e45 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public Scalar bulk_contraction(Flector other) {
        return Scalar(
            /* scalar */
            this.e45 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public DualNum bulk_contraction(Motor other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e45 * other.e5 * -1.0, 1.0) * float2(1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (this.e45 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e4 * -1.0) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4(other.group0.yx[1]) * this.group0, 
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
public RoundPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4 * -1.0) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public FlatPoint bulk_contraction(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public RoundPoint bulk_contraction(VersorEven other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e4 * -1.0) * this.group0 * float4(-1.0), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g0[3] * this.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (this.e45 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Flector> operator /(Flector lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy)), -(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[3] * this.e45)), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g0 * this.group1.www), right_dual_g2[3] * this.e45) - (right_dual_g1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g2[3]) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group1.zxy), 0.0) - float4((right_dual_g2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g3.yzx * this.group1.zxy) - (right_dual_g3.zxy * this.group1.yzx)), right_dual_g2[3] * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual_g3.www, other.e423)) + float4(float3(0.0), (other.e431 * this.e4315) + (other.e412 * this.e4125)) - float4((right_dual_g3.xyz * this.group1.www), right_dual_g2[3] * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g2[2] * this.e4125)) + float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Flector bulk_contraction(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Motor bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual_g1.yzx * this.group1.zxy) - (right_dual_g1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (right_dual_g1.wwwx * float4(this.group1.xyz, this.e15)) + float4(float3(0.0), (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)) - (this.group1.wwwx * float4(right_dual_g1.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public FlatPoint bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Flector bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + (float4(right_dual_g0[3]) * this.group0) - (this.group1.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Motor bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0.xyz, (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)) * float4(-1.0, -1.0, -1.0, 1.0)) + (right_dual_g0.wwwx * float4(this.group1.xyz, this.e15))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public RoundPoint bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group1.yzxx * float4(right_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public RoundPoint bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group1.yzxx * float4(right_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g2[1] * this.e4125) + (right_dual_g1[0] * this.e3215), (right_dual_g2[2] * this.e4235) + (right_dual_g1[1] * this.e3215), (right_dual_g2[0] * this.e4315) + (right_dual_g1[2] * this.e3215), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[3] * this.e45)) - float4((right_dual_g2.zxy * this.group1.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group1.zxyx * float4(right_dual_g2.yzx, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[3] * this.e45)) - float4((right_dual_g2.zxy * this.group1.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiFlector bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.group1.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), right_dual_g0[0] * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiMotor bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0[3]) * float4(this.group1.xyz, this.e45) * float4(-1.0), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiMotor bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group1.xyz * right_dual_g0.www * float3(-1.0)), (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) - (right_dual_g0[3] * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiPlane bulk_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e4315 * other.e435, this.e4125 * other.e415, this.e4235 * other.e425, -(this.e4315 * other.e315) - (this.e4125 * other.e125)) - (this.group1.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiFlector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.group1.xyz * right_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g0.zxy * this.group1.yzx), right_dual_g1[3] * this.e45) - (this.group1.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g10 * this.e15) - (this.e4125 * other.e425), -(right_dual_g10 * this.e25) - (this.e4235 * other.e435), -(right_dual_g10 * this.e35) - (this.e4315 * other.e415), (this.e4315 * other.e431) + (this.e4125 * other.e412)) + (this.group1.yzxx * float4(other.group6.zxy, other.e423)) - float4((other.group7.xyz * this.group1.www), right_dual_g10 * this.e45), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45) + (this.e3215 * other.e321) - (this.e4235 * other.e235) - (this.e4315 * other.e315) - (this.e4125 * other.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125)) + (float4(right_dual_g0[1]) * this.group0) - float4((right_dual_g8.zxy * this.group1.yzx), right_dual_g6[0] * this.e4235), 
            /* e41, e42, e43 */
            (right_dual_g7.zxy * this.group1.yzx) - (right_dual_g7.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) - (float3(right_dual_g6[3]) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g9.yzx * this.group1.zxy) - (right_dual_g9.zxy * this.group1.yzx)), right_dual_g10 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_dual_g10) * this.group1.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_dual_g9[3]) * this.group1.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Scalar bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g1) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx)), right_dual_g1 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) - (float3(this.e3215) * right_dual_g0.xyz)), right_dual_g1 * this.e45 * -1.0), 
            /* e1, e2, e3, e5 */
            float4((float3(right_dual_g1) * this.group0.xyz * float3(-1.0)), (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Flector bulk_contraction(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public Scalar bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group1.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g3.yzx * this.group1.zxy) - (right_dual_g3.zxy * this.group1.yzx)), right_dual_g2[3] * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(right_dual_g3.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)) - float4((right_dual_g3.xyz * this.group1.www), right_dual_g2[3] * this.e45), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g1[3] * this.e3215)) + float4((right_dual_g1.zxy * this.group1.yzx), right_dual_g3[0] * this.e15) - (right_dual_g2.wwwx * float4(this.group0.xyz, this.e4235)) - (this.group1.zxyy * float4(right_dual_g1.yzx, right_dual_g2[1])) - (this.group1.wwwz * float4(right_dual_g0.xyz, right_dual_g2[2]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Flector {
    public VersorOdd bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group1.yzxx * float4(right_dual_g0.zxy, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[3] * this.e45)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (right_dual_g0 * float4(this.group1.www, this.e45)) + float4(float3(0.0), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group1.zxy), 0.0) - float4((right_dual_g2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group1
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Line> operator /(Line lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiDipoleInversion bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * right_dual_g2.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g2[3]) * this.group1.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) - float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g2[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiCircleRotor bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g2[3]) * this.group1.xyz), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e431 * this.e315) - (other.e412 * this.e125)) - float4((this.group1.zxy * right_dual_g3.yzx), other.e423 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkContractionInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Line bulk_contraction(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public FlatPoint bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g1[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g1[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - (right_dual_g1.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public DualNum bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Motor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((this.group0.xyz * right_dual_g0.www), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public FlatPoint bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Scalar bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Scalar bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public RoundPoint bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g0[0] * this.e415), 
            /* e5 */
            -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public RoundPoint bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g0[0] * this.e415), 
            /* e5 */
            -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public FlatPoint bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((this.group0.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Line> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiPlane bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiPlane bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(right_dual_g0[3]) * this.group0.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Scalar bulk_contraction(Line other) {
        return Scalar(
            /* scalar */
            -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((this.group0.xyz * (other.group1 * float4(1.0, 1.0, 1.0, -1.0)).www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g7[2] * this.e315) + (right_dual_g6[3] * this.e415), (right_dual_g7[0] * this.e125) + (right_dual_g6[3] * this.e425), (right_dual_g7[1] * this.e235) + (right_dual_g6[3] * this.e435), -(right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435)) - float4((right_dual_g7.yzx * this.group1.zxy), right_dual_g7[0] * this.e415), 
            /* e5 */
            -(right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (right_dual_g9[3] * this.e415), (right_dual_g9[0] * this.e125) + (right_dual_g9[3] * this.e425), (right_dual_g9[1] * this.e235) + (right_dual_g9[3] * this.e435), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) - (right_dual_g9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            float3(right_dual_g10) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(right_dual_g10) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * float3(right_dual_g0.yy, right_dual_g0[1])), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[1]) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Dipole bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return Dipole(
            /* e41, e42, e43 */
            float3(right_dual_g1) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g1) * this.group1.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e15, e25, e35 */
            (float3(right_dual_g0[3]) * this.group0.xyz) + (this.group1.yzx * right_dual_g0.zxy) - (this.group1.zxy * right_dual_g0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkContractionInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public Line bulk_contraction(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiCircleRotor bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(right_dual_g2[3]) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g2[3]) * this.group1.xyz), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - float4((this.group1.zxy * right_dual_g3.yzx), right_dual_g1[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Line {
    public AntiDipoleInversion bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((this.group0.xyz * right_dual_g0.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) - float4((this.group1.zxy * right_dual_g0.yzx), right_dual_g2[0] * this.e415)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Motor> operator /(Motor lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(right_dual_g0, right_dual_g2[3]), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g2[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), right_dual_g1[3] * this.e12345), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(this.group0.www, this.e5)) + float4((this.group1.xyz * right_dual_g2.www), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g0[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((other.group0.xyz * this.group0.www) + (this.group0.xyz * right_dual_g2.www)), (right_dual_g2[3] * this.e5) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125)), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.e12345)) + float4((this.group1.xyz * right_dual_g2.www), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Motor bulk_contraction(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.scalar, (other.e3215 * this.e12345) + (other.scalar * this.e5)) * float4(this.group1.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public FlatPoint bulk_contraction(AntiFlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group0.xyz, other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Flector bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g1[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g1[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + (float4(this.e12345) * other.group0 * float4(1.0, 1.0, 1.0, -1.0)) - (right_dual_g1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Motor bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((right_dual_g0 * this.group0.www), 0.0), 
            /* e235, e315, e125, e5 */
            float4((right_dual_g1 * float3(this.e12345)), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Motor bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e12345), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Flector bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiCircleRotor bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(this.e12345)), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group1.zxy), right_dual_g2[0] * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[3] * this.e415) + (right_dual_g3[0] * this.e12345), (right_dual_g1[3] * this.e425) + (right_dual_g3[1] * this.e12345), (right_dual_g1[3] * this.e435) + (right_dual_g3[2] * this.e12345), -(right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) + float4((right_dual_g0.zxy * this.group1.yzx), right_dual_g3[3] * this.e12345) - (this.group1.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiMotor bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_dual_g0[1] * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0[0]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiFlector bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiFlector bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            (float4(this.e12345, this.e12345, this.e12345, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435))) + (this.group0 * float4(right_dual_g0.www, right_dual_g1[3]))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiMotor bulk_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * this.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * right_dual_g0.xyz), (right_dual_g0[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), right_dual_g1[3] * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) - (this.e415 * other.e415) - (this.e425 * other.e425) - (this.e435 * other.e435) - (this.e235 * other.e423) - (this.e315 * other.e431) - (this.e125 * other.e412), right_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * this.e12345) + (right_dual_g6[3] * this.e415), (right_dual_g1[1] * this.e12345) + (right_dual_g6[3] * this.e425), (right_dual_g1[2] * this.e12345) + (right_dual_g6[3] * this.e435), -(right_dual_g7[1] * this.e425) - (right_dual_g7[2] * this.e435)) + float4((right_dual_g7.zxy * this.group1.yzx), right_dual_g1[3] * this.e12345) - float4((right_dual_g7.yzx * this.group1.zxy), right_dual_g7[0] * this.e415), 
            /* e5 */
            (right_dual_g0[1] * this.e5) + (this.e12345 * other.e3215) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (this.e12345 * other.e235), (right_dual_g9[0] * this.e125) + (this.e12345 * other.e315), (right_dual_g9[1] * this.e235) + (this.e12345 * other.e125), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + (this.group0 * float4(right_dual_g9.www, other.e321 * -1.0)) - (right_dual_g9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group6.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[1]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g6.xyz)), right_dual_g6[3] * this.e12345), 
            /* e423, e431, e412 */
            right_dual_g7 * float3(this.e12345), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public VersorOdd bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(right_dual_g1) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4((float3(right_dual_g1) * this.group1.xyz), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (right_dual_g0.zxy * this.group1.yzx) - (right_dual_g0.yzx * this.group1.zxy)), right_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public Motor bulk_contraction(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public RoundPoint bulk_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e12345 * other.e3215
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public VersorOdd bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0 * float4(right_dual_g2.www, right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + float4((right_dual_g0.xyz * this.group0.www), right_dual_g2[3] * this.e5), 
            /* e23, e31, e12, e45 */
            (right_dual_g1 * float4(this.e12345)) + float4((this.group1.xyz * right_dual_g2.www), -(right_dual_g3[0] * this.e415) - (right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group1.yzx) - (right_dual_g3.yzx * this.group1.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Motor {
    public VersorEven bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            right_dual_g0 * float4(this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), right_dual_g1[3] * this.e12345), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group1) + float4(float3(0.0), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g3[0] * this.e12345), (right_dual_g0[0] * this.e125) + (right_dual_g3[1] * this.e12345), (right_dual_g0[1] * this.e235) + (right_dual_g3[2] * this.e12345), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + (this.group0 * float4(right_dual_g1.www, right_dual_g3[3])) - (right_dual_g0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<MultiVector> operator /(MultiVector lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g2[3] * this.scalar) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43), right_dual_g2[3] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412) + (right_dual_g2[3] * this.e1), (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423) + (right_dual_g2[3] * this.e2), (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431) + (right_dual_g2[3] * this.e3), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((right_dual_g0.zxy * this.group8.yzx), right_dual_g2[3] * this.e4) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((this.group7.yzx * right_dual_g2.zxy), right_dual_g0[1] * this.e425), 
            /* e5 */
            (right_dual_g2[3] * this.e5) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g2[1] * this.e4125) + (right_dual_g2[3] * this.e15), (right_dual_g2[2] * this.e4235) + (right_dual_g2[3] * this.e25), (right_dual_g2[0] * this.e4315) + (right_dual_g2[3] * this.e35), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g1.xyz * this.group9.www), right_dual_g2[3] * this.e45) - (this.group9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group4.xyz) + (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(this.e3215)) + (float3(right_dual_g2[3]) * this.group5.xyz) + (float3(this.e1234) * right_dual_g2.xyz) - (float3(right_dual_g1[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g2[3]) * this.group6), 
            /* e423, e431, e412 */
            (right_dual_g0 * float3(this.e12345)) + (float3(right_dual_g2[3]) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g2[3]) * this.group8.xyz) + (float3(this.e12345) * right_dual_g2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group9, 
            /* e1234 */
            right_dual_g2[3] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g2[3] * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual_g2.xyz, right_dual_g1[3])) + (this.group9.yzxz * float4(right_dual_g1.zxy, other.e412)) + float4((this.group4.xyz * right_dual_g3.www), other.e423 * this.e4235) + float4((this.group5.yzx * right_dual_g3.zxy), other.e431 * this.e4315) - (float4(right_dual_g2[3]) * this.group3) - (right_dual_g3.yzxy * float4(this.group5.zxy, this.e42)) - float4((other.group0.xyz * this.group9.www), right_dual_g3[0] * this.e41) - float4((right_dual_g1.yzx * this.group9.zxy), right_dual_g3[2] * this.e43), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + (float4(this.e12345) * float4(right_dual_g2.xyz, right_dual_g1[3])) - float4((this.group8.zxy * right_dual_g3.yzx), right_dual_g3[0] * this.e415), 
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * right_dual_g3.yzx) - (this.group7.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g2[3]) * this.group8.xyz) + (float3(right_dual_g3[3]) * this.group7.xyz) + (float3(this.e12345) * right_dual_g1.xyz) - (float3(this.e321) * right_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345), 
            /* e1234 */
            right_dual_g2[3] * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiDualNum other) {
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            right_dual_g0 * float4(this.e12345), 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e1234) + (right_dual_g1[2] * this.e31), (right_dual_g0[1] * this.e1234) + (right_dual_g1[0] * this.e12), (right_dual_g0[2] * this.e1234) + (right_dual_g1[1] * this.e23), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((this.group4.xyz * right_dual_g1.www), right_dual_g0[3] * this.e1234) - (right_dual_g1.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (right_dual_g1[0] * this.e15) + (right_dual_g1[1] * this.e25) + (right_dual_g1[2] * this.e35) + (right_dual_g1[3] * this.e45) - (right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[2] * this.e315) + (right_dual_g1[3] * this.e415), (right_dual_g1[0] * this.e125) + (right_dual_g1[3] * this.e425), (right_dual_g1[1] * this.e235) + (right_dual_g1[3] * this.e435), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) + (right_dual_g0 * float4(this.e12345)) - float4((this.group8.zxy * right_dual_g1.yzx), right_dual_g1[0] * this.e415), 
            /* e41, e42, e43 */
            (this.group7.zxy * right_dual_g1.yzx) - (this.group7.yzx * right_dual_g1.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g1[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1.yzx * this.group9.zxy) - (right_dual_g1.zxy * this.group9.yzx)), right_dual_g1[3] * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g1.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual_g1[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) - float4((right_dual_g1.zxy * this.group7.yzx), right_dual_g0[0] * this.e423), 
            /* e5 */
            -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) - (this.group9.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e41, e42, e43 */
            right_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12 */
            right_dual_g1 * float3(this.e1234), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * float3(this.group0.yy, this.e12345)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            right_dual_g1 * float3(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[3] * this.scalar) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43), right_dual_g0[3] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g0[3] * this.e1), (right_dual_g0[1] * this.e321) + (right_dual_g0[3] * this.e2), (right_dual_g0[2] * this.e321) + (right_dual_g0[3] * this.e3), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + float4((this.group7.zxy * right_dual_g1.yzx), right_dual_g0[3] * this.e4) - float4((this.group7.yzx * right_dual_g1.zxy), right_dual_g0[0] * this.e423), 
            /* e5 */
            (right_dual_g0[3] * this.e5) + (right_dual_g1[3] * this.e12345) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[3] * this.e15) + (right_dual_g1[1] * this.e4125), (right_dual_g0[3] * this.e25) + (right_dual_g1[2] * this.e4235), (right_dual_g0[3] * this.e35) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) + (right_dual_g0 * float4(this.group9.www, this.e45)) - (this.group9.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_dual_g0.xyz), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[3]) * this.group5.xyz) + (float3(this.e1234) * right_dual_g1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * this.group6.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e321), 
            /* e423, e431, e412 */
            float3(right_dual_g0[3]) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group8.xyz) + (float3(this.e12345) * right_dual_g1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group9, 
            /* e1234 */
            right_dual_g0[3] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.zxy * right_dual_g0.yzx) - (this.group7.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group9.zxy) - (right_dual_g0.zxy * this.group9.yzx)), right_dual_g0[3] * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g0.xyz, 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group9.yzxy * float4(right_dual_g1.zxy, other.e431)) + float4((float3(this.e1234) * other.group2.xyz), other.e423 * this.e4235), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, right_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual_g1.xyz, 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group9.yzxx * float4(right_dual_g1.zxy, other.e423)) + float4((float3(this.e1234) * right_dual_g2.xyz), other.e431 * this.e4315), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(right_dual_g2.xyz, right_dual_g1[3]), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * right_dual_g1.xyz, 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g2[1] * this.e412) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g2[2] * this.e423) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g2[0] * this.e431) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((right_dual_g2.zxy * this.group7.yzx), right_dual_g0[1] * this.e425), 
            /* e5 */
            -(right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g2[1] * this.e4125) + (right_dual_g1[0] * this.e3215), (right_dual_g2[2] * this.e4235) + (right_dual_g1[1] * this.e3215), (right_dual_g2[0] * this.e4315) + (right_dual_g1[2] * this.e3215), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (this.group9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(this.e3215)) + (right_dual_g2 * float3(this.e1234)) - (float3(right_dual_g1[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e235, e315, e125 */
            right_dual_g2 * float3(this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g2[3] * this.e3215) + (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412), (right_dual_g0[0] * this.e125) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423), (right_dual_g0[1] * this.e235) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + (float4(this.e12345) * float4(right_dual_g3.xyz, right_dual_g2[3])) - float4((right_dual_g0.yzx * this.group8.zxy), right_dual_g0[0] * this.e415) - float4((this.group7.yzx * right_dual_g2.zxy), right_dual_g0[1] * this.e425), 
            /* e5 */
            (right_dual_g3[3] * this.e12345) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[0] * this.e3215) + (right_dual_g2[1] * this.e4125), (right_dual_g1[1] * this.e3215) + (right_dual_g2[2] * this.e4235), (right_dual_g1[2] * this.e3215) + (right_dual_g2[0] * this.e4315), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (this.group9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g0 * float3(this.e3215)) + (float3(this.e1234) * right_dual_g2.xyz) - (float3(right_dual_g1[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual_g2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * this.e4) + (right_dual_g0[1] * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group4.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0), 
            /* e5 */
            right_dual_g0[0] * this.e45, 
            /* e15, e25, e35, e45 */
            float4((this.group6.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual_g0[0]) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[0] * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g0[0]) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g0[0] * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2(-(right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[3]) * this.group6.xyz), 0.0) + float4((this.group7.zxy * right_dual_g0.yzx), 0.0) - float4((this.group7.yzx * right_dual_g0.zxy), 0.0), 
            /* e5 */
            -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_dual_g0.yzx * this.group9.zxy) - (right_dual_g0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[3] * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual_g0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[0] * this.e41) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0[3]) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * right_dual_g1.xyz), 0.0) + float4((this.group7.zxy * right_dual_g0.yzx), 0.0) - float4((this.group7.yzx * right_dual_g0.zxy), 0.0), 
            /* e5 */
            (right_dual_g1[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((right_dual_g0.yzx * this.group9.zxy) - (right_dual_g0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g0[3] * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * right_dual_g0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((other.group1.xyz * float3(this.group0.yy, this.e12345)), 0.0), 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[3] * this.e12345) + (right_dual_g1[3] * this.e4) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g1[3]) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * right_dual_g1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group9.yzx), 0.0) - float4((right_dual_g0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (right_dual_g1[3] * this.e45) - (right_dual_g1[0] * this.e4235) - (right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((float3(right_dual_g1[3]) * this.group6.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(right_dual_g1[3]) * this.group7.xyz) + (float3(this.e12345) * right_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), right_dual_g1[3] * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(right_dual_g1[3]) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), right_dual_g1[3] * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) + (right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), right_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g3[0] * this.e1234) + (right_dual_g6[0] * this.e321) + (right_dual_g6[3] * this.e415) + (right_dual_g9[2] * this.e31), (right_dual_g3[1] * this.e1234) + (right_dual_g6[1] * this.e321) + (right_dual_g6[3] * this.e425) + (right_dual_g9[0] * this.e12), (right_dual_g3[2] * this.e1234) + (right_dual_g6[2] * this.e321) + (right_dual_g6[3] * this.e435) + (right_dual_g9[1] * this.e23), -(right_dual_g10 * this.e45) - (right_dual_g6[0] * this.e423) - (right_dual_g6[1] * this.e431) - (right_dual_g6[2] * this.e412)) + (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group1) + (this.group9.yzxx * float4(other.group6.zxy, other.e423)) + float4((this.group4.xyz * right_dual_g9.www), right_dual_g3[3] * this.e1234) + float4((right_dual_g7.zxy * this.group8.yzx), other.e431 * this.e4315) + float4((right_dual_g8.yzx * this.group7.zxy), other.e412 * this.e4125) - (right_dual_g9.yzxy * float4(this.group5.zxy, this.e42)) - float4((float3(right_dual_g10) * this.group3.xyz), right_dual_g9[2] * this.e43) - float4((other.group7.xyz * this.group9.www), right_dual_g7[0] * this.e415) - float4((right_dual_g7.yzx * this.group8.zxy), right_dual_g7[2] * this.e435) - float4((right_dual_g8.zxy * this.group7.yzx), right_dual_g9[0] * this.e41) - float4((other.group6.yzx * this.group9.zxy), right_dual_g7[1] * this.e425), 
            /* e5 */
            (right_dual_g0[1] * this.e5) + (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45) + (other.e3215 * this.e12345) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g3[0] * this.e4235) - (right_dual_g3[1] * this.e4315) - (right_dual_g3[2] * this.e4125) - (right_dual_g3[3] * this.e3215) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215) + (right_dual_g9[2] * this.e315) + (right_dual_g9[3] * this.e415), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215) + (right_dual_g9[0] * this.e125) + (right_dual_g9[3] * this.e425), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215) + (right_dual_g9[1] * this.e235) + (right_dual_g9[3] * this.e435), -(right_dual_g6[2] * this.e4125) - (right_dual_g9[0] * this.e415) - (right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + (right_dual_g3 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group3) - (this.group9.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])) - float4((this.group8.zxy * right_dual_g9.yzx), right_dual_g6[1] * this.e4315), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group6.xyz) + (float3(right_dual_g0[1]) * this.group4.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * right_dual_g6.xyz) + (right_dual_g7.zxy * this.group9.yzx) + (this.group7.zxy * right_dual_g9.yzx) - (right_dual_g7.yzx * this.group9.zxy) - (this.group7.yzx * right_dual_g9.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g10) * this.group8.xyz) + (float3(right_dual_g0[1]) * this.group5.xyz) + (float3(right_dual_g9[3]) * this.group7.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(right_dual_g6[3]) * this.group9.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group6) + (right_dual_g9.yzxw * float4(this.group9.zxy, this.e1234)) - float4((right_dual_g9.zxy * this.group9.yzx), right_dual_g10 * this.e3215), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group7.xyz) + (float3(this.e1234) * right_dual_g9.xyz) - (float3(right_dual_g10) * this.group9.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group8.xyz) + (float3(right_dual_g9[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (right_dual_g9 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group9), 
            /* e1234 */
            (right_dual_g10 * this.e12345) + (right_dual_g0[1] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0.xyz * float3(this.group0.yy, this.e12345)), 0.0), 
            /* e5 */
            right_dual_g0[3] * this.e12345, 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1 * this.e5) + (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e31) + (right_dual_g0[3] * this.e41), (right_dual_g0[0] * this.e12) + (right_dual_g0[3] * this.e42), (right_dual_g0[1] * this.e23) + (right_dual_g0[3] * this.e43), -(right_dual_g1 * this.e45) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(right_dual_g1) * this.group3.xyz), right_dual_g0[1] * this.e42), 
            /* e5 */
            (right_dual_g0[0] * this.e15) + (right_dual_g0[1] * this.e25) + (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g1) * this.group6.xyz) + (this.group7.zxy * right_dual_g0.yzx) - (this.group7.yzx * right_dual_g0.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g1) * this.group8.xyz) + (float3(right_dual_g0[3]) * this.group7.xyz) - (float3(this.e321) * right_dual_g0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0.yzx * this.group9.zxy), right_dual_g0[3] * this.e1234) - (this.group9.yzxw * float4(right_dual_g0.zxy, right_dual_g1)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g1) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345), 
            /* e1234 */
            right_dual_g1 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Scalar other) {
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (this.e1234 * other.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e12345), 
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
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[3] * this.e12345) + (right_dual_g2[3] * this.e5) + (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(right_dual_g2.xyz, right_dual_g1[3])) + (this.group9.yzxz * float4(right_dual_g1.zxy, right_dual_g0[2])) + float4((this.group4.xyz * right_dual_g3.www), right_dual_g0[0] * this.e4235) + float4((this.group5.yzx * right_dual_g3.zxy), right_dual_g0[1] * this.e4315) - (right_dual_g3.yzxx * float4(this.group5.zxy, this.e41)) - float4((right_dual_g1.yzx * this.group9.zxy), right_dual_g3[2] * this.e43) - float4((right_dual_g0.xyz * this.group9.www), right_dual_g2[3] * this.e45) - float4((this.group3.xyz * right_dual_g2.www), right_dual_g3[1] * this.e42), 
            /* e5 */
            (right_dual_g3[0] * this.e15) + (right_dual_g3[1] * this.e25) + (right_dual_g3[2] * this.e35) + (right_dual_g3[3] * this.e45) - (right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g3[2] * this.e315) + (right_dual_g3[3] * this.e415), (right_dual_g3[0] * this.e125) + (right_dual_g3[3] * this.e425), (right_dual_g3[1] * this.e235) + (right_dual_g3[3] * this.e435), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + (float4(this.e12345) * float4(right_dual_g2.xyz, right_dual_g1[3])) - (right_dual_g3.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g2[3]) * this.group6.xyz) + (float3(this.e12345) * right_dual_g0.xyz) + (this.group7.zxy * right_dual_g3.yzx) - (this.group7.yzx * right_dual_g3.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g2[3]) * this.group8.xyz) + (float3(right_dual_g3[3]) * this.group7.xyz) + (float3(this.e12345) * right_dual_g1.xyz) - (float3(this.e321) * right_dual_g3.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3.yzx * this.group9.zxy), right_dual_g3[3] * this.e1234) - (this.group9.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g3[3]) * this.group9.xyz) - (float3(this.e3215) * right_dual_g3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345), 
            /* e1234 */
            right_dual_g2[3] * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension MultiVector {
    public MultiVector bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[3] * this.scalar) + (right_dual_g2[3] * this.e1234) + (right_dual_g3[0] * this.e4235) + (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[0] * this.e15) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43), right_dual_g0[3] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g0[3] * this.e1) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g0[3] * this.e2) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g0[3] * this.e3) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[2] * this.e412)) + (right_dual_g3 * float4(this.e12345)) + float4((this.group7.zxy * right_dual_g2.yzx), right_dual_g0[3] * this.e4) - float4((this.group7.yzx * right_dual_g2.zxy), right_dual_g1[0] * this.e423) - float4((this.group8.zxy * right_dual_g0.yzx), right_dual_g1[1] * this.e431), 
            /* e5 */
            (right_dual_g0[3] * this.e5) + (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g1[0] * this.e3215) + (right_dual_g2[1] * this.e4125), (right_dual_g1[1] * this.e3215) + (right_dual_g2[2] * this.e4235), (right_dual_g1[2] * this.e3215) + (right_dual_g2[0] * this.e4315), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + (float4(right_dual_g0[3]) * this.group3) - (this.group9.yzxx * float4(right_dual_g2.zxy, right_dual_g1[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[3]) * this.group4.xyz) + (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group9.yzx) - (right_dual_g0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(right_dual_g0[3]) * this.group5.xyz) + (float3(this.e3215) * right_dual_g0.xyz) + (float3(this.e1234) * right_dual_g2.xyz) - (float3(right_dual_g1[3]) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group6), 
            /* e423, e431, e412 */
            (float3(right_dual_g0[3]) * this.group7.xyz) + (float3(this.e12345) * right_dual_g0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group8.xyz) + (float3(this.e12345) * right_dual_g2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group9, 
            /* e1234 */
            right_dual_g0[3] * this.e1234
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Plane> operator /(Plane lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public DipoleInversion bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - float4((right_dual_g2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx)), right_dual_g2[3] * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g3.xyz, (other.e431 * this.e4315) + (other.e412 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, other.e423)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1.zxy * this.group0.yzx), -(right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1[3])) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkContractionInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Plane bulk_contraction(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(BulkContractionInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public DualNum bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(BulkContractionInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Motor bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4((this.group0.xyz * right_dual_g1.www), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)) - (this.group0.wwwx * float4(right_dual_g1.xyz, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(BulkContractionInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public FlatPoint bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(BulkContractionInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Flector bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(BulkContractionInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Line bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Line(
            /* e415, e425, e435 */
            (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public RoundPoint bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public RoundPoint bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_dual_g1.zxy, other.e423)), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Dipole bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group0.zxyx * float4(right_dual_g2.yzx, right_dual_g3[0])) + float4((right_dual_g2.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiFlatPoint bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((this.group0.xyz * float3(right_dual_g0.xx, right_dual_g0[0])), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiLine bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual_g0[3]) * this.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiMotor bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((this.group0.xyz * right_dual_g0.www * float3(-1.0)), (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiPlane bulk_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e435 * this.e4315, other.e415 * this.e4125, other.e425 * this.e4235, -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiFlector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((this.group0.xyz * right_dual_g1.www), 0.0), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0.zxy * this.group0.yzx), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public MultiVector bulk_contraction(MultiVector other) {
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e425 * this.e4125) - (other.e423 * this.e3215), -(other.e435 * this.e4235) - (other.e431 * this.e3215), -(other.e415 * this.e4315) - (other.e412 * this.e3215), (other.e431 * this.e4315) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(other.group6.zxy, other.e423)), 
            /* e5 */
            (other.e321 * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125)) - (this.group0.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])), 
            /* e41, e42, e43 */
            (right_dual_g7.zxy * this.group0.yzx) - (right_dual_g7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) - (float3(right_dual_g6[3]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g9.yzx * this.group0.zxy) - (right_dual_g9.zxy * this.group0.yzx)), right_dual_g10 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_dual_g10) * this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.group0.yx[1]) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Scalar bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkContractionInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Circle bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual_g1) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)), right_dual_g1 * this.e3215 * -1.0), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(BulkContractionInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Plane bulk_contraction(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public Scalar bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual_g2[3]) * this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g3.yzx * this.group0.zxy) - (right_dual_g3.zxy * this.group0.yzx)), right_dual_g2[3] * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, right_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1.zxy * this.group0.yzx), -(right_dual_g1[3] * this.e3215) - (right_dual_g2[2] * this.e4125)) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0])) - (this.group0.wwwy * float4(right_dual_g0.xyz, right_dual_g2[1]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Plane {
    public VersorOdd bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g3[0])) + float4((right_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215)), 
            /* e23, e31, e12, e45 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125))) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * right_dual_g1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group0.zxy), 0.0) - float4((right_dual_g2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<RoundPoint> operator /(RoundPoint lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiCircleRotor other) {
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g2[3]) * this.group0, 
            /* e5 */
            right_dual_g2[3] * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiDualNum other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiFlector other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g1[0] * this.e1) + (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e5 */
            right_dual_g0[3] * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(DualNum other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(Motor other) {
        return Scalar(
            /* scalar */
            other.e5 * this.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (other.e4 * this.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[1]) * this.group0, 
            /* e5 */
            right_dual_g0[1] * this.e5, 
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
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e1) + (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4) - (other.e4 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public Scalar bulk_contraction(VersorEven other) {
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return Scalar(
            /* scalar */
            (right_dual_g3[0] * this.e1) + (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (this.e5 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension RoundPoint {
    public RoundPoint bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e5 */
            right_dual_g0[3] * this.e5
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Scalar> operator /(Scalar lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(MultiVector other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Scalar {
    public Scalar bulk_contraction(VersorOdd other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<Sphere> operator /(Sphere lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(BulkContractionInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public DipoleInversion bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)), right_dual_g2[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g3.xyz, (right_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, other.e423)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[2] * this.e4315) + (right_dual_g2[0] * this.e1234), (right_dual_g1[0] * this.e4125) + (right_dual_g2[1] * this.e1234), (right_dual_g1[1] * this.e4235) + (right_dual_g2[2] * this.e1234), -(right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1[3])) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public VersorOdd bulk_contraction(AntiDualNum other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx)), right_dual_g1[3] * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(right_dual_g1[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g1.xyz)), right_dual_g0[3] * this.e1234), 
            /* e1, e2, e3, e5 */
            float4((float3(this.e1234) * right_dual_g0.xyz), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Dipole bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            right_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * float3(this.e1234)), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35 */
            (right_dual_g0 * float3(this.e3215)) + (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public VersorOdd bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4(right_dual_g0.xyz, right_dual_g1[3]), 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * right_dual_g1.xyz), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group0.zxy) - (right_dual_g1.zxy * this.group0.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkContractionInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Circle bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return Circle(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)), right_dual_g0[3] * this.e1234), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public RoundPoint bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxy * float4(right_dual_g1.zxy, other.e431)) + float4((float3(this.e1234) * other.group2.xyz), other.e423 * this.e4235), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public RoundPoint bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(right_dual_g1.zxy, other.e423)) + float4((float3(this.e1234) * right_dual_g2.xyz), other.e431 * this.e4315), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(BulkContractionInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Dipole bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group0.yzx) - (right_dual_g0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group0.zxyx * float4(right_dual_g2.yzx, right_dual_g3[0])) + float4((right_dual_g2.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(BulkContractionInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiFlatPoint bulk_contraction(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e5 * -1.0) * float4(this.group0.xyz, this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(BulkContractionInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiLine bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiLine(
            /* e23, e31, e12 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group0.xyz), 
            /* e15, e25, e35 */
            (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiMotor bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(this.e1234) * right_dual_g0.xyz) - (this.group0.xyz * right_dual_g0.www)), (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiPlane bulk_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e435 * this.e4315) + (other.e235 * this.e1234), (other.e415 * this.e4125) + (other.e315 * this.e1234), (other.e425 * this.e4235) + (other.e125 * this.e1234), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group0.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiFlector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e4315) + (right_dual_g1[0] * this.e1234), (right_dual_g0[0] * this.e4125) + (right_dual_g1[1] * this.e1234), (right_dual_g0[1] * this.e4235) + (right_dual_g1[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (this.group0.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(other.e425 * this.e4125) - (other.e423 * this.e3215), -(other.e435 * this.e4235) - (other.e431 * this.e3215), -(other.e415 * this.e4315) - (other.e412 * this.e3215), (right_dual_g3_w * this.e1234) + (other.e412 * this.e4125)) + (this.group0.yzxx * float4(other.group6.zxy, other.e423)) + float4((float3(this.e1234) * other.group8.xyz), other.e431 * this.e4315), 
            /* e5 */
            -(right_dual_g3_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125)) - (this.group0.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g6.xyz) + (right_dual_g7.zxy * this.group0.yzx) - (right_dual_g7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) - (float3(right_dual_g6[3]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g9.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual_g9.zxy * this.group0.yzx), right_dual_g10 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g9.xyz) - (float3(right_dual_g10) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g9[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * this.group0, 
            /* e1234 */
            right_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Scalar bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(BulkContractionInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Circle bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g1) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g0.yzxw * float4(this.group0.zxy, this.e1234)) - float4((right_dual_g0.zxy * this.group0.yzx), right_dual_g1 * this.e3215), 
            /* e235, e315, e125 */
            (float3(right_dual_g0[3]) * this.group0.xyz) - (float3(this.e3215) * right_dual_g0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(BulkContractionInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Sphere bulk_contraction(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public Scalar bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g3.yzx * this.group0.zxy), right_dual_g3[3] * this.e1234) - (this.group0.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (float4(this.e3215, this.e3215, this.e3215, 1.0) * float4(right_dual_g3.xyz, (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g1[3] * this.e1234)) * float4(-1.0, -1.0, -1.0, 1.0)) + (this.group0.xyzx * float4(right_dual_g3.www, right_dual_g0[0])), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[2] * this.e4315) + (right_dual_g2[0] * this.e1234), (right_dual_g1[0] * this.e4125) + (right_dual_g2[1] * this.e1234), (right_dual_g1[1] * this.e4235) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[3] * this.e3215) - (right_dual_g2[2] * this.e4125)) - (this.group0.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0])) - (this.group0.wwwy * float4(right_dual_g0.xyz, right_dual_g2[1]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension Sphere {
    public VersorOdd bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.yzxx * float4(right_dual_g0.zxy, right_dual_g3[0])) + float4((right_dual_g0.yzx * this.group0.zxy * float3(-1.0)), (right_dual_g2[3] * this.e1234) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215)) + float4((float3(this.e1234) * right_dual_g1.xyz), right_dual_g3[1] * this.e4315), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<VersorEven> operator /(VersorEven lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((right_dual_g0 * float3(this.e12345)) + (float3(right_dual_g2[3]) * this.group0.xyz)), right_dual_g2[3] * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g2[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(this.group0.www, this.e5)) + float4((this.group2.xyz * right_dual_g2.www), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412) + (right_dual_g2[3] * this.e1), (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423) + (right_dual_g2[3] * this.e2), (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431) + (right_dual_g2[3] * this.e3), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + float4((right_dual_g0.zxy * this.group2.yzx), right_dual_g2[3] * this.e4) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g0[0] * this.e415) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g0[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g3.yzxy * float4(this.group0.zxy, this.e2)) + float4(float3(0.0), (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e431 * this.e315) - (other.e412 * this.e125)) + float4((other.group0.xyz * this.group0.www), right_dual_g2[3] * this.e5) + float4((this.group1.xyz * right_dual_g2.www), right_dual_g3[0] * this.e1) - float4((right_dual_g3.zxy * this.group0.yzx), other.e423 * this.e235), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[3] * this.e235) + (right_dual_g3[3] * this.e423), (right_dual_g2[3] * this.e315) + (right_dual_g3[3] * this.e431), (right_dual_g2[3] * this.e125) + (right_dual_g3[3] * this.e412), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + (right_dual_g1 * float4(this.e12345)) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group2.yzx) - (right_dual_g3.yzx * this.group2.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.scalar) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.group0.yy, other.scalar, (other.e3215 * this.e12345) + (other.scalar * this.e5)) * float4(this.group2.xyz, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), right_dual_g0[3] * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual_g0.xyz), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (right_dual_g1[1] * this.e2) + (right_dual_g1[2] * this.e3) + (right_dual_g1[3] * this.e4) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0])), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(right_dual_g1.www, right_dual_g0[3])) + float4(float3(0.0), -(right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - (right_dual_g1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * right_dual_g0.xyz), 0.0) + float4((right_dual_g1.zxy * this.group2.yzx), 0.0) - float4((right_dual_g1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g1 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((right_dual_g0 * this.group0.www), 0.0), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g1 * float3(this.e12345)), -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[0] * this.e321) + (right_dual_g1[1] * this.e412), (right_dual_g0[1] * this.e321) + (right_dual_g1[2] * this.e423), (right_dual_g0[2] * this.e321) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)) - float4((right_dual_g1.zxy * this.group0.yzx), right_dual_g0[0] * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorEven bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(right_dual_g0[3]) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e321), 
            /* e235, e315, e125, e5 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g0[3] * this.e1) + (right_dual_g1[1] * this.e412), (right_dual_g0[3] * this.e2) + (right_dual_g1[2] * this.e423), (right_dual_g0[3] * this.e3) + (right_dual_g1[0] * this.e431), -(right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412)) + (right_dual_g0 * float4(this.group1.www, this.e4)) - (this.group0.yzxx * float4(right_dual_g1.zxy, right_dual_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4((right_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g0[1] * this.e2) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)), 
            /* e23, e31, e12, e45 */
            float4((this.group0.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((right_dual_g0.zxy * this.group2.yzx), 0.0) - float4((right_dual_g0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public Scalar bulk_contraction(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.e12345 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiCircleRotor bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            right_dual_g1 * float4(this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[0] * this.e423) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((right_dual_g2 * float3(this.e12345)), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[2] * this.e315) + (right_dual_g2[1] * this.e412) + (right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415), (right_dual_g0[0] * this.e125) + (right_dual_g2[2] * this.e423) + (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425), (right_dual_g0[1] * this.e235) + (right_dual_g2[0] * this.e431) + (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435), -(right_dual_g2[2] * this.e435) - (right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125)) - float4((right_dual_g0.yzx * this.group2.zxy), right_dual_g2[0] * this.e415) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g2[1] * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            right_dual_g0 * float3(this.e12345), 
            /* e415, e425, e435, e321 */
            right_dual_g1 * float4(this.e12345), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * right_dual_g2.xyz), (right_dual_g2[3] * this.e12345) - (right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412) + (right_dual_g3[0] * this.e12345), (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423) + (right_dual_g3[1] * this.e12345), (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431) + (right_dual_g3[2] * this.e12345), -(right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)) + float4((right_dual_g0.zxy * this.group2.yzx), right_dual_g3[3] * this.e12345) - (this.group2.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0])) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g1[1] * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(right_dual_g0.xx, right_dual_g0[0], (right_dual_g0[0] * this.e4) + (right_dual_g0[1] * this.e12345)) * float4(this.group0.xyz, 1.0), 
            /* e15, e25, e35, e3215 */
            float4(right_dual_g0[0]) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiFlector bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[1] * this.e412) + (right_dual_g0[3] * this.e415), (right_dual_g0[2] * this.e423) + (right_dual_g0[3] * this.e425), (right_dual_g0[0] * this.e431) + (right_dual_g0[3] * this.e435), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiFlector bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            right_dual_g0 * float4(this.e12345), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g0[3] * this.e415) + (right_dual_g1[0] * this.e12345), (right_dual_g0[3] * this.e425) + (right_dual_g1[1] * this.e12345), (right_dual_g0[3] * this.e435) + (right_dual_g1[2] * this.e12345), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) + (this.group0.zxyw * float4(right_dual_g0.yzx, right_dual_g1[3])) - (right_dual_g0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor bulk_contraction(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e235 * this.e423) - (other.e315 * this.e431) - (other.e125 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((other.group1.xyz * this.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiMotor bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (right_dual_g0 * float4(this.e12345)) + (float4(right_dual_g1[3]) * float4(this.group0.xyz, this.e4)) + float4(float3(0.0), -(right_dual_g0[0] * this.e415) - (right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(right_dual_g1[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g1.xyz)), right_dual_g1[3] * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group8.xyz, other.e321 * -1.0);
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g10 * this.e5) + (right_dual_g0[0] * this.e12345) + (right_dual_g9[0] * this.e1) + (right_dual_g9[1] * this.e2) + (right_dual_g9[2] * this.e3) + (right_dual_g9[3] * this.e4) - (right_dual_g3[0] * this.e423) - (right_dual_g3[1] * this.e431) - (right_dual_g3[2] * this.e412) - (right_dual_g3[3] * this.e321) - (other.e415 * this.e415) - (other.e425 * this.e425) - (other.e435 * this.e435) - (other.e423 * this.e235) - (other.e431 * this.e315) - (other.e412 * this.e125), right_dual_g0[1] * this.e12345), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g8[1] * this.e412) + (right_dual_g1[0] * this.e12345) + (right_dual_g6[0] * this.e321) + (right_dual_g6[3] * this.e415), (right_dual_g8[2] * this.e423) + (right_dual_g1[1] * this.e12345) + (right_dual_g6[1] * this.e321) + (right_dual_g6[3] * this.e425), (right_dual_g8[0] * this.e431) + (right_dual_g1[2] * this.e12345) + (right_dual_g6[2] * this.e321) + (right_dual_g6[3] * this.e435), -(right_dual_g7[2] * this.e435) - (right_dual_g6[0] * this.e423) - (right_dual_g6[1] * this.e431) - (right_dual_g6[2] * this.e412)) + (float4(right_dual_g0[1]) * this.group3) + float4((right_dual_g7.zxy * this.group2.yzx), right_dual_g1[3] * this.e12345) - float4((right_dual_g7.yzx * this.group2.zxy), right_dual_g7[0] * this.e415) - float4((right_dual_g8.zxy * this.group0.yzx), right_dual_g7[1] * this.e425), 
            /* e5 */
            (right_dual_g0[1] * this.e5) + (other.e3215 * this.e12345) - (right_dual_g8[0] * this.e415) - (right_dual_g8[1] * this.e425) - (right_dual_g8[2] * this.e435) - (right_dual_g6[0] * this.e235) - (right_dual_g6[1] * this.e315) - (right_dual_g6[2] * this.e125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g9[2] * this.e315) + (right_dual_g9[3] * this.e415), (right_dual_g9[0] * this.e125) + (right_dual_g9[3] * this.e425), (right_dual_g9[1] * this.e235) + (right_dual_g9[3] * this.e435), -(right_dual_g9[1] * this.e425) - (right_dual_g9[2] * this.e435)) + (right_dual_g3 * float4(this.e12345)) - (right_dual_g9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(right_dual_g10) * this.group1.xyz) + (float3(this.e12345) * other.group7.xyz) + (right_dual_g9.yzx * this.group0.zxy) - (right_dual_g9.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_dual_g10) * this.group2.xyz) + (float3(right_dual_g9[3]) * this.group0.xyz) + (float3(this.e12345) * other.group6.xyz) - (float3(this.e321) * right_dual_g9.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g6 * float4(this.e12345)) + (float4(right_dual_g0[1]) * this.group1), 
            /* e423, e431, e412 */
            (right_dual_g7 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group0.xyz), 
            /* e235, e315, e125 */
            (right_dual_g8 * float3(this.e12345)) + (float3(right_dual_g0[1]) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g9 * float4(this.e12345), 
            /* e1234 */
            right_dual_g10 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public AntiPlane bulk_contraction(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * float4(other.e4235 * -1.0, other.e4315 * -1.0, other.e4125 * -1.0, other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0.yzxx * float4(this.group0.zxy, this.e1)) + float4((right_dual_g0.zxy * this.group0.yzx * float3(-1.0)), (right_dual_g1 * this.e5) + (right_dual_g0[2] * this.e3) + (right_dual_g0[3] * this.e4)) + float4((float3(right_dual_g1) * this.group1.xyz), right_dual_g0[1] * this.e2), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * this.e235) + (right_dual_g0[3] * this.e423), (right_dual_g1 * this.e315) + (right_dual_g0[3] * this.e431), (right_dual_g1 * this.e125) + (right_dual_g0[3] * this.e412), -(right_dual_g0[1] * this.e425) - (right_dual_g0[2] * this.e435)) - (right_dual_g0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group1.xyz) + (right_dual_g0.zxy * this.group2.yzx) - (right_dual_g0.yzx * this.group2.zxy)), right_dual_g1 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g0 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorEven bulk_contraction(Scalar other) {
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
public RoundPoint operator /(BulkContractionInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public RoundPoint bulk_contraction(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            other.e3215 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorOdd bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.zxyw * float4(right_dual_g3.yzx, right_dual_g0[3])) + float4(float3(0.0), (right_dual_g3[1] * this.e2) + (right_dual_g3[2] * this.e3) + (right_dual_g3[3] * this.e4) - (right_dual_g0[0] * this.e235) - (right_dual_g0[1] * this.e315) - (right_dual_g0[2] * this.e125) - (right_dual_g1[0] * this.e415) - (right_dual_g1[1] * this.e425) - (right_dual_g1[2] * this.e435) - (right_dual_g1[3] * this.e321) - (right_dual_g2[1] * this.e431) - (right_dual_g2[2] * this.e412)) + float4((right_dual_g0.xyz * this.group0.www), right_dual_g2[3] * this.e5) + float4((this.group1.xyz * right_dual_g2.www), right_dual_g3[0] * this.e1) - (this.group0.yzxx * float4(right_dual_g3.zxy, right_dual_g2[0])), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1[0] * this.e12345) + (right_dual_g2[3] * this.e235), (right_dual_g1[1] * this.e12345) + (right_dual_g2[3] * this.e315), (right_dual_g1[2] * this.e12345) + (right_dual_g2[3] * this.e125), -(right_dual_g3[1] * this.e425) - (right_dual_g3[2] * this.e435)) + (this.group0 * float4(right_dual_g3.www, right_dual_g1[3])) - (right_dual_g3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g3[3]) * this.group1.xyz) + (float3(this.e12345) * right_dual_g2.xyz) + (right_dual_g3.zxy * this.group2.yzx) - (right_dual_g3.yzx * this.group2.zxy)), right_dual_g2[3] * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            right_dual_g3 * float4(this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(BulkContractionInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorEven {
    public VersorEven bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(right_dual_g0[3]) * this.group0.xyz) + (float3(this.e12345) * right_dual_g0.xyz)), right_dual_g0[3] * this.e12345), 
            /* e415, e425, e435, e321 */
            (right_dual_g1 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group1), 
            /* e235, e315, e125, e5 */
            (right_dual_g2 * float4(this.e12345)) + (float4(right_dual_g0[3]) * this.group2) + float4(float3(0.0), -(right_dual_g1[0] * this.e235) - (right_dual_g1[1] * this.e315) - (right_dual_g1[2] * this.e125) - (right_dual_g2[0] * this.e415) - (right_dual_g2[1] * this.e425) - (right_dual_g2[2] * this.e435)), 
            /* e1, e2, e3, e4 */
            float4((right_dual_g1[0] * this.e321) + (right_dual_g1[3] * this.e415) + (right_dual_g2[1] * this.e412) + (right_dual_g3[0] * this.e12345), (right_dual_g1[1] * this.e321) + (right_dual_g1[3] * this.e425) + (right_dual_g2[2] * this.e423) + (right_dual_g3[1] * this.e12345), (right_dual_g1[2] * this.e321) + (right_dual_g1[3] * this.e435) + (right_dual_g2[0] * this.e431) + (right_dual_g3[2] * this.e12345), -(right_dual_g0[2] * this.e435) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412)) + (right_dual_g0.zxyw * float4(this.group2.yzx, this.e4)) + float4((this.group3.xyz * right_dual_g0.www), right_dual_g3[3] * this.e12345) - (right_dual_g0.yzxx * float4(this.group2.zxy, this.e415)) - float4((right_dual_g2.zxy * this.group0.yzx), right_dual_g0[1] * this.e425)
        );
    }
}
// Fancy infix trick (first half)
public BulkContractionInfixPartial<VersorOdd> operator /(VersorOdd lhs, BulkContractionInfix rhs) {
    return BulkContractionInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiCircleRotor other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((right_dual_g1[0] * this.e1234) + (right_dual_g2[3] * this.e41), (right_dual_g1[1] * this.e1234) + (right_dual_g2[3] * this.e42), (right_dual_g1[2] * this.e1234) + (right_dual_g2[3] * this.e43), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g2[3] * this.scalar) - float4((right_dual_g0.yzx * this.group3.zxy), right_dual_g0[0] * this.e15), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g2[0] * this.e1234) + (right_dual_g2[3] * this.e23), (right_dual_g2[1] * this.e1234) + (right_dual_g2[3] * this.e31), (right_dual_g2[2] * this.e1234) + (right_dual_g2[3] * this.e12), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g0 * this.group3.www), right_dual_g2[3] * this.e45) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g2[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group3.zxy) - (right_dual_g2.zxy * this.group3.yzx)), right_dual_g2[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g2[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiDipoleInversion other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = other.group3 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual_g3.www, other.e423)) + float4(float3(0.0), (right_dual_g1[3] * this.e1234) + (other.e431 * this.e4315) + (other.e412 * this.e4125) - (right_dual_g3[0] * this.e41) - (right_dual_g3[1] * this.e42) - (right_dual_g3[2] * this.e43)) - float4((right_dual_g3.xyz * this.group3.www), right_dual_g2[3] * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual_g2.xyz, right_dual_g3[1])) + float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0])) - float4((right_dual_g3.yzx * this.group1.zxy), right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.group0.yy, other.scalar, (other.e3215 * this.e1234) + (other.scalar * this.scalar)) * float4(this.group0.xyz, 1.0), 
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
public RoundPoint operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(AntiFlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            right_dual_g0 * float4(this.e1234), 
            /* e5 */
            -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiFlector other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g1.yzx * this.group3.zxy) - (right_dual_g1.zxy * this.group3.yzx)), right_dual_g1[3] * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((this.group3.xyz * right_dual_g1.www), right_dual_g0[3] * this.e1234) - (right_dual_g1.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_dual_g1.zxyy * float4(this.group1.yzx, this.e25)) + (right_dual_g1.wwwz * float4(this.group0.xyz, this.e35)) + (this.group2.wwwx * float4(right_dual_g0.xyz, right_dual_g1[0])) + float4(float3(0.0), (right_dual_g1[3] * this.e45) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125) - (right_dual_g0[3] * this.e3215)) - float4((right_dual_g1.yzx * this.group1.zxy), right_dual_g0[0] * this.e4235)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(AntiLine other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float3 right_dual_g1 = other.group1.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            right_dual_g0 * float3(this.e1234), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g1 * float3(this.e1234)), -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g1[1] * this.e4125), (right_dual_g0[1] * this.e3215) + (right_dual_g1[2] * this.e4235), (right_dual_g0[2] * this.e3215) + (right_dual_g1[0] * this.e4315), -(right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) - float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g0[0] * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(AntiMotor other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0 * float4(this.group2.www, this.scalar)) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12) - (right_dual_g1[0] * this.e41) - (right_dual_g1[1] * this.e42) - (right_dual_g1[2] * this.e43)) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g1[3] * this.e1234), 
            /* e23, e31, e12, e45 */
            (float4(right_dual_g0[3]) * this.group1) + (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual_g1.xyz, -(right_dual_g0[0] * this.e4235) - (right_dual_g0[1] * this.e4315) - (right_dual_g0[2] * this.e4125))), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g0.xyz) + (right_dual_g1.yzx * this.group3.zxy) - (right_dual_g1.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(AntiPlane other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * right_dual_g0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e235, e315, e125, e4 */
            float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            (right_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual_g0.wwwy * float4(this.group0.xyz, this.e25)) + float4((right_dual_g0.yzx * this.group1.zxy * float3(-1.0)), (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(Circle other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxy * float4(right_dual_g1.zxy, other.e431)) + float4((other.group2.xyz * this.group2.www), other.e423 * this.e4235), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(BulkContractionInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public RoundPoint bulk_contraction(CircleRotor other) {
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = other.group2 * float4(1.0, 1.0, 1.0, -1.0);
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual_g1[1] * this.e4125) - (other.e423 * this.e3215), -(right_dual_g1[2] * this.e4235) - (other.e431 * this.e3215), -(right_dual_g1[0] * this.e4315) - (other.e412 * this.e3215), (right_dual_g1[3] * this.e1234) + (other.e412 * this.e4125)) + (this.group3.yzxx * float4(right_dual_g1.zxy, other.e423)) + float4((right_dual_g2.xyz * this.group2.www), other.e431 * this.e4315), 
            /* e5 */
            -(right_dual_g1[3] * this.e3215) - (right_dual_g2[0] * this.e4235) - (right_dual_g2[1] * this.e4315) - (right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(Dipole other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g2 = other.group2.xyz * float3(-1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4((right_dual_g2[1] * this.e4125) + (right_dual_g1[0] * this.e3215), (right_dual_g2[2] * this.e4235) + (right_dual_g1[1] * this.e3215), (right_dual_g2[0] * this.e4315) + (right_dual_g1[2] * this.e3215), -(right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(BulkContractionInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor bulk_contraction(DipoleInversion other) {
        float3 right_dual_g0 = other.group0.xyz * float3(-1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = other.group2 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = other.group3 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * right_dual_g1.xyz) + (right_dual_g0.zxy * this.group3.yzx) - (right_dual_g0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[0] * this.e3215) + (right_dual_g2[0] * this.e1234), (right_dual_g0[1] * this.e3215) + (right_dual_g2[1] * this.e1234), (right_dual_g0[2] * this.e3215) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(right_dual_g1.xyz, right_dual_g2[3])) + (this.group3.zxyx * float4(right_dual_g2.yzx, right_dual_g3[0])) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e1234) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) - float4((right_dual_g2.zxy * this.group3.yzx), right_dual_g0[0] * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiFlector bulk_contraction(DualNum other) {
        float2 right_dual_g0 = other.group0.xy * float2(-1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g0[0]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(right_dual_g0[0]) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiMotor bulk_contraction(FlatPoint other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_dual_g0.xyz, -(right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45))) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(BulkContractionInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiMotor bulk_contraction(Flector other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43) - (right_dual_g0[3] * this.e45)) + float4((right_dual_g0.xyz * this.group2.www), right_dual_g1[0] * this.e4235) - (right_dual_g0.wwwx * float4(this.group3.xyz, this.e41)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual_g0.yzx * this.group3.zxy) - (right_dual_g0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(BulkContractionInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiPlane bulk_contraction(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((other.e435 * this.e4315) + (other.e235 * this.e1234), (other.e415 * this.e4125) + (other.e315 * this.e1234), (other.e425 * this.e4235) + (other.e125 * this.e1234), -(other.e315 * this.e4315) - (other.e125 * this.e4125)) - (this.group3.zxyx * float4(other.group0.yzx, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(BulkContractionInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiFlector bulk_contraction(Motor other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(right_dual_g1[3]) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4((right_dual_g1[0] * this.e1234) + (right_dual_g1[3] * this.e41), (right_dual_g1[1] * this.e1234) + (right_dual_g1[3] * this.e42), (right_dual_g1[2] * this.e1234) + (right_dual_g1[3] * this.e43), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g1[3] * this.e45) - (this.group3.zxyx * float4(right_dual_g0.yzx, right_dual_g1[0]))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(BulkContractionInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public MultiVector bulk_contraction(MultiVector other) {
        float2 right_dual_g0 = other.group0.yx * float2(-1.0, 1.0);
        float4 right_dual_g1 = float4(other.group9.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        float right_dual_g3_w = other.e321 * -1.0;
        float4 right_dual_g6 = float4(other.group5.xyz, other.e45) * float4(-1.0, -1.0, -1.0, 1.0);
        float3 right_dual_g7 = other.group4.xyz * float3(-1.0);
        float3 right_dual_g8 = other.group3.xyz * float3(-1.0);
        float4 right_dual_g9 = float4(other.group1.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g10 = other.e4 * -1.0;
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual_g0[1] * this.scalar) + (right_dual_g1[0] * this.e4235) + (right_dual_g1[1] * this.e4315) + (right_dual_g1[2] * this.e4125) + (right_dual_g1[3] * this.e3215) + (other.e3215 * this.e1234) - (right_dual_g7[0] * this.e15) - (right_dual_g7[1] * this.e25) - (right_dual_g7[2] * this.e35) - (right_dual_g8[0] * this.e41) - (right_dual_g8[1] * this.e42) - (right_dual_g8[2] * this.e43) - (right_dual_g6[0] * this.e23) - (right_dual_g6[1] * this.e31) - (right_dual_g6[2] * this.e12) - (right_dual_g6[3] * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group3.yzxx * float4(other.group6.zxy, other.e423)) + float4((other.group8.xyz * this.group2.www), other.e431 * this.e4315) + float4((right_dual_g9.zxy * this.group1.yzx), other.e412 * this.e4125) + float4((this.group0.xyz * right_dual_g9.www), right_dual_g3_w * this.e1234) - (float4(right_dual_g10) * float4(this.group2.xyz, this.e45)) - (right_dual_g9.yzxz * float4(this.group1.zxy, this.e43)) - float4((other.group7.xyz * this.group3.www), right_dual_g9[0] * this.e41) - float4((other.group6.yzx * this.group3.zxy), right_dual_g9[1] * this.e42), 
            /* e5 */
            (right_dual_g9[0] * this.e15) + (right_dual_g9[1] * this.e25) + (right_dual_g9[2] * this.e35) + (right_dual_g9[3] * this.e45) - (right_dual_g3_w * this.e3215) - (other.e235 * this.e4235) - (other.e315 * this.e4315) - (other.e125 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4((right_dual_g8[1] * this.e4125) + (right_dual_g6[0] * this.e3215), (right_dual_g8[2] * this.e4235) + (right_dual_g6[1] * this.e3215), (right_dual_g8[0] * this.e4315) + (right_dual_g6[2] * this.e3215), -(right_dual_g6[1] * this.e4315) - (right_dual_g6[2] * this.e4125)) + (float4(right_dual_g0[1]) * float4(this.group2.xyz, this.e45)) - (this.group3.yzxx * float4(right_dual_g8.zxy, right_dual_g6[0])), 
            /* e41, e42, e43 */
            (float3(right_dual_g0[1]) * this.group0.xyz) + (float3(this.e1234) * right_dual_g6.xyz) + (right_dual_g7.zxy * this.group3.yzx) - (right_dual_g7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (right_dual_g7 * float3(this.e3215)) + (right_dual_g8 * float3(this.e1234)) + (float3(right_dual_g0[1]) * this.group1.xyz) - (float3(right_dual_g6[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g9.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual_g9.zxy * this.group3.yzx), right_dual_g10 * this.e3215), 
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g9.xyz) - (float3(right_dual_g10) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(right_dual_g9[3]) * this.group3.xyz) - (float3(this.e3215) * right_dual_g9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[1]) * this.group3, 
            /* e1234 */
            right_dual_g0[1] * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(BulkContractionInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public Scalar bulk_contraction(Plane other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(RoundPoint other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        float right_dual_g1 = other.e4 * -1.0;
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g0.xyz) - (float3(right_dual_g1) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g0.yzxw * float4(this.group3.zxy, this.e1234)) - float4((right_dual_g0.zxy * this.group3.yzx), right_dual_g1 * this.e3215), 
            /* e235, e315, e125, e4 */
            float4((this.group3.xyz * right_dual_g0.www), -(right_dual_g1 * this.e45) - (right_dual_g0[1] * this.e42) - (right_dual_g0[2] * this.e43)) - (right_dual_g0.xyzx * float4(this.group3.www, this.e41)), 
            /* e1, e2, e3, e5 */
            float4(-(right_dual_g1 * this.e15) - (right_dual_g0[1] * this.e12), -(right_dual_g1 * this.e25) - (right_dual_g0[2] * this.e23), -(right_dual_g1 * this.e35) - (right_dual_g0[0] * this.e31), (right_dual_g0[2] * this.e35) + (right_dual_g0[3] * this.e45)) + (right_dual_g0.zxyx * float4(this.group1.yzx, this.e15)) + (right_dual_g0.wwwy * float4(this.group0.xyz, this.e25))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(Scalar other) {
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
public Scalar operator /(BulkContractionInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public Scalar bulk_contraction(Sphere other) {
        float4 right_dual_g0 = float4(other.group0.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return Scalar(
            /* scalar */
            (right_dual_g0[0] * this.e4235) + (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g0[3] * this.e3215) + (other.e3215 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(BulkContractionInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion bulk_contraction(VersorEven other) {
        float4 right_dual_g0 = other.group0 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g1 = other.group1 * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e4) * float4(1.0, 1.0, 1.0, -1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e5) * float4(1.0, 1.0, 1.0, -1.0);
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * right_dual_g3.xyz) - (float3(right_dual_g2[3]) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (right_dual_g3.yzxw * float4(this.group3.zxy, this.e1234)) - (this.group3.yzxw * float4(right_dual_g3.zxy, right_dual_g2[3])), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(right_dual_g3.www, right_dual_g0[0])) + float4(float3(0.0), (right_dual_g0[1] * this.e4315) + (right_dual_g0[2] * this.e4125) + (right_dual_g1[3] * this.e1234) - (right_dual_g3[0] * this.e41) - (right_dual_g3[1] * this.e42) - (right_dual_g3[2] * this.e43)) - float4((right_dual_g3.xyz * this.group3.www), right_dual_g2[3] * this.e45), 
            /* e1, e2, e3, e5 */
            (float4(right_dual_g3[3]) * float4(this.group0.xyz, this.e45)) + (right_dual_g3.zxyz * float4(this.group1.yzx, this.e35)) + (this.group2.wwwy * float4(right_dual_g2.xyz, right_dual_g3[1])) + float4((right_dual_g1.zxy * this.group3.yzx), right_dual_g3[0] * this.e15) - (float4(this.e3215) * float4(right_dual_g0.xyz, right_dual_g1[3])) - (right_dual_g2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(right_dual_g1.yzx, right_dual_g2[0])) - float4((right_dual_g3.yzx * this.group1.zxy), right_dual_g2[2] * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(BulkContractionInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.bulk_contraction(rhs);
}
public extension VersorOdd {
    public VersorOdd bulk_contraction(VersorOdd other) {
        float4 right_dual_g0 = other.group0 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g1 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g2 = float4(other.group2.xyz, other.e3215) * float4(-1.0, -1.0, -1.0, 1.0);
        float4 right_dual_g3 = float4(other.group3.xyz, other.e1234) * float4(-1.0, -1.0, -1.0, 1.0);
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (right_dual_g0.zxyw * float4(this.group3.yzx, this.scalar)) + float4(float3(0.0), (right_dual_g3[1] * this.e4315) + (right_dual_g3[2] * this.e4125) + (right_dual_g3[3] * this.e3215) - (right_dual_g0[1] * this.e25) - (right_dual_g0[2] * this.e35) - (right_dual_g1[0] * this.e23) - (right_dual_g1[1] * this.e31) - (right_dual_g1[2] * this.e12) - (right_dual_g1[3] * this.e45) - (right_dual_g2[0] * this.e41) - (right_dual_g2[1] * this.e42) - (right_dual_g2[2] * this.e43)) + float4((right_dual_g1.xyz * this.group2.www), right_dual_g3[0] * this.e4235) + float4((this.group0.xyz * right_dual_g0.www), right_dual_g2[3] * this.e1234) - (right_dual_g0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            float4((right_dual_g0[3] * this.e23) + (right_dual_g2[0] * this.e1234), (right_dual_g0[3] * this.e31) + (right_dual_g2[1] * this.e1234), (right_dual_g0[3] * this.e12) + (right_dual_g2[2] * this.e1234), -(right_dual_g1[1] * this.e4315) - (right_dual_g1[2] * this.e4125)) + (right_dual_g0 * float4(this.group3.www, this.e45)) - (right_dual_g1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_dual_g0[3]) * this.group2.xyz) + (float3(this.e3215) * right_dual_g1.xyz) + (right_dual_g2.yzx * this.group3.zxy) - (right_dual_g2.zxy * this.group3.yzx)), right_dual_g0[3] * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual_g0[3]) * this.group3
        );
    }
}
