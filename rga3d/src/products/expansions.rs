//
// AUTO-GENERATED - DO NOT MODIFY
//
// To contribute to this file, see the adjacent codegen package.
// https://github.com/AndrewBrownK/projective_ga/
//

use crate::involutions::*;
use crate::products::exterior::*;
use crate::*;

/// Bulk Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}

/// Weight Expansion (Interior Product)
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait WeightExpansion<T> {
    type Output;
    fn weight_expansion(self, other: T) -> Self::Output;
}

impl BulkExpansion<Flector> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Flector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Flector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Flector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Flector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Flector {
    type Output = Motor;

    fn bulk_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Flector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for FlectorAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for FlectorAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for FlectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for FlectorAtInfinity {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for FlectorAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for FlectorAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for FlectorAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: MultiVector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Horizon {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Transflector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Line {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Line {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Line {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Line {
    type Output = Plane;

    fn bulk_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Line {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for LineAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for LineAtInfinity {
    type Output = Plane;

    fn bulk_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for LineAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Transflector) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Motor {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Motor {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Motor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Motor {
    type Output = Plane;

    fn bulk_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Motor {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Line) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Plane) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Point) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for MultiVector {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for MultiVector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Translator) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Line) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Plane) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Point) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: PointAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVector;

    fn bulk_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for MultiVectorAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Translator) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Flector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: Transflector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Origin {
    type Output = Rotor;

    fn bulk_expansion(self, other: Flector) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Origin {
    type Output = Rotor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Origin {
    type Output = Rotor;

    fn bulk_expansion(self, other: Transflector) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Horizon) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: MultiVector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Plane {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Transflector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Transflector) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Point {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Point {
    type Output = Motor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Point {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Point {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Point {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Point {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Point {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Point {
    type Output = Motor;

    fn bulk_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Point {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for PointAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for PointAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for PointAtInfinity {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn bulk_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for PointAtInfinity {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for PointAtInfinity {
    type Output = Motor;

    fn bulk_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Rotor {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Transflector) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Transflector {
    type Output = Motor;

    fn bulk_expansion(self, other: Flector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Transflector {
    type Output = Motor;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Transflector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Horizon) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Transflector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Transflector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: LineAtInfinity) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Transflector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Motor) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Transflector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Transflector {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Transflector {
    type Output = Rotor;

    fn bulk_expansion(self, other: Plane) -> Rotor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Point> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<PointAtInfinity> for Transflector {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: PointAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Transflector {
    type Output = Motor;

    fn bulk_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Transflector {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Translator) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Flector> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Flector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<FlectorAtInfinity> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: FlectorAtInfinity) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Horizon> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Horizon) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Line> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<LineAtInfinity> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: LineAtInfinity) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Motor> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Motor) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVector> for Translator {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<MultiVectorAtInfinity> for Translator {
    type Output = MultiVectorAtOrigin;

    fn bulk_expansion(self, other: MultiVectorAtInfinity) -> MultiVectorAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Plane> for Translator {
    type Output = PlaneAtOrigin;

    fn bulk_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Transflector> for Translator {
    type Output = Plane;

    fn bulk_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.dual())
    }
}

impl BulkExpansion<Translator> for Translator {
    type Output = AntiScalar;

    fn bulk_expansion(self, other: Translator) -> AntiScalar {
        self.wedge(other.dual())
    }
}

impl WeightExpansion<Flector> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Flector) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Flector {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Plane) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for Flector {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Flector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Flector {
    type Output = Motor;

    fn weight_expansion(self, other: Transflector) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Flector) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for FlectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for FlectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for FlectorAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Transflector) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: Motor) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: MultiVector) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Horizon {
    type Output = Horizon;

    fn weight_expansion(self, other: Rotor) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Line {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Line {
    type Output = Plane;

    fn weight_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Flector) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for LineAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Transflector) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for LineAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Transflector) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Flector) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Plane) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Motor {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Motor {
    type Output = Plane;

    fn weight_expansion(self, other: Transflector) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Flector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Line) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Origin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Plane) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Point) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for MultiVector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Transflector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Flector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Line) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for MultiVectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Origin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Plane) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for MultiVectorAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Point) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for MultiVectorAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Transflector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Flector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Line) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: LineAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for MultiVectorAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Plane) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for MultiVectorAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for MultiVectorAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Transflector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Origin {
    type Output = Rotor;

    fn weight_expansion(self, other: Flector) -> Rotor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Line) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: LineAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Origin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Origin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Origin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Plane) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for Origin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Origin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Origin {
    type Output = LineAtOrigin;

    fn weight_expansion(self, other: Transflector) -> LineAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Plane {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Plane {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Transflector) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Flector) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Plane) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: PlaneAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for PlaneAtOrigin {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Transflector) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Point {
    type Output = Motor;

    fn weight_expansion(self, other: Flector) -> Motor {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: Line) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Point {
    type Output = Plane;

    fn weight_expansion(self, other: LineAtOrigin) -> Plane {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Origin> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Origin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Plane) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Point {
    type Output = Line;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Point> for Point {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Point) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Point {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Point {
    type Output = Line;

    fn weight_expansion(self, other: Transflector) -> Line {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Flector) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for PointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for PointAtInfinity {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Plane) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: PlaneAtOrigin) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for PointAtInfinity {
    type Output = MultiVectorAtInfinity;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn weight_expansion(self, other: Transflector) -> LineAtInfinity {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Flector) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: Line) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn weight_expansion(self, other: LineAtOrigin) -> AntiScalar {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Motor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVector) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Plane) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Rotor {
    type Output = MultiVectorAtOrigin;

    fn weight_expansion(self, other: Rotor) -> MultiVectorAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Rotor {
    type Output = PlaneAtOrigin;

    fn weight_expansion(self, other: Transflector) -> PlaneAtOrigin {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: Flector) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Line> for Transflector {
    type Output = Horizon;

    fn weight_expansion(self, other: Line) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<LineAtOrigin> for Transflector {
    type Output = Horizon;

    fn weight_expansion(self, other: LineAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: Plane) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Transflector {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Transflector {
    type Output = Translator;

    fn weight_expansion(self, other: Transflector) -> Translator {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Flector> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Flector) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Motor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Motor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVector> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVector) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<MultiVectorAtOrigin> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: MultiVectorAtOrigin) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Plane> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Plane) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: PlaneAtOrigin) -> Horizon {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Rotor> for Translator {
    type Output = MultiVector;

    fn weight_expansion(self, other: Rotor) -> MultiVector {
        self.wedge(other.anti_dual())
    }
}

impl WeightExpansion<Transflector> for Translator {
    type Output = Horizon;

    fn weight_expansion(self, other: Transflector) -> Horizon {
        self.wedge(other.anti_dual())
    }
}
