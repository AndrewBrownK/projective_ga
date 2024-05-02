//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::characteristics::Attitude;
use crate::norms::*;
use crate::products::contractions::WeightContraction;
use crate::products::exterior::Wedge;
use crate::products::projections::*;
use crate::unitize::Unitize;
use crate::*;

/// Euclidean distance between objects
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Euclidean_distance
/// distance(a,b) = bulk_norm(attitude(a wedge b)) + weight_norm(a wedge attitude(b))
/// where attitude(c) = c anti_wedge complement(e4) where e4 is the projective dimension
pub trait Distance<T> {
    type Output;
    fn distance(self, other: T) -> Self::Output;
}

/// The cosine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait CosineAngle<T> {
    type Output;
    fn cosine_angle(self, other: T) -> Self::Output;
}

/// The sine of the angle between two objects.
/// https://projectivegeometricalgebra.org/projgeomalg.pdf
pub trait SineAngle<T> {
    type Output;
    fn sine_angle(self, other: T) -> Self::Output;
}

impl CosineAngle<Circle> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleBulk> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: CircleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Dipole> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<LineAtOrigin> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for Circle {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Circle> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleBulk> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: CircleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Dipole> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for CircleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Circle> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleBulk> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: CircleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Dipole> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleBulk> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPoint> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for Dipole {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleBulk> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPoint> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for DipoleBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleBulk> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPoint> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPoint> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for FlatPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPoint> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Circle> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for Line {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Dipole> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for Line {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for Line {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for Line {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for Line {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<LineAtOrigin> for Line {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for Line {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for Line {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for Line {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for Line {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for Line {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Circle> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Dipole> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Circle> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<PlaneAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: PlaneAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<SpacialCurvature> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: SpacialCurvature) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Sphere> for Plane {
    type Output = f32;

    fn cosine_angle(self, other: Sphere) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Circle> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: PlaneAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Sphere> for PlaneAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: Sphere) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for RoundPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointBulk> for RoundPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for RoundPoint {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for RoundPointAtInfinity {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointAtInfinity {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for RoundPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for RoundPointBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for RoundPointCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Circle> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleBulk> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: CircleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<SpacialCurvature> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: SpacialCurvature) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Sphere> for SpacialCurvature {
    type Output = f32;

    fn cosine_angle(self, other: Sphere) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Circle> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: Circle) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleBulk> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: CircleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<CircleCarrierAspect> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: CircleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Dipole> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: Dipole) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleBulk> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: DipoleBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<DipoleCarrierAspect> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: DipoleCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPoint> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: FlatPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<FlatPointAtOrigin> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: FlatPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Line> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: Line) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<LineAtOrigin> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: LineAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<Plane> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: Plane) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<PlaneAtOrigin> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: PlaneAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<RoundPoint> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: RoundPoint) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtInfinity> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtInfinity) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointAtOrigin> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointAtOrigin) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointBulk> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointBulk) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<RoundPointCarrierAspect> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        self.unitize().weight_contraction(other.unitize()).bulk_norm().group0()
    }
}

impl CosineAngle<SpacialCurvature> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: SpacialCurvature) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl CosineAngle<Sphere> for Sphere {
    type Output = f32;

    fn cosine_angle(self, other: Sphere) -> f32 {
        self.unitize().weight_contraction(other.unitize()).group0()
    }
}

impl Distance<MultiVector> for Circle {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Circle {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for Circle {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for Circle {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for Circle {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for CircleBulk {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for CircleBulk {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for CircleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for CircleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for CircleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for CircleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for CircleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for CircleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for CircleWeight {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Dipole {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for Dipole {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for Dipole {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Dipole {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Dipole {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for Dipole {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for Dipole {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for Dipole {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for DipoleBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for DipoleWeight {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for DipoleWeight {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for DualNum {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for DualNum {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for DualNum {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for DualNum {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for DualNum {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for DualNum {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for DualNum {
    type Output = DualNum;

