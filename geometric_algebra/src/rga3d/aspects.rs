
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;
use crate::rga3d::products::geometric::GeometricProduct;


/// The Bulk of an object usually describes the object's relationship with the origin.
/// An object with a Bulk of zero contains the origin.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Bulk {
    type Output;
    fn bulk(self) -> Self::Output;
}


/// The Weight of an object usually describes the object's attitude and orientation.
/// An object with zero weight is contained by the horizon.
/// Also known as the attitude operator.
/// http://rigidgeometricalgebra.org/wiki/index.php?title=Bulk_and_weight
pub trait Weight {
    type Output;
    fn weight(self) -> Self::Output;
}

impl Bulk for Flector {
    type Output = Flector;

    fn bulk(self) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl Bulk for Horizon {
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        self
    }
}

impl Bulk for Line {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Bulk for LineAtInfinity {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        self
    }
}

impl Bulk for Magnitude {
    type Output = Scalar;

    fn bulk(self) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * 1.0 } }
    }
}

impl Bulk for Motor {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Bulk for MultiVector {
    type Output = MultiVector;

    fn bulk(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from([0.0, 0.0, 0.0]), g3: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g4: Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl Bulk for Plane {
    type Output = Horizon;

    fn bulk(self) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * 1.0 } }
    }
}

impl Bulk for Point {
    type Output = PointAtInfinity;

    fn bulk(self) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Bulk for PointAtInfinity {
    type Output = PointAtInfinity;

    fn bulk(self) -> PointAtInfinity {
        self
    }
}

impl Bulk for Scalar {
    type Output = Scalar;

    fn bulk(self) -> Scalar {
        self
    }
}

impl Bulk for Translator {
    type Output = LineAtInfinity;

    fn bulk(self) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Weight for AntiScalar {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        self
    }
}

impl Weight for Flector {
    type Output = Flector;

    fn weight(self) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl Weight for Line {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Weight for LineAtOrigin {
    type Output = LineAtOrigin;

    fn weight(self) -> LineAtOrigin {
        self
    }
}

impl Weight for Magnitude {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * 1.0 } }
    }
}

impl Weight for Motor {
    type Output = Rotor;

    fn weight(self) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl Weight for MultiVector {
    type Output = MultiVector;

    fn weight(self) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[1]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]), g3: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from([0.0, 0.0, 0.0]), g4: Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl Weight for Origin {
    type Output = Origin;

    fn weight(self) -> Origin {
        self
    }
}

impl Weight for Plane {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from([1.0, 1.0, 1.0]) } }
    }
}

impl Weight for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight(self) -> PlaneAtOrigin {
        self
    }
}

impl Weight for Point {
    type Output = Origin;

    fn weight(self) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * 1.0 } }
    }
}

impl Weight for Rotor {
    type Output = Rotor;

    fn weight(self) -> Rotor {
        self
    }
}

impl Weight for Translator {
    type Output = AntiScalar;

    fn weight(self) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * 1.0 } }
    }
}

