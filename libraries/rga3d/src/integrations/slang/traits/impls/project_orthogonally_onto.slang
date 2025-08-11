implementing rga3d;
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<DualNum> operator /(DualNum lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<DualNum>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor project_orthogonally_onto(Flector other) {
        float4 wedge_g1 = float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group0.xyz * float3(float2(this.scalar), 1.0)), 0.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0, 0.0, (wedge_g1[0] * other.e431) - (wedge_g1[1] * other.e423), 0.0) + float4(((wedge_g1.yz * other.group1.zx) - (wedge_g1.zx * other.group1.yz)), 0.0, 0.0), 
            /* e23, e31, e12, scalar */
            (wedge_g1.wwwx * float4(other.group1.xyz, other.e1)) + float4(-(float3(other.e321) * wedge_g1.xyz), this.scalar * other.e321 * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Horizon other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group1.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Motor project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e1234) * wedge_g0.xyz)), wedge_g0[3] * other.e1234), 
            /* e23, e31, e12, scalar */
            float4((float4(wedge_g0[3]).xyz * other.group1.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0_y = this.scalar * other.scalar;
        float4 wedge_g1 = float4(float3(0.0), this.scalar * other.e321 * -1.0);
        float3 wedge_g2 = float3(this.scalar * -1.0) * other.group3.xyz;
        float4 wedge_g4 = float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group1.xyz * float3(float2(this.scalar), 1.0)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Plane other) {
        return Scalar(
            /* scalar */
            this.scalar * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Point other) {
        float4 wedge_g0 = float4(1.0, 1.0, this.scalar, 0.0) * float4((other.group0.xyz * float3(float2(this.scalar), 1.0)), 0.0);
        return Scalar(
            /* scalar */
            (wedge_g0[0] * other.e1) + (wedge_g0[1] * other.e2) + (wedge_g0[2] * other.e3) + (wedge_g0[3] * other.e4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<DualNum> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension DualNum {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Flector> operator /(Flector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Flector>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Flector other) {
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float wedge_g0_w = (other.e1 * this.e1) - (right_anti_dual_g0_w * this.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(wedge_g0_w) * other.group0.xyz) - (float3(right_anti_dual_g0_w * other.e321) * this.group0.xyz)), wedge_g0_w * other.e4), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Horizon other) {
        float4 wedge_g0 = float4(other.e321) * float4(this.group0.xyz, this.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321) * wedge_g0.xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge_g0[3] * other.e321)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Point project_orthogonally_onto(Line other) {
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = (right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(0.0, 0.0, (this.e1 * other.e31) - (this.e2 * other.e23), 0.0) + float4(((this.group0.yz * other.group1.zx) - (this.group0.zx * other.group1.yz)), 0.0, 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (wedge_g0[1] * other.e23) - (wedge_g0[0] * other.e31), 0.0) + float4(((float3(wedge_g0[3]) * other.group0.xyz) + float3(((wedge_g0.zx * other.group1.yz) - (wedge_g0.yz * other.group1.zx)), 0.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((wedge_g0.xyz * float4(other.e1234).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g4_xyz[0] * this.e1) + (right_anti_dual_g4_xyz[1] * this.e2) + (right_anti_dual_g4_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321);
        float3 wedge_g2 = float3(right_anti_dual_g1_w * -1.0) * this.group0.xyz;
        float4 wedge_g4 = float4(((right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g0_y) * other.group1.xyz) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), wedge_g0_y * other.e4), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Flector project_orthogonally_onto(Plane other) {
        float4 wedge_g0 = float4(other.e321) * float4(this.group0.xyz, this.e321);
        return Flector(
            /* e1, e2, e3, e4 */
            float4((wedge_g0.xyz * float4(other.e321).xyz), 0.0), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0[3]) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Flector> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Flector {
    public Point project_orthogonally_onto(Point other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Horizon> operator /(Horizon lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Horizon>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Flector project_orthogonally_onto(Flector other) {
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
public Horizon operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Horizon project_orthogonally_onto(Horizon other) {
        return Horizon(
            /* e321 */
            other.e321 * other.e321 * this.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0 = other.e321 * this.e321;
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
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Horizon> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Horizon {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Line> operator /(Line lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Line>(lhs);
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Flector other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g0_xyz.yzx * other.group1.zxy) - (wedge_g0_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((wedge_g0_xyz * float4(other.e321).xyz * float3(-1.0)), wedge_g0_xyz[0] * other.e1)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Horizon other) {
        return Line(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other.e321 * other.e321) * this.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Line other) {
        float wedge_g0 = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12);
        return Line(
            /* e41, e42, e43 */
            float3(wedge_g0) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(wedge_g0) * other.group1.xyz
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Motor project_orthogonally_onto(Motor other) {
        float wedge_g0 = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(wedge_g0) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(wedge_g0) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0_y = (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        float4 wedge_g4 = float4((this.group1.xyz * float4(other.e321).xyz * float3(-1.0)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((float3(wedge_g0_y) * other.group1.xyz) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), wedge_g0_y * other.e4), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Line operator /(ProjectOrthogonallyOntoInfixPartial<Line> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Line {
    public Line project_orthogonally_onto(Plane other) {
        float3 wedge_g0_xyz = float3(other.e321 * -1.0) * this.group1.xyz;
        return Line(
            /* e41, e42, e43 */
            (wedge_g0_xyz.yzx * other.group0.zxy) - (wedge_g0_xyz.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            wedge_g0_xyz * float3(other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Motor> operator /(Motor lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Motor>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Flector other) {
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 wedge_g1_xyz = (float3(right_anti_dual_g0_w) * this.group1.xyz) + (float3(this.scalar) * other.group0.xyz);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group1.zxy) - (wedge_g1_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float4(other.e321).xyz * float3(-1.0)), (wedge_g1_xyz[0] * other.e1) - (right_anti_dual_g0_w * other.e321 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Horizon other) {
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e321 * other.e321) * this.group1
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Line other) {
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = right_anti_dual_g0 * float3(this.scalar);
        float wedge_g0_w = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4((float3(wedge_g0_w) * other.group0.xyz), 0.0), 
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group1.xyz), -(wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Motor other) {
        float3 wedge_g0_xyz = float3(this.scalar * -1.0) * other.group1.xyz;
        float wedge_g0_w = (other.e23 * this.e23) + (other.e31 * this.e31) + (other.e12 * this.e12) + (other.scalar * this.scalar);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g0_xyz * float3(other.e1234)) + (float3(wedge_g0_w) * other.group0.xyz)), wedge_g0_w * other.e1234), 
            /* e23, e31, e12, scalar */
            float4((float3(wedge_g0_w) * other.group1.xyz), (wedge_g0_w * other.scalar) - (wedge_g0_xyz[0] * other.e23) - (wedge_g0_xyz[1] * other.e31) - (wedge_g0_xyz[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float wedge_g0_y = (other.scalar * this.scalar) - (right_anti_dual_g2[0] * this.e23) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12);
        float4 wedge_g1 = float4(float3(0.0), right_anti_dual_g1_w * this.scalar);
        float3 wedge_g2 = right_anti_dual_g2 * float3(this.scalar);
        float4 wedge_g4 = float4(((float3(right_anti_dual_g1_w) * this.group1.xyz) + (float3(this.scalar) * other.group1.xyz)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Motor project_orthogonally_onto(Plane other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        float3 wedge_g1_xyz = float3(right_anti_dual_g0) * this.group1.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group0.zxy) - (wedge_g1_xyz.zxy * other.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float4(other.e321).xyz * float3(-1.0)), right_anti_dual_g0 * this.scalar * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Scalar project_orthogonally_onto(Point other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Motor> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Motor {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<MultiVector> operator /(MultiVector lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<MultiVector>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Flector other) {
        float right_anti_dual_g0_w = other.e321 * -1.0;
        float3 right_anti_dual_g1_xyz = other.group0.xyz;
        float wedge_g0_y = (right_anti_dual_g1_xyz[0] * this.e1) + (right_anti_dual_g1_xyz[1] * this.e2) + (right_anti_dual_g1_xyz[2] * this.e3) - (right_anti_dual_g0_w * this.e321);
        float3 wedge_g4_xyz = (right_anti_dual_g1_xyz * float3(this.scalar)) + (float3(right_anti_dual_g0_w) * this.group3.xyz);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g4_xyz[0] * other.e1) + (wedge_g4_xyz[1] * other.e2) + (wedge_g4_xyz[2] * other.e3) - (right_anti_dual_g0_w * this.scalar * other.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(wedge_g0_y) * other.group0.xyz) - (float3(right_anti_dual_g0_w * other.e321) * this.group1.xyz)), wedge_g0_y * other.e4), 
            /* e41, e42, e43 */
            (wedge_g4_xyz.yzx * other.group1.zxy) - (wedge_g4_xyz.zxy * other.group1.yzx), 
            /* e23, e31, e12 */
            wedge_g4_xyz * float3(other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0_y) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Horizon other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual_g0 * this.scalar * other.e321 * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g0 * other.e321 * -1.0) * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(right_anti_dual_g0 * other.e321 * -1.0) * this.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), right_anti_dual_g0 * this.e321 * other.e321 * -1.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Line other) {
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        float wedge_g0_y = -(right_anti_dual_g0[0] * this.e23) - (right_anti_dual_g0[1] * this.e31) - (right_anti_dual_g0[2] * this.e12);
        float3 wedge_g2 = right_anti_dual_g0 * float3(this.scalar);
        float3 wedge_g4_xyz = (right_anti_dual_g0.yzx * this.group1.zxy) - (right_anti_dual_g0.zxy * this.group1.yzx);
        return MultiVector(
            /* scalar, e1234 */
            float2(-(wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), 0.0), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g4_xyz.zxy * other.group1.yzx) - (wedge_g4_xyz.yzx * other.group1.zxy)), 0.0), 
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
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Motor other) {
        float wedge_g0_y = (this.scalar * other.scalar) + (this.e23 * other.e23) + (this.e31 * other.e31) + (this.e12 * other.e12);
        float3 wedge_g2 = float3(this.scalar * -1.0) * other.group1.xyz;
        float4 wedge_g4 = float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4(((other.group1.zx * this.group1.yz) - (other.group1.yz * this.group1.zx)), 0.0, 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (wedge_g4[1] * other.e23) - (wedge_g4[0] * other.e31), 0.0) + float4(((float3(wedge_g4[3]) * other.group0.xyz) + float3(((wedge_g4.zx * other.group1.yz) - (wedge_g4.yz * other.group1.zx)), 0.0)), 0.0), 
            /* e41, e42, e43 */
            (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group0.xyz), 
            /* e23, e31, e12 */
            float3(wedge_g0_y) * other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4((wedge_g4.xyz * float4(other.e1234).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float right_anti_dual_g1_w = other.e321 * -1.0;
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        float wedge_g0_y = (other.scalar * this.scalar) + (right_anti_dual_g4_xyz[0] * this.e1) + (right_anti_dual_g4_xyz[1] * this.e2) + (right_anti_dual_g4_xyz[2] * this.e3) - (right_anti_dual_g1_w * this.e321) - (right_anti_dual_g2[0] * this.e23) - (right_anti_dual_g2[1] * this.e31) - (right_anti_dual_g2[2] * this.e12);
        float4 wedge_g1 = float4(float3(0.0), right_anti_dual_g1_w * this.scalar);
        float3 wedge_g2 = (right_anti_dual_g2 * float3(this.scalar)) - (float3(right_anti_dual_g1_w) * this.group1.xyz);
        float4 wedge_g4 = float4(((right_anti_dual_g4_xyz * float3(this.scalar)) + (float3(right_anti_dual_g1_w) * this.group3.xyz) + (right_anti_dual_g2.yzx * this.group1.zxy) - (right_anti_dual_g2.zxy * this.group1.yzx)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Plane other) {
        float right_anti_dual_g0 = other.e321 * -1.0;
        float3 wedge_g4_xyz = float3(right_anti_dual_g0) * this.group3.xyz;
        return MultiVector(
            /* scalar, e1234 */
            float2(right_anti_dual_g0 * this.scalar * other.e321 * -1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4((float3(right_anti_dual_g0 * -1.0) * float4(other.e321).xyz * this.group1.xyz), 0.0), 
            /* e41, e42, e43 */
            (wedge_g4_xyz.yzx * other.group0.zxy) - (wedge_g4_xyz.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            wedge_g4_xyz * float3(other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual_g0 * this.e321 * -1.0) * other.group0
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public MultiVector project_orthogonally_onto(Point other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        float4 wedge_g4 = float4(1.0, 1.0, this.scalar, 0.0) * float4((right_anti_dual_g0_xyz * float3(float2(this.scalar), 1.0)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4), 0.0), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0), 
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
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<MultiVector> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension MultiVector {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            this.scalar * other.scalar * other.scalar
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Plane> operator /(Plane lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Plane>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Flector project_orthogonally_onto(Flector other) {
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
public Horizon operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Horizon project_orthogonally_onto(Horizon other) {
        return Horizon(
            /* e321 */
            this.e321 * other.e321 * other.e321
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0 = other.e321 * this.e321;
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
public Plane operator /(ProjectOrthogonallyOntoInfixPartial<Plane> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Plane {
    public Plane project_orthogonally_onto(Plane other) {
        return Plane(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Point> operator /(Point lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Point>(lhs);
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Flector project_orthogonally_onto(Flector other) {
        float wedge_g0_w = other.e1 * this.e1;
        return Flector(
            /* e1, e2, e3, e4 */
            float4(((float3(wedge_g0_w) * other.group0.xyz) + (float3(other.e321 * other.e321) * this.group0.xyz)), wedge_g0_w * other.e4), 
            /* e423, e431, e412, e321 */
            float4(wedge_g0_w) * other.group1
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Horizon other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4((float3(other.e321 * other.e321) * this.group0.xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Line other) {
        float3 right_anti_dual_g0 = other.group1.xyz * float3(-1.0);
        float3 wedge_g0_xyz = (right_anti_dual_g0.yzx * this.group0.zxy) - (right_anti_dual_g0.zxy * this.group0.yzx);
        return Point(
            /* e1, e2, e3, e4 */
            float4(((wedge_g0_xyz.zxy * other.group1.yzx) - (wedge_g0_xyz.yzx * other.group1.zxy)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Flector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Flector project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(0.0, 0.0, (other.e31 * this.e1) - (other.e23 * this.e2), 0.0) + float4(((other.group1.zx * this.group0.yz) - (other.group1.yz * this.group0.zx)), 0.0, 0.0);
        return Flector(
            /* e1, e2, e3, e4 */
            float4(0.0, 0.0, (wedge_g0[1] * other.e23) - (wedge_g0[0] * other.e31), 0.0) + float4(((float3(wedge_g0[3]) * other.group0.xyz) + float3(((wedge_g0.zx * other.group1.yz) - (wedge_g0.yz * other.group1.zx)), 0.0)), 0.0), 
            /* e423, e431, e412, e321 */
            float4((wedge_g0.xyz * float4(other.e1234).xyz), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float3 right_anti_dual_g2 = other.group3.xyz * float3(-1.0);
        float3 right_anti_dual_g4_xyz = other.group1.xyz;
        float wedge_g0_y = (right_anti_dual_g4_xyz[0] * this.e1) + (right_anti_dual_g4_xyz[1] * this.e2) + (right_anti_dual_g4_xyz[2] * this.e3);
        float3 wedge_g2 = float3(other.e321) * this.group0.xyz;
        float4 wedge_g4 = float4(((right_anti_dual_g2.yzx * this.group0.zxy) - (right_anti_dual_g2.zxy * this.group0.yzx)), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g0_y) * other.group1.xyz) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), wedge_g0_y * other.e4), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Plane other) {
        return Point(
            /* e1, e2, e3, e4 */
            float4(1.0, 1.0, other.e321, 0.0) * float4((float4(other.e321).xyz * this.group0.xyz * float3(other.group0.ww, 1.0)), 0.0)
        );
    }
}
// Fancy infix trick (second half)
public Point operator /(ProjectOrthogonallyOntoInfixPartial<Point> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Point {
    public Point project_orthogonally_onto(Point other) {
        float3 right_anti_dual_g0_xyz = other.group0.xyz;
        return Point(
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual_g0_xyz[0] * this.e1) * other.group0) + (float4(right_anti_dual_g0_xyz[1] * this.e2) * other.group0) + (float4(right_anti_dual_g0_xyz[2] * this.e3) * other.group0)
        );
    }
}
// Fancy infix trick (first half)
public ProjectOrthogonallyOntoInfixPartial<Scalar> operator /(Scalar lhs, ProjectOrthogonallyOntoInfix rhs) {
    return ProjectOrthogonallyOntoInfixPartial<Scalar>(lhs);
}
// Fancy infix trick (second half)
public DualNum operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, DualNum rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public DualNum project_orthogonally_onto(DualNum other) {
        return DualNum(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0.xy
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Flector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Motor project_orthogonally_onto(Flector other) {
        float3 wedge_g1_xyz = float3(this.scalar) * other.group0.xyz;
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((wedge_g1_xyz.yzx * other.group1.zxy) - (wedge_g1_xyz.zxy * other.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4((wedge_g1_xyz * float4(other.e321).xyz * float3(-1.0)), (wedge_g1_xyz[0] * other.e1) + (other.e321 * other.e321 * this.scalar))
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Horizon rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Horizon other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Line rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Line other) {
        float3 wedge_g0 = float3(this.scalar * -1.0) * other.group1.xyz;
        return Scalar(
            /* scalar */
            -(wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12)
        );
    }
}
// Fancy infix trick (second half)
public Motor operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Motor rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Motor project_orthogonally_onto(Motor other) {
        float4 wedge_g0 = float4(this.scalar) * other.group1 * float4(-1.0, -1.0, -1.0, 1.0);
        return Motor(
            /* e41, e42, e43, e1234 */
            float4(((float3(wedge_g0[3]) * other.group0.xyz) + (float3(other.e1234) * wedge_g0.xyz)), wedge_g0[3] * other.e1234), 
            /* e23, e31, e12, scalar */
            float4((float4(wedge_g0[3]).xyz * other.group1.xyz), (wedge_g0[3] * other.scalar) - (wedge_g0[0] * other.e23) - (wedge_g0[1] * other.e31) - (wedge_g0[2] * other.e12))
        );
    }
}
// Fancy infix trick (second half)
public MultiVector operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, MultiVector rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public MultiVector project_orthogonally_onto(MultiVector other) {
        float wedge_g0_y = other.scalar * this.scalar;
        float4 wedge_g1 = float4(float3(0.0), other.e321 * this.scalar * -1.0);
        float3 wedge_g2 = float3(this.scalar * -1.0) * other.group3.xyz;
        float4 wedge_g4 = float4((float3(this.scalar) * other.group1.xyz), 0.0);
        return MultiVector(
            /* scalar, e1234 */
            float2((wedge_g0_y * other.scalar) + (wedge_g4[0] * other.e1) + (wedge_g4[1] * other.e2) + (wedge_g4[2] * other.e3) + (wedge_g4[3] * other.e4) - (wedge_g2[0] * other.e23) - (wedge_g2[1] * other.e31) - (wedge_g2[2] * other.e12) - (wedge_g1[0] * other.e423) - (wedge_g1[1] * other.e431) - (wedge_g1[2] * other.e412) - (wedge_g1[3] * other.e321), wedge_g0_y * other.e1234), 
            /* e1, e2, e3, e4 */
            (wedge_g1 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group1) + float4(((wedge_g2 * float3(other.e321)) + (float3(wedge_g4[3]) * other.group2.xyz) + (other.group3.yzx * wedge_g4.zxy) - (other.group3.zxy * wedge_g4.yzx)), 0.0), 
            /* e41, e42, e43 */
            float3((wedge_g4[1] * other.e412) - (wedge_g4[2] * other.e431), (wedge_g4[2] * other.e423) - (wedge_g4[0] * other.e412), (wedge_g4[0] * other.e431) - (wedge_g4[1] * other.e423)) + (wedge_g2 * float3(other.e1234)) + (float3(wedge_g0_y) * other.group2.xyz), 
            /* e23, e31, e12 */
            (float3(wedge_g0_y) * other.group3.xyz) + (float3(wedge_g4[3]) * other.group4.xyz) - (float3(other.e321) * wedge_g4.xyz), 
            /* e423, e431, e412, e321 */
            (wedge_g4 * float4(other.e1234)) + (float4(wedge_g0_y) * other.group4)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Plane rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Plane other) {
        return Scalar(
            /* scalar */
            other.e321 * other.e321 * this.scalar
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Point rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Point other) {
        float3 wedge_g0_xyz = float3(this.scalar) * other.group0.xyz;
        return Scalar(
            /* scalar */
            (wedge_g0_xyz[0] * other.e1) + (wedge_g0_xyz[1] * other.e2) + (wedge_g0_xyz[2] * other.e3)
        );
    }
}
// Fancy infix trick (second half)
public Scalar operator /(ProjectOrthogonallyOntoInfixPartial<Scalar> lhs, Scalar rhs) {
    return lhs.a.project_orthogonally_onto(rhs);
}
public extension Scalar {
    public Scalar project_orthogonally_onto(Scalar other) {
        return Scalar(
            /* scalar */
            other.scalar * other.scalar * this.scalar
        );
    }
}
