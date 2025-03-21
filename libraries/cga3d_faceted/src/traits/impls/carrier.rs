// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 77
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
impl Carrier for AntiCircleOnOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
impl Carrier for AntiCircleRotor {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[scalar]),
        )
    }
}
impl Carrier for AntiCircleRotorAligningOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().with_w(0.0),
            // e235, e315, e125, e5
            self.group1().with_w(self[scalar]),
        )
    }
}
impl Carrier for AntiCircleRotorAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().with_w(self[scalar]))
    }
}
impl Carrier for AntiCircleRotorAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[scalar]))
    }
}
impl Carrier for AntiCircleRotorOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
impl Carrier for AntiDipoleInversion {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]),
            // e4235, e4315, e4125, e3215
            self.group0().with_w(self[e321]),
        )
    }
}
impl Carrier for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group2().xyz().with_w(self[e321]))
    }
}
impl Carrier for AntiDipoleInversionOnOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group1().yzwx(), /* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl Carrier for AntiDipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl Carrier for AntiDipoleOnOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl Carrier for AntiDualNum {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x3::from(0.0).with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(self[scalar]),
        )
    }
}
impl Carrier for AntiFlatOrigin {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for AntiFlatPoint {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for AntiFlector {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]))
    }
}
impl Carrier for AntiFlectorOnOrigin {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzwx())
    }
}
impl Carrier for AntiLine {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl Carrier for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0())
    }
}
impl Carrier for AntiMotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0())
    }
}
impl Carrier for AntiMotorOnOrigin {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0())
    }
}
impl Carrier for AntiMysteryCircleRotor {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0().xyz().with_w(self[scalar]))
    }
}
impl Carrier for AntiMysteryDipoleInversion {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]))
    }
}
impl Carrier for AntiPlane {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz())
    }
}
impl Carrier for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    fn carrier(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0())
    }
}
impl Carrier for AntiSphereOnOrigin {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0())
    }
}
impl Carrier for AntiVersorEvenOnOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
impl Carrier for Circle {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().with_w(self[e321]))
    }
}
impl Carrier for CircleAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Carrier for CircleAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for CircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Carrier for CircleOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Carrier for CircleOrthogonalOrigin {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl Carrier for CircleRotor {
    type Output = Plane;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0().with_w(self[e321]))
    }
}
impl Carrier for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Carrier for CircleRotorAtInfinity {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0().xyz())
    }
}
impl Carrier for Dipole {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1().xyz())
    }
}
impl Carrier for DipoleAligningOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl Carrier for DipoleAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl Carrier for DipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl Carrier for DipoleInversion {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125, e12345 */ self.group1().xyz().with_w(self[e1234]))
    }
}
impl Carrier for DipoleInversionAligningOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl Carrier for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl Carrier for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl Carrier for DipoleInversionOnOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e1234]))
    }
}
impl Carrier for DipoleInversionOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        CircleRotorAligningOriginAtInfinity::from_groups(/* e415, e425, e435 */ self.group0().xyz(), /* e235, e315, e125, e12345 */ self.group1().with_w(self[e1234]))
    }
}
impl Carrier for DipoleOnOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz())
    }
}
impl Carrier for DipoleOrthogonalOrigin {
    type Output = Line;
    fn carrier(self) -> Self::Output {
        Line::from_groups(/* e415, e425, e435 */ self.group0(), /* e235, e315, e125 */ self.group1())
    }
}
impl Carrier for DualNum {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
    }
}
impl Carrier for MultiVector {
    type Output = MultiVector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[scalar],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).with_w(self[e4]),
            // e15, e25, e35
            self.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group3().xyz().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group5(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e423], self[e431], self[e412]]),
            // e3215
            self[e321],
        )
    }
}
impl Carrier for MysteryCircle {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for MysteryCircleRotor {
    type Output = Horizon;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e321])
    }
}
impl Carrier for MysteryDipole {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl Carrier for MysteryDipoleInversion {
    type Output = LineAtInfinity;
    fn carrier(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl Carrier for MysteryVersorEven {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzw().with_w(self[e321]))
    }
}
impl Carrier for MysteryVersorOdd {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[scalar]))
    }
}
impl Carrier for NullCircleAtOrigin {
    type Output = PlaneOnOrigin;
    fn carrier(self) -> Self::Output {
        PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0())
    }
}
impl Carrier for NullDipoleAtOrigin {
    type Output = LineOnOrigin;
    fn carrier(self) -> Self::Output {
        LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0())
    }
}
impl Carrier for NullDipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    fn carrier(self) -> Self::Output {
        MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0())
    }
}
impl Carrier for NullSphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl Carrier for NullVersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz())
    }
}
impl Carrier for Origin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
    }
}
impl Carrier for RoundPoint {
    type Output = FlatPoint;
    fn carrier(self) -> Self::Output {
        FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0())
    }
}
impl Carrier for RoundPointAtOrigin {
    type Output = FlatOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlatOrigin::from_groups(/* e45 */ self[e4])
    }
}
impl Carrier for Scalar {
    type Output = Infinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[scalar])
    }
}
impl Carrier for Sphere {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl Carrier for SphereAtOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl Carrier for SphereOnOrigin {
    type Output = AntiScalar;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        AntiScalar::from_groups(/* e12345 */ self[e1234])
    }
}
impl Carrier for VersorEven {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Flector::from_groups(
            // e15, e25, e35, e45
            self.group3(),
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e321]),
        )
    }
}
impl Carrier for VersorEvenAligningOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl Carrier for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0().yzw().with_w(self[e321]))
    }
}
impl Carrier for VersorEvenAtOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0().wxyz())
    }
}
impl Carrier for VersorEvenOnOrigin {
    type Output = FlectorOnOrigin;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e4], self[e423], self[e431], self[e412]]))
    }
}
impl Carrier for VersorEvenOrthogonalOrigin {
    type Output = Flector;
    fn carrier(self) -> Self::Output {
        Flector::from_groups(/* e15, e25, e35, e45 */ self.group2(), /* e4235, e4315, e4125, e3215 */ self.group0())
    }
}
impl Carrier for VersorOdd {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
impl Carrier for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1().xyz().with_w(self[scalar]))
    }
}
impl Carrier for VersorOddOrthogonalOrigin {
    type Output = Motor;
    fn carrier(self) -> Self::Output {
        use crate::elements::*;
        Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0().xyz().with_w(self[e1234]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]),
        )
    }
}
