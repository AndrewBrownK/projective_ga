implementing cga3d;
// Fancy postfix trick
public Motor operator /(AntiCircleRotor lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Motor operator /(CarrierPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.carrier();
}
public extension AntiCircleRotor {
    public Motor carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, this.scalar)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiDipoleInversion lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Flector operator /(CarrierPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.carrier();
}
public extension AntiDipoleInversion {
    public Flector carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group3.xyz, this.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiDualNum lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public DualNum operator /(CarrierPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.carrier();
}
public extension AntiDualNum {
    public DualNum carrier() {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiDualNum operator /(AntiFlatPoint lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public AntiDualNum operator /(CarrierPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.carrier();
}
public extension AntiFlatPoint {
    public AntiDualNum carrier() {
        return AntiDualNum(
            /* e3215, scalar */
            float2(this.e321, 0.0)
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiFlector lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Flector operator /(CarrierPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.carrier();
}
public extension AntiFlector {
    public Flector carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e321)
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(AntiLine lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(CarrierPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.carrier();
}
public extension AntiLine {
    public AntiFlatPoint carrier() {
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(AntiMotor lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public AntiFlector operator /(CarrierPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.carrier();
}
public extension AntiMotor {
    public AntiFlector carrier() {
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.scalar)
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiPlane lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public FlatPoint operator /(CarrierPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.carrier();
}
public extension AntiPlane {
    public FlatPoint carrier() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Circle lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Plane operator /(CarrierPrefixOrPostfix lhs, Circle rhs) {
    return rhs.carrier();
}
public extension Circle {
    public Plane carrier() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy postfix trick
public Plane operator /(CircleRotor lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Plane operator /(CarrierPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.carrier();
}
public extension CircleRotor {
    public Plane carrier() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy postfix trick
public Line operator /(Dipole lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Line operator /(CarrierPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.carrier();
}
public extension Dipole {
    public Line carrier() {
        return Line(
            /* e415, e425, e435 */
            this.group0.xyz, 
            /* e235, e315, e125 */
            this.group1.xyz
        );
    }
}
// Fancy postfix trick
public Motor operator /(DipoleInversion lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Motor operator /(CarrierPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.carrier();
}
public extension DipoleInversion {
    public Motor carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public MultiVector operator /(CarrierPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.carrier();
}
public extension MultiVector {
    public MultiVector carrier() {
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5 */
            this.scalar, 
            /* e15, e25, e35, e45 */
            this.group1, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group4.xyz, 0.0), 
            /* e423, e431, e412 */
            float3(0.0), 
            /* e235, e315, e125 */
            this.group5.xyz, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group7.xyz, this.e321), 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(RoundPoint lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public FlatPoint operator /(CarrierPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.carrier();
}
public extension RoundPoint {
    public FlatPoint carrier() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0
        );
    }
}
// Fancy postfix trick
public DualNum operator /(Scalar lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public DualNum operator /(CarrierPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.carrier();
}
public extension Scalar {
    public DualNum carrier() {
        return DualNum(
            /* e5, e12345 */
            float2(this.scalar, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(Sphere lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public AntiScalar operator /(CarrierPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.carrier();
}
public extension Sphere {
    public AntiScalar carrier() {
        return AntiScalar(
            /* e12345 */
            this.e1234
        );
    }
}
// Fancy postfix trick
public Flector operator /(VersorEven lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Flector operator /(CarrierPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.carrier();
}
public extension VersorEven {
    public Flector carrier() {
        return Flector(
            /* e15, e25, e35, e45 */
            this.group3, 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e321)
        );
    }
}
// Fancy postfix trick
public Motor operator /(VersorOdd lhs, CarrierPrefixOrPostfix rhs)  {
    return lhs.carrier();
    }
// Fancy prefix trick
public Motor operator /(CarrierPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.carrier();
}
public extension VersorOdd {
    public Motor carrier() {
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(this.group0.xyz, this.e1234), 
            /* e235, e315, e125, e5 */
            float4(this.group1.xyz, this.scalar)
        );
    }
}
