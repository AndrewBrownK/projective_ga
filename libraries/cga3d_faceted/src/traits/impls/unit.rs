// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl Unit for AntiCircleOnOrigin {
    fn unit() -> Self {
        AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e23, e31, e12 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiCircleRotor {
    fn unit() -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, scalar
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for AntiCircleRotorAligningOrigin {
    fn unit() -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e15, e25, e35, scalar
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for AntiCircleRotorAligningOriginAtInfinity {
    fn unit() -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0), /* e15, e25, e35, scalar */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiCircleRotorAtInfinity {
    fn unit() -> Self {
        AntiCircleRotorAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0), /* e15, e25, e35, scalar */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiCircleRotorOnOrigin {
    fn unit() -> Self {
        AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from(1.0), /* e23, e31, e12 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiDipoleInversion {
    fn unit() -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e4
            Simd32x4::from(1.0),
            // e1, e2, e3, e5
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for AntiDipoleInversionAtInfinity {
    fn unit() -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
            // e1, e2, e3, e5
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for AntiDipoleInversionOnOrigin {
    fn unit() -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0), /* e4, e1, e2, e3 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiDipoleInversionOrthogonalOrigin {
    fn unit() -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(1.0),
            // e415, e425, e435
            Simd32x3::from(1.0),
            // e235, e315, e125, e4
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for AntiDipoleOnOrigin {
    fn unit() -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiDualNum {
    fn unit() -> Self {
        AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(1.0))
    }
}
impl Unit for AntiFlatOrigin {
    fn unit() -> Self {
        AntiFlatOrigin::from_groups(/* e321 */ 1.0)
    }
}
impl Unit for AntiFlatPoint {
    fn unit() -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiFlector {
    fn unit() -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0), /* e1, e2, e3, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiFlectorOnOrigin {
    fn unit() -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiLine {
    fn unit() -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiLineOnOrigin {
    fn unit() -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiMotor {
    fn unit() -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0), /* e15, e25, e35, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiMotorOnOrigin {
    fn unit() -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiMysteryCircleRotor {
    fn unit() -> Self {
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0), /* scalar */ 1.0)
    }
}
impl Unit for AntiMysteryDipoleInversion {
    fn unit() -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0), /* e1, e2, e3 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiPlane {
    fn unit() -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiPlaneOnOrigin {
    fn unit() -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiScalar {
    fn unit() -> Self {
        AntiScalar::from_groups(/* e12345 */ 1.0)
    }
}
impl Unit for AntiSphereOnOrigin {
    fn unit() -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiVersorEvenOnOrigin {
    fn unit() -> Self {
        AntiVersorEvenOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from(1.0), /* e23, e31, e12, e1234 */ Simd32x4::from(1.0))
    }
}
impl Unit for Circle {
    fn unit() -> Self {
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
        )
    }
}
impl Unit for CircleAligningOrigin {
    fn unit() -> Self {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
        )
    }
}
impl Unit for CircleAtInfinity {
    fn unit() -> Self {
        CircleAtInfinity::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for CircleAtOrigin {
    fn unit() -> Self {
        CircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for CircleOnOrigin {
    fn unit() -> Self {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0), /* e415, e425, e435 */ Simd32x3::from(1.0))
    }
}
impl Unit for CircleOrthogonalOrigin {
    fn unit() -> Self {
        CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for CircleRotor {
    fn unit() -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e12345
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for CircleRotorAligningOrigin {
    fn unit() -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435
            Simd32x3::from(1.0),
            // e235, e315, e125, e12345
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for CircleRotorAligningOriginAtInfinity {
    fn unit() -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0), /* e235, e315, e125, e12345 */ Simd32x4::from(1.0))
    }
}
impl Unit for CircleRotorAtInfinity {
    fn unit() -> Self {
        CircleRotorAtInfinity::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0), /* e235, e315, e125, e12345 */ Simd32x4::from(1.0))
    }
}
impl Unit for CircleRotorOnOrigin {
    fn unit() -> Self {
        CircleRotorOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(1.0), /* e415, e425, e435 */ Simd32x3::from(1.0))
    }
}
impl Unit for Dipole {
    fn unit() -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
        )
    }
}
impl Unit for DipoleAligningOrigin {
    fn unit() -> Self {
        DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for DipoleAtInfinity {
    fn unit() -> Self {
        DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for DipoleAtOrigin {
    fn unit() -> Self {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for DipoleInversion {
    fn unit() -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for DipoleInversionAligningOrigin {
    fn unit() -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for DipoleInversionAtInfinity {
    fn unit() -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for DipoleInversionAtOrigin {
    fn unit() -> Self {
        DipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e3215 */ Simd32x4::from(1.0), /* e15, e25, e35, e1234 */ Simd32x4::from(1.0))
    }
}
impl Unit for DipoleInversionOnOrigin {
    fn unit() -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0), /* e1234, e4235, e4315, e4125 */ Simd32x4::from(1.0))
    }
}
impl Unit for DipoleInversionOrthogonalOrigin {
    fn unit() -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for DipoleOnOrigin {
    fn unit() -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0))
    }
}
impl Unit for DipoleOrthogonalOrigin {
    fn unit() -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
        )
    }
}
impl Unit for DualNum {
    fn unit() -> Self {
        DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(1.0))
    }
}
impl Unit for FlatOrigin {
    fn unit() -> Self {
        FlatOrigin::from_groups(/* e45 */ 1.0)
    }
}
impl Unit for FlatPoint {
    fn unit() -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0))
    }
}
impl Unit for FlatPointAtInfinity {
    fn unit() -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for FlectorAtInfinity {
    fn unit() -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for FlectorOnOrigin {
    fn unit() -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from(1.0))
    }
}
impl Unit for Horizon {
    fn unit() -> Self {
        Horizon::from_groups(/* e3215 */ 1.0)
    }
}
impl Unit for Infinity {
    fn unit() -> Self {
        Infinity::from_groups(/* e5 */ 1.0)
    }
}
impl Unit for Line {
    fn unit() -> Self {
        Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for LineAtInfinity {
    fn unit() -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for LineOnOrigin {
    fn unit() -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0))
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for MotorAtInfinity {
    fn unit() -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for MotorOnOrigin {
    fn unit() -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0))
    }
}
impl Unit for MultiVector {
    fn unit() -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
            // e5
            1.0,
            // e41, e42, e43, e45
            Simd32x4::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(1.0),
            // e3215
            1.0,
        )
    }
}
impl Unit for MysteryCircle {
    fn unit() -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for MysteryCircleRotor {
    fn unit() -> Self {
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0), /* e12345 */ 1.0)
    }
}
impl Unit for MysteryDipole {
    fn unit() -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0))
    }
}
impl Unit for MysteryDipoleInversion {
    fn unit() -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0), /* e4235, e4315, e4125 */ Simd32x3::from(1.0))
    }
}
impl Unit for MysteryVersorEven {
    fn unit() -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ Simd32x4::from(1.0), /* e415, e425, e435, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for MysteryVersorOdd {
    fn unit() -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ Simd32x4::from(1.0), /* e23, e31, e12, e45 */ Simd32x4::from(1.0))
    }
}
impl Unit for NullCircleAtOrigin {
    fn unit() -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0))
    }
}
impl Unit for NullDipoleAtOrigin {
    fn unit() -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0))
    }
}
impl Unit for NullDipoleInversionAtOrigin {
    fn unit() -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(1.0))
    }
}
impl Unit for NullSphereAtOrigin {
    fn unit() -> Self {
        NullSphereAtOrigin::from_groups(/* e1234 */ 1.0)
    }
}
impl Unit for NullVersorEvenAtOrigin {
    fn unit() -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(1.0))
    }
}
impl Unit for Origin {
    fn unit() -> Self {
        Origin::from_groups(/* e4 */ 1.0)
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for PlaneOnOrigin {
    fn unit() -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(1.0))
    }
}
impl Unit for RoundPoint {
    fn unit() -> Self {
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e5 */ 1.0)
    }
}
impl Unit for RoundPointAtOrigin {
    fn unit() -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(1.0))
    }
}
impl Unit for Scalar {
    fn unit() -> Self {
        Scalar::from_groups(/* scalar */ 1.0)
    }
}
impl Unit for Sphere {
    fn unit() -> Self {
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0), /* e1234 */ 1.0)
    }
}
impl Unit for SphereAtOrigin {
    fn unit() -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(1.0))
    }
}
impl Unit for SphereOnOrigin {
    fn unit() -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from(1.0))
    }
}
impl Unit for VersorEven {
    fn unit() -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorEvenAligningOrigin {
    fn unit() -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(1.0),
            // e415, e425, e435, e4
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorEvenAtInfinity {
    fn unit() -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorEvenAtOrigin {
    fn unit() -> Self {
        VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for VersorEvenOnOrigin {
    fn unit() -> Self {
        VersorEvenOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(1.0), /* e415, e425, e435, e4 */ Simd32x4::from(1.0))
    }
}
impl Unit for VersorEvenOrthogonalOrigin {
    fn unit() -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorOdd {
    fn unit() -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorOddAtInfinity {
    fn unit() -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for VersorOddOrthogonalOrigin {
    fn unit() -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(1.0),
            // e23, e31, e12, e3215
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
        )
    }
}