    fn distance(self, other: Flector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for DualNum {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for DualNum {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for DualNum {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for DualNum {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for DualNum {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SpacialCurvature> for DualNum {
    type Output = DualNum;

    fn distance(self, other: SpacialCurvature) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for DualNum {
    type Output = DualNum;

    fn distance(self, other: Sphere) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SphereWeight> for DualNum {
    type Output = DualNum;

    fn distance(self, other: SphereWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for FlatPoint {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for FlatPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for FlatPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Flector {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for FlectorAtInfinity {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Horizon {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Infinity {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Line {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtInfinity {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for LineAtOrigin {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Motor {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DualNum> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: DualNum) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<FlatPoint> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: FlatPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<FlatPointAtOrigin> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: FlatPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Flector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Line> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Line) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<LineAtOrigin> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: LineAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Motor> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Motor) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Plane> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Plane) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<PlaneAtOrigin> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: PlaneAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Rotor> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Rotor) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SpacialCurvature> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: SpacialCurvature) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Sphere) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SphereWeight> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: SphereWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Transflector> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Transflector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Translator> for MultiVector {
    type Output = DualNum;

    fn distance(self, other: Translator) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for Origin {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Origin {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DualNum> for Origin {
    type Output = DualNum;

    fn distance(self, other: DualNum) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Origin {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Plane {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for PlaneAtOrigin {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Rotor {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DualNum> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: DualNum) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for RoundPoint {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DualNum> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: DualNum) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DualNum> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: DualNum) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Circle> for Scalar {
    type Output = DualNum;

    fn distance(self, other: Circle) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleCarrierAspect> for Scalar {
    type Output = DualNum;

    fn distance(self, other: CircleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<CircleWeight> for Scalar {
    type Output = DualNum;

    fn distance(self, other: CircleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Dipole> for Scalar {
    type Output = DualNum;

    fn distance(self, other: Dipole) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleCarrierAspect> for Scalar {
    type Output = DualNum;

    fn distance(self, other: DipoleCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<DipoleWeight> for Scalar {
    type Output = DualNum;

    fn distance(self, other: DipoleWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Flector> for Scalar {
    type Output = DualNum;

    fn distance(self, other: Flector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Scalar {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Origin> for Scalar {
    type Output = DualNum;

    fn distance(self, other: Origin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPoint> for Scalar {
    type Output = DualNum;

    fn distance(self, other: RoundPoint) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointAtOrigin> for Scalar {
    type Output = DualNum;

    fn distance(self, other: RoundPointAtOrigin) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<RoundPointCarrierAspect> for Scalar {
    type Output = DualNum;

    fn distance(self, other: RoundPointCarrierAspect) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SpacialCurvature> for Scalar {
    type Output = DualNum;

    fn distance(self, other: SpacialCurvature) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<Sphere> for Scalar {
    type Output = DualNum;

    fn distance(self, other: Sphere) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<SphereWeight> for Scalar {
    type Output = DualNum;

    fn distance(self, other: SphereWeight) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for SpacialCurvature {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Sphere {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for SphereWeight {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Transflector {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl Distance<MultiVector> for Translator {
    type Output = DualNum;

    fn distance(self, other: MultiVector) -> DualNum {
        self.wedge(other).attitude().bulk_norm().add(self.wedge(other.attitude()).weight_norm())
    }
}

impl SineAngle<Circle> for Circle {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for Circle {
    type Output = f32;

    fn sine_angle(self, other: CircleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Circle {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Circle {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Circle {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Circle {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Circle {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Circle {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for Circle {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Circle {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Circle {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for Circle {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Circle {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for Circle {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for Circle {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: CircleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for CircleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: CircleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for CircleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for Dipole {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for DipoleBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for DipoleCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for FlatPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for FlatPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Line {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Line {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Line {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Line {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Line {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Line {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for Line {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Line {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Line {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for Line {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Line {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for Line {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for Line {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for LineAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Plane {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Plane {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Plane {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Plane {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: PlaneAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Plane {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for Plane {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Plane {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for Plane {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for Plane {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for Plane {
    type Output = f32;

    fn sine_angle(self, other: SpacialCurvature) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Plane {
    type Output = f32;

    fn sine_angle(self, other: Sphere) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: PlaneAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for PlaneAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: Sphere) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for RoundPoint {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointAtInfinity {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointAtInfinity {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointAtInfinity {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointAtInfinity {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for RoundPointAtInfinity {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for RoundPointAtOrigin {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for RoundPointBulk {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for RoundPointCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for RoundPointCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for RoundPointCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for RoundPointCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for RoundPointCarrierAspect {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: CircleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: SpacialCurvature) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for SpacialCurvature {
    type Output = f32;

    fn sine_angle(self, other: Sphere) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Circle> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: Circle) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleBulk> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: CircleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<CircleCarrierAspect> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: CircleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Dipole> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: Dipole) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleBulk> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: DipoleBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<DipoleCarrierAspect> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: DipoleCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPoint> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: FlatPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<FlatPointAtOrigin> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: FlatPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Line> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: Line) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<LineAtOrigin> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: LineAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Plane> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: Plane) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<PlaneAtOrigin> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: PlaneAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPoint> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: RoundPoint) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtInfinity> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtInfinity) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointAtOrigin> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: RoundPointAtOrigin) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointBulk> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: RoundPointBulk) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<RoundPointCarrierAspect> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: RoundPointCarrierAspect) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<SpacialCurvature> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: SpacialCurvature) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}

impl SineAngle<Sphere> for Sphere {
    type Output = f32;

    fn sine_angle(self, other: Sphere) -> f32 {
        let mut cos: f32 = self.cosine_angle(other);
        let mut cos_squared: f32 = cos * cos;
        let mut sub: f32 = 1.0 - cos_squared;
        sub.sqrt()
    }
}
