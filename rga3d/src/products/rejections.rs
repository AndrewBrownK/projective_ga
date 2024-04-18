//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::contractions::*;
use crate::products::expansions::*;
use crate::products::exterior::AntiWedge;
use crate::products::exterior::Wedge;
use crate::*;

/// Orthogonal Rejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `ProjectOrthogonallyOnto`.
pub trait RejectOrthogonallyFrom<T> {
    type Output;
    fn reject_orthogonally_from(self, other: T) -> Self::Output;
}

/// Orthogonal AntiRejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `AntiProjectOrthogonallyOnto`.
pub trait AntiRejectOrthogonallyFrom<T> {
    type Output;
    fn anti_reject_orthogonally_from(self, other: T) -> Self::Output;
}

/// Central (from origin) Rejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `ProjectViaOriginOnto`.
pub trait RejectViaOriginFrom<T> {
    type Output;
    fn reject_via_origin_from(self, other: T) -> Self::Output;
}

/// Outward (from horizon) AntiRejection
/// Rejection and Projection are counterparts to one another.
/// This is the counterpart to `AntiProjectViaHorizonOnto`.
pub trait AntiRejectViaHorizonFrom<T> {
    type Output;
    fn anti_reject_via_horizon_from(self, other: T) -> Self::Output;
}

