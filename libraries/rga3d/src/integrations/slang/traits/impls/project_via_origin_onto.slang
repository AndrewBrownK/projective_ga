implementing rga3d;
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<DualNum> operator /(DualNum lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public DualNum project_via_origin_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Motor project_via_origin_onto(Flector other) {
        float3 wedge_g1_xyz = other.group0.xyz * float3(this.group0.xx, this.scalar);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group1.zxy) - (wedge_g1_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(wedge_g1_xyz, 0.0).wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (wedge_g1_xyz[1] * other.e2) + (wedge_g1_xyz[2] * other.e3) + (this.scalar * pow(other.e321, 2))) - (other.group1.wwwx * float4(wedge_g1_xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Scalar project_via_origin_onto(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Scalar project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = float4(this.group0.xx, this.scalar, right_dual_g0[3] * this.scalar) * float4(right_dual_g0.xyz, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e1234) * wedge_g0.xyz)), wedge_g0[3] * other.e1234), 
            /* e23, e31, e12, scalar */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 wedge_g0 = float2(1.0, this.scalar * other.scalar) * float2(0.0, 1.0);
        float4 wedge_g1 = float4(float3(0.0), this.scalar * other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group3.xyz * float3(-1.0);
        float4 wedge_g4 = float4((other.group1.xyz * float3(this.group0.xx, this.scalar)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.e1234) + (wedge_g0[1] * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0[1] * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0[1]) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0[1]) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0[1]) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0[1]) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Scalar project_via_origin_onto(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Scalar project_via_origin_onto(Point other) {
        float3 wedge_g0_xyz = other.group0.xyz * float3(this.group0.xx, this.scalar);
        return Scalar(
            /* scalar */
            (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension DualNum {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Flector> operator /(Flector lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float4 wedge_g0 = (this.group0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4(float3(0.0), (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321)) - (right_dual_g0.wwwx * float4(this.group0.xyz, this.e423));
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g0.xyz, -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412))) - (other.group1.yzxx * float4(float3(0.0), wedge_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Horizon other) {
        float4 wedge_g0 = float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Point project_via_origin_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), -(wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), -(wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)), 
            /* e423, e431, e412, e321 */
            float4((wedge_g0_xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        float3 wedge_g2 = (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group0.xyz);
        float4 wedge_g4 = float4(((right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (float4(wedge_g0_y) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Flector project_via_origin_onto(Plane other) {
        float4 wedge_g0 = float4(other.e321 * -1.0) * float4(this.group0.xyz, this.e321) * float4(-1.0);
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g0.xyz, -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412))) - (other.group0.yzxx * float4(float3(0.0), wedge_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Flector {
    public Point project_via_origin_onto(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Horizon> operator /(Horizon lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Horizon {
    public Flector project_via_origin_onto(Flector other) {
        float wedge_g0 = other.e321 * this.e321;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(ProjectViaOriginOntoInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Horizon {
    public Horizon project_via_origin_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Horizon {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float wedge_g0 = this.e321 * other.e321;
        return MultiVector(
            /* scalar, e1234 */
            float2(wedge_g0) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group1, 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(wedge_g0) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Horizon {
    public Plane project_via_origin_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Line> operator /(Line lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Motor project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float4 wedge_g0 = float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((this.group0.yzx * right_dual_g0.zxy), 0.0) - float4((this.group0.zxy * right_dual_g0.yzx), 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g0.yzx * other.group1.zxy) - (wedge_g0.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g0.xyz, (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)) * float4(-1.0, -1.0, -1.0, 1.0)) + (wedge_g0.wwwx * float4(other.group1.xyz, other.e1))
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321) * float3(other.e321 * -1.0) * this.group1.xyz * float3(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Line(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(wedge_g0) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12);
        float4 wedge_g4 = float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((this.group0.yzx * right_dual_g1.zxy), 0.0) - float4((this.group0.zxy * right_dual_g1.yzx), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            (float4(((float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy)), wedge_g4[2] * other.e43) * float4(1.0, 1.0, 1.0, -1.0)) + (float4(wedge_g0_y) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (float3(wedge_g0_y) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectViaOriginOntoInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Line {
    public Line project_via_origin_onto(Plane other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group1.xyz;
        return Line(
            /* e41, e42, e43 */
            (wedge_g0_xyz.yzx * other.group0.zxy) - (wedge_g0_xyz.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            wedge_g0_xyz * float3(other.e321) * float3(-1.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Motor> operator /(Motor lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public DualNum project_via_origin_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float4 wedge_g1 = float4((float3(right_dual_g0[3]) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group0.xyz), 0.0) + float4((right_dual_g0.zxy * this.group0.yzx), 0.0) - float4((right_dual_g0.yzx * this.group0.zxy), 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1.yzx * other.group1.zxy) - (wedge_g1.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge_g1.wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (wedge_g1[1] * other.e2) + (wedge_g1[2] * other.e3) + (wedge_g1[3] * other.e4) - (right_dual_g0[3] * other.e321 * this.scalar)) - (other.group1.wwwx * float4(wedge_g1.xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Horizon other) {
        float right_dual_g0 = other.e321 * -1.0;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4((float3(right_dual_g0) * this.group1.xyz), right_dual_g0 * this.scalar) * float4(-1.0)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group1.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float4 wedge_g0 = (right_dual_g0 * float4(this.scalar)) + float4(float3(0.0), -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12));
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e1234) * wedge_g0.xyz)), wedge_g0[3] * other.e1234), 
            /* e23, e31, e12, scalar */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (this.scalar * other.scalar) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12);
        float4 wedge_g1 = float4(float3(0.0), right_dual_g1[3] * this.scalar);
        float3 wedge_g2 = right_dual_g2 * float3(this.scalar);
        float4 wedge_g4 = float4((float3(right_dual_g1[3]) * this.group1.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((right_dual_g1.zxy * this.group0.yzx), 0.0) - float4((right_dual_g1.yzx * this.group0.zxy), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Motor project_via_origin_onto(Plane other) {
        float right_dual_g0 = other.e321 * -1.0;
        float3 wedge_g1_xyz = float3(right_dual_g0) * this.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group0.zxy) - (wedge_g1_xyz.zxy * other.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(float3(0.0), right_dual_g0 * this.scalar * other.e321) * float4(0.0, 0.0, 0.0, -1.0)) - (other.group0.wwwx * float4(wedge_g1_xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Scalar project_via_origin_onto(Point other) {
        float3 wedge_g0_xyz = other.group0.xyz * this.group1.www;
        return Scalar(
            /* scalar */
            (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Motor {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<MultiVector> operator /(MultiVector lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public DualNum project_via_origin_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float wedge_g0_y = (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        float3 wedge_g2 = (float3(this.e4) * right_dual_g0.xyz) - (float3(right_dual_g0[3]) * this.group1.xyz);
        float4 wedge_g4 = float4((float3(right_dual_g0[3]) * this.group3.xyz), 0.0) + float4((float3(this.scalar) * other.group0.xyz), 0.0) + float4((this.group2.yzx * right_dual_g0.zxy), 0.0) - float4((this.group2.zxy * right_dual_g0.yzx), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (right_dual_g0[3] * other.e321 * this.scalar), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(wedge_g0_y) * other.group0) + (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g2, -(wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))) - (other.group1.yzxx * float4(float3(0.0), wedge_g2[0])), 
            /* e41, e42, e43 */
            (wedge_g4.yzx * other.group1.zxy) - (wedge_g4.zxy * other.group1.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g4[3]) * other.group1.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0_y) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Horizon other) {
        float right_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g0 * other.e321 * this.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_dual_g0) * float3(other.e321) * this.group1.xyz * float3(-1.0)), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_dual_g0) * float3(other.e321) * this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_dual_g0 * other.e321 * this.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float wedge_g0_y = -(right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float3 wedge_g2 = right_dual_g0 * float3(this.scalar);
        float3 wedge_g4_xyz = (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g4_xyz.zxy * other.group1.yzx) - (wedge_g4_xyz.yzx * other.group1.zxy)), -(wedge_g4_xyz[1] * other.e42) - (wedge_g4_xyz[2] * other.e43)), 
            /* e41, e42, e43 */
            float3(wedge_g0_y) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float wedge_g0_y = (right_dual_g0[3] * this.scalar) - (right_dual_g0[0] * this.e23) - (right_dual_g0[1] * this.e31) - (right_dual_g0[2] * this.e12);
        float3 wedge_g2 = float3(this.scalar) * right_dual_g0.xyz;
        float3 wedge_g4_xyz = (right_dual_g0.yzx * this.group1.zxy) - (right_dual_g0.zxy * this.group1.yzx);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g4_xyz.zxy * other.group1.yzx) - (wedge_g4_xyz.yzx * other.group1.zxy)), -(wedge_g4_xyz[1] * other.e42) - (wedge_g4_xyz[2] * other.e43)), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group0.xyz), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((wedge_g4_xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (other.scalar * this.scalar) + (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3) - (right_dual_g2[0] * this.e23) - (right_dual_g2[1] * this.e31) - (right_dual_g2[2] * this.e12) - (right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        float4 wedge_g1 = float4(float3(0.0), right_dual_g1[3] * this.scalar);
        float3 wedge_g2 = (right_dual_g2 * float3(this.scalar)) + (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group1.xyz);
        float4 wedge_g4 = float4((float3(right_dual_g1[3]) * this.group3.xyz), 0.0) + float4((float3(this.scalar) * other.group1.xyz), 0.0) + float4((right_dual_g2.yzx * this.group1.zxy), 0.0) + float4((this.group2.yzx * right_dual_g1.zxy), 0.0) - float4((right_dual_g2.zxy * this.group1.yzx), 0.0) - float4((this.group2.zxy * right_dual_g1.yzx), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Plane other) {
        float right_dual_g0 = other.e321 * -1.0;
        float3 wedge_g2 = float3(right_dual_g0) * this.group1.xyz * float3(-1.0);
        float3 wedge_g4_xyz = float3(right_dual_g0) * this.group3.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_dual_g0 * this.scalar * other.e321, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g2, -(wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412))) - (other.group0.yzxx * float4(float3(0.0), wedge_g2[0])), 
            /* e41, e42, e43 */
            (wedge_g4_xyz.yzx * other.group0.zxy) - (wedge_g4_xyz.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            wedge_g4_xyz * float3(other.e321) * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(right_dual_g0 * this.e321 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_via_origin_onto(Point other) {
        float3 wedge_g4_xyz = other.group0.xyz * float3(this.group0.xx, this.scalar);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g4_xyz[0] * other.e1) + (wedge_g4_xyz[1] * other.e2) + (wedge_g4_xyz[2] * other.e3), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3)) * other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension MultiVector {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Plane> operator /(Plane lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float wedge_g0 = -(right_dual_g0[0] * this.e423) - (right_dual_g0[1] * this.e431) - (right_dual_g0[2] * this.e412) - (right_dual_g0[3] * this.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Horizon operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Horizon project_via_origin_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float wedge_g0 = -(right_dual_g1[0] * this.e423) - (right_dual_g1[1] * this.e431) - (right_dual_g1[2] * this.e412) - (right_dual_g1[3] * this.e321);
        return MultiVector(
            /* scalar, e1234 */
            float2(wedge_g0) * other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(wedge_g0) * other.group1, 
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group2.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(wedge_g0) * other.group4
        );
    }
}
// Fancy infix trick (second half)
public Plane operator /(ProjectViaOriginOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Plane {
    public Plane project_via_origin_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Point> operator /(Point lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Flector project_via_origin_onto(Flector other) {
        float4 right_dual_g0 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float4 wedge_g0 = (this.group0.wwwx * float4(right_dual_g0.xyz, other.e1)) + float4((this.group0.xyz * right_dual_g0.www * float3(-1.0)), (other.e2 * this.e2) + (other.e3 * this.e3));
        return Flector(
            /* e1, e2, e3, e4 */
            (float4(wedge_g0[3]) * other.group0) + (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g0.xyz, -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412))) - (other.group1.yzxx * float4(float3(0.0), wedge_g0[0])), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0[3]) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Point project_via_origin_onto(Horizon other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Point project_via_origin_onto(Line other) {
        float3 right_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), -(wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43))
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Flector project_via_origin_onto(Motor other) {
        float4 right_dual_g0 = other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        float3 wedge_g0_xyz = (right_dual_g0.yzx * this.group0.zxy) - (right_dual_g0.zxy * this.group0.yzx);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), -(wedge_g0_xyz[1] * other.e42) - (wedge_g0_xyz[2] * other.e43)), 
            /* e423, e431, e412, e321 */
            float4((wedge_g0_xyz * other.group0.www), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float4 right_dual_g1 = float4(float3(0.0), other.e321) * float4(0.0, 0.0, 0.0, -1.0);
        float3 right_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3);
        float3 wedge_g2 = (float3(this.e4) * right_dual_g1.xyz) - (float3(right_dual_g1[3]) * this.group0.xyz);
        float4 wedge_g4 = float4(((right_dual_g2.yzx * this.group0.zxy) - (right_dual_g2.zxy * this.group0.yzx)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (float4(wedge_g0_y) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Point project_via_origin_onto(Plane other) {
        float3 wedge_g0 = float3(other.e321 * -1.0) * this.group0.xyz * float3(-1.0);
        return Point(
            /* e1, e2, e3, e4 */
            (float4(other.e321, other.e321, other.e321, 1.0) * float4(wedge_g0, -(wedge_g0[1] * other.e431) - (wedge_g0[2] * other.e412))) - (other.group0.yzxx * float4(float3(0.0), wedge_g0[0]))
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectViaOriginOntoInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Point {
    public Point project_via_origin_onto(Point other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((other.e1 * this.e1) + (other.e2 * this.e2) + (other.e3 * this.e3)) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectViaOriginOntoInfixPartial<Scalar> operator /(Scalar lhs, ProjectViaOriginOntoInfix rhs) {
    return ProjectViaOriginOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public DualNum project_via_origin_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Motor project_via_origin_onto(Flector other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group1.zxy) - (wedge_g1_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (float4(wedge_g1_xyz, 0.0).wwwx * float4(other.group1.xyz, other.e1)) + float4(float3(0.0), (wedge_g1_xyz[1] * other.e2) + (wedge_g1_xyz[2] * other.e3) + (other.e321 * other.e321 * this.scalar)) - (other.group1.wwwx * float4(wedge_g1_xyz, 0.0))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar) * other.group1.xyz * float3(-1.0);
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Motor project_via_origin_onto(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e1234) * wedge_g0.xyz)), wedge_g0[3] * other.e1234), 
            /* e23, e31, e12, scalar */
            (float4(wedge_g0[3]) * other.group1) + float4(float3(0.0), -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public MultiVector project_via_origin_onto(MultiVector other) {
        float2 wedge_g0 = float2(1.0, other.scalar * this.scalar) * float2(0.0, 1.0);
        float4 wedge_g1 = float4(float3(0.0), other.e321 * this.scalar) * float4(0.0, 0.0, 0.0, -1.0);
        float3 wedge_g2 = float3(this.scalar) * other.group3.xyz * float3(-1.0);
        float4 wedge_g4 = float4((float3(this.scalar) * other.group1.xyz), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0[0] * other.e1234) + (wedge_g0[1] * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0[1] * other.e1234), 
            /* e1, e2, e3, e4 */
            float4((wedge_g2[0] * other.e321) + (wedge_g4[2] * other.e31) + (wedge_g4[3] * other.e41), (wedge_g2[1] * other.e321) + (wedge_g4[0] * other.e12) + (wedge_g4[3] * other.e42), (wedge_g2[2] * other.e321) + (wedge_g4[1] * other.e23) + (wedge_g4[3] * other.e43), -(wedge_g2[0] * other.e423) - (wedge_g2[1] * other.e431) - (wedge_g2[2] * other.e412) - (wedge_g4[2] * other.e43)) + (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0[1]) * other.group1) - (wedge_g4.yzxx * float4(other.group3.zxy, other.e41)) - float4(float3(0.0), wedge_g4[1] * other.e42), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0[1]) * other.group2.xyz) + (wedge_g4.yzx * other.group4.zxy) - (wedge_g4.zxy * other.group4.yzx), 
            /* e23, e31, e12 */
            (float3(wedge_g0[1]) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0[1]) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Point other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectViaOriginOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.project_via_origin_onto(rhs);
}
public extension Scalar {
    public Scalar project_via_origin_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
