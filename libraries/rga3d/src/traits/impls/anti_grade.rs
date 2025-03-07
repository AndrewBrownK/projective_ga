// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 7
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
impl AntiGrade for AntiScalar {
    fn anti_grade() -> usize {
        0
    }
}
impl AntiGrade for Horizon {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for Line {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for Origin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for Plane {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for Point {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for Scalar {
    fn anti_grade() -> usize {
        4
    }
}
