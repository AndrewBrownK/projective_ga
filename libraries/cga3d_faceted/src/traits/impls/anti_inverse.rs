// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 83
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0     N/A
//   Median:         3      10       0     N/A
//  Average:         4      11       0     N/A
//  Maximum:        23      49       2     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         0       0       0       0
//   Median:         4      17       0       0
//  Average:         5      18       0       0
//  Maximum:        23      70       2       4
impl AntiInverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       28        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       26        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       18        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       22        0      N/A
    //  no simd       10       36        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd        6       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd        6       31        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        0        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        2      N/A
    //  no simd        0        7        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(1.0 / self[e321]) * (Simd32x3::from(1.0 / self[e321]) * self.group0().xyz()).with_w(1.0),
        )
    }
}
impl AntiInverse for AntiDualNum {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        2        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        4        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(-1.0 / self[scalar]) * Simd32x2::from([self[e1234] / self[scalar], 1.0]))
    }
}
impl AntiInverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        0        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiFlatOrigin::from_groups(/* e321 */ 1.0 / self[e321])
    }
}
impl AntiInverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        0        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        2        2      N/A
    //  no simd        0        7        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(1.0 / self[e321]) * (Simd32x3::from(1.0 / self[e321]) * self.group0().xyz()).with_w(1.0),
        )
    }
}
impl AntiInverse for AntiFlector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
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
    //      add/sub      mul      div      pow
    // f32       12       24        0        4
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
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::powi(self.group0(), 3)
                + (Simd32x3::from([self[e31] * self[e31], self[e23] * self[e23], self[e23] * self[e23]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e12] * self[e12]).with_z(self[e31] * self[e31])),
        )
    }
}
impl AntiInverse for AntiMotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
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
    //      add/sub      mul      div      pow
    // f32       12       24        0        4
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
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       11        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e45] * self[e45] - self[e23] * self[e23] - self[e31] * self[e31] - self[e12] * self[e12] - self[scalar] * self[scalar];
        AntiMysteryCircleRotor::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(other_g0 * -1.0) * self.group0(), /* scalar */ other_g0 * self[scalar])
    }
}
impl AntiInverse for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       15        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiPlane::from_groups(
            // e1, e2, e3, e5
            -(Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                - (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl AntiInverse for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            -Simd32x3::powi(self.group0(), 3)
                - (Simd32x3::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_z(self[e2] * self[e2])),
        )
    }
}
impl AntiInverse for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        0        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ 1.0 / self[e12345])
    }
}
impl AntiInverse for AntiSphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            -(Simd32x4::from([self[e1] * self[e1], self[e2] * self[e2], self[e3] * self[e3], self[e1] * self[e1]]) * self.group0())
                - (Simd32x4::from([self[e2] * self[e2], self[e1] * self[e1], self[e1] * self[e1], self[e2] * self[e2]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e3] * self[e3]).with_zw(self[e2] * self[e2], self[e3] * self[e3])),
        )
    }
}
impl AntiInverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       23        0        0
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
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd        5       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       13        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        2       14        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       28        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       26        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        4        6        0        0
    //    simd4        0        3        0      N/A
    // Totals...
    // yes simd        4        9        0      N/A
    //  no simd        4       18        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       13        0        0
    //    simd3        0        2        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       16        0      N/A
    //  no simd        6       23        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3       11        0      N/A
    //  no simd        3       16        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       13        0        0
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
    //           add/sub      mul      div      pow
    //      f32        2        8        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2       10        0      N/A
    //  no simd        2       14        0        0
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
    //           add/sub      mul      div      pow
    //      f32       10       17        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       10       22        0      N/A
    //  no simd       10       36        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       29        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6        9        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        6       12        0      N/A
    //  no simd        6       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd        3       24        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
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
    //           add/sub      mul      div      pow
    //      f32        6       12        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        6       17        0      N/A
    //  no simd        6       31        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        8        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(-1.0 / self[e45]) * (Simd32x3::from(1.0 / self[e45]) * self.group0().xyz()).with_w(1.0),
        )
    }
}
impl AntiInverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        5       12        0        0
    //    simd3        0        3        0      N/A
    // Totals...
    // yes simd        5       15        0      N/A
    //  no simd        5       21        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd2        0        1        0      N/A
    // Totals...
    // yes simd        0        2        2      N/A
    //  no simd        0        3        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(1.0 / self[e12345]) * Simd32x2::from([self[e4] / self[e12345], 1.0]))
    }
}
impl AntiInverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ -1.0 / self[e45])
    }
}
impl AntiInverse for FlatPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        1        2        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        0        3        2      N/A
    //  no simd        0        8        2        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(-1.0 / self[e45]) * (Simd32x3::from(1.0 / self[e45]) * self.group0().xyz()).with_w(1.0),
        )
    }
}
impl AntiInverse for Flector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        5        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       13        0        0
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
    //      add/sub      mul      div      pow
    // f32       12       24        0        4
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
    //           add/sub      mul      div      pow
    //      f32        2        5        0        0
    //    simd3        0        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        2       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        LineOnOrigin::from_groups(
            // e415, e425, e435
            -Simd32x3::powi(self.group0(), 3)
                - (Simd32x3::from([self[e425] * self[e425], self[e415] * self[e415], self[e415] * self[e415]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e435] * self[e435]).with_z(self[e425] * self[e425])),
        )
    }
}
impl AntiInverse for Motor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
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
    //      add/sub      mul      div      pow
    // f32       12       24        0        4
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
    //           add/sub      mul      div      pow
    //      f32       23       40        0        0
    //    simd2        0        1        0      N/A
    //    simd3        0        4        0      N/A
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd       23       49        0      N/A
    //  no simd       23       70        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0       12        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3       16        0      N/A
    //  no simd       12       28        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        MysteryCircle::from_groups(
            // e415, e425, e435, e321
            (Simd32x4::from(self[e321] * self[e321]) * self.group0())
                - (Simd32x4::from([self[e415] * self[e415], self[e425] * self[e425], self[e435] * self[e435], self[e415] * self[e415]]) * self.group0())
                - (Simd32x4::from([self[e425] * self[e425], self[e415] * self[e415], self[e415] * self[e415], self[e425] * self[e425]]) * self.group0())
                - (self.group0() * Simd32x2::from(self[e435] * self[e435]).with_zw(self[e425] * self[e425], self[e435] * self[e435])),
        )
    }
}
impl AntiInverse for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        4        7        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        4        8        0      N/A
    //  no simd        4       11        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0       12        0        0
    //    simd4        3        4        0      N/A
    // Totals...
    // yes simd        3       16        0      N/A
    //  no simd       12       28        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        MysteryDipole::from_groups(
            // e23, e31, e12, e45
            (Simd32x4::from([self[e23] * self[e23], self[e31] * self[e31], self[e12] * self[e12], self[e23] * self[e23]]) * self.group0())
                + (Simd32x4::from([self[e31] * self[e31], self[e23] * self[e23], self[e23] * self[e23], self[e31] * self[e31]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e12] * self[e12]).with_zw(self[e31] * self[e31], self[e12] * self[e12]))
                - (Simd32x4::from(self[e45] * self[e45]) * self.group0()),
        )
    }
}
impl AntiInverse for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        6        8        0        0
    //    simd3        0        1        0      N/A
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        6       10        0      N/A
    //  no simd        6       15        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd        7       17        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd4        0        2        0      N/A
    // Totals...
    // yes simd        7       11        0      N/A
    //  no simd        7       17        0        0
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
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([self[e4235] * self[e4235], self[e4315] * self[e4315], self[e4125] * self[e4125], self[e4235] * self[e4235]]) * self.group0())
                + (Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125])),
        )
    }
}
impl AntiInverse for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0        5        0        3
    //    simd3        2        2        0      N/A
    // Totals...
    // yes simd        2        7        0      N/A
    //  no simd        6       11        0        3
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::powi(self.group0(), 3)
                + (Simd32x3::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_z(self[e4315] * self[e4315])),
        )
    }
}
impl AntiInverse for RoundPoint {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = 2.0 * (self[e4] * self[e5]) - self[e1] * self[e1] - self[e2] * self[e2] - self[e3] * self[e3];
        RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other_g0) * self.group0(), /* e5 */ other_g0 * self[e5])
    }
}
impl AntiInverse for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        1      N/A
    // no simd        0        2        2        0
    fn anti_inverse(self) -> Self {
        RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(2.0) / self.group0().yx())
    }
}
impl AntiInverse for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        0        1        1        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        Scalar::from_groups(/* scalar */ -1.0 / self[scalar])
    }
}
impl AntiInverse for Sphere {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        3        6        0        0
    //    simd4        0        1        0      N/A
    // Totals...
    // yes simd        3        7        0      N/A
    //  no simd        3       10        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        let other_g0 = self[e4235] * self[e4235] + self[e4315] * self[e4315] + self[e4125] * self[e4125] - 2.0 * (self[e3215] * self[e1234]);
        Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other_g0) * self.group0(), /* e1234 */ other_g0 * self[e1234])
    }
}
impl AntiInverse for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div      pow
    //   simd2        0        1        1      N/A
    // no simd        0        2        2        0
    fn anti_inverse(self) -> Self {
        SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(-2.0) / self.group0().yx())
    }
}
impl AntiInverse for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32        0       11        0        0
    //    simd4        2        3        0      N/A
    // Totals...
    // yes simd        2       14        0      N/A
    //  no simd        8       23        0        0
    fn anti_inverse(self) -> Self {
        use crate::elements::*;
        SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            (Simd32x4::from([self[e4235] * self[e4235], self[e4315] * self[e4315], self[e4125] * self[e4125], self[e4235] * self[e4235]]) * self.group0())
                + (Simd32x4::from([self[e4315] * self[e4315], self[e4235] * self[e4235], self[e4235] * self[e4235], self[e4315] * self[e4315]]) * self.group0())
                + (self.group0() * Simd32x2::from(self[e4125] * self[e4125]).with_zw(self[e4315] * self[e4315], self[e4125] * self[e4125])),
        )
    }
}
impl AntiInverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       11       41        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       36        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd        7       25        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        8        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3       12        0      N/A
    //  no simd        3       24        0        0
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
    //           add/sub      mul      div      pow
    //      f32        3        4        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        3        8        0      N/A
    //  no simd        3       20        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       13        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       17        0      N/A
    //  no simd        7       29        0        0
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
    //           add/sub      mul      div      pow
    //      f32       11       17        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd       11       23        0      N/A
    //  no simd       11       41        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7        9        0        0
    //    simd4        0        4        0      N/A
    // Totals...
    // yes simd        7       13        0      N/A
    //  no simd        7       25        0        0
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
    //           add/sub      mul      div      pow
    //      f32        7       12        0        0
    //    simd4        0        6        0      N/A
    // Totals...
    // yes simd        7       18        0      N/A
    //  no simd        7       36        0        0
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
