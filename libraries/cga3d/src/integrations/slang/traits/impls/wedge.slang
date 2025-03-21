implementing cga3d;
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiCircleRotor> operator /(AntiCircleRotor lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiCircleRotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            float4((other.e15 * this.scalar) + (other.scalar * this.e15), (other.e25 * this.scalar) + (other.scalar * this.e25), (other.e35 * this.scalar) + (other.scalar * this.e35), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(this.scalar) * other.group0.xyz), 0.0) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            (this.group2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group3.yz), 0.0) - float3((this.group2.yz * other.group3.zx), 0.0)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.scalar * other.e3215)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor wedge(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.scalar * other.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorEven wedge(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e45 * other.e321)) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), this.scalar * other.e321) - (this.group1.wwwx * other.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group1.yz), 0.0) - float3((this.group2.yz * other.group1.zx), 0.0)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - float4((this.group0.zxy * other.group1.yzx), this.e23 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd wedge(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * float4(this.group0.xyz, this.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e45 * other.scalar), 
            /* e15, e25, e35, e1234 */
            float4((this.e15 * other.scalar) + (this.scalar * other.e15), (this.e25 * other.scalar) + (this.scalar * other.e25), (this.e35 * other.scalar) + (this.scalar * other.e35), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.zxyw * float4(this.group0.yzx, this.scalar)) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion wedge(AntiPlane other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - float4((this.group2.yzx * other.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiCircleRotor> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - float4((this.group0.zxy * other.group2.yzx), this.e23 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiCircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group3) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiCircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.scalar) * other.group0) + float4(float3(0.0), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)) + float4((float3(other.e5) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group1.xyz)), this.scalar * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiCircleRotor> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.scalar * other.e12345) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (this.group2 * float4(float3(other.scalar), other.e45)) + float4((float3(this.scalar) * other.group3.xyz), this.e45 * other.scalar), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e415, e425, e435, e321 */
            (this.group2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(this.scalar) * other.group6.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group1.yz), 0.0) - float3((this.group2.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group9) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) + float4((other.group4.yzx * this.group2.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group3.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx), 
            /* e1234 */
            (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<AntiCircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiDipoleInversion wedge(RoundPoint other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group0.yz), 0.0) - float3((this.group2.yz * other.group0.zx), 0.0)), this.scalar * other.e4), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * float4(other.group0.xyz, other.e5)
        );
    }
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public AntiCircleRotor wedge(Scalar other) {
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
public Sphere operator /(WedgeInfixPartial<AntiCircleRotor> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiCircleRotor> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.scalar) * other.group0) + float4(float3(0.0), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(other.e4) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) - float4((this.group0.zxy * other.group3.yzx), this.e41 * other.e235), 
            /* e415, e425, e435, e321 */
            (this.group2 * float4(float3(other.e4), other.e321)) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group3.yz), 0.0) - float3((this.group2.yz * other.group3.zx), 0.0)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiCircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiCircleRotor {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group0.xyz)), this.scalar * other.scalar), 
            /* e23, e31, e12, e45 */
            (float4(this.scalar) * other.group1) + (float4(other.scalar) * this.group1), 
            /* e15, e25, e35, e1234 */
            (this.group2 * float4(float3(other.scalar), other.e1234)) + float4(float3(0.0), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + float4((float3(this.scalar) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group3) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - (this.group2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiDipoleInversion> operator /(AntiDipoleInversion lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiDipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(other.scalar) * this.group0.xyz), 0.0) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.scalar) * this.group1.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group3.yz), 0.0) - float3((other.group2.yz * this.group3.zx), 0.0)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(this.e2, this.e3, this.e1, this.e4) * other.group3.zxyw) - (float4(other.e2, other.e3, other.e1, other.e4) * this.group3.zxyw), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group3.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxy * float4(this.group1.zxy, this.e315)) + (this.group2.wwwx * float4(other.group2.xyz, other.e1)) + float4(float3(0.0), (other.e3 * this.e125) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group3.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group2.xyz, this.e2)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), this.e4 * other.e3215), 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * float4(this.group2.xyz, this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(this.e1 * other.e235) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)), 
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group1.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group3.zx * other.group1.yz), 0.0)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((float3(other.e5) * this.group3.xyz), this.e4 * other.e321) - (other.group1.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group0.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e1 * other.e235) - (this.e2 * other.e315) - (this.e3 * other.e125) - (this.e5 * other.e321)) - float4((this.group1.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(AntiLine other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group3.yzx), 0.0) - float4((other.group1.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12)) + float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e1 * other.e23) - (this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group0.xyz) + (float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group3.yz * other.group1.zx), 0.0) - float3((this.group3.zx * other.group1.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(AntiPlane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group3.yzx * other.group0.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group3.zx * other.group0.yz), 0.0)), this.e4 * other.e5), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e431 * other.e2) - (this.e412 * other.e3)) + float4((float3(other.e5) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((this.group1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((this.group3.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((this.group3.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group3.yzx), 0.0) - float4((other.group2.yzx * this.group3.zxy), this.e423 * other.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (this.e4 * other.e3215) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (other.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - (float4(other.e45) * float4(this.group3.xyz, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35)) - (float4(other.e45) * float4(this.group3.xyz, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((this.group3.yzx * other.group0.zxy) - (this.group3.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group3.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group3.zxy * other.group0.yzx), 0.0) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * float4(this.e1, this.e2, this.e3, this.e4)) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (this.group3.yzx * other.group1.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group3.zx * other.group1.yz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((float3(this.e5) * other.group4.xyz), 0.0) - (this.group3.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group3.yz * other.group3.zx), 0.0) - float3((this.group3.zx * other.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (this.group2.wwwy * float4(other.group8.xyz, other.e2)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group3.zxy * other.group6.yzx), 0.0) - (float4(this.e5) * float4(other.e423, other.e431, other.e412, other.e321)) - (this.group3.yzxx * float4(other.group6.zxy, other.e235)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group1.zxy), 0.0), 
            /* e1234 */
            (this.e4 * other.e321) + (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group0.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)) + float4((float3(other.e5) * this.group3.xyz), 0.0) - (other.group0 * float4(float3(this.e5), this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion wedge(Scalar other) {
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
public AntiScalar operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e4 * other.e3215) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            float4((this.group3.yzx * other.group3.zxy), this.e4 * other.e5) - (this.group3.zxyw * other.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            (this.group3.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (this.e4 * other.e321) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)) - (other.group3 * float4(float3(this.e5), this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group2 * float4(float3(this.e4), this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(other.e5) * this.group0.xyz), this.e315 * other.e2) + float4((this.group3.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDipoleInversion {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.e4) * float4(other.group1.xyz, other.e3215)) + (this.group3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e315 * other.e42) - (this.e125 * other.e43)) + float4((float3(other.scalar) * this.group0.xyz), this.e2 * other.e4315) - (other.group0.zxyx * float4(this.group3.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group3.yz * other.group2.zx), 0.0) - float3((this.group3.zx * other.group2.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(this.group3.xyz, this.e4)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiDualNum> operator /(AntiDualNum lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiDualNum>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.scalar * this.e3215)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), other.e4 * this.e3215), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * float4(other.group2.xyz, other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiDualNum wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((other.e3215 * this.scalar) + (other.scalar * this.e3215), other.scalar * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiLine wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * other.group1.xyz), (this.e3215 * other.scalar) + (this.scalar * other.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiPlane wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiDualNum> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<AntiDualNum> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Circle wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiDualNum> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<AntiDualNum> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Dipole wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiDualNum> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<AntiDualNum> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<AntiDualNum> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public FlatPoint wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiDualNum> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<AntiDualNum> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Line wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiDualNum> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiDualNum> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.e3215 * other.e4) + (this.scalar * other.e12345)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.scalar) * other.group9.xyz), (this.e3215 * other.scalar) + (this.scalar * other.e3215)), 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiDualNum> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDualNum> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), this.e3215 * other.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiDualNum> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public AntiDualNum wedge(Scalar other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiDualNum> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiDualNum> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), (this.e3215 * other.e4) + (this.scalar * other.e12345)), 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiDualNum> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiDualNum {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.scalar) * other.group3.xyz), (this.e3215 * other.scalar) + (this.scalar * other.e3215))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiFlatPoint> operator /(AntiFlatPoint lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiFlatPoint>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.scalar * this.e321), 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321)), 
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(AntiDualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(AntiFlector other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(AntiMotor other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiFlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(AntiPlane other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2((this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlatPoint> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiFlatPoint> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(DualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e5, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlatPoint> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlatPoint> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            this.e321 * other.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiFlatPoint> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiDualNum wedge(Motor other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321 * other.e5, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiFlatPoint> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 
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
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiFlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiFlatPoint> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public AntiFlatPoint wedge(Scalar other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiFlatPoint> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5)), 
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiFlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlatPoint {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321 * other.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiFlector> operator /(AntiFlector lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiFlector>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiFlector> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e45 * this.e321)) + float4((other.group0.yzx * this.group1.zxy), 0.0) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), other.scalar * this.e321) - (other.group1.wwwx * this.group1.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group1.yz), 0.0) - float3((other.group2.yz * this.group1.zx), 0.0)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiFlector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group1.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group3.yz * this.group1.zx), 0.0)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e4 * this.e321)) - float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) + (other.e5 * this.e321) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group1.zxy), other.e1 * this.e235) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group0.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiFlector> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector wedge(AntiDualNum other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiFlector> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiFlector> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(other.e5) * float4(this.e1, this.e2, this.e3, this.e321)) + float4(float3(0.0), (other.e1 * this.e235) + (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3)) - (float4(this.e5) * float4(other.e1, other.e2, other.e3, other.e321))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiFlector> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group1.yzx), 0.0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiFlector> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((this.group1.yzx * other.group1.zxy), 0.0) - (this.group1.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiFlector> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiMotor wedge(AntiPlane other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0), 
            /* e15, e25, e35, e3215 */
            (float4(other.e5) * float4(this.e1, this.e2, this.e3, this.e321)) + float4(float3(0.0), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiFlector> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiFlector> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiFlector> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) + float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group1.yzx), 0.0) - float4((other.group2.yzx * this.group1.zxy), this.e235 * other.e41)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiFlector> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group1.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - float4((this.group1.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiFlector> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiFlector> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e45 * -1.0) * float4(this.group1.xyz, this.e321), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiFlector> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * this.group1.xyz), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e321 * other.e45)), 
            /* e235, e315, e125, e5 */
            float4(((this.group1.yzx * other.group0.zxy) - (this.group1.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiFlector> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiFlector> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group0.yzx), this.e321 * other.e5) - (this.group1.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiFlector> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group1.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group1.zx * other.group1.yz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group4.xyz), this.e321 * other.scalar) - (this.group1.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (other.group4.yzx * this.group1.zxy) - (other.group4.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group1.yz * other.group3.zx), 0.0) - float3((this.group1.zx * other.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group6.yzx), this.e235 * other.e1) - (float4(this.e5) * float4(other.e423, other.e431, other.e412, other.e321)) - (this.group1.yzxx * float4(other.group6.zxy, other.e235)) - float4((float3(other.e4) * this.group0.xyz), 0.0), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlector> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiFlector> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group0.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group1.zx * other.group0.yz), 0.0)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), this.e321 * other.e4 * -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e4 * -1.0) * this.group0.xyz), (this.e235 * other.e1) + (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiFlector> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiFlector wedge(Scalar other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(other.scalar) * this.group0, 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiFlector> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiFlector> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group1.yzx * other.group3.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group1.zx * other.group3.yz), 0.0)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group1.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - (other.group3 * float4(float3(this.e5), this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3) + (this.e321 * other.e5) - (this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group1.zxy * other.group1.yzx), this.e235 * other.e1) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(other.e4) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiFlector> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiFlector {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group1.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234) - (this.e315 * other.e42) - (this.e125 * other.e43) - (this.e321 * other.e45)) - (other.group0.zxyx * float4(this.group1.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (other.group0 * float4(float3(this.e5), this.e321)) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) - (this.group1.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group1.yz * other.group2.zx), 0.0) - float3((this.group1.zx * other.group2.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiLine> operator /(AntiLine lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiLine>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiLine> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - float4((other.group0.zxy * this.group1.yzx), other.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiLine> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group3.yzx), 0.0) - float4((this.group1.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WedgeInfixPartial<AntiLine> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiLine wedge(AntiDualNum other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiLine> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiFlatPoint wedge(AntiFlector other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group1.yzx), 0.0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiLine> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiDualNum wedge(AntiLine other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiLine> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiLine> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiFlatPoint wedge(AntiPlane other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiLine> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiLine> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiLine> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiLine> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiLine> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiFlatPoint wedge(DualNum other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(other.e5) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiLine> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiLine> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiLine> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiLine> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e5) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiLine> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group1.yzx) - (this.group1.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group4.yzx), 0.0) - float4((this.group1.yzx * other.group4.zxy), this.e23 * other.e15), 
            /* e1234 */
            -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<AntiLine> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public Circle wedge(RoundPoint other) {
        return Circle(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (this.group1.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WedgeInfixPartial<AntiLine> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public AntiLine wedge(Scalar other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(other.scalar) * this.group0.xyz, 
            /* e15, e25, e35 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiLine> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.e4) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group3.yzx), 0.0) - float4((this.group1.yzx * other.group3.zxy), this.e23 * other.e415)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiLine> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiLine {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group1.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((this.group1.yzx * other.group0.zxy), this.e23 * other.e15)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiMotor> operator /(AntiMotor lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiMotor>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiMotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * float4(other.group0.xyz, other.scalar), 
            /* e23, e31, e12, e45 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e45 * this.scalar), 
            /* e15, e25, e35, e1234 */
            float4((other.e15 * this.scalar) + (other.scalar * this.e15), (other.e25 * this.scalar) + (other.scalar * this.e25), (other.e35 * this.scalar) + (other.scalar * this.e35), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)), 
            /* e4235, e4315, e4125, e3215 */
            (this.group1.zxyw * float4(other.group0.yzx, other.scalar)) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiMotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12)) + float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e1 * this.e23) - (other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group0.xyz) + (float3(this.scalar) * other.group2.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group3.yz * this.group1.zx), 0.0) - float3((other.group3.zx * this.group1.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiMotor> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor wedge(AntiDualNum other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4((float3(other.scalar) * this.group1.xyz), (other.e3215 * this.scalar) + (other.scalar * this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiMotor> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiMotor> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((other.group1.yzx * this.group1.zxy), 0.0) - (other.group1.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiMotor> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor wedge(AntiLine other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e3215 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiMotor> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e15, e25, e35, e3215 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiMotor> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiFlector wedge(AntiPlane other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((this.group1.zxy * other.group0.yzx), 0.0) - (other.group0.zxyx * float4(this.group1.yzx, this.e23)), 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiMotor> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            this.scalar * other.e12345
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiMotor> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiMotor> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiMotor> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(other.e45) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiMotor> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (this.group0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.scalar) * other.group3.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiMotor> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), this.scalar * other.e12345), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiMotor> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.e45) * this.group0.xyz), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiMotor> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (this.group0 * float4(float3(other.e45), other.e3215)) + float4(float3(0.0), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiMotor> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiMotor> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), (this.scalar * other.e12345) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.e5) * this.group0.xyz)), this.scalar * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiMotor> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.scalar * other.e12345) + (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(this.scalar) * other.group3.xyz) + (float3(other.scalar) * this.group1.xyz)), this.scalar * other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group5.xyz) + (float3(other.scalar) * this.group0.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group6) + float4(float3(0.0), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group7.xyz) + (float3(other.e4) * this.group0.xyz), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group8.xyz) + (float3(other.e5) * this.group0.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group1.zx * other.group1.yz), 0.0) - float3((this.group1.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group0 * float4(float3(other.e45), other.e3215)) + (this.group1.zxyw * float4(other.group4.yzx, other.scalar)) + float4(float3(0.0), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.scalar) * other.group9.xyz), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group1.yzxx), 
            /* e1234 */
            (this.scalar * other.e1234) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiMotor> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiMotor> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(other.e4) * float4(this.group0.xyz, this.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e4) * this.group1.xyz), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group0.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group1.zx * other.group0.yz), 0.0) - float3((this.group1.yz * other.group0.zx), 0.0)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiMotor> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public AntiMotor wedge(Scalar other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group0, 
            /* e15, e25, e35, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiMotor> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiMotor> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0 * float4(float3(other.e4), other.e12345)) + float4(float3(0.0), (this.e3215 * other.e4) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(this.e23 * other.e1) - (this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group0.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group1.zx * other.group3.yz), 0.0) - float3((this.group1.yz * other.group3.zx), 0.0)), this.scalar * other.e5), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<AntiMotor> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiMotor {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group0.xyz)), this.scalar * other.e45), 
            /* e15, e25, e35, e1234 */
            (float4(this.scalar) * other.group2) + float4(float3(0.0), -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) + float4((float3(other.scalar) * this.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (this.group0 * float4(float3(other.e45), other.e3215)) + (this.group1.zxyw * other.group0.yzxw) + float4(float3(0.0), -(this.e23 * other.e15) - (this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.scalar) * other.group3.xyz), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiPlane> operator /(AntiPlane lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiPlane>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<AntiPlane> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiDipoleInversion wedge(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group2.yzx * this.group0.zxy), 0.0), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiPlane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group3.zxy * this.group0.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group3.yz * this.group0.zx), 0.0)), other.e4 * this.e5 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group0.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WedgeInfixPartial<AntiPlane> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane wedge(AntiDualNum other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<AntiPlane> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(-(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<AntiPlane> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiMotor wedge(AntiFlector other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(((other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)), 0.0), 
            /* e15, e25, e35, e3215 */
            float4(float3(0.0), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (float4(this.e5) * float4(other.e1, other.e2, other.e3, other.e321))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<AntiPlane> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, other.e23))
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<AntiPlane> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiFlector wedge(AntiMotor other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - (this.group0.zxyx * float4(other.group1.yzx, other.e23)), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WedgeInfixPartial<AntiPlane> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiLine wedge(AntiPlane other) {
        return AntiLine(
            /* e23, e31, e12 */
            (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiPlane> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<AntiPlane> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<AntiPlane> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Circle wedge(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<AntiPlane> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125)) - float4((this.group0.zxy * other.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<AntiPlane> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public FlatPoint wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<AntiPlane> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Line wedge(FlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e45 * -1.0) * this.group0.xyz, 
            /* e235, e315, e125 */
            (this.group0.yzx * other.group0.zxy) + float3(float2(0.0), (this.e2 * other.e15) * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<AntiPlane> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e45 * -1.0) * this.group0.xyz), (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<AntiPlane> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<AntiPlane> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(other.e5) * this.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<AntiPlane> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), this.e5 * other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group0.zx * other.group1.yz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group4.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (this.group0.yzx * other.group3.zxy) + float3(float2(0.0), (this.e2 * other.e15) * -1.0) - float3((this.group0.zx * other.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group0.zxy * other.group6.yzx), 0.0) - (float4(this.e5) * float4(other.e423, other.e431, other.e412, other.e321)) - (this.group0.yzxx * float4(other.group6.zxy, other.e235)), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiPlane> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<AntiPlane> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public Dipole wedge(RoundPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group0.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group0.zx * other.group0.yz), 0.0)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WedgeInfixPartial<AntiPlane> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiPlane wedge(Scalar other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiPlane> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<AntiPlane> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.e4 * -1.0) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((this.group0.yzx * other.group3.zxy) + float3(float2(0.0), this.e2 * other.e1 * -1.0) - float3((this.group0.zx * other.group3.yz), 0.0)), this.e5 * other.e4 * -1.0), 
            /* e15, e25, e35, e1234 */
            (this.group0.xyzx * float4(float3(other.e5), other.e423)) + float4(float3(0.0), (this.e2 * other.e431) + (this.e3 * other.e412)) - float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<AntiPlane> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiPlane {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (this.group0.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) - float4((this.group0.yzx * other.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) + float3(float2(0.0), this.e2 * other.e15 * -1.0) - float3((this.group0.zx * other.group2.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(other.scalar) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<AntiScalar> operator /(AntiScalar lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<AntiScalar>(lhs);
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(AntiCircleRotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(AntiDualNum other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(AntiMotor other) {
        return AntiScalar(
            /* e12345 */
            other.scalar * this.e12345
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(MultiVector other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(Scalar other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<AntiScalar> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension AntiScalar {
    public AntiScalar wedge(VersorOdd other) {
        return AntiScalar(
            /* e12345 */
            this.e12345 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Circle> operator /(Circle lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Circle>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Circle> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Circle> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((other.group3.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<Circle> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Circle wedge(AntiDualNum other) {
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
public Sphere operator /(WedgeInfixPartial<Circle> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Sphere wedge(AntiFlector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Circle> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Circle> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public CircleRotor wedge(AntiMotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Circle> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Sphere wedge(AntiPlane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group0.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Circle> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Circle> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Circle> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Circle> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Circle> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Circle> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Plane wedge(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Circle> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group1.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Circle> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<Circle> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Circle wedge(Scalar other) {
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
public Sphere operator /(WedgeInfixPartial<Circle> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group3.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Circle> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Circle {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<CircleRotor> operator /(CircleRotor lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<CircleRotor>(lhs);
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<CircleRotor> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor wedge(AntiCircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<CircleRotor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Sphere wedge(AntiDipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((other.group3.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<CircleRotor> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor wedge(AntiDualNum other) {
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
public Sphere operator /(WedgeInfixPartial<CircleRotor> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Sphere wedge(AntiFlector other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group1.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<CircleRotor> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<CircleRotor> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor wedge(AntiMotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412))
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<CircleRotor> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Sphere wedge(AntiPlane other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group0.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<CircleRotor> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<CircleRotor> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<CircleRotor> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Plane wedge(DualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<CircleRotor> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public AntiScalar wedge(FlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<CircleRotor> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public AntiScalar wedge(Flector other) {
        return AntiScalar(
            /* e12345 */
            -(this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e321 * other.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<CircleRotor> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Plane wedge(Motor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<CircleRotor> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4(other.scalar) * this.group1, 
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group1.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<CircleRotor> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Sphere wedge(RoundPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<CircleRotor> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor wedge(Scalar other) {
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
public Sphere operator /(WedgeInfixPartial<CircleRotor> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public Sphere wedge(VersorEven other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((this.group1.yzx * other.group3.zxy), 0.0), 
            /* e1234 */
            -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<CircleRotor> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension CircleRotor {
    public CircleRotor wedge(VersorOdd other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(other.scalar) * this.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(other.scalar) * this.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e12345 * other.scalar) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Dipole> operator /(Dipole lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Dipole>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<Dipole> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - float4((other.group0.zxy * this.group2.yzx), other.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Dipole> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group3.yzx), 0.0) - float4((this.group2.yzx * other.group3.zxy), other.e423 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<Dipole> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Dipole wedge(AntiDualNum other) {
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
public AntiScalar operator /(WedgeInfixPartial<Dipole> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Dipole> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public CircleRotor wedge(AntiFlector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) + float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group1.yzx), 0.0) - float4((this.group2.yzx * other.group1.zxy), other.e235 * this.e41)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Dipole> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Sphere wedge(AntiLine other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<Dipole> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public DipoleInversion wedge(AntiMotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<Dipole> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Circle wedge(AntiPlane other) {
        return Circle(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Dipole> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Dipole> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Dipole> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - float4((other.group0.zxy * this.group2.yzx), other.e23 * this.e15) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Dipole> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25), 
            /* e1234 */
            -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Dipole> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Line wedge(DualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.e5) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.e5) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Dipole> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Dipole> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Dipole> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Dipole> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e5) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Dipole> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group1.yzx) - (this.group2.yzx * other.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) + float4((this.group2.zxy * other.group4.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group3.yzxx) - float4((this.group2.yzx * other.group4.zxy), this.e31 * other.e25), 
            /* e1234 */
            -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<Dipole> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Circle wedge(RoundPoint other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (this.group2.zxy * other.group0.yzx) - (this.group2.yzx * other.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<Dipole> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public Dipole wedge(Scalar other) {
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
public CircleRotor operator /(WedgeInfixPartial<Dipole> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((float3(other.e4) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(other.e5) * this.group1.xyz), 0.0) + float4((this.group2.zxy * other.group3.yzx), 0.0) - float4((this.group2.yzx * other.group3.zxy), this.e41 * other.e235)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<Dipole> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Dipole {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), -(this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e12 * other.e35) - (this.e15 * other.e23) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - float4((this.group2.yzx * other.group0.zxy), this.e31 * other.e25)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<DipoleInversion> operator /(DipoleInversion lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<DipoleInversion>(lhs);
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiCircleRotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiDipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (other.e4 * this.e3215) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (this.group2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiDualNum other) {
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
public AntiScalar operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiFlector other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group1.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - float4((other.group1.zxy * this.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Sphere wedge(AntiLine other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion wedge(AntiMotor other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.scalar) * this.group3.xyz), 0.0) + float4((this.group0.yzx * other.group1.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<DipoleInversion> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public CircleRotor wedge(AntiPlane other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125)) - float4((other.group0.zxy * this.group2.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<DipoleInversion> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<DipoleInversion> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<DipoleInversion> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Sphere wedge(Dipole other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<DipoleInversion> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Sphere wedge(DipoleInversion other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DipoleInversion> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.e5) * float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e5) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<DipoleInversion> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Plane wedge(FlatPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<DipoleInversion> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Plane wedge(Flector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<DipoleInversion> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public AntiScalar wedge(Line other) {
        return AntiScalar(
            /* e12345 */
            -(this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DipoleInversion> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), (this.e1234 * other.e5) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.e5) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<DipoleInversion> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1234 * other.e5) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * float4(this.group2.xyz, this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group1.zxy) - (this.group0.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group2.zx * other.group1.yz), 0.0) - float3((this.group2.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group3.zxy), 0.0) + float4((other.group4.yzx * this.group2.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group3.yzxx) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx), 
            /* e1234 */
            (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<DipoleInversion> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public CircleRotor wedge(RoundPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group0.zxy) - (this.group0.zxy * other.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (this.group1.wwwx * other.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group0.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)) - float4((this.group2.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<DipoleInversion> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion wedge(Scalar other) {
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
public CircleRotor operator /(WedgeInfixPartial<DipoleInversion> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public CircleRotor wedge(VersorEven other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (this.group0.yzx * other.group3.zxy) - (this.group0.zxy * other.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((float3(other.e4) * this.group2.xyz), 0.0) - (this.group1.wwwx * other.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(other.e5) * float4(this.group1.xyz, this.e1234)) + (other.group3.yzxx * float4(this.group2.zxy, this.e4235)) + float4(float3(0.0), (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435) - (this.e45 * other.e321) - (this.e25 * other.e431) - (this.e35 * other.e412)) - (this.group2.yzxx * float4(other.group3.zxy, other.e423))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<DipoleInversion> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension DipoleInversion {
    public DipoleInversion wedge(VersorOdd other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(other.scalar) * this.group2.xyz), (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(this.e31 * other.e25) - (this.e12 * other.e35) - (this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((this.group0.yzx * other.group2.zxy), 0.0) + float4((this.group2.zxy * other.group0.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - (this.group2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<DualNum> operator /(DualNum lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DualNum> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<DualNum> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Flector wedge(AntiDipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * float4(other.group3.xyz, other.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<DualNum> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public DualNum wedge(AntiDualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<DualNum> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * this.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<DualNum> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Flector wedge(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), other.e321 * this.e5 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<DualNum> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public AntiFlatPoint wedge(AntiLine other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DualNum> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), other.scalar * this.e12345), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<DualNum> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public FlatPoint wedge(AntiPlane other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<DualNum> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Plane wedge(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<DualNum> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Plane wedge(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<DualNum> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Line wedge(Dipole other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e5) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.e5) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DualNum> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Motor wedge(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e5) * float4(other.group0.xyz, other.e1234), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e5 * other.e1234) + (this.e12345 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e5) * other.group4.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.e5) * other.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group7.xyz, other.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<DualNum> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public FlatPoint wedge(RoundPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public DualNum wedge(Scalar other) {
        return DualNum(
            /* e5, e12345 */
            float2(other.scalar) * this.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<DualNum> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<DualNum> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Flector wedge(VersorEven other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<DualNum> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension DualNum {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), (this.e5 * other.e1234) + (this.e12345 * other.scalar)), 
            /* e235, e315, e125, e5 */
            float4(this.e5) * float4(other.group1.xyz, other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<FlatPoint> operator /(FlatPoint lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<FlatPoint>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<FlatPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Flector wedge(AntiCircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<FlatPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Motor wedge(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35)) + float4((float3(other.e4) * this.group0.xyz), 0.0) - (float4(this.e45) * float4(other.group3.xyz, other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<FlatPoint> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public FlatPoint wedge(AntiDualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<FlatPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<FlatPoint> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Motor wedge(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.e45 * -1.0) * float4(other.group1.xyz, other.e321), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<FlatPoint> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Plane wedge(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<FlatPoint> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Flector wedge(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<FlatPoint> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Line wedge(AntiPlane other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.e45 * -1.0) * other.group0.xyz, 
            /* e235, e315, e125 */
            (other.group0.yzx * this.group0.zxy) + float3(float2(0.0), (other.e2 * this.e15) * -1.0) - float3((other.group0.zx * this.group0.yz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<FlatPoint> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<FlatPoint> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<FlatPoint> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Plane wedge(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<FlatPoint> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Plane wedge(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<FlatPoint> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group1.yzx) + float3(float2(0.0), (this.e15 * other.e2) * -1.0) - float3((this.group0.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((other.group4.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<FlatPoint> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Line wedge(RoundPoint other) {
        return Line(
            /* e415, e425, e435 */
            (float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group0.xyz), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group0.yzx) + float3(float2(0.0), (this.e15 * other.e2) * -1.0) - float3((this.group0.yz * other.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<FlatPoint> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public FlatPoint wedge(Scalar other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<FlatPoint> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Motor wedge(VersorEven other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) + float4((float3(other.e4) * this.group0.xyz), 0.0) - (float4(this.e45) * float4(other.group3.xyz, other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<FlatPoint> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension FlatPoint {
    public Flector wedge(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Flector> operator /(Flector lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(AntiCircleRotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(AntiDipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35)) - (float4(this.e45) * float4(other.group3.xyz, other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(AntiDualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Flector> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public AntiScalar wedge(AntiFlatPoint other) {
        return AntiScalar(
            /* e12345 */
            other.e321 * this.e45 * -1.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(AntiFlector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45 * -1.0) * other.group1.xyz), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e321 * this.e45)), 
            /* e235, e315, e125, e5 */
            float4(((other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Flector> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Plane wedge(AntiLine other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e45) * other.group0.xyz), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(AntiMotor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(this.e45), this.e3215)) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(AntiPlane other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e45 * -1.0) * other.group0.xyz), (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Flector> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public AntiScalar wedge(Circle other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Flector> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public AntiScalar wedge(CircleRotor other) {
        return AntiScalar(
            /* e12345 */
            -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e321 * this.e45)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Flector> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Plane wedge(Dipole other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Flector> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Plane wedge(DipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e4) * this.group0.xyz) - (float3(this.e45) * other.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (this.group0.zxy * other.group1.yzx) + float3(float2(0.0), (this.e15 * other.e2) * -1.0) - float3((this.group0.yz * other.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((other.group4.yzx * this.group0.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group0.yzxx), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(RoundPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3)) - float4((float3(this.e45) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group0.yzx) - (this.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(Scalar other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Flector> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Motor wedge(VersorEven other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e4) * float4(this.group0.xyz, this.e3215)) + float4(float3(0.0), (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412)) - (float4(this.e45) * float4(other.group3.xyz, other.e321)), 
            /* e235, e315, e125, e5 */
            float4(((this.group0.zxy * other.group3.yzx) - (this.group0.yzx * other.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Flector> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Flector {
    public Flector wedge(VersorOdd other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.scalar) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e25 * other.e31) - (this.e35 * other.e12)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((this.group0.zxy * other.group0.yzx), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Line> operator /(Line lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Line> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(AntiDipoleInversion other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group3.yzxx) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((float3(other.e4) * this.group1.xyz), 0.0) - float4((this.group0.yzx * other.group3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Line> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Line wedge(AntiDualNum other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(AntiFlector other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group1.yzxx) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((this.group0.yzx * other.group1.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Line> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public AntiScalar wedge(AntiLine other) {
        return AntiScalar(
            /* e12345 */
            -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Line> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(AntiPlane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group0.yzxx) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Line> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public AntiScalar wedge(Dipole other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Line> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public AntiScalar wedge(DipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
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
            float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group1.yzxx) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group1.xyz), 0.0) - float4((this.group0.yzx * other.group1.zxy), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(RoundPoint other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group0.yzxx) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Line> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Line wedge(Scalar other) {
        return Line(
            /* e415, e425, e435 */
            float3(other.scalar) * this.group0.xyz, 
            /* e235, e315, e125 */
            float3(other.scalar) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Line> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Plane wedge(VersorEven other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            (float4(this.e435, this.e415, this.e425, this.e235) * other.group3.yzxx) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group1.xyz), 0.0) - float4((this.group0.yzx * other.group3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Line> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Line {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4((float3(other.scalar) * this.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Motor> operator /(Motor lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(AntiCircleRotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.scalar) * this.group0) + float4(float3(0.0), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)) + float4((float3(this.e5) * other.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group1.xyz)), other.scalar * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(AntiDipoleInversion other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * float4(other.group3.xyz, other.e4), 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - (this.group1 * float4(float3(other.e4), other.e321)) - float4((float3(this.e5) * other.group0.xyz), 0.0) - float4((other.group3.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(AntiDualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<Motor> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public AntiDualNum wedge(AntiFlatPoint other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(other.e321 * this.e5 * -1.0, 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(AntiFlector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e321 * this.e5)) - float4((other.group1.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(AntiLine other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(AntiMotor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), (other.scalar * this.e12345) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.e5) * other.group0.xyz)), other.scalar * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(AntiPlane other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4((float3(this.e5 * -1.0) * other.group0.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group0.zxy * this.group0.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Motor> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Plane wedge(Circle other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Motor> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Plane wedge(CircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.e5 * -1.0) * float4(other.group0.xyz, other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(Dipole other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(DipoleInversion other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.e5) * other.group0.xyz), (other.e1234 * this.e5) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.e5) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e12345 * other.scalar) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e5) * other.group4.xyz) + (float3(other.scalar) * this.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (float3(other.scalar) * this.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - (this.group1 * float4(float3(other.e4), other.e321)) - float4((float3(this.e5) * other.group7.xyz), 0.0) - float4((this.group0.yzx * other.group1.zxy), 0.0), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(RoundPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (other.group0.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group1.xyz), 0.0) - float4((this.group0.yzx * other.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(Scalar other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(other.scalar) * this.group0, 
            /* e235, e315, e125, e5 */
            float4(other.scalar) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Motor> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            this.e5 * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Motor> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Flector wedge(VersorEven other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.e5 * -1.0) * other.group3, 
            /* e4235, e4315, e4125, e3215 */
            (other.group3.yzxx * float4(this.group0.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - (this.group1 * float4(float3(other.e4), other.e321)) - float4((float3(this.e5) * other.group0.xyz), 0.0) - float4((this.group0.yzx * other.group3.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Motor> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Motor {
    public Motor wedge(VersorOdd other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e5) * float4(other.group0.xyz, other.e1234)) + (float4(other.scalar) * this.group0) + float4(float3(0.0), -(this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group1.xyz)), this.e5 * other.scalar)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<MultiVector> operator /(MultiVector lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiCircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (other.group2 * float4(float3(this.scalar), this.e45)) + float4((float3(other.scalar) * this.group3.xyz), other.e45 * this.scalar), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.scalar) * this.group6.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group1.yz), 0.0) - float3((other.group2.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group9) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) + float4((this.group4.yzx * other.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group3.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx), 
            /* e1234 */
            (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiDipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * float4(other.e1, other.e2, other.e3, other.e4)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (other.group3.zxy * this.group1.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group3.yz * this.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((float3(other.e5) * this.group4.xyz), 0.0) - (other.group3.xyzx * float4(float3(this.e45), this.e23)), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group2.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group3.yz * this.group3.zx), 0.0) - float3((other.group3.zx * this.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.e423, this.e431, this.e412, this.e321)) + (other.group3.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (other.group2.wwwy * float4(this.group8.xyz, this.e2)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - float4((other.group3.zxy * this.group6.yzx), 0.0), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4) - (other.e4 * this.e321) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiDualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.e3215 * this.e4) + (other.scalar * this.e12345)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
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
            float4((float3(other.scalar) * this.group9.xyz), (other.e3215 * this.scalar) + (other.scalar * this.e3215)), 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiFlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
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
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group0.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiFlector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group1.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group1.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group1.yz * this.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group4.xyz), other.e321 * this.scalar) - (other.group1.xyzx * float4(float3(this.e45), this.e23)), 
            /* e423, e431, e412 */
            (this.group4.yzx * other.group1.zxy) - (this.group4.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group1.yz * this.group3.zx), 0.0) - float3((other.group1.zx * this.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.e423, this.e431, this.e412, this.e321)) + (other.group1.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - float4((other.group1.zxy * this.group6.yzx), 0.0), 
            /* e1234 */
            (other.e321 * this.e4) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiLine other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
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
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group1.yzx) - (other.group1.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group4.yzx), 0.0) - float4((other.group1.yzx * this.group4.zxy), other.e23 * this.e15), 
            /* e1234 */
            -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiMotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.scalar) * this.group3.xyz) + (float3(this.scalar) * other.group1.xyz)), other.scalar * this.e45), 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group4.xyz, 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group0.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group6) + float4(float3(0.0), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(this.e4) * other.group0.xyz), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(this.e5) * other.group0.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group1.zx * this.group1.yz), 0.0) - float3((other.group1.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(this.e45), this.e3215)) + (other.group1.zxyw * float4(this.group4.yzx, this.scalar)) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.scalar) * this.group9.xyz), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group1.yzxx), 
            /* e1234 */
            (other.scalar * this.e1234) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(AntiPlane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(((float3(other.e5) * this.group1.xyz) - (float3(this.e5) * other.group0.xyz)), other.e5 * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group0.yz * this.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group4.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e45), this.e23)), 
            /* e423, e431, e412 */
            (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (other.group0.yzx * this.group3.zxy) + float3(float2(0.0), (other.e2 * this.e15) * -1.0) - float3((other.group0.zx * this.group3.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.e423, this.e431, this.e412, this.e321)) + (other.group0.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group0.zxy * this.group6.yzx), 0.0), 
            /* e1234 */
            -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<MultiVector> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Circle other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(CircleRotor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4(this.scalar) * other.group1, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Dipole other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group1.yzx) - (other.group2.yzx * this.group1.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) + float4((other.group2.zxy * this.group4.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group3.yzxx) - float4((other.group2.yzx * this.group4.zxy), other.e31 * this.e25), 
            /* e1234 */
            -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(DipoleInversion other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1234 * this.e5) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * float4(other.group2.xyz, other.e45), 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1.xyz, 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group1.xyzx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group1.zxy) - (other.group0.zxy * this.group1.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group1.yz), 0.0) - float3((other.group2.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group3) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) + float4((this.group4.yzx * other.group2.zxy), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group3.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx), 
            /* e1234 */
            (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(DualNum other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e5 * this.e1234) + (other.e12345 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(other.e5) * this.group4.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(other.e5) * this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(FlatPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group1.yzx) + float3(float2(0.0), (other.e15 * this.e2) * -1.0) - float3((other.group0.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((this.group4.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Flector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group1.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group1.yzx) + float3(float2(0.0), (other.e15 * this.e2) * -1.0) - float3((other.group0.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((this.group4.yzx * other.group0.zxy), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group0.yzxx), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Line other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
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
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group1.yzxx), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Motor other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e12345 * this.scalar) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(((float3(other.e5) * this.group4.xyz) + (float3(this.scalar) * other.group0.xyz)), 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group1.xyz), 
            /* e4235, e4315, e4125, e3215 */
            (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(other.e5) * this.group7.xyz), 0.0) + float4((other.group0.yzx * this.group1.zxy), 0.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e235)), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.scalar * this.e12345) + (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e5 */
            (other.scalar * this.e5) + (other.e5 * this.scalar), 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * this.group3) + (float4(other.e5) * this.group1) + (float4(this.scalar) * other.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group4.xyz) + (float3(this.scalar) * other.group4.xyz) + (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group5.xyz) + (float3(this.scalar) * other.group5.xyz) + float3(float2(0.0), (other.e2 * this.e1) - (other.e1 * this.e2)) + float3((other.group1.zx * this.group1.yz), 0.0) - float3((other.group1.yz * this.group1.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group6) + (float4(this.scalar) * other.group6) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12) - (other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((float3(other.e5) * this.group4.xyz), 0.0) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((float3(this.e5) * other.group4.xyz), 0.0) - (other.group1.xyzx * float4(float3(this.e45), this.e23)) - (this.group1.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group7.xyz) + (float3(other.e4) * this.group5.xyz) + (float3(this.scalar) * other.group7.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group1.zxy) + (this.group4.yzx * other.group1.zxy) - (other.group4.zxy * this.group1.yzx) - (this.group4.zxy * other.group1.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group8.xyz) + (float3(other.e5) * this.group5.xyz) + (float3(this.scalar) * other.group8.xyz) + (float3(this.e5) * other.group5.xyz) + float3(float2(0.0), (other.e1 * this.e25) + (other.e25 * this.e1) - (other.e2 * this.e15) - (other.e15 * this.e2)) + float3((other.group1.yz * this.group3.zx), 0.0) + float3((other.group3.zx * this.group1.yz), 0.0) - float3((other.group1.zx * this.group3.yz), 0.0) - float3((other.group3.yz * this.group1.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group9) + (float4(other.e5) * float4(this.e423, this.e431, this.e412, this.e321)) + (float4(this.scalar) * other.group9) + (other.group1.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), -(other.e35 * this.e12) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(other.e45) * this.group5.xyz), other.e3 * this.e125) + float4((float3(this.e4) * other.group8.xyz), 0.0) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((other.group4.yzx * this.group3.zxy), 0.0) + float4((this.group4.yzx * other.group3.zxy), other.e2 * this.e315) + float4((other.group6.yzx * this.group1.zxy), 0.0) - (float4(this.e5) * float4(other.e423, other.e431, other.e412, other.e321)) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group3.yzxx) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group3.yzxx) - (this.group1.yzxx * float4(other.group6.zxy, other.e235)) - float4((float3(other.e4) * this.group8.xyz), other.e25 * this.e31) - float4((other.group1.zxy * this.group6.yzx), 0.0), 
            /* e1234 */
            (other.scalar * this.e1234) + (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e1234 * this.scalar) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Plane other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215)), 
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
            float4(this.scalar) * other.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(RoundPoint other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group0), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group1.zx * other.group0.yz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e4) * this.group3.xyz), 0.0) + float4((float3(other.e5) * this.group4.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e45), this.e23)), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group0.zxy) - (this.group4.zxy * other.group0.yzx), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group5.xyz) + (this.group3.zxy * other.group0.yzx) + float3(float2(0.0), (this.e15 * other.e2) * -1.0) - float3((this.group3.yz * other.group0.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.e423, this.e431, this.e412, this.e321)) + (other.group0.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (this.e315 * other.e2) + (this.e125 * other.e3)) - float4((float3(other.e4) * this.group8.xyz), 0.0) - float4((this.group6.yzx * other.group0.zxy), 0.0), 
            /* e1234 */
            -(this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Scalar other) {
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
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(Sphere other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)), 
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
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(VersorEven other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (this.scalar * other.e12345) + (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5) - (this.e15 * other.e423) - (this.e25 * other.e431) - (this.e35 * other.e412) - (this.e45 * other.e321) - (this.e41 * other.e235) - (this.e42 * other.e315) - (this.e43 * other.e125) - (this.e23 * other.e415) - (this.e31 * other.e425) - (this.e12 * other.e435)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3, 
            /* e5 */
            this.scalar * other.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group1) - (float4(this.e5) * other.group3), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group1.xyz), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group3.zxy) + float3(float2(0.0), (this.e2 * other.e1) * -1.0) - float3((this.group1.zx * other.group3.yz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(this.e31 * other.e2) - (this.e12 * other.e3)) + float4((float3(other.e5) * this.group4.xyz), 0.0) + float4((float3(other.e4) * this.group3.xyz), 0.0) - (other.group3.xyzx * float4(float3(this.e45), this.e23)), 
            /* e423, e431, e412 */
            (float3(this.scalar) * other.group0.xyz) + (float3(other.e4) * this.group5.xyz) + (this.group4.yzx * other.group3.zxy) - (this.group4.zxy * other.group3.yzx), 
            /* e235, e315, e125 */
            (float3(this.scalar) * other.group2.xyz) + (float3(other.e5) * this.group5.xyz) + float3(float2(0.0), (this.e25 * other.e1) - (this.e15 * other.e2)) + float3((this.group3.zx * other.group3.yz), 0.0) - float3((this.group3.yz * other.group3.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group2 * float4(float3(this.e4), this.e321)) + (other.group3.yzxx * float4(this.group6.zxy, this.e235)) + float4(float3(0.0), (this.e125 * other.e3) - (this.e3 * other.e125)) + float4((float3(other.e5) * this.group7.xyz), this.e315 * other.e2) + float4((this.group1.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group1.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(other.e4) * this.group8.xyz), this.e2 * other.e315) - float4((this.group6.yzx * other.group3.zxy), 0.0), 
            /* e1234 */
            (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321) - (this.e321 * other.e4) - (this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<MultiVector> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension MultiVector {
    public MultiVector wedge(VersorOdd other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar * other.scalar, (this.e12345 * other.scalar) + (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e235 * other.e41) - (this.e315 * other.e42) - (this.e125 * other.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e5 */
            this.e5 * other.scalar, 
            /* e15, e25, e35, e45 */
            (float4(this.scalar) * float4(other.e15, other.e25, other.e35, other.e45)) + (float4(other.scalar) * this.group3), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(other.scalar) * this.group4.xyz), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (float3(other.scalar) * this.group5.xyz), 
            /* e415, e425, e435, e321 */
            (float4(this.e5, this.e5, this.e5, this.e321) * other.group0) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(other.scalar) * this.group6.xyz), 0.0) - (this.group1.xyzx * other.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.scalar) * this.group7.xyz) + (this.group1.zxy * other.group0.yzx) + float3(float2(0.0), (this.e1 * other.e42) * -1.0) - float3((this.group1.yz * other.group0.zx), 0.0), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group8.xyz) + (this.group1.yzx * other.group2.zxy) + float3(float2(0.0), (this.e2 * other.e15) * -1.0) - float3((this.group1.zx * other.group2.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group3) + (float4(other.scalar) * this.group9) + float4(float3(0.0), -(this.e25 * other.e31) - (this.e35 * other.e12) - (this.e31 * other.e25) - (this.e12 * other.e35)) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((float3(other.e45) * this.group5.xyz), 0.0) + float4((this.group4.yzx * other.group2.zxy), 0.0) + float4((this.group3.zxy * other.group0.yzx), 0.0) - (float4(this.e43, this.e41, this.e42, this.e23) * other.group2.yzxx) - (this.group3.yzxx * float4(other.group0.zxy, other.e23)), 
            /* e1234 */
            (this.scalar * other.e1234) + (this.e1234 * other.scalar) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Plane> operator /(Plane lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(AntiCircleRotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(AntiDualNum other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(AntiMotor other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215)), 
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
            float4(other.scalar) * this.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(Scalar other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Plane> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public AntiScalar wedge(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Plane> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Plane {
    public Plane wedge(VersorOdd other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<RoundPoint> operator /(RoundPoint lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<RoundPoint>(lhs);
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<RoundPoint> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public AntiDipoleInversion wedge(AntiCircleRotor other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group0.yz), 0.0) - float3((other.group2.yz * this.group0.zx), 0.0)), other.scalar * this.e4), 
            /* e1, e2, e3, e5 */
            float4(other.scalar) * float4(this.group0.xyz, this.e5)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<RoundPoint> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group0.yzxw) + float4(float3(0.0), (other.e4 * this.e5) * -1.0) - float4((other.group3.yzx * this.group0.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (this.group0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3)) - float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<RoundPoint> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), other.e3215 * this.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(float3(0.0), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<RoundPoint> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<RoundPoint> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group0.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group1.xyz)), other.e321 * this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5))
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<RoundPoint> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Circle wedge(AntiLine other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group0.xyz) + (other.group1.zxy * this.group0.yzx) - (other.group1.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<RoundPoint> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.e4) * float4(other.group0.xyz, other.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group0.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group1.zx * this.group0.yz), 0.0) - float3((other.group1.yz * this.group0.zx), 0.0)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<RoundPoint> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Dipole wedge(AntiPlane other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group0.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0)), other.e5 * this.e4), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<RoundPoint> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<RoundPoint> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<RoundPoint> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Circle wedge(Dipole other) {
        return Circle(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group1.xyz) + (other.group2.zxy * this.group0.yzx) - (other.group2.yzx * this.group0.zxy)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<RoundPoint> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group0.zxy) - (other.group0.zxy * this.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group0.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)) - float4((other.group2.yzx * this.group0.zxy), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<RoundPoint> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public FlatPoint wedge(DualNum other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<RoundPoint> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Line wedge(FlatPoint other) {
        return Line(
            /* e415, e425, e435 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e45) * this.group0.xyz), 
            /* e235, e315, e125 */
            (other.group0.zxy * this.group0.yzx) + float3(float2(0.0), (other.e15 * this.e2) * -1.0) - float3((other.group0.yz * this.group0.zx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<RoundPoint> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3)) - float4((float3(other.e45) * this.group0.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group0.yzx) - (other.group0.yzx * this.group0.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<RoundPoint> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group0.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<RoundPoint> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group0.zxy), 0.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<RoundPoint> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group0) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group3.xyz), 0.0) + float4((float3(this.e5) * other.group4.xyz), 0.0) - (this.group0.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group0.zxy) - (other.group4.zxy * this.group0.yzx), 
            /* e235, e315, e125 */
            (float3(this.e5) * other.group5.xyz) + (other.group3.zxy * this.group0.yzx) + float3(float2(0.0), (other.e15 * this.e2) * -1.0) - float3((other.group3.yz * this.group0.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group8.xyz), 0.0) + float4((other.group6.yzx * this.group0.zxy), 0.0) - (float4(this.e5) * float4(other.e423, other.e431, other.e412, other.e321)) - (this.group0.yzxx * float4(other.group6.zxy, other.e235)), 
            /* e1234 */
            (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<RoundPoint> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<RoundPoint> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public Dipole wedge(RoundPoint other) {
        return Dipole(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (this.group0.yzxw * float4(other.group0.zxy, other.e5)) - (other.group0.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35 */
            (float3(other.e5) * this.group0.xyz) - (float3(this.e5) * other.group0.xyz)
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WedgeInfixPartial<RoundPoint> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public RoundPoint wedge(Scalar other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e5 */
            this.e5 * other.scalar
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<RoundPoint> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (this.e1 * other.e4235) + (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e4 * other.e3215) + (this.e5 * other.e1234)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<RoundPoint> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group0.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e3, other.e1, other.e2, other.e5) * this.group0.yzxw) - (other.group3.yzxw * float4(this.group0.zxy, this.e5)), 
            /* e15, e25, e35, e1234 */
            (this.group0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412)) - float4((float3(this.e5) * other.group3.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(this.e2 * other.e315) - (this.e3 * other.e125)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((this.group0.zxy * other.group1.yzx), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group0.yzxx * float4(other.group1.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<RoundPoint> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension RoundPoint {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.e4) * float4(other.group1.xyz, other.e3215)) + (this.group0.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e2 * other.e4315) + (this.e3 * other.e4125) + (this.e5 * other.e1234)) - float4((this.group0.yzx * other.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (this.group0.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (this.group0.yzx * other.group2.zxy) + float3(float2(0.0), this.e2 * other.e15 * -1.0) - float3((this.group0.zx * other.group2.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Scalar> operator /(Scalar lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public AntiCircleRotor operator /(WedgeInfixPartial<Scalar> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiCircleRotor wedge(AntiCircleRotor other) {
        return AntiCircleRotor(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, scalar */
            float4(this.scalar) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public AntiDipoleInversion operator /(WedgeInfixPartial<Scalar> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiDipoleInversion wedge(AntiDipoleInversion other) {
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e4 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public AntiDualNum operator /(WedgeInfixPartial<Scalar> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiDualNum wedge(AntiDualNum other) {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public AntiFlatPoint operator /(WedgeInfixPartial<Scalar> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiFlatPoint wedge(AntiFlatPoint other) {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiFlector operator /(WedgeInfixPartial<Scalar> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiFlector wedge(AntiFlector other) {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.scalar) * other.group0, 
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiLine operator /(WedgeInfixPartial<Scalar> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiLine wedge(AntiLine other) {
        return AntiLine(
            /* e23, e31, e12 */
            float3(this.scalar) * other.group0.xyz, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public AntiMotor operator /(WedgeInfixPartial<Scalar> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiMotor wedge(AntiMotor other) {
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group0, 
            /* e15, e25, e35, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public AntiPlane operator /(WedgeInfixPartial<Scalar> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiPlane wedge(AntiPlane other) {
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Scalar> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Circle operator /(WedgeInfixPartial<Scalar> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Circle wedge(Circle other) {
        return Circle(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<Scalar> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4(this.scalar) * other.group2
        );
    }
}
// Fancy infix trick (second half)
public Dipole operator /(WedgeInfixPartial<Scalar> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Dipole wedge(Dipole other) {
        return Dipole(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35 */
            float3(this.scalar) * other.group2.xyz
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<Scalar> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public DualNum operator /(WedgeInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public DualNum wedge(DualNum other) {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public FlatPoint operator /(WedgeInfixPartial<Scalar> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public FlatPoint wedge(FlatPoint other) {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(WedgeInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Line wedge(Line other) {
        return Line(
            /* e415, e425, e435 */
            float3(this.scalar) * other.group0.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.scalar) * other.group0, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(this.scalar) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group3, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group4.xyz, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group5.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group6, 
            /* e423, e431, e412 */
            float3(this.scalar) * other.group7.xyz, 
            /* e235, e315, e125 */
            float3(this.scalar) * other.group8.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group9, 
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public RoundPoint operator /(WedgeInfixPartial<Scalar> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public RoundPoint wedge(RoundPoint other) {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e5 */
            other.e5 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(WedgeInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Scalar wedge(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Scalar> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            this.scalar * other.e1234
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<Scalar> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4(this.scalar) * other.group0, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e5 */
            float4(this.scalar) * other.group2, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<Scalar> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Scalar {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4(this.scalar) * other.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<Sphere> operator /(Sphere lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<Sphere>(lhs);
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Sphere> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public Sphere wedge(AntiCircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(AntiDipoleInversion other) {
        return AntiScalar(
            /* e12345 */
            (other.e4 * this.e3215) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Sphere> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public Sphere wedge(AntiDualNum other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(AntiFlector other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Sphere> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public Sphere wedge(AntiMotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(AntiPlane other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(DualNum other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(Motor other) {
        return AntiScalar(
            /* e12345 */
            other.e5 * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<Sphere> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)), 
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
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(RoundPoint other) {
        return AntiScalar(
            /* e12345 */
            (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Sphere> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public Sphere wedge(Scalar other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            other.scalar * this.e1234
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<Sphere> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public AntiScalar wedge(VersorEven other) {
        return AntiScalar(
            /* e12345 */
            (this.e4235 * other.e1) + (this.e4315 * other.e2) + (this.e4125 * other.e3) + (this.e3215 * other.e4) + (this.e1234 * other.e5)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<Sphere> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension Sphere {
    public Sphere wedge(VersorOdd other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(other.scalar) * this.group0, 
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<VersorEven> operator /(VersorEven lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<VersorEven>(lhs);
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorEven> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public VersorEven wedge(AntiCircleRotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.scalar) * this.group0) + float4(float3(0.0), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - float4((other.group0.zxy * this.group3.yzx), other.e41 * this.e235), 
            /* e415, e425, e435, e321 */
            (other.group2 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(other.scalar) * this.group1.xyz), 0.0) + float4((float3(this.e5) * other.group0.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group1.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group2.zx * this.group3.yz), 0.0) - float3((other.group2.yz * this.group3.zx), 0.0)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorEven> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public DipoleInversion wedge(AntiDipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (other.group3.zxyw * this.group3.yzxw) + float4(float3(0.0), (other.e4 * this.e5) * -1.0) - float4((other.group3.yzx * this.group3.zxy), 0.0), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e4 * this.e321) - (other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group3.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - (this.group2 * float4(float3(other.e4), other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(this.e5) * other.group0.xyz), other.e315 * this.e2) - float4((other.group3.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorEven> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public VersorEven wedge(AntiDualNum other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            float4((float3(other.scalar) * this.group0.xyz), (other.e3215 * this.e4) + (other.scalar * this.e12345)), 
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
public Sphere operator /(WedgeInfixPartial<VersorEven> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Sphere wedge(AntiFlatPoint other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(this.e4) * other.group0.xyz), -(other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)), 
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorEven> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public DipoleInversion wedge(AntiFlector other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group1.zxy * this.group3.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group1.yz * this.group3.zx), 0.0)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group1.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e235 * this.e1) - (other.e315 * this.e2) - (other.e125 * this.e3) - (other.e321 * this.e5)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - float4((other.group1.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorEven> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public CircleRotor wedge(AntiLine other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.e4) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(this.e4) * other.group1.xyz), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group3.yzx), 0.0) - float4((other.group1.yzx * this.group3.zxy), other.e23 * this.e415)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorEven> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public VersorEven wedge(AntiMotor other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0 * float4(float3(this.e4), this.e12345)) + float4(float3(0.0), (other.e3215 * this.e4) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(other.scalar) * this.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(other.e23 * this.e1) - (other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group0.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group1.zx * this.group3.yz), 0.0) - float3((other.group1.yz * this.group3.zx), 0.0)), other.scalar * this.e5), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorEven> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public DipoleInversion wedge(AntiPlane other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((other.group0.zxy * this.group3.yzx) + float3(float2(0.0), other.e1 * this.e2 * -1.0) - float3((other.group0.yz * this.group3.zx), 0.0)), other.e5 * this.e4), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(other.e2 * this.e431) - (other.e3 * this.e412)) + float4((float3(other.e5) * this.group3.xyz), 0.0) - (other.group0.xyzx * float4(float3(this.e5), this.e423)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((other.group0.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<VersorEven> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Sphere wedge(Circle other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public Sphere operator /(WedgeInfixPartial<VersorEven> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Sphere wedge(CircleRotor other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group2.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - (float4(this.e5) * float4(other.group0.xyz, other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)), 
            /* e1234 */
            (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorEven> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public CircleRotor wedge(Dipole other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((float3(this.e4) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), -(other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(this.e5) * other.group1.xyz), 0.0) + float4((other.group2.zxy * this.group3.yzx), 0.0) - float4((other.group2.yzx * this.group3.zxy), other.e41 * this.e235)
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorEven> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public CircleRotor wedge(DipoleInversion other) {
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(this.e4) * other.group1.xyz) + (other.group0.yzx * this.group3.zxy) - (other.group0.zxy * this.group3.yzx), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((float3(this.e4) * other.group2.xyz), 0.0) - (other.group1.wwwx * this.group3.xyzx), 
            /* e235, e315, e125, e12345 */
            (float4(this.e5) * float4(other.group1.xyz, other.e1234)) + (this.group3.yzxx * float4(other.group2.zxy, other.e4235)) + float4(float3(0.0), (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435) - (other.e45 * this.e321) - (other.e25 * this.e431) - (other.e35 * this.e412)) - (other.group2.yzxx * float4(this.group3.zxy, this.e423))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<VersorEven> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Flector wedge(DualNum other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(other.e5) * float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<VersorEven> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Motor wedge(FlatPoint other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), -(other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) + float4((float3(this.e4) * other.group0.xyz), 0.0) - (float4(other.e45) * float4(this.group3.xyz, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<VersorEven> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Motor wedge(Flector other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(this.e4) * float4(other.group0.xyz, other.e3215)) + float4(float3(0.0), (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412)) - (float4(other.e45) * float4(this.group3.xyz, this.e321)), 
            /* e235, e315, e125, e5 */
            float4(((other.group0.zxy * this.group3.yzx) - (other.group0.yzx * this.group3.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<VersorEven> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Plane wedge(Line other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - (float4(other.e435, other.e415, other.e425, other.e235) * this.group3.yzxx)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<VersorEven> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public Flector wedge(Motor other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(other.e5) * this.group3, 
            /* e4235, e4315, e4125, e3215 */
            (other.group1 * float4(float3(this.e4), this.e321)) + float4(float3(0.0), -(other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((other.group0.yzx * this.group3.zxy), 0.0) - (this.group3.yzxx * float4(other.group0.zxy, other.e235))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<VersorEven> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (other.scalar * this.e12345) + (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5) - (other.e15 * this.e423) - (other.e25 * this.e431) - (other.e35 * this.e412) - (other.e45 * this.e321) - (other.e41 * this.e235) - (other.e42 * this.e315) - (other.e43 * this.e125) - (other.e23 * this.e415) - (other.e31 * this.e425) - (other.e12 * this.e435)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3, 
            /* e5 */
            other.scalar * this.e5, 
            /* e15, e25, e35, e45 */
            (float4(other.e5) * this.group3) - (float4(this.e5) * other.group1), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group3.yzx) + float3(float2(0.0), (other.e1 * this.e2) * -1.0) - float3((other.group1.yz * this.group3.zx), 0.0), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(other.e31 * this.e2) - (other.e12 * this.e3)) + float4((float3(this.e5) * other.group4.xyz), 0.0) + float4((float3(this.e4) * other.group3.xyz), 0.0) - (this.group3.xyzx * float4(float3(other.e45), other.e23)), 
            /* e423, e431, e412 */
            (float3(other.scalar) * this.group0.xyz) + (float3(this.e4) * other.group5.xyz) + (other.group4.yzx * this.group3.zxy) - (other.group4.zxy * this.group3.yzx), 
            /* e235, e315, e125 */
            (float3(other.scalar) * this.group2.xyz) + (float3(this.e5) * other.group5.xyz) + float3(float2(0.0), (other.e25 * this.e1) - (other.e15 * this.e2)) + float3((other.group3.zx * this.group3.yz), 0.0) - float3((other.group3.yz * this.group3.zx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group1.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e3 * this.e125) - (other.e125 * this.e3)) + float4((float3(this.e4) * other.group8.xyz), other.e2 * this.e315) + float4((other.group6.yzx * this.group3.zxy), 0.0) - (this.group2 * float4(float3(other.e4), other.e321)) - (this.group3.yzxx * float4(other.group6.zxy, other.e235)) - float4((float3(this.e5) * other.group7.xyz), other.e315 * this.e2) - float4((other.group1.zxy * this.group1.yzx), 0.0), 
            /* e1234 */
            (other.e321 * this.e4) + (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<VersorEven> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public AntiScalar wedge(Plane other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorEven> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public DipoleInversion wedge(RoundPoint other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (this.group3.yzxw * float4(other.group0.zxy, other.e5)) - (float4(this.e3, this.e1, this.e2, this.e5) * other.group0.yzxw), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) + float4((float3(other.e5) * this.group3.xyz), 0.0) - (other.group0 * float4(float3(this.e5), this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.e5) * float4(this.group0.xyz, this.e321)) + (other.group0.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125)) - float4((float3(other.e4) * this.group2.xyz), 0.0) - float4((other.group0.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorEven> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public VersorEven wedge(Scalar other) {
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
public AntiScalar operator /(WedgeInfixPartial<VersorEven> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public AntiScalar wedge(Sphere other) {
        return AntiScalar(
            /* e12345 */
            (other.e4235 * this.e1) + (other.e4315 * this.e2) + (other.e4125 * this.e3) + (other.e3215 * this.e4) + (other.e1234 * this.e5)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorEven> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public DipoleInversion wedge(VersorEven other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(this.e4) * other.group3.xyz) - (float3(other.e4) * this.group3.xyz), 
            /* e23, e31, e12, e45 */
            (float4(other.e3, other.e1, other.e2, other.e5) * this.group3.yzxw) - (float4(this.e3, this.e1, this.e2, this.e5) * other.group3.yzxw), 
            /* e15, e25, e35, e1234 */
            (this.group3 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), (other.e423 * this.e1) + (other.e431 * this.e2) + (other.e412 * this.e3) - (other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412)) - (other.group3 * float4(float3(this.e5), this.e321)), 
            /* e4235, e4315, e4125, e3215 */
            (other.group2 * float4(float3(this.e4), this.e321)) + (other.group3.yzxx * float4(this.group1.zxy, this.e235)) + float4(float3(0.0), (other.e2 * this.e315) + (other.e3 * this.e125) - (other.e315 * this.e2) - (other.e125 * this.e3)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((other.group1.yzx * this.group3.zxy), 0.0) - (this.group2 * float4(float3(other.e4), other.e321)) - (this.group3.yzxx * float4(other.group1.zxy, other.e235)) - float4((float3(this.e5) * other.group0.xyz), 0.0) - float4((other.group3.zxy * this.group1.yzx), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorEven> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorEven {
    public VersorEven wedge(VersorOdd other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(this.e4) * float4(other.group1.xyz, other.e3215)) + (float4(other.scalar) * this.group0) + (this.group3.zxyx * float4(other.group0.yzx, other.e4235)) + float4(float3(0.0), (this.e5 * other.e1234) + (this.e2 * other.e4315) + (this.e3 * other.e4125) - (this.e423 * other.e15) - (this.e431 * other.e25) - (this.e412 * other.e35) - (this.e415 * other.e23) - (this.e425 * other.e31) - (this.e435 * other.e12) - (this.e321 * other.e45) - (this.e315 * other.e42) - (this.e125 * other.e43)) - (other.group0.zxyx * float4(this.group3.yzx, this.e235)), 
            /* e415, e425, e435, e321 */
            (float4(other.scalar) * this.group1) + float4(float3(0.0), -(this.e2 * other.e31) - (this.e3 * other.e12)) + float4((float3(this.e5) * other.group0.xyz), 0.0) + float4((float3(this.e4) * other.group2.xyz), 0.0) - (this.group3.xyzx * other.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(this.e5) * other.group1.xyz) + (float3(other.scalar) * this.group2.xyz) + float3(float2(0.0), (this.e1 * other.e25) - (this.e2 * other.e15)) + float3((this.group3.yz * other.group2.zx), 0.0) - float3((this.group3.zx * other.group2.yz), 0.0)), this.e5 * other.scalar), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group3
        );
    }
}
// Fancy infix trick (first half)
public WedgeInfixPartial<VersorOdd> operator /(VersorOdd lhs, WedgeInfix rhs) {
    return WedgeInfixPartial<VersorOdd>(lhs);
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<VersorOdd> lhs, AntiCircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd wedge(AntiCircleRotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            (other.group2 * float4(float3(this.scalar), this.e1234)) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + float4((float3(other.scalar) * this.group2.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (other.group2.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorOdd> lhs, AntiDipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorEven wedge(AntiDipoleInversion other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(this.group1.xyz, this.e3215)) + (other.group3.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e315 * this.e42) - (other.e125 * this.e43)) + float4((float3(this.scalar) * other.group0.xyz), other.e2 * this.e4315) - (this.group0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group3.yz * this.group2.zx), 0.0) - float3((other.group3.zx * this.group2.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * float4(other.group3.xyz, other.e4)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<VersorOdd> lhs, AntiDualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd wedge(AntiDualNum other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(other.scalar) * this.group1, 
            /* e15, e25, e35, e1234 */
            float4(other.scalar) * this.group2, 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(other.scalar) * this.group3.xyz), (other.e3215 * this.scalar) + (other.scalar * this.e3215))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorOdd> lhs, AntiFlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public CircleRotor wedge(AntiFlatPoint other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), other.e321 * this.scalar), 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorOdd> lhs, AntiFlector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorEven wedge(AntiFlector other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group1.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234) - (other.e315 * this.e42) - (other.e125 * this.e43) - (other.e321 * this.e45)) - (this.group0.zxyx * float4(other.group1.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (this.group0 * float4(float3(other.e5), other.e321)) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) - (other.group1.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group1.yz * this.group2.zx), 0.0) - float3((other.group1.zx * this.group2.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorOdd> lhs, AntiLine rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public DipoleInversion wedge(AntiLine other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12, e45 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0), 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group1.xyz), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(this.e45) * other.group0.xyz), 0.0) + float4((other.group1.zxy * this.group0.yzx), 0.0) - float4((other.group1.yzx * this.group0.zxy), other.e23 * this.e15)
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<VersorOdd> lhs, AntiMotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd wedge(AntiMotor other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, e45 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.e45), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar) * this.group2) + float4(float3(0.0), -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + float4((float3(this.scalar) * other.group1.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (other.group0 * float4(float3(this.e45), this.e3215)) + (other.group1.zxyw * this.group0.yzxw) + float4(float3(0.0), -(other.e23 * this.e15) - (other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.scalar) * this.group3.xyz), 0.0) - (other.group1.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorOdd> lhs, AntiPlane rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorEven wedge(AntiPlane other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (other.group0.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) - float4((other.group0.yzx * this.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) + float3(float2(0.0), other.e2 * this.e15 * -1.0) - float3((other.group0.zx * this.group2.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4((float3(this.scalar) * other.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public AntiScalar operator /(WedgeInfixPartial<VersorOdd> lhs, AntiScalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public AntiScalar wedge(AntiScalar other) {
        return AntiScalar(
            /* e12345 */
            other.e12345 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorOdd> lhs, Circle rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public CircleRotor wedge(Circle other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public CircleRotor operator /(WedgeInfixPartial<VersorOdd> lhs, CircleRotor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public CircleRotor wedge(CircleRotor other) {
        return CircleRotor(
            /* e423, e431, e412 */
            float3(this.scalar) * other.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.scalar) * other.group1, 
            /* e235, e315, e125, e12345 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e12345 * this.scalar) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43))
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorOdd> lhs, Dipole rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public DipoleInversion wedge(Dipole other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e12 * this.e35) - (other.e15 * this.e23) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - float4((other.group2.yzx * this.group0.zxy), other.e31 * this.e25)
        );
    }
}
// Fancy infix trick (second half)
public DipoleInversion operator /(WedgeInfixPartial<VersorOdd> lhs, DipoleInversion rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public DipoleInversion wedge(DipoleInversion other) {
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(this.scalar) * other.group1, 
            /* e15, e25, e35, e1234 */
            float4((float3(this.scalar) * other.group2.xyz), (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group3) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (other.group2.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<VersorOdd> lhs, DualNum rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Motor wedge(DualNum other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(other.e5) * this.group0.xyz), (other.e5 * this.e1234) + (other.e12345 * this.scalar)), 
            /* e235, e315, e125, e5 */
            float4(other.e5) * float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<VersorOdd> lhs, FlatPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Flector wedge(FlatPoint other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), -(other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(WedgeInfixPartial<VersorOdd> lhs, Flector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Flector wedge(Flector other) {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.scalar) * other.group0, 
            /* e4235, e4315, e4125, e3215 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((other.group0.zxy * this.group0.yzx), 0.0) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<VersorOdd> lhs, Line rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Motor wedge(Line other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4((float3(this.scalar) * other.group0.xyz), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4((float3(this.scalar) * other.group1.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(WedgeInfixPartial<VersorOdd> lhs, Motor rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Motor wedge(Motor other) {
        return Motor(
            /* e415, e425, e435, e12345 */
            (float4(other.e5) * float4(this.group0.xyz, this.e1234)) + (float4(this.scalar) * other.group0) + float4(float3(0.0), -(other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), other.e5 * this.scalar)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(WedgeInfixPartial<VersorOdd> lhs, MultiVector rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public MultiVector wedge(MultiVector other) {
        return MultiVector(
            /* scalar, e12345 */
            float2(other.scalar * this.scalar, (other.e12345 * this.scalar) + (other.e1 * this.e4235) + (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e4 * this.e3215) + (other.e5 * this.e1234) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e235 * this.e41) - (other.e315 * this.e42) - (other.e125 * this.e43)), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e5 */
            other.e5 * this.scalar, 
            /* e15, e25, e35, e45 */
            (float4(other.scalar) * float4(this.e15, this.e25, this.e35, this.e45)) + (float4(this.scalar) * other.group3), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group4.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (float3(this.scalar) * other.group5.xyz), 
            /* e415, e425, e435, e321 */
            (float4(other.e5, other.e5, other.e5, other.e321) * this.group0) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(this.scalar) * other.group6.xyz), 0.0) - (other.group1.xyzx * this.group1.wwwx), 
            /* e423, e431, e412 */
            (float3(other.e4) * this.group1.xyz) + (float3(this.scalar) * other.group7.xyz) + (other.group1.zxy * this.group0.yzx) + float3(float2(0.0), (other.e1 * this.e42) * -1.0) - float3((other.group1.yz * this.group0.zx), 0.0), 
            /* e235, e315, e125 */
            (float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group8.xyz) + (other.group1.yzx * this.group2.zxy) + float3(float2(0.0), (other.e2 * this.e15) * -1.0) - float3((other.group1.zx * this.group2.yz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + (float4(this.scalar) * other.group9) + float4(float3(0.0), -(other.e25 * this.e31) - (other.e35 * this.e12) - (other.e31 * this.e25) - (other.e12 * this.e35)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group5.xyz), 0.0) + float4((other.group4.yzx * this.group2.zxy), 0.0) + float4((other.group3.zxy * this.group0.yzx), 0.0) - (float4(other.e43, other.e41, other.e42, other.e23) * this.group2.yzxx) - (other.group3.yzxx * float4(this.group0.zxy, this.e23)), 
            /* e1234 */
            (other.scalar * this.e1234) + (other.e1234 * this.scalar) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(WedgeInfixPartial<VersorOdd> lhs, Plane rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Plane wedge(Plane other) {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorOdd> lhs, RoundPoint rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorEven wedge(RoundPoint other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(this.group1.xyz, this.e3215)) + (other.group0.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e2 * this.e4315) + (other.e3 * this.e4125) + (other.e5 * this.e1234)) - float4((other.group0.yzx * this.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e4) * this.group2.xyz), 0.0) + float4((float3(other.e5) * this.group0.xyz), 0.0) - (other.group0.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (other.group0.yzx * this.group2.zxy) + float3(float2(0.0), other.e2 * this.e15 * -1.0) - float3((other.group0.zx * this.group2.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<VersorOdd> lhs, Scalar rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd wedge(Scalar other) {
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
public Sphere operator /(WedgeInfixPartial<VersorOdd> lhs, Sphere rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public Sphere wedge(Sphere other) {
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * other.group0, 
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public VersorEven operator /(WedgeInfixPartial<VersorOdd> lhs, VersorEven rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorEven wedge(VersorEven other) {
        return VersorEven(
            /* e423, e431, e412, e12345 */
            (float4(other.e4) * float4(this.group1.xyz, this.e3215)) + (float4(this.scalar) * other.group0) + (other.group3.zxyx * float4(this.group0.yzx, this.e4235)) + float4(float3(0.0), (other.e5 * this.e1234) + (other.e2 * this.e4315) + (other.e3 * this.e4125) - (other.e423 * this.e15) - (other.e431 * this.e25) - (other.e412 * this.e35) - (other.e415 * this.e23) - (other.e425 * this.e31) - (other.e435 * this.e12) - (other.e321 * this.e45) - (other.e315 * this.e42) - (other.e125 * this.e43)) - (this.group0.zxyx * float4(other.group3.yzx, other.e235)), 
            /* e415, e425, e435, e321 */
            (float4(this.scalar) * other.group1) + float4(float3(0.0), -(other.e2 * this.e31) - (other.e3 * this.e12)) + float4((float3(other.e5) * this.group0.xyz), 0.0) + float4((float3(other.e4) * this.group2.xyz), 0.0) - (other.group3.xyzx * this.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4(((float3(other.e5) * this.group1.xyz) + (float3(this.scalar) * other.group2.xyz) + float3(float2(0.0), (other.e1 * this.e25) - (other.e2 * this.e15)) + float3((other.group3.yz * this.group2.zx), 0.0) - float3((other.group3.zx * this.group2.yz), 0.0)), other.e5 * this.scalar), 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group3
        );
    }
}
// Fancy infix trick (second half)
public VersorOdd operator /(WedgeInfixPartial<VersorOdd> lhs, VersorOdd rhs) {
    return lhs.a.wedge(rhs);
}
public extension VersorOdd {
    public VersorOdd wedge(VersorOdd other) {
        return VersorOdd(
            /* e41, e42, e43, scalar */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.scalar) * other.group0.xyz)), other.scalar * this.scalar), 
            /* e23, e31, e12, e45 */
            (float4(other.scalar) * this.group1) + (float4(this.scalar) * other.group1), 
            /* e15, e25, e35, e1234 */
            (float4(other.scalar) * this.group2) + (float4(this.scalar) * other.group2) + float4(float3(0.0), -(other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)), 
            /* e4235, e4315, e4125, e3215 */
            (float4(other.scalar) * this.group3) + (float4(this.scalar) * other.group3) + float4(float3(0.0), -(other.e31 * this.e25) - (other.e12 * this.e35) - (other.e25 * this.e31) - (other.e35 * this.e12)) + float4((float3(other.e45) * this.group1.xyz), 0.0) + float4((float3(this.e45) * other.group1.xyz), 0.0) + float4((other.group0.yzx * this.group2.zxy), 0.0) + float4((other.group2.zxy * this.group0.yzx), 0.0) - (other.group2.yzxx * float4(this.group0.zxy, this.e23)) - (this.group2.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