impl AntiRejectOrthogonallyFrom<Flector> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Flector {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Flector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Line {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Line {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Line {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for LineAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Motor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Plane) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Origin {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Origin {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Origin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Origin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Origin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Origin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Origin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Origin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Origin {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Origin {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Plane {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Plane {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for PlaneAtOrigin {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Point {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Point {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Line) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtInfinity> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Point {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Motor) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Point {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Point {
    type Output = FlectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> FlectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Translator> for Point {
    type Output = LineAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Flector> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Flector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<FlectorAtInfinity> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Line> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Line) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<LineAtOrigin> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: LineAtOrigin) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Magnitude> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Motor> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Motor) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Origin> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Origin) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Point> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Point) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<PointAtInfinity> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Rotor) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Scalar> for Transflector {
    type Output = Scalar;

    fn anti_reject_orthogonally_from(self, other: Scalar) -> Scalar {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = PointAtInfinity;

    fn anti_reject_orthogonally_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).weight_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Flector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Flector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for FlectorAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for FlectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for FlectorAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Horizon {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Horizon {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Horizon {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Line {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Line {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Line {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Line {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Line {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Line {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Line {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Line {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for LineAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for LineAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for LineAtInfinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for LineAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for LineAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Motor {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Motor {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Motor {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Motor {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Motor {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Motor {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Motor {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Point) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Horizon> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Point) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Plane {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Plane {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for Point {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Line) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Point {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for Point {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Point {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Point {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Point {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Point {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Point {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Point {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for Point {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Point {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Point {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Point {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Translator) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for PointAtInfinity {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for PointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Line) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for PointAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for PointAtInfinity {
    type Output = Line;

    fn anti_reject_via_horizon_from(self, other: Motor) -> Line {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for PointAtInfinity {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for PointAtInfinity {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> LineAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for PointAtInfinity {
    type Output = Scalar;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for PointAtInfinity {
    type Output = Transflector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Transflector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> LineAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Line> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtInfinity> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<LineAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Motor> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Point) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Rotor> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Rotor) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Transflector {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Transflector {
    type Output = Flector;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> Flector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Translator> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Translator) -> MultiVectorAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Flector> for Translator {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Flector) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<FlectorAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: FlectorAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Magnitude> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Magnitude) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtInfinity> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtOrigin;

    fn anti_reject_via_horizon_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Origin> for Translator {
    type Output = Origin;

    fn anti_reject_via_horizon_from(self, other: Origin) -> Origin {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Point> for Translator {
    type Output = Point;

    fn anti_reject_via_horizon_from(self, other: Point) -> Point {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: PointAtInfinity) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Scalar> for Translator {
    type Output = MultiVector;

    fn anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
        self.wedge(other).bulk_contraction(self)
    }
}

impl AntiRejectViaHorizonFrom<Transflector> for Translator {
    type Output = PointAtInfinity;

    fn anti_reject_via_horizon_from(self, other: Transflector) -> PointAtInfinity {
        self.wedge(other).bulk_contraction(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: AntiScalar) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Flector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Line) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Flector {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Magnitude) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Flector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Translator) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Line {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Line {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Line {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Line {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Line {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Line {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Flector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for LineAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for LineAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for LineAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for LineAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for LineAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Transflector) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Motor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Point> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PointAtInfinity> for Motor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Motor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Point> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for MultiVector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Point> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PointAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for MultiVectorAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Origin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Origin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Plane {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Plane {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Plane {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Plane {
    type Output = Line;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Plane {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Plane {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Plane {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Plane {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Plane {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Plane {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for PlaneAtOrigin {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for PlaneAtOrigin {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for PlaneAtOrigin {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for PlaneAtOrigin {
    type Output = Plane;

    fn reject_orthogonally_from(self, other: Plane) -> Plane {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for PlaneAtOrigin {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for PlaneAtOrigin {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Point {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Point {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Point {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Point {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Point {
    type Output = AntiScalar;

    fn reject_orthogonally_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: AntiScalar) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: Horizon) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Line) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Magnitude) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Motor) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Origin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Point> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Point) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PointAtInfinity> for Rotor {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: PointAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn reject_orthogonally_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Rotor {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Translator) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<AntiScalar> for Transflector {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: AntiScalar) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: FlectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Horizon> for Transflector {
    type Output = Horizon;

    fn reject_orthogonally_from(self, other: Horizon) -> Horizon {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Line> for Transflector {
    type Output = Line;

    fn reject_orthogonally_from(self, other: Line) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtInfinity> for Transflector {
    type Output = LineAtInfinity;

    fn reject_orthogonally_from(self, other: LineAtInfinity) -> LineAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<LineAtOrigin> for Transflector {
    type Output = Line;

    fn reject_orthogonally_from(self, other: LineAtOrigin) -> Line {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Magnitude> for Transflector {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Magnitude) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Motor> for Transflector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVectorAtInfinity;

    fn reject_orthogonally_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtInfinity {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Rotor> for Transflector {
    type Output = Motor;

    fn reject_orthogonally_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_orthogonally_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectOrthogonallyFrom<Translator> for Transflector {
    type Output = Translator;

    fn reject_orthogonally_from(self, other: Translator) -> Translator {
        self.anti_wedge(other).weight_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: AntiScalar) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Line) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Magnitude) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Flector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Flector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Translator) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: AntiScalar) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Line) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Magnitude) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for FlectorAtInfinity {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for FlectorAtInfinity {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Translator) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Horizon {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Horizon {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Horizon {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Horizon {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Horizon {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Horizon {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Motor) -> Rotor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Horizon {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Horizon {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Rotor) -> Rotor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Horizon {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Horizon {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Line {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Line {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Line {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Line {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for LineAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for LineAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Motor {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Point> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Motor {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Motor {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Origin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Origin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Point> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: AntiScalar) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Translator) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Plane {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Horizon) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Plane {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Plane {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> LineAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Plane {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Plane {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Motor) -> Rotor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Plane {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Rotor) -> Rotor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Plane {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Plane {
    type Output = Rotor;

    fn reject_via_origin_from(self, other: Translator) -> Rotor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Point {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Point {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Point {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Point {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Point {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Point {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Point {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for PointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for PointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Magnitude) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for PointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Motor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for PointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Rotor) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for PointAtInfinity {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: AntiScalar) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Flector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Horizon> for Transflector {
    type Output = Plane;

    fn reject_via_origin_from(self, other: Horizon) -> Plane {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Line) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Magnitude) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Motor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Plane) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Rotor) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Transflector {
    type Output = MultiVector;

    fn reject_via_origin_from(self, other: Transflector) -> MultiVector {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Transflector {
    type Output = Motor;

    fn reject_via_origin_from(self, other: Translator) -> Motor {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<AntiScalar> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: AntiScalar) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Flector> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Flector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<FlectorAtInfinity> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: FlectorAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Line> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Line) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtInfinity> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: LineAtInfinity) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<LineAtOrigin> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Magnitude> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Magnitude) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Motor> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Motor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVector> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtInfinity> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<MultiVectorAtOrigin> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Plane> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Plane) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PlaneAtOrigin> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Point> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Point) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<PointAtInfinity> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: PointAtInfinity) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Rotor> for Translator {
    type Output = MultiVectorAtOrigin;

    fn reject_via_origin_from(self, other: Rotor) -> MultiVectorAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Transflector> for Translator {
    type Output = PlaneAtOrigin;

    fn reject_via_origin_from(self, other: Transflector) -> PlaneAtOrigin {
        self.anti_wedge(other).bulk_expansion(self)
    }
}

impl RejectViaOriginFrom<Translator> for Translator {
    type Output = AntiScalar;

    fn reject_via_origin_from(self, other: Translator) -> AntiScalar {
        self.anti_wedge(other).bulk_expansion(self)
    }
}
