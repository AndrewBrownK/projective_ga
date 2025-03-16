// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information 😁
//
// Total Implementations: 55
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
impl FlatBulk for AntiCircleRotor {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2().xyz())
    }
}
impl FlatBulk for AntiCircleRotorAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2().xyz())
    }
}
impl FlatBulk for AntiCircleRotorAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1().xyz())
    }
}
impl FlatBulk for AntiCircleRotorAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1().xyz())
    }
}
impl FlatBulk for AntiDipoleInversion {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl FlatBulk for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl FlatBulk for AntiDipoleInversionOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl FlatBulk for AntiFlatPoint {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0().xyz())
    }
}
impl FlatBulk for AntiFlector {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
    }
}
impl FlatBulk for AntiLine {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl FlatBulk for AntiMotor {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group1())
    }
}
impl FlatBulk for AntiPlane {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl FlatBulk for Circle {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2())
    }
}
impl FlatBulk for CircleAligningOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2())
    }
}
impl FlatBulk for CircleAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl FlatBulk for CircleAtOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl FlatBulk for CircleOrthogonalOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl FlatBulk for CircleRotor {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2().xyz())
    }
}
impl FlatBulk for CircleRotorAligningOrigin {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group2().xyz())
    }
}
impl FlatBulk for CircleRotorAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1().xyz())
    }
}
impl FlatBulk for CircleRotorAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1().xyz())
    }
}
impl FlatBulk for Dipole {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2())
    }
}
impl FlatBulk for DipoleAligningOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl FlatBulk for DipoleAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl FlatBulk for DipoleAtOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group1())
    }
}
impl FlatBulk for DipoleInversion {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for DipoleInversionAligningOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for DipoleInversionAtOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for DipoleInversionOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for DipoleOrthogonalOrigin {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group2())
    }
}
impl FlatBulk for FlatPoint {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0().xyz())
    }
}
impl FlatBulk for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for Flector {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for FlectorAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for Horizon {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for Infinity {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for Line {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group1())
    }
}
impl FlatBulk for LineAtInfinity {
    type Output = LineAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for Motor {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl FlatBulk for MotorAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        self
    }
}
impl FlatBulk for MultiVector {
    type Output = MultiVector;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            self[e3215],
        )
    }
}
impl FlatBulk for Plane {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl FlatBulk for RoundPoint {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl FlatBulk for RoundPointAtOrigin {
    type Output = Infinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Infinity::from_groups(/* e5 */ self[e5])
    }
}
impl FlatBulk for Sphere {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl FlatBulk for SphereAtOrigin {
    type Output = Horizon;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        Horizon::from_groups(/* e3215 */ self[e3215])
    }
}
impl FlatBulk for VersorEven {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl FlatBulk for VersorEvenAligningOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl FlatBulk for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group2())
    }
}
impl FlatBulk for VersorEvenAtOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl FlatBulk for VersorEvenOrthogonalOrigin {
    type Output = MotorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group1())
    }
}
impl FlatBulk for VersorOdd {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
impl FlatBulk for VersorOddOrthogonalOrigin {
    type Output = FlectorAtInfinity;
    fn flat_bulk(self) -> Self::Output {
        use crate::elements::*;
        FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
    }
}
