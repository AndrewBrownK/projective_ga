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
impl Zero for AntiCircleOnOrigin {
    fn zero() -> Self {
        AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiCircleRotor {
    fn zero() -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for AntiCircleRotorAligningOrigin {
    fn zero() -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for AntiCircleRotorAligningOriginAtInfinity {
    fn zero() -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35, scalar */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiCircleRotorAtInfinity {
    fn zero() -> Self {
        AntiCircleRotorAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* e15, e25, e35, scalar */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiCircleRotorOnOrigin {
    fn zero() -> Self {
        AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiDipoleInversion {
    fn zero() -> Self {
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for AntiDipoleInversionAtInfinity {
    fn zero() -> Self {
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for AntiDipoleInversionOnOrigin {
    fn zero() -> Self {
        AntiDipoleInversionOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0), /* e4, e1, e2, e3 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiDipoleInversionOrthogonalOrigin {
    fn zero() -> Self {
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for AntiDipoleOnOrigin {
    fn zero() -> Self {
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiDualNum {
    fn zero() -> Self {
        AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(0.0))
    }
}
impl Zero for AntiFlatOrigin {
    fn zero() -> Self {
        AntiFlatOrigin::from_groups(/* e321 */ 0.0)
    }
}
impl Zero for AntiFlatPoint {
    fn zero() -> Self {
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiFlector {
    fn zero() -> Self {
        AntiFlector::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(0.0), /* e1, e2, e3, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiFlectorOnOrigin {
    fn zero() -> Self {
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiLine {
    fn zero() -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiLineOnOrigin {
    fn zero() -> Self {
        AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiMotor {
    fn zero() -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiMotorOnOrigin {
    fn zero() -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiMysteryCircleRotor {
    fn zero() -> Self {
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* scalar */ 0.0)
    }
}
impl Zero for AntiMysteryDipoleInversion {
    fn zero() -> Self {
        AntiMysteryDipoleInversion::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0), /* e1, e2, e3 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiPlane {
    fn zero() -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiPlaneOnOrigin {
    fn zero() -> Self {
        AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiScalar {
    fn zero() -> Self {
        AntiScalar::from_groups(/* e12345 */ 0.0)
    }
}
impl Zero for AntiSphereOnOrigin {
    fn zero() -> Self {
        AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiVersorEvenOnOrigin {
    fn zero() -> Self {
        AntiVersorEvenOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from(0.0), /* e23, e31, e12, e1234 */ Simd32x4::from(0.0))
    }
}
impl Zero for Circle {
    fn zero() -> Self {
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        )
    }
}
impl Zero for CircleAligningOrigin {
    fn zero() -> Self {
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        )
    }
}
impl Zero for CircleAtInfinity {
    fn zero() -> Self {
        CircleAtInfinity::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for CircleAtOrigin {
    fn zero() -> Self {
        CircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for CircleOnOrigin {
    fn zero() -> Self {
        CircleOnOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0), /* e415, e425, e435 */ Simd32x3::from(0.0))
    }
}
impl Zero for CircleOrthogonalOrigin {
    fn zero() -> Self {
        CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for CircleRotor {
    fn zero() -> Self {
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for CircleRotorAligningOrigin {
    fn zero() -> Self {
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for CircleRotorAligningOriginAtInfinity {
    fn zero() -> Self {
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0), /* e235, e315, e125, e12345 */ Simd32x4::from(0.0))
    }
}
impl Zero for CircleRotorAtInfinity {
    fn zero() -> Self {
        CircleRotorAtInfinity::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0), /* e235, e315, e125, e12345 */ Simd32x4::from(0.0))
    }
}
impl Zero for CircleRotorOnOrigin {
    fn zero() -> Self {
        CircleRotorOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(0.0), /* e415, e425, e435 */ Simd32x3::from(0.0))
    }
}
impl Zero for Dipole {
    fn zero() -> Self {
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        )
    }
}
impl Zero for DipoleAligningOrigin {
    fn zero() -> Self {
        DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for DipoleAtInfinity {
    fn zero() -> Self {
        DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for DipoleAtOrigin {
    fn zero() -> Self {
        DipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for DipoleInversion {
    fn zero() -> Self {
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for DipoleInversionAligningOrigin {
    fn zero() -> Self {
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for DipoleInversionAtInfinity {
    fn zero() -> Self {
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for DipoleInversionAtOrigin {
    fn zero() -> Self {
        DipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e3215 */ Simd32x4::from(0.0), /* e15, e25, e35, e1234 */ Simd32x4::from(0.0))
    }
}
impl Zero for DipoleInversionOnOrigin {
    fn zero() -> Self {
        DipoleInversionOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(0.0), /* e1234, e4235, e4315, e4125 */ Simd32x4::from(0.0))
    }
}
impl Zero for DipoleInversionOrthogonalOrigin {
    fn zero() -> Self {
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for DipoleOnOrigin {
    fn zero() -> Self {
        DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(0.0))
    }
}
impl Zero for DipoleOrthogonalOrigin {
    fn zero() -> Self {
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        )
    }
}
impl Zero for DualNum {
    fn zero() -> Self {
        DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(0.0))
    }
}
impl Zero for FlatOrigin {
    fn zero() -> Self {
        FlatOrigin::from_groups(/* e45 */ 0.0)
    }
}
impl Zero for FlatPoint {
    fn zero() -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0))
    }
}
impl Zero for FlatPointAtInfinity {
    fn zero() -> Self {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for FlectorAtInfinity {
    fn zero() -> Self {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for FlectorOnOrigin {
    fn zero() -> Self {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from(0.0))
    }
}
impl Zero for Horizon {
    fn zero() -> Self {
        Horizon::from_groups(/* e3215 */ 0.0)
    }
}
impl Zero for Infinity {
    fn zero() -> Self {
        Infinity::from_groups(/* e5 */ 0.0)
    }
}
impl Zero for Line {
    fn zero() -> Self {
        Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for LineAtInfinity {
    fn zero() -> Self {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for LineOnOrigin {
    fn zero() -> Self {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0))
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for MotorAtInfinity {
    fn zero() -> Self {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for MotorOnOrigin {
    fn zero() -> Self {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0))
    }
}
impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        )
    }
}
impl Zero for MysteryCircle {
    fn zero() -> Self {
        MysteryCircle::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for MysteryCircleRotor {
    fn zero() -> Self {
        MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0), /* e12345 */ 0.0)
    }
}
impl Zero for MysteryDipole {
    fn zero() -> Self {
        MysteryDipole::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0))
    }
}
impl Zero for MysteryDipoleInversion {
    fn zero() -> Self {
        MysteryDipoleInversion::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125 */ Simd32x3::from(0.0))
    }
}
impl Zero for MysteryVersorEven {
    fn zero() -> Self {
        MysteryVersorEven::from_groups(/* e12345, e1, e2, e3 */ Simd32x4::from(0.0), /* e415, e425, e435, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for MysteryVersorOdd {
    fn zero() -> Self {
        MysteryVersorOdd::from_groups(/* scalar, e4235, e4315, e4125 */ Simd32x4::from(0.0), /* e23, e31, e12, e45 */ Simd32x4::from(0.0))
    }
}
impl Zero for NullCircleAtOrigin {
    fn zero() -> Self {
        NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0))
    }
}
impl Zero for NullDipoleAtOrigin {
    fn zero() -> Self {
        NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0))
    }
}
impl Zero for NullDipoleInversionAtOrigin {
    fn zero() -> Self {
        NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(0.0))
    }
}
impl Zero for NullSphereAtOrigin {
    fn zero() -> Self {
        NullSphereAtOrigin::from_groups(/* e1234 */ 0.0)
    }
}
impl Zero for NullVersorEvenAtOrigin {
    fn zero() -> Self {
        NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(0.0))
    }
}
impl Zero for Origin {
    fn zero() -> Self {
        Origin::from_groups(/* e4 */ 0.0)
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for PlaneOnOrigin {
    fn zero() -> Self {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(0.0))
    }
}
impl Zero for RoundPoint {
    fn zero() -> Self {
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0), /* e5 */ 0.0)
    }
}
impl Zero for RoundPointAtOrigin {
    fn zero() -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(0.0))
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        Scalar::from_groups(/* scalar */ 0.0)
    }
}
impl Zero for Sphere {
    fn zero() -> Self {
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0), /* e1234 */ 0.0)
    }
}
impl Zero for SphereAtOrigin {
    fn zero() -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(0.0))
    }
}
impl Zero for SphereOnOrigin {
    fn zero() -> Self {
        SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from(0.0))
    }
}
impl Zero for VersorEven {
    fn zero() -> Self {
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorEvenAligningOrigin {
    fn zero() -> Self {
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorEvenAtInfinity {
    fn zero() -> Self {
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorEvenAtOrigin {
    fn zero() -> Self {
        VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for VersorEvenOnOrigin {
    fn zero() -> Self {
        VersorEvenOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(0.0), /* e415, e425, e435, e4 */ Simd32x4::from(0.0))
    }
}
impl Zero for VersorEvenOrthogonalOrigin {
    fn zero() -> Self {
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorOdd {
    fn zero() -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorOddAtInfinity {
    fn zero() -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for VersorOddOrthogonalOrigin {
    fn zero() -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}
