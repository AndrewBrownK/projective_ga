// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:         4       6       0
//  Maximum:        23      33       1
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4      12       0
//  Average:         5      13       0
//  Maximum:        23      54       2
impl AntiInverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       23        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       22        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[scalar] * self[scalar]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - self[scalar] * self[scalar];
        AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - self[scalar] * self[scalar];
        AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       29        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl AntiInverse for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        5        0
    //  no simd        6       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 =
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] - self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group2(),
        )
    }
}
impl AntiInverse for AntiDipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e4, e1, e2, e3
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       28        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([
            self[e423] / (self[e321] * self[e321]),
            self[e431] / (self[e321] * self[e321]),
            self[e412] / (self[e321] * self[e321]),
            1.0 / self[e321],
        ]))
    }
}
impl AntiInverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        2        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiDualNum::from_groups(
            // e1234, scalar
            Simd32x2::from([self[e1234] / (self[scalar] * self[scalar]), 1.0 / self[scalar]]) * Simd32x2::from(-1.0),
        )
    }
}
impl AntiInverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ 1.0 / self[e321])
    }
}
impl AntiInverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from([
            self[e235] / (self[e321] * self[e321]),
            self[e315] / (self[e321] * self[e321]),
            self[e125] / (self[e321] * self[e321]),
            1.0 / self[e321],
        ]))
    }
}
impl AntiInverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([
            f32::powi(self[e321], 3) + (self[e1] * self[e1] * self[e321]) + (self[e2] * self[e2] * self[e321]) + (self[e3] * self[e3] * self[e321]),
            -f32::powi(self[e1], 3) - (self[e321] * self[e321] * self[e1]) - (self[e2] * self[e2] * self[e1]) - (self[e3] * self[e3] * self[e1]),
            -f32::powi(self[e2], 3) - (self[e321] * self[e321] * self[e2]) - (self[e1] * self[e1] * self[e2]) - (self[e3] * self[e3] * self[e2]),
            -f32::powi(self[e3], 3) - (self[e321] * self[e321] * self[e3]) - (self[e1] * self[e1] * self[e3]) - (self[e2] * self[e2] * self[e3]),
        ]))
    }
}
impl AntiInverse for AntiLine {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn anti_inverse(self) -> Self {
        AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::powi(self.group0(), 3) + (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) + (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - self[scalar] * self[scalar];
        AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            f32::powi(self[e23], 3) + (self[e31] * self[e31] * self[e23]) + (self[e12] * self[e12] * self[e23]) + (self[scalar] * self[scalar] * self[e23]),
            f32::powi(self[e31], 3) + (self[e23] * self[e23] * self[e31]) + (self[e12] * self[e12] * self[e31]) + (self[scalar] * self[scalar] * self[e31]),
            f32::powi(self[e12], 3) + (self[e23] * self[e23] * self[e12]) + (self[e31] * self[e31] * self[e12]) + (self[scalar] * self[scalar] * self[e12]),
            -f32::powi(self[scalar], 3) - (self[e23] * self[e23] * self[scalar]) - (self[e31] * self[e31] * self[scalar]) - (self[e12] * self[e12] * self[scalar]),
        ]))
    }
}
impl AntiInverse for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4        6        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - self[scalar] * self[scalar];
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(other_g0 * -1.0) * self.group0(), /* scalar */ other_g0 * self[scalar])
    }
}
impl AntiInverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 =
            self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] - self[e321] * self[e321] - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e1, e2, e3
            Simd32x3::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for AntiPlane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                f32::powi(self[e1], 3) * -1.0,
                f32::powi(self[e2], 3) * -1.0,
                f32::powi(self[e3], 3) * -1.0,
                self[e3] * self[e3] * self[e5] * -1.0,
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn anti_inverse(self) -> Self {
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            -Simd32x3::powi(self.group0(), 3) - (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) - (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ 1.0 / self[e12345])
    }
}
impl AntiInverse for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        8       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                f32::powi(self[e1], 3) * -1.0,
                f32::powi(self[e2], 3) * -1.0,
                f32::powi(self[e3], 3) * -1.0,
                self[e3] * self[e3] * self[e4] * -1.0,
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[scalar] * self[scalar] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       19        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321];
        Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl AntiInverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       18        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl AntiInverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] - self[e321] * self[e321];
        CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       14        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]);
        CircleAtOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       15        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) - self[e321] * self[e321];
        CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       23        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345]
            - self[e321] * self[e321];
        CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       22        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e12345] * self[e12345];
        CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345];
        CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345] - self[e321] * self[e321];
        CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       19        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35]);
        Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl AntiInverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       15        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12];
        DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       14        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        DipoleAtOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for DipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       10        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       10       29        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl AntiInverse for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       25        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group2(),
        )
    }
}
impl AntiInverse for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        5        0
    //  no simd        6       13        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12];
        DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group2(),
        )
    }
}
impl AntiInverse for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       24        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]) - 2.0 * (self[e3215] * self[e1234]);
        DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125];
        DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       28        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                self[e41] / (self[e45] * self[e45]),
                self[e42] / (self[e45] * self[e45]),
                self[e43] / (self[e45] * self[e45]),
                1.0 / self[e45],
            ]) * Simd32x4::from(-1.0),
        )
    }
}
impl AntiInverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       18        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 =
            -self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - 2.0 * (self[e41] * self[e15]) - 2.0 * (self[e42] * self[e25]) - 2.0 * (self[e43] * self[e35]);
        DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group2(),
        )
    }
}
impl AntiInverse for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        0        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([self[e4] / (self[e12345] * self[e12345]), 1.0 / self[e12345]]))
    }
}
impl AntiInverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ -1.0 / self[e45])
    }
}
impl AntiInverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        0        1
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        1        1
    //  no simd        0        4        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                self[e15] / (self[e45] * self[e45]),
                self[e25] / (self[e45] * self[e45]),
                self[e35] / (self[e45] * self[e45]),
                1.0 / self[e45],
            ]) * Simd32x4::from(-1.0),
        )
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125];
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            -f32::powi(self[e45], 3) - (self[e4235] * self[e4235] * self[e45]) - (self[e4315] * self[e4315] * self[e45]) - (self[e4125] * self[e4125] * self[e45]),
            f32::powi(self[e4235], 3) + (self[e45] * self[e45] * self[e4235]) + (self[e4315] * self[e4315] * self[e4235]) + (self[e4125] * self[e4125] * self[e4235]),
            f32::powi(self[e4315], 3) + (self[e45] * self[e45] * self[e4315]) + (self[e4235] * self[e4235] * self[e4315]) + (self[e4125] * self[e4125] * self[e4315]),
            f32::powi(self[e4125], 3) + (self[e45] * self[e45] * self[e4125]) + (self[e4235] * self[e4235] * self[e4125]) + (self[e4315] * self[e4315] * self[e4125]),
        ]))
    }
}
impl AntiInverse for Line {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn anti_inverse(self) -> Self {
        LineOnOrigin::from_groups(
            // e415, e425, e435
            -Simd32x3::powi(self.group0(), 3) - (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) - (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345];
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            -f32::powi(self[e415], 3) - (self[e425] * self[e425] * self[e415]) - (self[e435] * self[e435] * self[e415]) - (self[e12345] * self[e12345] * self[e415]),
            -f32::powi(self[e425], 3) - (self[e415] * self[e415] * self[e425]) - (self[e435] * self[e435] * self[e425]) - (self[e12345] * self[e12345] * self[e425]),
            -f32::powi(self[e435], 3) - (self[e415] * self[e415] * self[e435]) - (self[e425] * self[e425] * self[e435]) - (self[e12345] * self[e12345] * self[e435]),
            f32::powi(self[e12345], 3) + (self[e415] * self[e415] * self[e12345]) + (self[e425] * self[e425] * self[e12345]) + (self[e435] * self[e435] * self[e12345]),
        ]))
    }
}
impl AntiInverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       24        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       23       54        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e4] * self[e5])
            + 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + self[e12345] * self[e12345]
            + self[e45] * self[e45]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            + self[e4235] * self[e4235]
            + self[e4315] * self[e4315]
            + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - self[e321] * self[e321]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(other_g0) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group1(),
            // e5
            other_g0 * self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(other_g0 * -1.0) * self.group3(),
            // e15, e25, e35
            Simd32x3::from(other_g0 * -1.0) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other_g0 * -1.0) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other_g0 * -1.0) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other_g0 * -1.0) * self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(other_g0) * self.group9(),
            // e3215
            other_g0 * self[e3215],
        )
    }
}
impl AntiInverse for MysteryCircle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        MysteryCircle::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self[e321] * self[e321] * self[e415]) - f32::powi(self[e415], 3),
                (self[e321] * self[e321] * self[e425]) - f32::powi(self[e425], 3),
                (self[e321] * self[e321] * self[e435]) - f32::powi(self[e435], 3),
                f32::powi(self[e321], 3) - (self[e435] * self[e435] * self[e321]),
            ]) - (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                - (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4        6        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435] + self[e12345] * self[e12345] - self[e321] * self[e321];
        MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e12345
            other_g0 * self[e12345],
        )
    }
}
impl AntiInverse for MysteryDipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       12       12        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        MysteryDipole::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                f32::powi(self[e23], 3) - (self[e45] * self[e45] * self[e23]),
                f32::powi(self[e31], 3) - (self[e45] * self[e45] * self[e31]),
                f32::powi(self[e12], 3) - (self[e45] * self[e45] * self[e12]),
                (self[e12] * self[e12] * self[e45]) - f32::powi(self[e45], 3),
            ]) + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6        8        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12];
        MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e4235, e4315, e4125
            Simd32x3::from(other_g0) * self.group1(),
        )
    }
}
impl AntiInverse for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e321] * self[e321];
        MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(other_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] + self[e45] * self[e45]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12];
        MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from(other_g0) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
        )
    }
}
impl AntiInverse for Plane {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([f32::powi(self[e4235], 3), f32::powi(self[e4315], 3), f32::powi(self[e4125], 3), self[e4125] * self[e4125] * self[e3215]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        2        0
    // no simd        6        6        0
    fn anti_inverse(self) -> Self {
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::powi(self.group0(), 3) + (Simd32x3::powi(self.group0().yxx(), 2) * self.group0()) + (Simd32x3::powi(self.group0().zzy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e4] * self[e5]) - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other_g0) * self.group0(), /* e5 */ other_g0 * self[e5])
    }
}
impl AntiInverse for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        1
    // no simd        0        2        2
    fn anti_inverse(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(2.0) / self.group0().yx())
    }
}
impl AntiInverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        1
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -1.0 / self[scalar])
    }
}
impl AntiInverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3        7        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] - 2.0 * (self[e3215] * self[e1234]);
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other_g0) * self.group0(), /* e1234 */ other_g0 * self[e1234])
    }
}
impl AntiInverse for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        1
    // no simd        0        2        2
    fn anti_inverse(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(-2.0) / self.group0().yx())
    }
}
impl AntiInverse for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        8        9        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from([f32::powi(self[e4235], 3), f32::powi(self[e4315], 3), f32::powi(self[e4125], 3), self[e4125] * self[e4125] * self[e1234]])
                + (Simd32x4::powi(self.group0().yxxx(), 2) * self.group0())
                + (Simd32x4::powi(self.group0().zzyy(), 2) * self.group0()),
        )
    }
}
impl AntiInverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       33        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e5] * self[e4])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435]
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl AntiInverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       32        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235])
            + 2.0 * (self[e431] * self[e315])
            + 2.0 * (self[e412] * self[e125])
            + 2.0 * (self[e4] * self[e5])
            + self[e12345] * self[e12345]
            + self[e415] * self[e415]
            + self[e425] * self[e425]
            + self[e435] * self[e435];
        VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7       17        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3]
            - self[e321] * self[e321];
        VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(other_g0) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       24        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e4] * self[e5]);
        VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e12345] * self[e12345] + self[e415] * self[e415] + self[e425] * self[e425] + self[e435] * self[e435];
        VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
