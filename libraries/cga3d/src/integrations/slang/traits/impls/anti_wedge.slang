implementing cga3d;
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane anti_wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiPlane anti_wedge(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(AntiScalar other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(DualNum other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4(other.e12345) * this.group2
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Flector other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(Motor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group9.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group1.xyz, 
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
public RoundPoint operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Plane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiCircleRotor {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e3215) * float4(this.group0.xyz, this.e4), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), (this.e4 * other.e3215) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(AntiScalar other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((this.group0.yzx * other.group2.zxy), this.e423 * other.e415) - float4((other.group0.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(other.e12345) * this.group3) + float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (this.group2.zxyx * float4(other.group3.yzx, other.e41))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(DualNum other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4(other.e12345) * this.group2, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Flector other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) - float4((this.group2.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public RoundPoint anti_wedge(Line other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e431 * other.e425) - (this.e412 * other.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(Motor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            (other.group0 * float4(float3(this.e321), this.e5)) + float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(other.e12345) * this.group3.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * float4(this.e1, this.e2, this.e3, this.e4)) + float4(float3(0.0), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group8.yzx), 0.0) + float4((other.group7.zxy * this.group2.yzx), 0.0) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((other.group7.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Plane other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125)) - float4((this.group2.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(Sphere other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group0.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) - float4((this.group2.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), (this.e4 * other.e12345) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            (float4(other.e12345) * this.group3) + float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) - (this.group2.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(other.e3215) * float4(this.group1.xyz, this.e4)) + (other.group3.zxyx * float4(this.group2.yzx, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (this.group2.zxyx * float4(other.group3.yzx, other.e41))
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiPlane anti_wedge(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(AntiDipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e3215) * float4(other.group0.xyz, other.e4), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_wedge(AntiScalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiLine anti_wedge(Circle other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e3215) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e3215) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0.xy.xxxy * float4(other.group0.xyz, other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiPlane anti_wedge(Dipole other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(DipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiDualNum anti_wedge(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e12345) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e45 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e3215 * other.e45 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e3215 * other.e4) + (this.scalar * other.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e3215 * -1.0) * other.group4.xyz), 0.0), 
            /* e5 */
            this.e3215 * other.e45 * -1.0, 
            /* e15, e25, e35, e45 */
            float4((float3(this.e3215) * other.group6.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e3215) * other.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215 * -1.0) * other.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint anti_wedge(Plane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint anti_wedge(Sphere other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), (this.e3215 * other.e4) + (this.scalar * other.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(this.e3215) * float4(other.group1.xyz, other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlector anti_wedge(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(AntiDipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_wedge(AntiScalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(CircleRotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e1234) * this.group0.xyz), 0.0) - (float4(this.e321) * float4(other.group3.xyz, other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint anti_wedge(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e321 * -1.0) * float4(other.group1.xyz, other.e45), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiPlane anti_wedge(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(Motor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.zxy * this.group0.yzx), 0.0) - float4((other.group7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiLine anti_wedge(Plane other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e321 * -1.0) * other.group0.xyz, 
            /* e15, e25, e35 */
            (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e315 * other.e4235) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiLine anti_wedge(Sphere other) {
        return AntiLine(
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e15, e25, e35 */
            (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e315 * other.e4235) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlector anti_wedge(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiMotor anti_wedge(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e1234) * this.group0.xyz), 0.0) - (float4(this.e321) * float4(other.group3.xyz, other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiFlector> operator /(AntiFlector lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_wedge(AntiDipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_wedge(AntiScalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_wedge(Circle other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_wedge(CircleRotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor anti_wedge(DipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (float4(this.e321) * float4(other.group3.xyz, other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_wedge(DualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            this.e321 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Flector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * other.group1.xyz), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiPlane anti_wedge(Line other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(this.e321) * other.group0.xyz), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_wedge(Motor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            (other.group0 * float4(float3(this.e321), this.e5)) + float4(float3(0.0), -(this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(other.e12345) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((other.group7.zxy * this.group0.yzx), 0.0) - float4((other.group7.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            (this.e5 * other.e12345) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(((this.group0.yzx * other.group9.zxy) - (this.group0.zxy * other.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group0.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Plane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e321 * -1.0) * other.group0.xyz), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor anti_wedge(Sphere other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)) - float4((float3(this.e321) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector anti_wedge(VersorEven other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e12345) * this.group0, 
            /* e1, e2, e3, e5 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (this.group0.zxyx * float4(other.group0.yzx, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor anti_wedge(VersorOdd other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e1234) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (float4(this.e321) * float4(other.group3.xyz, other.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiLine> operator /(AntiLine lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiLine> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiLine anti_wedge(AntiScalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e12345) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiPlane anti_wedge(DipoleInversion other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e31, this.e12, this.e23, this.e15) * other.group3.zxyx) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiLine anti_wedge(DualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiPlane anti_wedge(Flector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e31, this.e12, this.e23, this.e15) * other.group1.zxyx) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e12345) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4((float3(other.e12345) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group0.xyz, 
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
public AntiPlane operator /(AntiWedgeInfixPartial<AntiLine> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiPlane anti_wedge(Plane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e31, this.e12, this.e23, this.e15) * other.group0.zxyx) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiLine> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiPlane anti_wedge(Sphere other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e31, this.e12, this.e23, this.e15) * other.group0.zxyx) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e12345) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiLine {
    public AntiPlane anti_wedge(VersorOdd other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            (float4(this.e31, this.e12, this.e23, this.e15) * other.group3.zxyx) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiMotor> operator /(AntiMotor lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiPlane anti_wedge(AntiCircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(AntiDipoleInversion other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), (other.e4 * this.e3215) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(AntiScalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Circle other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e3215) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(CircleRotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(this.e3215) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiPlane anti_wedge(Dipole other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector anti_wedge(DipoleInversion other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234), 
            /* e1, e2, e3, e5 */
            (other.group3.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - (this.group1 * float4(float3(other.e1234), other.e45)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(DualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e12345) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e3215 * other.e45 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Flector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45)) - float4((this.group0.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Line other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(Motor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e12345) * this.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.group0.yzx * other.group9.zxy), 0.0) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group9.yzx), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            float4(((float3(this.e3215) * other.group6.xyz) + (float3(other.e12345) * this.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e3215 * -1.0) * other.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215 * other.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Plane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(this.e3215 * -1.0) * other.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            (other.group0.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            this.e3215 * other.e4
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector anti_wedge(Sphere other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group0.xyz, other.e1234), 
            /* e1, e2, e3, e5 */
            (other.group0.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(other.e1234) * this.group1.xyz), 0.0) - float4((this.group0.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor anti_wedge(VersorEven other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e3215) * float4(other.group0.xyz, other.e4)) + (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e3215 * other.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector anti_wedge(VersorOdd other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e3215 * -1.0) * float4(other.group3.xyz, other.e1234), 
            /* e1, e2, e3, e5 */
            (other.group3.zxyx * float4(this.group0.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - (this.group1 * float4(float3(other.e1234), other.e45)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((this.group0.zxy * other.group3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiPlane> operator /(AntiPlane lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiPlane> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_wedge(AntiScalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_wedge(CircleRotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public Scalar anti_wedge(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_wedge(DualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_wedge(Motor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e5 */
            this.e5 * other.e12345, 
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
public Scalar operator /(AntiWedgeInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public Scalar anti_wedge(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane anti_wedge(VersorEven other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiPlane {
    public Scalar anti_wedge(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<AntiScalar> operator /(AntiScalar lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiDualNum anti_wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiFlatPoint anti_wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiLine anti_wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(AntiWedgeInfixPartial<AntiScalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar anti_wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Circle anti_wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiWedgeInfixPartial<AntiScalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public CircleRotor anti_wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.e12345) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Dipole anti_wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<AntiScalar> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public DipoleInversion anti_wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<AntiScalar> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<AntiScalar> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Line anti_wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.e12345) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5 */
            this.e12345 * other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<AntiScalar> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiWedgeInfixPartial<AntiScalar> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public Sphere anti_wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<AntiScalar> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension AntiScalar {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e12345) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Circle> operator /(Circle lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiLine anti_wedge(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e3215) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.e3215) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Circle> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<Circle> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Circle anti_wedge(AntiScalar other) {
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
public RoundPoint operator /(AntiWedgeInfixPartial<Circle> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group0.yzx), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Circle> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) - float4((this.group2.zxy * other.group0.yzx), this.e425 * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group3.zxy), 0.0) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Circle anti_wedge(DualNum other) {
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
public Scalar operator /(AntiWedgeInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Scalar anti_wedge(FlatPoint other) {
        return Scalar(
            /* scalar */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiCircleRotor anti_wedge(Flector other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (this.group0.zxy * other.group1.yzx) - (this.group0.yzx * other.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - float4((this.group2.zxy * other.group1.yzx), this.e423 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Circle> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public RoundPoint anti_wedge(Line other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e431 * other.e425) - (this.e412 * other.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(Motor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group8.yzx), 0.0) + float4((this.group2.yzx * other.group7.zxy), 0.0) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((this.group2.zxy * other.group7.yzx), this.e431 * other.e425), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Circle> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Dipole anti_wedge(Plane other) {
        return Dipole(
            /* e41, e42, e43 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Circle> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public Dipole anti_wedge(Sphere other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group1.xyz) + (this.group2.yzx * other.group0.zxy) - (this.group2.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e12345) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx) - float4((this.group2.zxy * other.group0.yzx), this.e425 * other.e315)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Circle {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group3.zxy), 0.0) - float4((this.group2.zxy * other.group3.yzx), this.e423 * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<CircleRotor> operator /(CircleRotor lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group3) + float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group2.zxyx) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group2.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            other.group0.xy.xxxy * float4(this.group0.xyz, this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (float4(this.e431, this.e412, this.e423, this.e415) * other.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(other.e3215) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor anti_wedge(AntiScalar other) {
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
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group2.zxyx) - float4((other.group2.zxy * this.group0.yzx), other.e425 * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<CircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            float4((other.e235 * this.e12345) + (other.e12345 * this.e235), (other.e315 * this.e12345) + (other.e12345 * this.e315), (other.e125 * this.e12345) + (other.e12345 * this.e125), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415) - float4((this.group0.yzx * other.group2.zxy), other.e431 * this.e425)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group3.yzx), 0.0) - float4((this.group0.yzx * other.group3.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            (this.group2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group2.yz * other.group3.zx), 0.0) - float3((this.group2.zx * other.group3.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorEven anti_wedge(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            this.group2 * float4(float3(other.e12345), other.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiCircleRotor anti_wedge(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e12345 * other.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorOdd anti_wedge(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)) + float4((this.group0.zxy * other.group1.yzx), 0.0) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), this.e12345 * other.e45) - (this.group1.wwwx * other.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group1.zxy), 0.0) - float4((this.group2.zxy * other.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public AntiDipoleInversion anti_wedge(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - float4((this.group0.yzx * other.group1.zxy), this.e415 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorEven anti_wedge(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * float4(this.group0.xyz, this.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group1.xyz)), this.e321 * other.e12345), 
            /* e235, e315, e125, e5 */
            (this.group2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e12345) * other.group1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e431 * other.e425) - (this.e412 * other.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - float4((this.group0.yzx * other.group1.zxy), this.e423 * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(this.e412 * other.e435) - (this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group8.yzx), 0.0) + float4((other.group7.zxy * this.group2.yzx), 0.0) - float4((this.group0.yzx * other.group8.zxy), this.e423 * other.e415) - float4((other.group7.yzx * this.group2.zxy), this.e431 * other.e425), 
            /* e5 */
            (this.e12345 * other.e5) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * other.group3) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group9.yzx) - (this.group0.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group5.xyz) + (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group6) + (float4(other.e12345) * this.group1), 
            /* e423, e431, e412 */
            (float3(this.e12345) * other.group7.xyz) + (float3(other.e12345) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public DipoleInversion anti_wedge(Plane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - float4((this.group2.zxy * other.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<CircleRotor> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public DipoleInversion anti_wedge(Sphere other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group2.yz * other.group0.zx), 0.0) - float3((this.group2.zx * other.group0.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e12345), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group1), 
            /* e235, e315, e125, e5 */
            (this.group2 * float4(float3(other.e12345), other.e5)) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(this.e12345) * other.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e12345) * other.group3) + float4(float3(0.0), -(this.e431 * other.e425) - (this.e412 * other.e435) - (this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) + float4((this.group2.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group2.zxy, this.e415)) - float4((this.group0.yzx * other.group2.zxy), this.e423 * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension CircleRotor {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e12345) * other.group0) + float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e1234) * this.group1.xyz), 0.0) + float4((this.group0.zxy * other.group3.yzx), 0.0) - float4((this.group0.yzx * other.group3.zxy), this.e423 * other.e15), 
            /* e23, e31, e12, e45 */
            (this.group2 * float4(float3(other.e1234), other.e45)) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group2.yz * other.group3.zx), 0.0) - float3((this.group2.zx * other.group3.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Dipole> operator /(Dipole lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public AntiPlane anti_wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public AntiPlane anti_wedge(AntiMotor other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Dipole> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Dipole anti_wedge(AntiScalar other) {
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
public Scalar operator /(AntiWedgeInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Dipole anti_wedge(DualNum other) {
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
public RoundPoint operator /(AntiWedgeInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public RoundPoint anti_wedge(Flector other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public Scalar anti_wedge(Line other) {
        return Scalar(
            /* scalar */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(Motor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group9.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e12345) * this.group1.xyz, 
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
public RoundPoint operator /(AntiWedgeInfixPartial<Dipole> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public RoundPoint anti_wedge(Plane other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Dipole> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e12345) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e12345) * this.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Dipole {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group3.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (other.group2.zxyx * float4(this.group3.yzx, this.e41))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e1234) * other.group0.xyz), 0.0) - (float4(other.e321) * float4(this.group3.xyz, this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e1234) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (float4(other.e321) * float4(this.group3.xyz, this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e31, other.e12, other.e23, other.e15) * this.group3.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            (other.group1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion anti_wedge(AntiScalar other) {
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
public AntiCircleRotor operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group3.zxy), 0.0) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - float4((other.group0.yzx * this.group3.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group2.yz * this.group3.zx), 0.0) - float3((other.group2.zx * this.group3.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group3.yzxw) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group3.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyy * float4(this.group1.yzx, this.e25)) + (this.group2.wwwx * float4(other.group2.xyz, other.e4235)) + float4(float3(0.0), (other.e4125 * this.e35) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group3.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(other.e12345) * this.group0.xyz), this.e1234 * other.e5), 
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
public RoundPoint operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group1.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group3.yz * other.group1.zx), 0.0)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group3.xyz), this.e1234 * other.e45) - (other.group1.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group0.xyz, other.e4315)) + (other.group1.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45)) - float4((this.group1.zxy * other.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiCircleRotor anti_wedge(Line other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - float4((other.group1.zxy * this.group3.yzx), this.e41 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e1234) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e4235 * other.e415) - (this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group0.xyz) + (float3(other.e12345) * this.group2.xyz) + float3(float2(0.0), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + float3((this.group3.zx * other.group1.yz), 0.0) - float3((this.group3.yz * other.group1.zx), 0.0)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group3) + (this.group3.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), this.e4125 * other.e43) + float4((float3(other.e3215) * this.group0.xyz), this.e4315 * other.e42) + float4((this.group1.yzx * other.group9.zxy), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(this.e3215) * other.group4.xyz), this.e43 * other.e4125) - float4((other.group5.yzx * this.group3.zxy), this.e42 * other.e4315), 
            /* e5 */
            (this.e45 * other.e3215) + (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(other.e12345) * this.group1.xyz) - (float3(other.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group9.yzxw) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234 */
            this.e1234 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Plane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group3.zxy * other.group0.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group3.yz * other.group0.zx), 0.0)), this.e1234 * other.e3215), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((this.group1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group0.yzxw) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((this.group1.zxy * other.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e1234) * float4(other.group1.xyz, other.e5)) + (this.group3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(other.e12345) * this.group0.xyz), this.e4315 * other.e2) - (other.group0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (this.group3.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (float3(other.e12345) * this.group2.xyz) + float3(float2(0.0), (this.e4315 * other.e235) - (this.e4235 * other.e315)) + float3((this.group3.zx * other.group2.yz), 0.0) - float3((this.group3.yz * other.group2.zx), 0.0)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DipoleInversion {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group3.yzxw) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (this.e1234 * other.e45) + (this.e4315 * other.e42) + (this.e4125 * other.e43) - (this.e42 * other.e4315) - (this.e43 * other.e4125) - (this.e45 * other.e1234)) - (other.group3.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (this.group2.wwwy * float4(other.group2.xyz, other.e4315)) + (other.group3.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (this.e35 * other.e4125) - (this.e4125 * other.e35)) + float4((this.group3.yzx * other.group1.zxy), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group3.zxyy * float4(other.group1.yzx, other.e25)) - (other.group2.wwwx * float4(this.group2.xyz, this.e4235)) - float4((this.group1.zxy * other.group3.yzx), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<DualNum> operator /(DualNum lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.e12345) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.e12345) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiDualNum anti_wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e12345) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiFlatPoint anti_wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiLine anti_wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.e12345) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.e12345) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<DualNum> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public DualNum anti_wedge(AntiScalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e12345) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Circle anti_wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            other.group2 * float4(float3(this.e12345), this.e5), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Dipole anti_wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35 */
            float3(this.e12345) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(this.e12345) * other.group0.xyz), other.e1234 * this.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public DualNum anti_wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2((other.e5 * this.e12345) + (other.e12345 * this.e5), other.e12345 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<DualNum> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Line anti_wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e12345) * other.group0, 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * other.group1.xyz), (this.e5 * other.e12345) + (this.e12345 * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e5 * other.e1234) + (this.e12345 * other.scalar), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group1, 
            /* e5 */
            (this.e5 * other.e12345) + (this.e12345 * other.e5), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group3, 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group6, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Plane anti_wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), this.e5 * other.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * other.group2.xyz), (this.e5 * other.e12345) + (this.e12345 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension DualNum {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(this.e12345) * other.group0.xyz), (this.e5 * other.e1234) + (this.e12345 * other.scalar)), 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.e12345) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<FlatPoint> operator /(FlatPoint lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiDipoleInversion other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public DualNum anti_wedge(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e45, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiFlatPoint other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public Scalar anti_wedge(AntiFlector other) {
        return Scalar(
            /* scalar */
            other.e321 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public DualNum anti_wedge(AntiMotor other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.e3215 * this.e45, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_wedge(AntiScalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public Scalar anti_wedge(Circle other) {
        return Scalar(
            /* scalar */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_wedge(CircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e12345 * this.e45), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public RoundPoint anti_wedge(DipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234 * -1.0) * this.group0, 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<FlatPoint> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public DualNum anti_wedge(Flector other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public FlatPoint anti_wedge(Motor other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e1234 * -1.0) * this.group0, 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
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
public DualNum operator /(AntiWedgeInfixPartial<FlatPoint> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public DualNum anti_wedge(Plane other) {
        return DualNum(
            /* e5, e12345 */
            float2((this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public RoundPoint anti_wedge(Sphere other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234 * -1.0) * this.group0, 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public AntiCircleRotor anti_wedge(VersorEven other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), this.e45 * other.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(other.e12345) * this.group0.xyz), -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension FlatPoint {
    public RoundPoint anti_wedge(VersorOdd other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e1234 * -1.0) * this.group0, 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Flector> operator /(Flector lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group1.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) - float4((other.group2.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), other.e3215 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.e321 * -1.0) * float4(this.group1.xyz, this.e45), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * this.group1.xyz), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (float4(other.e31, other.e12, other.e23, other.e15) * this.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group1.yzx), other.e3215 * this.e45) - (this.group1.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(AntiScalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * this.group1.yzx) - (other.group0.yzx * this.group1.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) - float4((other.group2.zxy * this.group1.yzx), other.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)) + float4((other.group0.zxy * this.group1.yzx), 0.0) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), other.e12345 * this.e45) - (other.group1.wwwx * this.group1.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group1.zxy), 0.0) - float4((other.group2.zxy * this.group1.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group1.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group3.zx * this.group1.yz), 0.0)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) - (other.e1234 * this.e45)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group1.yzx), other.e4235 * this.e15) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group0.xyz, this.e4315)) - (this.group1.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e12345) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<Flector> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group1.zxy) - (other.group1.zxy * this.group1.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(other.e3215) * float4(this.e4235, this.e4315, this.e4125, this.e45)) + float4(float3(0.0), (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) - (float4(this.e3215) * float4(other.e4235, other.e4315, other.e4125, other.e45))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((this.group1.zxy * other.group1.yzx), 0.0) - (this.group1.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group1.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43)) - (float4(other.e1234) * this.group0) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((other.group5.yzx * this.group1.zxy), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * this.group0) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (other.group7.zxy * this.group1.yzx) - (other.group7.yzx * this.group1.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group9.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group1.yz * other.group9.zx), 0.0)), this.e3215 * other.e1234 * -1.0), 
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Motor anti_wedge(Plane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)), 0.0), 
            /* e235, e315, e125, e5 */
            (float4(other.e3215) * float4(this.e4235, this.e4315, this.e4125, this.e45)) + float4(float3(0.0), (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Flector> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group0.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group1.yz * other.group0.zx), 0.0)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group1.xyz) - (float3(this.e3215) * other.group0.xyz)), this.e45 * other.e1234 * -1.0), 
            /* e1, e2, e3, e5 */
            float4(other.e1234, other.e1234, other.e1234, (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215)) * float4((this.group0.xyz * float3(-1.0)), 1.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group1.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)) - (other.group0.yzxx * float4(this.group1.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((float3(other.e12345) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group2.yzx), 0.0) - float4((this.group1.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Flector {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group1.zxy * other.group3.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group1.yz * other.group3.zx), 0.0)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group1.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43) - (this.e45 * other.e1234)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4125 * other.e35)) + float4((this.group1.yzx * other.group1.zxy), this.e15 * other.e4235) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group1.zxyy * float4(other.group1.yzx, other.e25)) - (other.group2.wwwx * float4(this.group0.xyz, this.e4235))
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Line> operator /(Line lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Scalar anti_wedge(AntiCircleRotor other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public RoundPoint anti_wedge(AntiDipoleInversion other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public FlatPoint anti_wedge(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Line> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiPlane anti_wedge(AntiFlatPoint other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiPlane anti_wedge(AntiFlector other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Scalar anti_wedge(AntiLine other) {
        return Scalar(
            /* scalar */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Line> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Line anti_wedge(AntiScalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Line> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public RoundPoint anti_wedge(Circle other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Line> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_wedge(CircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - float4((other.group0.yzx * this.group1.zxy), other.e415 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Scalar anti_wedge(Dipole other) {
        return Scalar(
            /* scalar */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiCircleRotor anti_wedge(DipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - float4((this.group1.zxy * other.group3.yzx), other.e41 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Line> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Line anti_wedge(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e12345) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public FlatPoint anti_wedge(Flector other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public DualNum anti_wedge(Line other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group7.zxy), 0.0) - float4((this.group1.zxy * other.group7.yzx), this.e415 * other.e423), 
            /* e5 */
            -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e12345) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public FlatPoint anti_wedge(Plane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Line> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public Dipole anti_wedge(Sphere other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.e1234, other.e1234, other.e1234, -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)) * float4(this.group1.xyz, 1.0), 
            /* e15, e25, e35 */
            (float3(other.e3215) * this.group0.xyz) + (this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiDipoleInversion anti_wedge(VersorEven other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e12345) * this.group1.xyz), -(this.e415 * other.e423) - (this.e425 * other.e431) - (this.e435 * other.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - float4((this.group1.zxy * other.group0.yzx), this.e415 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Line {
    public AntiCircleRotor anti_wedge(VersorOdd other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(other.e1234) * this.group1.xyz), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group3.zxy), 0.0) - float4((this.group1.zxy * other.group3.yzx), this.e415 * other.e23)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Motor> operator /(Motor lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            (this.group0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(this.e12345) * other.group3.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4((float3(other.e321) * this.group0.xyz), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            (this.group0 * float4(float3(other.e321), other.e5)) + float4(float3(0.0), -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(AntiScalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.e12345) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group1.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * float4(other.group0.xyz, other.e12345), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e321 * this.e12345), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(this.e12345), this.e5)) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e12345) * this.group1.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435)) + float4((float3(other.e321) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group1.yzx), 0.0) - float4((other.group0.yzx * this.group1.zxy), other.e423 * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(this.group0.xyz, this.e5)) + float4(float3(0.0), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e4235 * this.e415) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e1234) * this.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + (float3(this.e12345) * other.group2.xyz) + float3(float2(0.0), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + float3((other.group3.zx * this.group1.yz), 0.0) - float3((other.group3.yz * this.group1.zx), 0.0)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e12345) * this.group0, 
            /* e235, e315, e125, e5 */
            float4((float3(other.e12345) * this.group1.xyz), (other.e5 * this.e12345) + (other.e12345 * this.e5))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Motor> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public FlatPoint anti_wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Flector anti_wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((other.group1.zxy * this.group1.yzx), 0.0) - (other.group1.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Motor anti_wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            (this.group0 * float4(float3(other.e321), other.e4)) + float4(float3(0.0), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((other.group7.zxy * this.group1.yzx), 0.0) - float4((other.group7.yzx * this.group1.zxy), this.e415 * other.e423), 
            /* e5 */
            (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(this.e12345) * other.group3.xyz), 0.0) + float4((this.group1.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group4.xyz) + (float3(other.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group5.xyz) + (float3(other.e1234) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group6.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321), 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group7.xyz, 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group8.xyz) + (float3(other.e12345) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Flector anti_wedge(Plane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e1234) * float4(this.group0.xyz, this.e5), 
            /* e23, e31, e12, e45 */
            float4(other.e1234, other.e1234, other.e1234, -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)) * float4(this.group1.xyz, 1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group0.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group1.yz * other.group0.zx), 0.0) - float3((this.group1.zx * other.group0.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e12345) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e12345) * other.group1.xyz) + (float3(other.e12345) * this.group0.xyz)), this.e12345 * other.e321), 
            /* e235, e315, e125, e5 */
            (float4(this.e12345) * other.group2) + (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435)), 
            /* e1, e2, e3, e4 */
            (this.group0 * float4(float3(other.e321), other.e4)) + float4(float3(0.0), -(this.e425 * other.e431) - (this.e435 * other.e412)) + float4((float3(this.e12345) * other.group3.xyz), 0.0) + float4((this.group1.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group1.zxy, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Motor {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0 * float4(float3(other.e1234), other.scalar)) + float4(float3(0.0), (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(this.e415 * other.e4235) - (this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e1234) * this.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group0.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group1.yz * other.group3.zx), 0.0) - float3((this.group1.zx * other.group3.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<MultiVector> operator /(MultiVector lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group1.xyz, 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e12345) * float4(other.e1, other.e2, other.e3, other.e4)) + float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group8.yzx), 0.0) + float4((this.group7.zxy * other.group2.yzx), 0.0) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((this.group7.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e3215 * this.e4) + (other.scalar * this.e12345), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e3215) * this.group4.xyz), 0.0), 
            /* e5 */
            other.e3215 * this.e45, 
            /* e15, e25, e35, e45 */
            float4((float3(other.e3215) * this.group6.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e3215) * this.group7.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((this.group7.zxy * other.group0.yzx), 0.0) - float4((this.group7.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            -(other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((this.group7.zxy * other.group0.yzx), 0.0) - float4((this.group7.yzx * other.group0.zxy), 0.0), 
            /* e5 */
            (other.e5 * this.e12345) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(((other.group0.yzx * this.group9.zxy) - (other.group0.zxy * this.group9.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.e12345), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group9.yzx), 0.0) - float4((other.group0.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e3215) * this.group6.xyz) + (float3(this.e12345) * other.group1.xyz)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e3215) * this.group9.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e3215 * this.e12345), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e5 */
            other.e5 * this.e12345, 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(AntiScalar other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.e12345) * this.group0.xy, 
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
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group8.yzx), 0.0) + float4((other.group2.yzx * this.group7.zxy), 0.0) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((other.group2.zxy * this.group7.yzx), other.e431 * this.e425), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group8.yzx), 0.0) + float4((this.group7.zxy * other.group2.yzx), 0.0) - float4((other.group0.yzx * this.group8.zxy), other.e423 * this.e415) - float4((this.group7.yzx * other.group2.zxy), other.e431 * this.e425), 
            /* e5 */
            (other.e12345 * this.e5) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * this.group3) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group9.yzx) - (other.group0.yzx * this.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group1), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group2.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e12345) * other.group1.xyz, 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e4125 * this.e43) * -1.0) + float4((float3(other.e3215) * this.group4.xyz), other.e43 * this.e4125) + float4((this.group5.yzx * other.group3.zxy), other.e42 * this.e4315) - (float4(other.e1234) * this.group3) - (other.group3.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group0.xyz), other.e4315 * this.e42) - float4((other.group1.yzx * this.group9.zxy), 0.0), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * float4(other.e15, other.e25, other.e35, other.e45)) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((this.group8.yzx * other.group3.zxy), 0.0) - (other.group3.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(this.e12345) * other.group1.xyz) - (float3(this.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234 */
            other.e1234 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e5 * this.e1234) + (other.e12345 * this.scalar), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group1, 
            /* e5 */
            (other.e5 * this.e12345) + (other.e12345 * this.e5), 
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
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(this.e12345) * other.group0, 
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
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group0) + float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((this.group5.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((this.group8.yzx * other.group1.zxy), 0.0) - (other.group1.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.zxy * other.group1.yzx) - (this.group7.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group9.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group1.zx * this.group9.yz), 0.0)), other.e3215 * this.e1234), 
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(-(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group7.zxy), 0.0) - float4((other.group1.zxy * this.group7.yzx), other.e415 * this.e423), 
            /* e5 */
            -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e12345) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((this.group7.zxy * other.group1.yzx), 0.0) - float4((this.group7.yzx * other.group1.zxy), other.e415 * this.e423), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            (other.group0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(other.e12345) * this.group3.xyz), 0.0) + float4((other.group1.yzx * this.group9.zxy), 0.0) - (this.group9.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(this.e1234) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(this.e1234) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group6.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321), 
            /* e423, e431, e412 */
            float3(other.e12345) * this.group7.xyz, 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(this.e12345) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1) + (float4(this.e1234) * other.group3) + (this.group9.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.zxy * this.group8.yzx), other.e42 * this.e4315) + float4((other.group8.yzx * this.group7.zxy), other.e43 * this.e4125) + float4((this.group5.yzx * other.group9.zxy), 0.0) - (float4(other.e1234) * this.group3) - (other.group9.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group4.xyz), other.e415 * this.e423) - float4((other.group5.yzx * this.group9.zxy), other.e425 * this.e431) - float4((other.group7.yzx * this.group8.zxy), other.e435 * this.e412) - float4((other.group8.zxy * this.group7.yzx), other.e423 * this.e415), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) + (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125) - (other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group9.zxy), 0.0) + float4((this.group8.yzx * other.group9.zxy), 0.0) - (other.group9.yzxx * float4(this.group8.zxy, this.e415)) - (this.group9.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group4.xyz) + (float3(other.e1234) * this.group6.xyz) + (float3(this.e12345) * other.group4.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group9.yzx) + (this.group7.zxy * other.group9.yzx) - (other.group7.yzx * this.group9.zxy) - (this.group7.yzx * other.group9.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group5.xyz) + (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(this.e12345) * other.group5.xyz) + (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group9.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group6) + (float4(this.e12345) * other.group6) + (other.group9.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group7.xyz) + (float3(this.e12345) * other.group7.xyz) + (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group8.xyz) + (float3(other.e3215) * this.group9.xyz) + (float3(this.e12345) * other.group8.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e12345) * this.group9) + (float4(this.e12345) * other.group9), 
            /* e1234 */
            (other.e12345 * this.e1234) + (other.e1234 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((this.group5.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((this.group8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((this.group9.zxy * other.group0.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group9.yz * other.group0.zx), 0.0)), this.e1234 * other.e3215), 
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            this.e12345 * other.e5, 
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
public Scalar operator /(AntiWedgeInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group4.xyz), 0.0) + float4((this.group5.yzx * other.group0.zxy), 0.0) - (float4(other.e1234) * this.group3) - (other.group0.yzxx * float4(this.group5.zxy, this.e41)), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((this.group8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group0.yzx) - (this.group7.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group7.xyz) + (float3(other.e1234) * this.group8.xyz) - (float3(this.e321) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group9.zxy, this.e1234)) - (this.group9.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group0.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435), this.e12345 * other.e12345), 
            /* e1, e2, e3, e4 */
            (float4(this.e12345) * other.group3) + (float4(other.e12345) * this.group1) + float4(float3(0.0), -(this.e435 * other.e412) - (this.e423 * other.e415) - (this.e431 * other.e425) - (this.e412 * other.e435)) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((float3(other.e321) * this.group6.xyz), 0.0) + float4((this.group7.zxy * other.group2.yzx), 0.0) + float4((this.group8.yzx * other.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group8.zxy, this.e415)) - float4((this.group7.yzx * other.group2.zxy), this.e425 * other.e431), 
            /* e5 */
            (this.e12345 * other.e5) + (this.e5 * other.e12345) - (this.e415 * other.e235) - (this.e425 * other.e315) - (this.e435 * other.e125) - (this.e235 * other.e415) - (this.e315 * other.e425) - (this.e125 * other.e435), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * this.group3) + float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((this.group9.zxy * other.group2.yzx), 0.0) - (this.group9.yzxx * float4(other.group2.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (float3(other.e12345) * this.group4.xyz) + float3(float2(0.0), (this.e4235 * other.e431) - (this.e4315 * other.e423)) + float3((this.group9.yz * other.group0.zx), 0.0) - float3((this.group9.zx * other.group0.yz), 0.0), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group0.xyz) + (float3(this.e1234) * other.group2.xyz) + (float3(other.e12345) * this.group5.xyz) - (float3(other.e321) * this.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e12345) * other.group1) + (float4(other.e12345) * this.group6), 
            /* e423, e431, e412 */
            (float3(this.e12345) * other.group0.xyz) + (float3(other.e12345) * this.group7.xyz), 
            /* e235, e315, e125 */
            (float3(this.e12345) * other.group2.xyz) + (float3(other.e12345) * this.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group9, 
            /* e1234 */
            this.e1234 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension MultiVector {
    public MultiVector anti_wedge(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group9.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), -(this.e42 * other.e4315) - (this.e43 * other.e4125)) + float4((float3(other.e3215) * this.group4.xyz), this.e4315 * other.e42) + float4((this.group5.yzx * other.group3.zxy), this.e4125 * other.e43) - (float4(other.e1234) * this.group3) - (other.group3.yzxx * float4(this.group5.zxy, this.e41)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((this.group9.zxy * other.group1.yzx), 0.0), 
            /* e5 */
            (this.e15 * other.e4235) + (this.e25 * other.e4315) + (this.e35 * other.e4125) + (this.e45 * other.e3215) - (this.e4235 * other.e15) - (this.e4315 * other.e25) - (this.e4125 * other.e35) - (this.e3215 * other.e45), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * float4(other.e15, other.e25, other.e35, other.e45)) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(other.e3215) * this.group6.xyz), 0.0) + float4((this.group8.yzx * other.group3.zxy), 0.0) - (other.group3.yzxx * float4(this.group8.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(this.e12345) * other.group0.xyz) + (float3(other.e1234) * this.group6.xyz) + (this.group7.zxy * other.group3.yzx) - (this.group7.yzx * other.group3.zxy), 
            /* e23, e31, e12 */
            (float3(this.e12345) * other.group1.xyz) + (float3(other.e1234) * this.group8.xyz) + (float3(other.e3215) * this.group7.xyz) - (float3(this.e321) * other.group3.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group9.zxy, this.e1234)) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group9.yzxw), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group9.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group9.xyz) - (float3(this.e3215) * other.group3.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3, 
            /* e1234 */
            this.e12345 * other.e1234
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Plane> operator /(Plane lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125)) - float4((other.group2.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiFlatPoint anti_wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<Plane> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiLine anti_wedge(AntiFlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.e321 * -1.0) * this.group0.xyz, 
            /* e15, e25, e35 */
            (other.group0.yzx * this.group0.zxy) + float3(float2(0.0), (other.e315 * this.e4235) * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e321 * -1.0) * this.group0.xyz), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Plane> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e31, other.e12, other.e23, other.e15) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4((float3(other.e3215) * this.group0.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<Plane> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Plane anti_wedge(AntiScalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Plane> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Dipole anti_wedge(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<Plane> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public DipoleInversion anti_wedge(CircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - float4((other.group2.zxy * this.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Plane> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Plane> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group3.yzx * this.group0.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group3.zx * this.group0.yz), 0.0)), other.e1234 * this.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(AntiWedgeInfixPartial<Plane> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Plane anti_wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(AntiWedgeInfixPartial<Plane> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public DualNum anti_wedge(FlatPoint other) {
        return DualNum(
            /* e5, e12345 */
            float2(-(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(AntiWedgeInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Motor anti_wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (float4(this.e3215) * float4(other.e4235, other.e4315, other.e4125, other.e45))
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(AntiWedgeInfixPartial<Plane> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public FlatPoint anti_wedge(Line other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(AntiWedgeInfixPartial<Plane> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Flector anti_wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group1.zxy, other.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            (this.group0.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((other.group5.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((other.group9.yzx * this.group0.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group9.zx * this.group0.yz), 0.0)), other.e1234 * this.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(AntiWedgeInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Line anti_wedge(Plane other) {
        return Line(
            /* e415, e425, e435 */
            (other.group0.yzx * this.group0.zxy) + float3(float2(0.0), (other.e4315 * this.e4235) * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<Plane> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public Circle anti_wedge(Sphere other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group0.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group0.yz * other.group0.zx), 0.0)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (this.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) - float4((this.group0.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((this.group0.zxy * other.group2.yzx), 0.0) - float4((this.group0.yzx * other.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Plane {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(other.e1234 * -1.0) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((this.group0.zxy * other.group3.yzx) + float3(float2(0.0), this.e4235 * other.e4315 * -1.0) - float3((this.group0.yz * other.group3.zx), 0.0)), this.e3215 * other.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e4315 * other.e25) - (this.e4125 * other.e35)) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<RoundPoint> operator /(RoundPoint lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(AntiDualNum other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(AntiMotor other) {
        return Scalar(
            /* scalar */
            other.e3215 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<RoundPoint> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_wedge(AntiScalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_wedge(CircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(DipoleInversion other) {
        return Scalar(
            /* scalar */
            (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_wedge(DualNum other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(Flector other) {
        return Scalar(
            /* scalar */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_wedge(Motor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            other.e12345 * this.e5, 
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
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(Plane other) {
        return Scalar(
            /* scalar */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(Sphere other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint anti_wedge(VersorEven other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group0, 
            /* e5 */
            this.e5 * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension RoundPoint {
    public Scalar anti_wedge(VersorOdd other) {
        return Scalar(
            /* scalar */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Scalar> operator /(Scalar lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(AntiScalar other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(CircleRotor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(DualNum other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(Motor other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(MultiVector other) {
        return Scalar(
            /* scalar */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Scalar {
    public Scalar anti_wedge(VersorEven other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e12345
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<Sphere> operator /(Sphere lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((float3(this.e1234) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group0.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) - float4((other.group2.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiFlatPoint anti_wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiLine anti_wedge(AntiFlatPoint other) {
        return AntiLine(
            /* e23, e31, e12 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e15, e25, e35 */
            (other.group0.yzx * this.group0.zxy) + float3(float2(0.0), (other.e315 * this.e4235) * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e1234) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)) - float4((float3(other.e321) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (float4(other.e31, other.e12, other.e23, other.e15) * this.group0.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group0.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(AntiWedgeInfixPartial<Sphere> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Sphere anti_wedge(AntiScalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Sphere> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Dipole anti_wedge(Circle other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((float3(this.e1234) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group1.xyz) + (other.group2.yzx * this.group0.zxy) - (other.group2.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(AntiWedgeInfixPartial<Sphere> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public DipoleInversion anti_wedge(CircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((float3(this.e1234) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group2.yz * this.group0.zx), 0.0) - float3((other.group2.zx * this.group0.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Sphere> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group0.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Sphere> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group0.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), other.e5 * this.e1234), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<Sphere> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Sphere> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group0.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group1.zx * this.group0.yz), 0.0)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group1.xyz)), other.e45 * this.e1234), 
            /* e1, e2, e3, e5 */
            float4(this.e1234, this.e1234, this.e1234, -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) * float4(other.group0.xyz, 1.0)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(AntiWedgeInfixPartial<Sphere> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Dipole anti_wedge(Line other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e1234, this.e1234, this.e1234, -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)) * float4(other.group1.xyz, 1.0), 
            /* e15, e25, e35 */
            (float3(this.e3215) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.e1234) * float4(other.group0.xyz, other.e5), 
            /* e23, e31, e12, e45 */
            float4(this.e1234, this.e1234, this.e1234, -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)) * float4(other.group1.xyz, 1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group0.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group1.yz * this.group0.zx), 0.0) - float3((other.group1.zx * this.group0.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group3) + (this.group0.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group4.xyz), 0.0) - float4((other.group5.yzx * this.group0.zxy), 0.0), 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group0.yzx) - (other.group7.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.e3215) * other.group7.xyz) + (float3(this.e1234) * other.group8.xyz) - (float3(other.e321) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group9.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<Sphere> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Circle anti_wedge(Plane other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group0.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0)), other.e3215 * this.e1234), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(AntiWedgeInfixPartial<Sphere> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public Circle anti_wedge(Sphere other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group0.yzxw * float4(this.group0.zxy, this.e1234)) - (this.group0.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group0.xyz) - (float3(this.e3215) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e1234) * float4(other.group1.xyz, other.e5)) + (this.group0.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) - float4((this.group0.zxy * other.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(this.e4315 * other.e425) - (this.e4125 * other.e435)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((float3(this.e1234) * other.group2.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e3215) * other.group1.xyz) + (this.group0.zxy * other.group2.yzx) + float3(float2(0.0), this.e4235 * other.e315 * -1.0) - float3((this.group0.yz * other.group2.zx), 0.0)), this.e1234 * other.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension Sphere {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (other.group3.yzxw * float4(this.group0.zxy, this.e1234)) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group0.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group0.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (this.e4315 * other.e42) + (this.e4125 * other.e43) + (this.e1234 * other.e45)) - float4((float3(this.e3215) * other.group3.xyz), 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(this.e4315 * other.e25) - (this.e4125 * other.e35)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e15))
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<VersorEven> operator /(VersorEven lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), (other.e4 * this.e12345) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group3) + float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group2.zxyx) - (other.group2.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.e3215) * this.group0.xyz), (other.e3215 * this.e4) + (other.scalar * this.e12345)), 
            /* e15, e25, e35, e3215 */
            float4(other.e3215) * float4(this.group1.xyz, this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiFlector anti_wedge(AntiFlatPoint other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiFlector anti_wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.e12345) * other.group0, 
            /* e1, e2, e3, e5 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (other.group0.zxyx * float4(this.group0.yzx, this.e415))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.e12345) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiMotor anti_wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(other.e3215) * float4(this.group0.xyz, this.e4)) + (float4(this.e12345) * other.group0) + float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e15, e25, e35, e3215 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group1.xyz)), other.e3215 * this.e12345)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiPlane anti_wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<VersorEven> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorEven anti_wedge(AntiScalar other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
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
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(Circle other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e12345) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.e12345) * other.group1, 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435) - (other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (float4(other.e431, other.e412, other.e423, other.e415) * this.group2.zxyx) - float4((other.group2.zxy * this.group0.yzx), other.e425 * this.e315)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorEven anti_wedge(CircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (other.group2 * float4(float3(this.e12345), this.e5)) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(other.e12345) * this.group2.xyz), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * this.group3) + float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group2.zxy, other.e415)) - float4((other.group0.yzx * this.group2.zxy), other.e423 * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(Dipole other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e12345) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.e12345) * other.group1, 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorOdd anti_wedge(DipoleInversion other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(this.group1.xyz, this.e5)) + (other.group3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(this.e12345) * other.group0.xyz), other.e4315 * this.e2) - (this.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e1234) * this.group2.xyz), 0.0) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (other.group3.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (float3(this.e12345) * other.group2.xyz) + float3(float2(0.0), (other.e4315 * this.e235) - (other.e4235 * this.e315)) + float3((other.group3.zx * this.group2.yz), 0.0) - float3((other.group3.yz * this.group2.zx), 0.0)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorEven anti_wedge(DualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(other.e12345) * this.group1, 
            /* e235, e315, e125, e5 */
            float4((float3(other.e12345) * this.group2.xyz), (other.e5 * this.e12345) + (other.e12345 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiCircleRotor anti_wedge(FlatPoint other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), other.e45 * this.e12345), 
            /* e15, e25, e35, scalar */
            float4((float3(this.e12345) * other.group0.xyz), -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorOdd anti_wedge(Flector other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group1.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((float3(this.e12345) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group2.yzx), 0.0) - float4((other.group1.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public AntiDipoleInversion anti_wedge(Line other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e12345) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e12345) * other.group1.xyz), -(other.e415 * this.e423) - (other.e425 * this.e431) - (other.e435 * this.e412)), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)) + float4((float3(this.e321) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - float4((other.group1.zxy * this.group0.yzx), other.e415 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorEven anti_wedge(Motor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e12345) * this.group0, 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e12345) * this.group1.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e321), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            (other.group0 * float4(float3(this.e321), this.e4)) + float4(float3(0.0), -(other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e12345) * this.group3.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group1.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435), other.e12345 * this.e12345), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group1) + float4(float3(0.0), -(other.e435 * this.e412) - (other.e423 * this.e415) - (other.e431 * this.e425) - (other.e412 * this.e435)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group6.xyz), 0.0) + float4((other.group7.zxy * this.group2.yzx), 0.0) + float4((other.group8.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group8.zxy, other.e415)) - float4((other.group7.yzx * this.group2.zxy), other.e425 * this.e431), 
            /* e5 */
            (other.e12345 * this.e5) + (other.e5 * this.e12345) - (other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435), 
            /* e15, e25, e35, e45 */
            (float4(this.e12345) * other.group3) + float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((other.group9.zxy * this.group2.yzx), 0.0) - (other.group9.yzxx * float4(this.group2.zxy, this.e415)), 
            /* e41, e42, e43 */
            (float3(other.e1234) * this.group1.xyz) + (float3(this.e12345) * other.group4.xyz) + float3(float2(0.0), (other.e4235 * this.e431) - (other.e4315 * this.e423)) + float3((other.group9.yz * this.group0.zx), 0.0) - float3((other.group9.zx * this.group0.yz), 0.0), 
            /* e23, e31, e12 */
            (float3(other.e3215) * this.group0.xyz) + (float3(other.e1234) * this.group2.xyz) + (float3(this.e12345) * other.group5.xyz) - (float3(this.e321) * other.group9.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group6), 
            /* e423, e431, e412 */
            (float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group7.xyz), 
            /* e235, e315, e125 */
            (float3(other.e12345) * this.group2.xyz) + (float3(this.e12345) * other.group8.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group9, 
            /* e1234 */
            other.e1234 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorOdd anti_wedge(Plane other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) - float4((other.group0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.e3215) * this.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) - float4((other.group0.yzx * this.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public RoundPoint anti_wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345) * other.group0, 
            /* e5 */
            other.e5 * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public Scalar anti_wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorOdd anti_wedge(Sphere other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e1234) * float4(this.group1.xyz, this.e5)) + (other.group0.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) - float4((other.group0.zxy * this.group0.yzx), 0.0), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e4315 * this.e425) - (other.e4125 * this.e435)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e3215) * this.group1.xyz) + (other.group0.zxy * this.group2.yzx) + float3(float2(0.0), other.e4235 * this.e315 * -1.0) - float3((other.group0.yz * this.group2.zx), 0.0)), other.e1234 * this.e12345), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(AntiWedgeInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorEven anti_wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(((float3(other.e12345) * this.group0.xyz) + (float3(this.e12345) * other.group0.xyz)), other.e12345 * this.e12345), 
            /* e415, e425, e435, e321 */
            (float4(other.e12345) * this.group1) + (float4(this.e12345) * other.group1), 
            /* e235, e315, e125, e5 */
            (float4(other.e12345) * this.group2) + (float4(this.e12345) * other.group2) + float4(float3(0.0), -(other.e415 * this.e235) - (other.e425 * this.e315) - (other.e435 * this.e125) - (other.e235 * this.e415) - (other.e315 * this.e425) - (other.e125 * this.e435)), 
            /* e1, e2, e3, e4 */
            (float4(other.e12345) * this.group3) + (float4(this.e12345) * other.group3) + float4(float3(0.0), -(other.e431 * this.e425) - (other.e412 * this.e435) - (other.e425 * this.e431) - (other.e435 * this.e412)) + float4((float3(other.e321) * this.group1.xyz), 0.0) + float4((float3(this.e321) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group2.yzx), 0.0) + float4((other.group2.yzx * this.group0.zxy), 0.0) - (other.group0.yzxx * float4(this.group2.zxy, this.e415)) - (this.group0.yzxx * float4(other.group2.zxy, other.e415))
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorEven {
    public VersorOdd anti_wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(this.e12345) * other.group0) + (float4(other.e1234) * float4(this.group1.xyz, this.e5)) + (other.group3.yzxx * float4(this.group0.zxy, this.e1)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (this.group0.yzxx * float4(other.group3.zxy, other.e15)), 
            /* e23, e31, e12, e45 */
            (this.group0 * float4(float3(other.e3215), other.e45)) + float4(float3(0.0), -(this.e425 * other.e4315) - (this.e435 * other.e4125)) + float4((float3(this.e12345) * other.group1.xyz), 0.0) + float4((float3(other.e1234) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(this.e12345) * other.group2.xyz) + (float3(other.e3215) * this.group1.xyz) + float3(float2(0.0), (this.e235 * other.e4315) - (this.e315 * other.e4235)) + float3((this.group2.yz * other.group3.zx), 0.0) - float3((this.group2.zx * other.group3.yz), 0.0)), this.e12345 * other.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e12345) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public AntiWedgeInfixPartial<VersorOdd> operator /(VersorOdd lhs, AntiWedgeInfix rhs) {
    return AntiWedgeInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public RoundPoint anti_wedge(AntiCircleRotor other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(AntiDipoleInversion other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            (float4(this.e3215) * float4(other.group1.xyz, other.e4)) + (this.group3.zxyx * float4(other.group2.yzx, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (other.group2.zxyx * float4(this.group3.yzx, this.e41))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiFlector anti_wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            float4(other.e3215) * float4(this.group0.xyz, this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_wedge(AntiFlatPoint other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e1234) * other.group0.xyz), 0.0) - (float4(other.e321) * float4(this.group3.xyz, this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiMotor anti_wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            (float4(this.e1234) * float4(other.group0.xyz, other.e5)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (float4(other.e321) * float4(this.group3.xyz, this.e45)), 
            /* e15, e25, e35, e3215 */
            float4(((other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiPlane anti_wedge(AntiLine other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (float4(other.e31, other.e12, other.e23, other.e15) * this.group3.zxyx)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiFlector anti_wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.e3215) * float4(this.group3.xyz, this.e1234), 
            /* e1, e2, e3, e5 */
            (other.group1 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(other.e25 * this.e4315) - (other.e35 * this.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - (this.group3.zxyx * float4(other.group0.yzx, other.e15))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public Scalar anti_wedge(AntiPlane other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_wedge(AntiScalar other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.e12345) * this.group0, 
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
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(Circle other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (float3(this.e1234) * other.group1.xyz) + (other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy), 
            /* e23, e31, e12, e45 */
            float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e1234) * other.group2.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e3215) * other.group1.xyz), 0.0) + float4((other.group2.yzx * this.group3.zxy), 0.0) - float4((other.group2.zxy * this.group3.yzx), other.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_wedge(CircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e12345) * this.group0) + float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e1234) * other.group1.xyz), 0.0) + float4((other.group0.zxy * this.group3.yzx), 0.0) - float4((other.group0.yzx * this.group3.zxy), other.e423 * this.e15), 
            /* e23, e31, e12, e45 */
            (other.group2 * float4(float3(this.e1234), this.e45)) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((float3(this.e3215) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group2.yz * this.group3.zx), 0.0) - float3((other.group2.zx * this.group3.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public RoundPoint anti_wedge(Dipole other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * float4(other.e15, other.e25, other.e35, other.e45)) + (this.group3.yzxx * float4(other.group1.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) - float4((float3(this.e3215) * other.group0.xyz), 0.0) - float4((other.group1.yzx * this.group3.zxy), 0.0), 
            /* e5 */
            -(other.e45 * this.e3215) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(DipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group3.yzxw) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group3.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyy * float4(this.group1.yzx, this.e25)) + (this.group2.wwwx * float4(other.group2.xyz, other.e4235)) + float4(float3(0.0), (other.e4125 * this.e35) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group3.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_wedge(DualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4((float3(other.e12345) * this.group0.xyz), (other.e5 * this.e1234) + (other.e12345 * this.scalar)), 
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
public RoundPoint operator /(AntiWedgeInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public RoundPoint anti_wedge(FlatPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * other.group0, 
            /* e5 */
            -(other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Flector other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group1.yzx * this.group3.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group1.zx * this.group3.yz), 0.0)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((float3(other.e3215) * this.group3.xyz), other.e45 * this.e1234) - (other.group1.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group1.zxyy * float4(this.group1.yzx, this.e25)) + (this.group2.wwwx * float4(other.group0.xyz, other.e4235)) + float4(float3(0.0), (other.e4125 * this.e35) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215)) - float4((other.group1.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiCircleRotor anti_wedge(Line other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4((float3(this.e1234) * other.group1.xyz), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), -(other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.e3215) * other.group0.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - float4((other.group1.zxy * this.group3.yzx), other.e415 * this.e23)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_wedge(Motor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (other.group0 * float4(float3(this.e1234), this.scalar)) + float4(float3(0.0), (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(other.e12345) * this.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (float4(other.e12345) * this.group1) + float4(float3(0.0), -(other.e415 * this.e4235) - (other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e1234) * other.group1.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group0.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group1.yz * this.group3.zx), 0.0) - float3((other.group1.zx * this.group3.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(AntiWedgeInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public MultiVector anti_wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2((other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(this.e1234) * other.group3) + (this.group3.yzxx * float4(other.group5.zxy, other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125)) + float4((float3(other.e3215) * this.group0.xyz), 0.0) + float4((other.group9.zxy * this.group1.yzx), 0.0) - (float4(other.e1234) * float4(this.e15, this.e25, this.e35, this.e45)) - (other.group9.yzxx * float4(this.group1.zxy, this.e41)) - float4((float3(this.e3215) * other.group4.xyz), other.e4315 * this.e42) - float4((other.group5.yzx * this.group3.zxy), other.e4125 * this.e43), 
            /* e5 */
            (other.e4235 * this.e15) + (other.e4315 * this.e25) + (other.e4125 * this.e35) + (other.e3215 * this.e45) - (other.e15 * this.e4235) - (other.e25 * this.e4315) - (other.e35 * this.e4125) - (other.e45 * this.e3215), 
            /* e15, e25, e35, e45 */
            (float4(other.e12345) * float4(this.e15, this.e25, this.e35, this.e45)) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(this.e3215) * other.group6.xyz), 0.0) + float4((other.group8.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(other.group8.zxy, other.e415)), 
            /* e41, e42, e43 */
            (float3(other.e12345) * this.group0.xyz) + (float3(this.e1234) * other.group6.xyz) + (other.group7.zxy * this.group3.yzx) - (other.group7.yzx * this.group3.zxy), 
            /* e23, e31, e12 */
            (float3(other.e12345) * this.group1.xyz) + (float3(this.e1234) * other.group8.xyz) + (float3(this.e3215) * other.group7.xyz) - (float3(other.e321) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group9.yzxw) - (this.group3.yzxw * float4(other.group9.zxy, other.e1234)), 
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group9.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e235, e315, e125 */
            (float3(other.e3215) * this.group3.xyz) - (float3(this.e3215) * other.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3, 
            /* e1234 */
            other.e12345 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Plane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.e1234) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(((other.group0.yzx * this.group3.zxy) + float3(float2(0.0), other.e4315 * this.e4235 * -1.0) - float3((other.group0.zx * this.group3.yz), 0.0)), other.e3215 * this.e1234), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35)) - float4((other.group0.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(AntiWedgeInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public Scalar anti_wedge(RoundPoint other) {
        return Scalar(
            /* scalar */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(Sphere other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group0.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group0.yzxw) - (this.group3.yzxw * float4(other.group0.zxy, other.e1234)), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), -(other.e4315 * this.e42) - (other.e4125 * this.e43) - (other.e1234 * this.e45)) + float4((float3(other.e3215) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group0.zxyx * float4(this.group1.yzx, this.e15)) + float4(float3(0.0), (other.e4315 * this.e25) + (other.e4125 * this.e35)) - float4((float3(other.e1234) * this.group2.xyz), 0.0) - float4((other.group0.yzx * this.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(AntiWedgeInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd anti_wedge(VersorEven other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            (float4(other.e12345) * this.group0) + (float4(this.e1234) * float4(other.group1.xyz, other.e5)) + (this.group3.yzxx * float4(other.group0.zxy, other.e1)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (other.group0.yzxx * float4(this.group3.zxy, this.e15)), 
            /* e23, e31, e12, e45 */
            (other.group0 * float4(float3(this.e3215), this.e45)) + float4(float3(0.0), -(other.e425 * this.e4315) - (other.e435 * this.e4125)) + float4((float3(other.e12345) * this.group1.xyz), 0.0) + float4((float3(this.e1234) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e12345) * this.group2.xyz) + (float3(this.e3215) * other.group1.xyz) + float3(float2(0.0), (other.e235 * this.e4315) - (other.e315 * this.e4235)) + float3((other.group2.yz * this.group3.zx), 0.0) - float3((other.group2.zx * this.group3.yz), 0.0)), other.e12345 * this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e12345) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(AntiWedgeInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.anti_wedge(rhs);
}
public extension VersorOdd {
    public AntiDipoleInversion anti_wedge(VersorOdd other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e1234) * other.group3.xyz) - (float3(other.e1234) * this.group3.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e4125, this.e4235, this.e4315, this.e1234) * other.group3.yzxw) - (float4(other.e4125, other.e4235, other.e4315, other.e1234) * this.group3.yzxw), 
            /* e235, e315, e125, e4 */
            (this.group3.xyzx * float4(float3(other.e3215), other.e41)) + float4(float3(0.0), (other.e42 * this.e4315) + (other.e43 * this.e4125) + (other.e45 * this.e1234) - (other.e1234 * this.e45) - (other.e4315 * this.e42) - (other.e4125 * this.e43)) - (other.group3.xyzx * float4(float3(this.e3215), this.e41)), 
            /* e1, e2, e3, e5 */
            (float4(other.e3215) * float4(this.group0.xyz, this.e45)) + (other.group3.zxyy * float4(this.group1.yzx, this.e25)) + (this.group2.wwwx * float4(other.group2.xyz, other.e4235)) + float4(float3(0.0), (other.e4125 * this.e35) - (other.e35 * this.e4125)) + float4((other.group1.zxy * this.group3.yzx), 0.0) - (float4(this.e3215) * float4(other.group0.xyz, other.e45)) - (other.group2.wwwy * float4(this.group2.xyz, this.e4315)) - (this.group3.zxyx * float4(other.group1.yzx, other.e15)) - float4((other.group3.yzx * this.group1.zxy), 0.0)
        );
    }
}
