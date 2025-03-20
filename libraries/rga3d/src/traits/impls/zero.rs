// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
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
impl Zero for AntiScalar {
    fn zero() -> Self {
        AntiScalar::from_groups(/* e1234 */ 0.0)
    }
}
impl Zero for DualNum {
    fn zero() -> Self {
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(0.0))
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        Flector::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0), /* e423, e431, e412, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for Horizon {
    fn zero() -> Self {
        Horizon::from_groups(/* e321 */ 0.0)
    }
}
impl Zero for Line {
    fn zero() -> Self {
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0))
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(0.0), /* e23, e31, e12, scalar */ Simd32x4::from(0.0))
    }
}
impl Zero for MultiVector {
    fn zero() -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        )
    }
}
impl Zero for Origin {
    fn zero() -> Self {
        Origin::from_groups(/* e4 */ 0.0)
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0))
    }
}
impl Zero for Point {
    fn zero() -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0))
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        Scalar::from_groups(/* scalar */ 0.0)
    }
}
