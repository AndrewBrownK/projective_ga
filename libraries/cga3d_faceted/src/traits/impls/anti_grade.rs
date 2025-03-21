// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 44
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
impl AntiGrade for AntiCircleOnOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for AntiDipoleOnOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for AntiFlatOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for AntiFlatPoint {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for AntiLine {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for AntiLineOnOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for AntiPlane {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for AntiPlaneOnOrigin {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for AntiScalar {
    fn anti_grade() -> usize {
        0
    }
}
impl AntiGrade for AntiSphereOnOrigin {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for Circle {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for CircleAligningOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for CircleAtInfinity {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for CircleAtOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for CircleOnOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for CircleOrthogonalOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for Dipole {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for DipoleAligningOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for DipoleAtInfinity {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for DipoleAtOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for DipoleOnOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for DipoleOrthogonalOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for FlatOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for FlatPoint {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for FlatPointAtInfinity {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for Horizon {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for Infinity {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for Line {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for LineAtInfinity {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for LineOnOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for MysteryCircle {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for MysteryDipole {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for NullCircleAtOrigin {
    fn anti_grade() -> usize {
        2
    }
}
impl AntiGrade for NullDipoleAtOrigin {
    fn anti_grade() -> usize {
        3
    }
}
impl AntiGrade for NullSphereAtOrigin {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for Origin {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for Plane {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for PlaneOnOrigin {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for RoundPoint {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for RoundPointAtOrigin {
    fn anti_grade() -> usize {
        4
    }
}
impl AntiGrade for Scalar {
    fn anti_grade() -> usize {
        5
    }
}
impl AntiGrade for Sphere {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for SphereAtOrigin {
    fn anti_grade() -> usize {
        1
    }
}
impl AntiGrade for SphereOnOrigin {
    fn anti_grade() -> usize {
        1
    }
}
