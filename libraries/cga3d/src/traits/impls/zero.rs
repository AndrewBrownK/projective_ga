// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         0       0       0     N/A
//  Average:         0       0       0     N/A
//  Maximum:         0       0       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         0       0       0       0
//  Average:         0       0       0       0
//  Maximum:         0       0       0       0
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
impl Zero for AntiDualNum {
    fn zero() -> Self {
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(0.0))
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
impl Zero for AntiLine {
    fn zero() -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0))
    }
}
impl Zero for AntiMotor {
    fn zero() -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiPlane {
    fn zero() -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(0.0))
    }
}
impl Zero for AntiScalar {
    fn zero() -> Self {
        AntiScalar::from_groups(/* e12345 */ 0.0)
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
impl Zero for DualNum {
    fn zero() -> Self {
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(0.0))
    }
}
impl Zero for FlatPoint {
    fn zero() -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0))
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for Line {
    fn zero() -> Self {
        Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0))
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0))
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        )
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0))
    }
}
impl Zero for RoundPoint {
    fn zero() -> Self {
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0), /* e5 */ 0.0)
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