impl AntiInverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        9        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       25        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e423] * self[e235]) + 2.0 * (self[e431] * self[e315]) + 2.0 * (self[e412] * self[e125]) + 2.0 * (self[e5] * self[e4])
            - self[e321] * self[e321]
            - self[e1] * self[e1]
            - self[e2] * self[e2]
            - self[e3] * self[e3];
        VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other_g0 * -1.0) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other_g0) * self.group2(),
        )
    }
}
impl AntiInverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        9        0
    //    simd4        0        6        0
    // Totals...
    // yes simd       11       15        0
    //  no simd       11       33        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e1234] * self[e3215]);
        VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group3(),
        )
    }
}
impl AntiInverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7        5        0
    //  no simd        7       17        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] + self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125]
            - self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12];
        VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(other_g0 * -1.0) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other_g0) * self.group2(),
        )
    }
}
impl AntiInverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        8        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        7       14        0
    //  no simd        7       32        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = -self[scalar] * self[scalar]
            - self[e23] * self[e23]
            - self[e31] * self[e31]
            - self[e12] * self[e12]
            - 2.0 * (self[e41] * self[e15])
            - 2.0 * (self[e42] * self[e25])
            - 2.0 * (self[e43] * self[e35])
            - 2.0 * (self[e3215] * self[e1234]);
        VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(other_g0) * self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        )
    }
}
