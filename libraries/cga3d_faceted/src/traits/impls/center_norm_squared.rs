// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 25
//
// Yes SIMD:   add/sub     mul     div     pow
//  Minimum:         3       4       0     N/A
//   Median:         6       7       0     N/A
//  Average:         5       6       0     N/A
//  Maximum:        15      16       0     N/A
//
//  No SIMD:   add/sub     mul     div     pow
//  Minimum:         3       4       0       0
//   Median:         6       7       0       0
//  Average:         5       6       0       0
//  Maximum:        15      16       0       0
impl CenterNormSquared for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - sub_type_g0[0] * sub_type_g0[0]
                - sub_type_g0[1] * sub_type_g0[1]
                - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - sub_type_g0[0] * sub_type_g0[0]
                - sub_type_g0[1] * sub_type_g0[1]
                - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - sub_type_g0[0] * sub_type_g0[0]
                - sub_type_g0[1] * sub_type_g0[1]
                - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl CenterNormSquared for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl CenterNormSquared for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl CenterNormSquared for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl CenterNormSquared for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32       15       16        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g1_xyz = self.group1().xyz();
        let sub_type_g6_xyz = self.group6().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g1_xyz[0] * sub_type_g1_xyz[0]
                + sub_type_g1_xyz[1] * sub_type_g1_xyz[1]
                + sub_type_g1_xyz[2] * sub_type_g1_xyz[2]
                + self[scalar] * self[scalar]
                + self[e23] * self[e23]
                + self[e31] * self[e31]
                + self[e12] * self[e12]
                + self[e321] * self[e321]
                - sub_type_g6_xyz[0] * sub_type_g6_xyz[0]
                - sub_type_g6_xyz[1] * sub_type_g6_xyz[1]
                - sub_type_g6_xyz[2] * sub_type_g6_xyz[2]
                - self[e12345] * self[e12345]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl CenterNormSquared for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] - sub_type_g0[0] * sub_type_g0[0] - sub_type_g0[1] * sub_type_g0[1] - sub_type_g0[2] * sub_type_g0[2],
        )
    }
}
impl CenterNormSquared for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        4        5        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl CenterNormSquared for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        3        4        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2] - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        6        7        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0 = self.group0().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0[0] * sub_type_g0[0] + sub_type_g0[1] * sub_type_g0[1] + sub_type_g0[2] * sub_type_g0[2]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl CenterNormSquared for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl CenterNormSquared for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125]
                - self[e45] * self[e45],
        )
    }
}
impl CenterNormSquared for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[e321] * self[e321] + self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3]
                - self[e12345] * self[e12345]
                - self[e415] * self[e415]
                - self[e425] * self[e425]
                - self[e435] * self[e435],
        )
    }
}
impl CenterNormSquared for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            self[e1] * self[e1] + self[e2] * self[e2] + self[e3] * self[e3] + self[e321] * self[e321]
                - sub_type_g0_xyz[0] * sub_type_g0_xyz[0]
                - sub_type_g0_xyz[1] * sub_type_g0_xyz[1]
                - sub_type_g0_xyz[2] * sub_type_g0_xyz[2]
                - self[e12345] * self[e12345],
        )
    }
}
impl CenterNormSquared for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        Scalar::from_groups(
            // scalar
            self[scalar] * self[scalar] + self[e23] * self[e23] + self[e31] * self[e31] + self[e12] * self[e12]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
impl CenterNormSquared for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div      pow
    // f32        7        8        0        0
    fn center_norm_squared(self) -> Scalar {
        use crate::elements::*;
        let sub_type_g0_xyz = self.group1().xyz();
        Scalar::from_groups(
            // scalar
            sub_type_g0_xyz[0] * sub_type_g0_xyz[0] + sub_type_g0_xyz[1] * sub_type_g0_xyz[1] + sub_type_g0_xyz[2] * sub_type_g0_xyz[2] + self[scalar] * self[scalar]
                - self[e45] * self[e45]
                - self[e4235] * self[e4235]
                - self[e4315] * self[e4315]
                - self[e4125] * self[e4125],
        )
    }
}
