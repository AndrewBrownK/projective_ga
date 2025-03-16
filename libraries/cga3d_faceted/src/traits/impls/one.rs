// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 16
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
impl One for AntiCircleRotor {
    fn one() -> Self {
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(1.0),
        )
    }
}
impl One for AntiCircleRotorAligningOrigin {
    fn one() -> Self {
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x3::from(0.0).with_w(1.0),
        )
    }
}
impl One for AntiCircleRotorAligningOriginAtInfinity {
    fn one() -> Self {
        AntiCircleRotorAligningOriginAtInfinity::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35, scalar */ Simd32x3::from(0.0).with_w(1.0))
    }
}
impl One for AntiCircleRotorAtInfinity {
    fn one() -> Self {
        AntiCircleRotorAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* e15, e25, e35, scalar */ Simd32x3::from(0.0).with_w(1.0))
    }
}
impl One for AntiCircleRotorOnOrigin {
    fn one() -> Self {
        AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x3::from(0.0).with_w(1.0), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl One for AntiDualNum {
    fn one() -> Self {
        AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([0.0, 1.0]))
    }
}
impl One for AntiMotor {
    fn one() -> Self {
        AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x3::from(0.0).with_w(1.0), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0))
    }
}
impl One for AntiMotorOnOrigin {
    fn one() -> Self {
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x3::from(0.0).with_w(1.0))
    }
}
impl One for AntiMysteryCircleRotor {
    fn one() -> Self {
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* scalar */ 1.0)
    }
}
impl One for AntiVersorEvenOnOrigin {
    fn one() -> Self {
        AntiVersorEvenOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x3::from(0.0).with_w(1.0), /* e23, e31, e12, e1234 */ Simd32x4::from(0.0))
    }
}
impl One for MultiVector {
    fn one() -> Self {
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, 0.0]),
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
impl One for MysteryVersorOdd {
    fn one() -> Self {
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([1.0, 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
        )
    }
}
impl One for Scalar {
    fn one() -> Self {
        Scalar::from_groups(/* scalar */ 1.0)
    }
}
impl One for VersorOdd {
    fn one() -> Self {
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl One for VersorOddAtInfinity {
    fn one() -> Self {
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([1.0, 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        )
    }
}
impl One for VersorOddOrthogonalOrigin {
    fn one() -> Self {
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(1.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        )
    }
}
