// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 11
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
impl Unit for AntiScalar {
    fn unit() -> Self {
        AntiScalar::from_groups(/* e1234 */ 1.0)
    }
}
impl Unit for DualNum {
    fn unit() -> Self {
        DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0))
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        Flector::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e423, e431, e412, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for Horizon {
    fn unit() -> Self {
        Horizon::from_groups(/* e321 */ 1.0)
    }
}
impl Unit for Line {
    fn unit() -> Self {
        Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e23, e31, e12 */ Simd32x3::from(1.0))
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(1.0), /* e23, e31, e12, scalar */ Simd32x4::from(1.0))
    }
}
impl Unit for MultiVector {
    fn unit() -> Self {
        MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e423, e431, e412, e321
            Simd32x4::from(1.0),
        )
    }
}
impl Unit for Origin {
    fn unit() -> Self {
        Origin::from_groups(/* e4 */ 1.0)
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0))
    }
}
impl Unit for Point {
    fn unit() -> Self {
        Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0))
    }
}
impl Unit for Scalar {
    fn unit() -> Self {
        Scalar::from_groups(/* scalar */ 1.0)
    }
}
