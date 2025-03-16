// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 12
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
impl Grade for AntiFlatPoint {
    fn grade() -> usize {
        3
    }
}
impl Grade for AntiLine {
    fn grade() -> usize {
        2
    }
}
impl Grade for AntiPlane {
    fn grade() -> usize {
        1
    }
}
impl Grade for AntiScalar {
    fn grade() -> usize {
        5
    }
}
impl Grade for Circle {
    fn grade() -> usize {
        3
    }
}
impl Grade for Dipole {
    fn grade() -> usize {
        2
    }
}
impl Grade for FlatPoint {
    fn grade() -> usize {
        2
    }
}
impl Grade for Line {
    fn grade() -> usize {
        3
    }
}
impl Grade for Plane {
    fn grade() -> usize {
        4
    }
}
impl Grade for RoundPoint {
    fn grade() -> usize {
        1
    }
}
impl Grade for Scalar {
    fn grade() -> usize {
        0
    }
}
impl Grade for Sphere {
    fn grade() -> usize {
        4
    }
}
