// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 5
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         3       4       0     N/A
//  Average:         3       4       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         3       4       0       0
//  Average:         3       4       0       0
//  Maximum:         7       8       0       0
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Flector {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * self[e4] + self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412])
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Line {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43])
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Motor {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e41] * self[e41] + self[e42] * self[e42] + self[e43] * self[e43] + self[e1234] * self[e1234])
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for MultiVector {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            self[e1234] * self[e1234]
                + self[e4] * self[e4]
                + self[e41] * self[e41]
                + self[e42] * self[e42]
                + self[e43] * self[e43]
                + self[e423] * self[e423]
                + self[e431] * self[e431]
                + self[e412] * self[e412],
        )
    }
}
impl std::ops::Div<NormSquaredPrefixOrPostfix> for Point {
    type Output = AntiScalar;
    fn div(self, _rhs: NormSquaredPrefixOrPostfix) -> Self::Output {
        self.norm_squared()
    }
}
impl NormSquared for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn norm_squared(self) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * self[e4])
    }
}
