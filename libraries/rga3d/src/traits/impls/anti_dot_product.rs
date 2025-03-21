// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 41
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0     N/A
//   Median:         0       1       0     N/A
//  Average:         1       2       0     N/A
//  Maximum:         7       8       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       1       0       0
//   Median:         0       1       0       0
//  Average:         1       2       0       0
//  Maximum:         7       8       0       0
impl std::ops::Div<AntiDotProductInfix> for AntiScalar {
    type Output = AntiDotProductInfixPartial<AntiScalar>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e1234])
    }
}
impl AntiDotProduct<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e1234])
    }
}
impl AntiDotProduct<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e1234])
    }
}
impl std::ops::Div<AntiDotProductInfix> for DualNum {
    type Output = AntiDotProductInfixPartial<DualNum>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e1234])
    }
}
impl AntiDotProduct<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e1234] * other[e1234])
    }
}
impl std::ops::Div<AntiDotProductInfix> for Flector {
    type Output = AntiDotProductInfixPartial<Flector>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<Flector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e4] * self[e4]) + (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
        )
    }
}
impl AntiDotProduct<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (self[e4] * other[e4]) + (self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]),
        )
    }
}
impl AntiDotProduct<Origin> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e4])
    }
}
impl AntiDotProduct<Plane> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]))
    }
}
impl AntiDotProduct<Point> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e4])
    }
}
impl std::ops::Div<AntiDotProductInfix> for Line {
    type Output = AntiDotProductInfixPartial<Line>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<Line> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]))
    }
}
impl AntiDotProduct<Motor> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]))
    }
}
impl AntiDotProduct<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]))
    }
}
impl std::ops::Div<AntiDotProductInfix> for Motor {
    type Output = AntiDotProductInfixPartial<Motor>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<Line> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]))
    }
}
impl AntiDotProduct<Motor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]) + (other[e1234] * self[e1234]),
        )
    }
}
impl AntiDotProduct<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (self[e41] * other[e41]) + (self[e42] * other[e42]) + (self[e43] * other[e43]) + (self[e1234] * other[e1234]),
        )
    }
}
impl std::ops::Div<AntiDotProductInfix> for MultiVector {
    type Output = AntiDotProductInfixPartial<MultiVector>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234])
    }
}
impl AntiDotProduct<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e4] * self[e4]) + (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
        )
    }
}
impl AntiDotProduct<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]))
    }
}
impl AntiDotProduct<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e41] * self[e41]) + (other[e42] * self[e42]) + (other[e43] * self[e43]) + (other[e1234] * self[e1234]),
        )
    }
}
impl AntiDotProduct<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(
            // e1234
            (other[e1234] * self[e1234])
                + (other[e4] * self[e4])
                + (other[e41] * self[e41])
                + (other[e42] * self[e42])
                + (other[e43] * self[e43])
                + (other[e423] * self[e423])
                + (other[e431] * self[e431])
                + (other[e412] * self[e412]),
        )
    }
}
impl AntiDotProduct<Origin> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e4])
    }
}
impl AntiDotProduct<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]))
    }
}
impl AntiDotProduct<Point> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e4])
    }
}
impl std::ops::Div<AntiDotProductInfix> for Origin {
    type Output = AntiDotProductInfixPartial<Origin>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<Flector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<MultiVector> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<Origin> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<Point> for Origin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ self[e4] * other[e4])
    }
}
impl std::ops::Div<AntiDotProductInfix> for Plane {
    type Output = AntiDotProductInfixPartial<Plane>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<Flector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]))
    }
}
impl AntiDotProduct<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]))
    }
}
impl AntiDotProduct<Plane> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        2        3        0        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]))
    }
}
impl std::ops::Div<AntiDotProductInfix> for Point {
    type Output = AntiDotProductInfixPartial<Point>;
    fn div(self, _rhs: AntiDotProductInfix) -> Self::Output {
        AntiDotProductInfixPartial(self)
    }
}
impl AntiDotProduct<Flector> for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<MultiVector> for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<Origin> for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Origin) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
impl AntiDotProduct<Point> for Point {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        0        0
    fn anti_dot_product(self, other: Point) -> AntiScalar {
        use crate::elements::*;
        AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4])
    }
}
