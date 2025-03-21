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
impl Unit for AntiDualNum {
    fn unit() -> Self {
        AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(1.0))
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
impl Unit for AntiLine {
    fn unit() -> Self {
        AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0))
    }
}
impl Unit for AntiMotor {
    fn unit() -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0), /* e15, e25, e35, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiPlane {
    fn unit() -> Self {
        AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(1.0))
    }
}
impl Unit for AntiScalar {
    fn unit() -> Self {
        AntiScalar::from_groups(/* e12345 */ 1.0)
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
impl Unit for DualNum {
    fn unit() -> Self {
        DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(1.0))
    }
}
impl Unit for FlatPoint {
    fn unit() -> Self {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0))
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for Line {
    fn unit() -> Self {
        Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0))
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0))
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
            // e15, e25, e35, e45
            Simd32x4::from(1.0),
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
            // e1234
            1.0,
        )
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0))
    }
}
impl Unit for RoundPoint {
    fn unit() -> Self {
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e5 */ 1.0)
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
