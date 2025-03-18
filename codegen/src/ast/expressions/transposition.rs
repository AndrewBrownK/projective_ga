
// TODO impl AntiProjectOrthogonallyOnto<Flector> for AntiScalar {
//  Old generation:
//  let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
//  Current generation:
//  let anti_wedge_g1_xyz = Simd32x3::from([self[e1234], self[e1234], other[e3]]) * other.group0().xy().with_z(self[e1234]);

use ExtractionStrength::WholeGroups;
use ExtractionStrength::Gather1;
use ExtractionStrength::Swizzle;
use ExtractionStrength::TruncateAndExtend;
use ExtractionStrength::NaturalExtend;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtractionStrength {
    WholeGroups = 0,
    Gather1 = 1,
    Swizzle = 2,
    NaturalExtend = 3,
    TruncateAndExtend = 4,
}
impl ExtractionStrength {
    pub const ASCENDING_STRENGTH: [ExtractionStrength; 5] = [
        WholeGroups,
        Gather1,
        Swizzle,
        NaturalExtend,
        TruncateAndExtend,
    ];
}

impl FloatExpr {
    fn undo_flat_access(&mut self) {
        if let FloatExpr::AccessMultiVecFlat(mve, flat_idx) = self {
            let mut flat_idx = *flat_idx;
            for (group_idx, group) in mve.mv_class.groups().into_iter().enumerate() {
                let group_width = group.simd_width();
                if flat_idx >= group_width {
                    flat_idx -= group_width;
                    continue
                }
                let mve = mve.take_as_owned();
                *self = match group {
                    BasisElementGroup::G1(_) => FloatExpr::AccessMultiVecGroup(mve, group_idx),
                    BasisElementGroup::G2(_, _) => FloatExpr::AccessVec2(Box::new(Vec2Expr::AccessMultiVecGroup(mve, group_idx)), flat_idx),
                    BasisElementGroup::G3(_, _, _) => FloatExpr::AccessVec3(Box::new(Vec3Expr::AccessMultiVecGroup(mve, group_idx)), flat_idx),
                    BasisElementGroup::G4(_, _, _, _) => FloatExpr::AccessVec4(Box::new(Vec4Expr::AccessMultiVecGroup(mve, group_idx)), flat_idx),
                };
                return
            }
        }
    }
    
    fn redo_flat_access(&mut self) {
        match self {
            FloatExpr::AccessVec2(box Vec2Expr::AccessMultiVecGroup(mve, target_group_idx), idx_in_vec) => {
                let mut flat_idx = 0;
                for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                    if scanning_group_idx == (*target_group_idx) {
                        *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                        return
                    }
                    flat_idx = flat_idx + g.simd_width();
                }
            }
            FloatExpr::AccessVec3(box Vec3Expr::AccessMultiVecGroup(mve, target_group_idx), idx_in_vec) => {
                let mut flat_idx = 0;
                for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                    if scanning_group_idx == (*target_group_idx) {
                        *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                        return
                    }
                    flat_idx = flat_idx + g.simd_width();
                }
            }
            FloatExpr::AccessVec4(box Vec4Expr::AccessMultiVecGroup(mve, target_group_idx), idx_in_vec) => {
                let mut flat_idx = 0;
                for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                    if scanning_group_idx == (*target_group_idx) {
                        *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                        return
                    }
                    flat_idx = flat_idx + g.simd_width();
                }
            }
            FloatExpr::AccessMultiVecGroup(mve, idx) => {
                let idx = *idx;
                let mv = mve.mv_class;
                let mut flat_idx = 0;
                for (i, g) in mv.groups().into_iter().enumerate() {
                    if i == idx {
                        *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx);
                        return
                    }
                    flat_idx = flat_idx + g.simd_width();
                }
            }
            _ => {}
        }
    }
}

// TODO incorporate new swizzles

#[tracing::instrument(level = "trace", skip_all)]
fn vec2_product_transpose(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut vec2_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_factor = false;
            float_product_1.retain_mut(|(e1, f1)| {
                if pulling_out_factor { return true; }
                pulling_out_factor = vec2_product_extract(extraction_strength, &mut vec2_product, &mut coalesce_product_literal, e0, f0, e1, f1);
                if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
            });
            if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
        });
    }

    if vec2_product.is_empty() && coalesce_product_literal == [1.0; 2] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_1.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec2_product.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::product(vec2_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec2_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec2_product_extract(
    extraction_strength: ExtractionStrength,
    vec2_product: &mut Vec<(Vec2Expr, f32)>,
    coalesce_product_literals: &mut [f32; 2],
    x: &mut FloatExpr,
    x_power: &mut f32,
    y: &mut FloatExpr,
    y_power: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    if let Literal(f) = x {
        coalesce_product_literals[0] *= f32::powf(*f, *x_power);
        *f = 1.0;
        *x_power = 1.0;
    }
    if let Literal(f) = y {
        coalesce_product_literals[1] *= f32::powf(*f, *y_power);
        *f = 1.0;
        *y_power = 1.0;
    }

    // Some critical match criteria that we can calculate up front.
    // xyzw all have the same power
    let xy = eqs!(x_power, y_power);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy { return false; }

    // The final power we will use when extracting
    let power = x_power.clone();

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xy && eqs!(x, y) {
        vec2_product.push((Vec2Expr::Gather1(x.clone()), power));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?})");
    match (x, y) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1)
        ) if extraction_strength >= WholeGroups && xy && eqs!(v0, v1) => {
            // The swizzle will later be simplified, if applicable
            vec2_product.push((v0.clone(), power));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if extraction_strength >= Swizzle && xy && eqs!(v0, v1) => {
            // The swizzle will later be simplified, if applicable
            vec2_product.push((Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), power));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            vec2_product.push((Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), power));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            vec2_product.push((Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1)
        ) if xy => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(v0, v1, a) else { return false };
            vec2_product.push((transposed, power));
            true
        }
        _ => false,
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec2_sum_transpose(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_addend = false;
            float_sum_1.retain_mut(|(e1, f1)| {
                if pulling_out_addend { return true; }
                pulling_out_addend = vec2_sum_extract(extraction_strength, &mut vec2_sum, &mut coalesce_sum_literal, e0, f0, e1, f1);
                if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
            });
            if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
        });
    }

    if vec2_sum.is_empty() && coalesce_sum_literal == [0.0; 2] {
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_1.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec2_sum.push((Vec2Expr::Gather2(p0, p1), 1.0));
    }
    let mut result = Vec2Expr::sum(vec2_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec2_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec2_sum_extract(
    extraction_strength: ExtractionStrength,
    vec2_sum: &mut Vec<(Vec2Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 2],
    x: &mut FloatExpr,
    x_coefficient: &mut f32,
    y: &mut FloatExpr,
    y_coefficient: &mut f32
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    if let Literal(f) = x {
        coalesce_sum_literals[0] += *f * *x_coefficient;
        *f = 0.0;
        *x_coefficient = 0.0;
    }
    if let Literal(f) = y {
        coalesce_sum_literals[1] += *f * *y_coefficient;
        *f = 0.0;
        *y_coefficient = 0.0;
    }

    // Some critical match criteria that we can calculate up front.
    // xy all have the same coefficient
    let xy = eqs!(x_coefficient, y_coefficient);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy { return false; }

    // The final coefficient we'll use when extracting
    let coefficient = x_coefficient.clone();

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xy && eqs!(x, y) {
        vec2_sum.push((Vec2Expr::Gather1(x.clone()), coefficient));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?})");
    match (x, y) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1)
        ) if extraction_strength >= WholeGroups && xy && eqs!(v0, v1) => {
            vec2_sum.push((v0.clone(), coefficient));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if extraction_strength >= Swizzle && xy && eqs!(v0, v1) => {
            // The swizzle will later be simplified, if applicable
            vec2_sum.push((Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), coefficient));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            vec2_sum.push((Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), coefficient));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            vec2_sum.push((Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1)
        ) if xy => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(v0, v1, a) else { return false };
            vec2_sum.push((transposed, coefficient));
            true
        }
        _ => false,
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec3_product_transpose(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut vec3_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_factor = false;
            float_product_1.retain_mut(|(e1, f1)| {
                if pulling_out_factor { return true; }
                float_product_2.retain_mut(|(e2, f2)| {
                    if pulling_out_factor { return true; }
                    pulling_out_factor = vec3_product_extract(extraction_strength, &mut vec3_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2);
                    if let Literal(1.0) = e2 { true } else if let Literal(0.0) = e2 { true } else { !pulling_out_factor }
                });
                if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
            });
            if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
        });
    }

    if vec3_product.is_empty() && coalesce_product_literal == [1.0; 3] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_2.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_2.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec3_product.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::product(vec3_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec3_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec3_product_extract(
    extraction_strength: ExtractionStrength,
    vec3_product: &mut Vec<(Vec3Expr, f32)>,
    coalesce_product_literals: &mut [f32; 3],
    x: &mut FloatExpr,
    x_power: &mut f32,
    y: &mut FloatExpr,
    y_power: &mut f32,
    z: &mut FloatExpr,
    z_power: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut z_is_zero_or_one = false;
    if let Literal(f) = x {
        coalesce_product_literals[0] *= f32::powf(*f, *x_power);
        *f = 1.0;
        *x_power = 1.0;
    }
    if let Literal(f) = y {
        coalesce_product_literals[1] *= f32::powf(*f, *y_power);
        *f = 1.0;
        *y_power = 1.0;
    }
    if let Literal(f) = z {
        coalesce_product_literals[2] *= f32::powf(*f, *z_power);
        *f = 1.0;
        *z_power = 1.0;
        z_is_zero_or_one = true;
    }
    let z_is_zero = coalesce_product_literals[2] == 0.0;

    // Some critical match criteria that we can calculate up front.
    // xyz all have the same power
    let xyz = eqs!(x_power, y_power, z_power);
    // z has the same power as xy, or is 0 or 1 so can accept any power
    let xy_z = eqs!(x_power, y_power) && (x_power == z_power || z_is_zero_or_one);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_z { return false; }

    // The final power we will use when extracting
    let power = x_power.clone();

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyz && eqs!(x, y, z) {
        vec3_product.push((Vec3Expr::Gather1(x.clone()), power));
        return true;
    }
    if extraction_strength >= Gather1 && xy_z && eqs!(x, y) && z_is_zero {
        vec3_product.push((Vec3Expr::Gather1(x.clone()), power));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    z.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?})");
    match (x, y, z) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2)
        ) if extraction_strength >= WholeGroups && xyz && eqs!(v0, v1, v2) => {
            vec3_product.push((v0.clone(), power));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if extraction_strength >= Swizzle && xyz && eqs!(v0, v1, v2) => {
            // The swizzle will later be simplified, if applicable
            vec3_product.push((Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), power));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if extraction_strength >= TruncateAndExtend && xy_z && eqs!(v0, v1) => {
            vec3_product.push((Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()), power));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if extraction_strength >= NaturalExtend && xy_z && eqs!(v0, v1) => {
            vec3_product.push((Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()), power));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if extraction_strength >= TruncateAndExtend && xyz && eqs!(v0, v1, v2) => {
            vec3_product.push((Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2)
        ) if xyz => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_sum_transpose(v0, v1, v2, a) else { return false };
            vec3_product.push((transposed, power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
        ) if extraction_strength >= TruncateAndExtend && xy_z => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(v0, v1, a) else { return false };
            vec3_product.push((Vec3Expr::Extend2to3(transposed, z.clone()), power));
            true
        }
        _ => false,
    }
}

// TODO some hideous changes in impl GeometricProduct<Motor> for MultiVector
//  Before:
//     // e41, e42, e43
//     (Simd32x3::from(self[scalar]) * other.group0().xyz())
//     + (Simd32x3::from(self[e1234]) * other.group1().xyz())
//     + (self.group2().xxy() * other.group1().wzx())
//     + (self.group2().zyz() * other.group1().yww())
//     + (self.group3().xxy() * other.group0().wzx())
//     + (self.group3().zyz() * other.group0().yww())
//     - (self.group2().yzx() * other.group1().zxy())
//     - (self.group3().yzx() * other.group0().zxy()),
//  After:
//     // e41, e42, e43
//     (Simd32x3::from(other[e1234]) * self.group3())
//     + (Simd32x3::from(other[scalar]) * self.group2())
//     + (other.group0().yzz() * self.group3().zx().with_z(self[scalar]))
//     + (other.group1().yzz() * self.group2().zx().with_z(self[e1234]))
//     + (Simd32x2::from(self[scalar]) * other.group0().xy()).with_z(other[e41] * self[e31])
//     + (Simd32x2::from(self[e1234]) * other.group1().xy()).with_z(other[e23] * self[e42])
//     - (self.group2().yzx() * other.group1().zxy())
//     - (self.group3().yzx() * other.group0().zxy()),


#[tracing::instrument(level = "trace", skip_all)]
fn vec3_sum_transpose(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_addend = false;
            float_sum_1.retain_mut(|(e1, f1)| {
                if pulling_out_addend { return true; }
                float_sum_2.retain_mut(|(e2, f2)| {
                    if pulling_out_addend { return true; }
                    pulling_out_addend = vec3_sum_extract(extraction_strength, &mut vec3_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2);
                    if let Literal(0.0) = e2 { true } else { !pulling_out_addend }
                });
                if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
            });
            if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
        });
    }

    if vec3_sum.is_empty() && coalesce_sum_literal == [0.0; 3] {
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_1.take_as_owned(), 0.0)
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_2.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec3_sum.push((Vec3Expr::Gather3(p0, p1, p2), 1.0));
    }
    let mut result = Vec3Expr::sum(vec3_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec3_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec3_sum_extract(
    extraction_strength: ExtractionStrength,
    vec3_sum: &mut Vec<(Vec3Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 3],
    x: &mut FloatExpr,
    x_coefficient: &mut f32,
    y: &mut FloatExpr,
    y_coefficient: &mut f32,
    z: &mut FloatExpr,
    z_coefficient: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut z_is_zero = false;
    if let Literal(f) = x {
        coalesce_sum_literals[0] += *f * *x_coefficient;
        *f = 0.0;
        *x_coefficient = 0.0;
    }
    if let Literal(f) = y {
        coalesce_sum_literals[1] += *f * *y_coefficient;
        *f = 0.0;
        *y_coefficient = 0.0;
    }
    if let Literal(f) = z {
        coalesce_sum_literals[2] += *f * *z_coefficient;
        *f = 0.0;
        *z_coefficient = 0.0;
        z_is_zero = true;
    }

    // Some critical match criteria that we can calculate up front.
    // xyz all have the same coefficient
    let xyz = eqs!(x_coefficient, y_coefficient, z_coefficient);
    // z has the same coefficient as xy, or is 0 so can accept any coefficient
    let xy_z = eqs!(x_coefficient, y_coefficient) && (x_coefficient == z_coefficient || z_is_zero);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_z { return false; }

    // The final coefficient we'll use when extracting
    let coefficient = x_coefficient.clone();

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyz && eqs!(x, y, z) {
        vec3_sum.push((Vec3Expr::Gather1(x.clone()), coefficient));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    z.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?})");
    match (x, y, z) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2),
        ) if extraction_strength >= WholeGroups && xyz && eqs!(v0, v1, v2) => {
            vec3_sum.push((v0.clone(), coefficient));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if extraction_strength >= Swizzle && xyz && eqs!(v0, v1, v2) => {
            // The swizzle will later be simplified, if applicable
            vec3_sum.push((Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), coefficient));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if extraction_strength >= TruncateAndExtend && xy_z && eqs!(v0, v1) => {
            vec3_sum.push((Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()), coefficient));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if extraction_strength >= NaturalExtend && xy_z && eqs!(v0, v1) => {
            vec3_sum.push((Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()), coefficient));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
        ) if extraction_strength >= TruncateAndExtend && xyz && eqs!(v0, v1, v2) => {
            vec3_sum.push((Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
        ) if xyz => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_product_transpose(v0, v1, v2, a) else { return false };
            vec3_sum.push((transposed, coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
        ) if extraction_strength >= TruncateAndExtend && xy_z => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(v0, v1, a) else { return false };
            vec3_sum.push((Vec3Expr::Extend2to3(transposed, z.clone()), coefficient));
            true
        }
        _ => false,
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec4_product_transpose(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut vec4_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_factor = false;
            float_product_1.retain_mut(|(e1, f1)| {
                if pulling_out_factor { return true; }
                float_product_2.retain_mut(|(e2, f2)| {
                    if pulling_out_factor { return true; }
                    float_product_3.retain_mut(|(e3, f3)| {
                        if pulling_out_factor { return true; }
                        pulling_out_factor = vec4_product_extract(extraction_strength, &mut vec4_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                        if let Literal(1.0) = e3 { true } else if let Literal(0.0) = e3 { true } else { !pulling_out_factor }
                    });
                    if let Literal(1.0) = e2 { true } else if let Literal(0.0) = e2 { true } else { !pulling_out_factor }
                });
                if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
            });
            if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
        });
    }

    if vec4_product.is_empty() && coalesce_product_literal == [1.0; 4] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_2.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_3.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_product_0.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_0.take_as_owned(), 1.0)
    };
    let p1 = if float_product_1.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_1.take_as_owned(), 1.0)
    };
    let p2 = if float_product_2.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_2.take_as_owned(), 1.0)
    };
    let p3 = if float_product_3.is_empty() {
        Literal(1.0)
    } else {
        keep_remaining = true;
        FloatExpr::product(float_product_3.take_as_owned(), 1.0)
    };
    if keep_remaining {
        vec4_product.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::product(vec4_product, coalesce_product_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec4_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec4_product_extract(
    extraction_strength: ExtractionStrength,
    vec4_product: &mut Vec<(Vec4Expr, f32)>,
    coalesce_product_literals: &mut [f32; 4],
    x: &mut FloatExpr,
    x_power: &mut f32,
    y: &mut FloatExpr,
    y_power: &mut f32,
    z: &mut FloatExpr,
    z_power: &mut f32,
    w: &mut FloatExpr,
    w_power: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut z_is_zero_or_one = false;
    let mut w_is_zero_or_one = false;
    if let Literal(f) = x {
        coalesce_product_literals[0] *= f32::powf(*f, *x_power);
        *f = 1.0;
        *x_power = 1.0;
    }
    if let Literal(f) = y {
        coalesce_product_literals[1] *= f32::powf(*f, *y_power);
        *f = 1.0;
        *y_power = 1.0;
    }
    if let Literal(f) = z {
        coalesce_product_literals[2] *= f32::powf(*f, *z_power);
        *f = 1.0;
        *z_power = 1.0;
        z_is_zero_or_one = true;
    }
    if let Literal(f) = w {
        coalesce_product_literals[3] *= f32::powf(*f, *w_power);
        *f = 1.0;
        *w_power = 1.0;
        w_is_zero_or_one = true;
    }
    let z_is_zero = coalesce_product_literals[2] == 0.0;
    let w_is_zero = coalesce_product_literals[3] == 0.0;

    // TODO what about leading Vec3Expr::Gather1(FloatExpr::Literal(0.0)) or Vec2Expr::Gather1(FloatExpr::Literal(0.0))?

    // Some critical match criteria that we can calculate up front.
    // xyzw all have the same power
    let xyzw = eqs!(x_power, y_power, z_power, w_power);
    // w has the same power as xyz, or is 0 or 1 so can accept any power
    let xyz_w = eqs!(x_power, y_power, z_power) && (x_power == w_power || w_is_zero_or_one);
    // zw has the same power as xy, or is 0 or 1 so can accept any power
    let xy_zw = eqs!(x_power, y_power) && (x_power == z_power || z_is_zero_or_one) && (x_power == w_power || w_is_zero_or_one);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_zw { return false; }

    // The final power we will use when extracting
    let power = x_power.clone();

    // TODO impl AntiProjectOrthogonallyOnto<Line> for Motor {
    //  // e41, e42, e43, e1234
    //  Before:
    //  (Simd32x3::from(anti_wedge_g1_w) * other.group0())
    //       .with_w(-(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
    //  After:
    //  (Simd32x2::from(anti_wedge_g1_w) * other.group0().xy()).with_zw(
    //      anti_wedge_g1_w * other[e43],
    //      -(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]),
    //  ),

    // TODO we really need a simd dot product thing
    //  impl AntiProjectViaHorizonOnto<Plane> for Flector {
    //  Before:
    //  Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0())
    //  After:
    //  Plane::from_groups(
    //      // e423, e431, e412, e321
    //      Simd32x4::from(self[e321]) * Simd32x4::from([other[e423] * other[e321], other[e431] * other[e321], other[e412] * other[e321], other[e321] * other[e321]]),
    //  )

    // TODO impl AntiProjectViaHorizonOnto<Point> for Horizon {
    //  Simd32x4::from([
    //      other[e4],
    //      other[e4],
    //      other[e4],
    //      -(anti_wedge_g1[0] * other[e1]) - (anti_wedge_g1[1] * other[e2]) - (anti_wedge_g1[2] * other[e3]),
    //  ]) * anti_wedge_g1.with_w(1.0),

    // TODO impl AntiProjectViaHorizonOnto<Line> for Motor {
    //  // e41, e42, e43, e1234
    //  Before:
    //  (Simd32x3::from(anti_wedge_g1_w) * other.group0())
    //      .with_w(-(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12])),
    //  After:
    //  (Simd32x2::from(anti_wedge_g1_w) * other.group0().xy()).with_zw(
    //      anti_wedge_g1_w * other[e43],
    //      -(anti_wedge_g0_xyz[0] * other[e23]) - (anti_wedge_g0_xyz[1] * other[e31]) - (anti_wedge_g0_xyz[2] * other[e12]),
    //  ),

    // TODO impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    //  let anti_wedge_g1 = Simd32x4::from([
    //      self[e321],
    //      self[e321],
    //      self[e321],
    //      -(right_dual_g0[0] * self[e423]) - (right_dual_g0[1] * self[e431]) - (right_dual_g0[2] * self[e412]),
    //  ]) * right_dual_g0.with_w(1.0);

    // TODO impl Sandwich<Flector> for DualNum {
    //  Simd32x4::from([
    //      self[scalar],
    //      self[scalar],
    //      self[scalar],
    //      (geometric_product_g1_w * self[e1234]) + (geometric_product_g0[3] * self[scalar]),
    //  ]) * geometric_product_g0.xyz().with_w(1.0),

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyzw && eqs!(x, y, z, w) {
        vec4_product.push((Vec4Expr::Gather1(x.clone()), power));
        return true;
    }
    if extraction_strength >= Gather1 && xyz_w && eqs!(x, y, z) && w_is_zero {
        vec4_product.push((Vec4Expr::Gather1(x.clone()), power));
        return true;
    }
    if extraction_strength >= Gather1 && xy_zw && eqs!(x, y) && z_is_zero && w_is_zero {
        vec4_product.push((Vec4Expr::Gather1(x.clone()), power));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    z.undo_flat_access();
    w.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?}, {w:?})");
    match (x, y, z, w) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if extraction_strength >= WholeGroups && xyzw && eqs!(v0, v1, v2, v3) => {
            vec4_product.push((v0.clone(), power));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if extraction_strength >= Swizzle && xyzw && eqs!(v0, v1, v2, v3) => {
            // The swizzle will later be simplified, if applicable
            vec4_product.push((Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3), power));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w && eqs!(v0, v1, v2) => {
            vec4_product.push((Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()), power));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()), power));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if extraction_strength >= NaturalExtend && xyz_w && eqs!(v0, v1, v2) => {
            vec4_product.push((Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()), power));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()), power));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if extraction_strength >= NaturalExtend && xy_zw && eqs!(v0, v1) => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()), power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            Sum(v3, a3),
        ) if xyzw => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = vec4_sum_transpose(v0, v1, v2, v3, a) else { return false };
            vec4_product.push((transposed, power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_sum_transpose(v0, v1, v2, a) else { return false };
            vec4_product.push((Vec4Expr::Extend3to4(transposed, w.clone()), power));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(v0, v1, a) else { return false };
            vec4_product.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), power));
            true
        }
        _ => false,
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec4_sum_transpose(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    float_sum_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Sum
    let mut vec4_sum = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            let mut pulling_out_addend = false;
            float_sum_1.retain_mut(|(e1, f1)| {
                if pulling_out_addend { return true; }
                float_sum_2.retain_mut(|(e2, f2)| {
                    if pulling_out_addend { return true; }
                    float_sum_3.retain_mut(|(e3, f3)| {
                        if pulling_out_addend { return true; }
                        pulling_out_addend = vec4_sum_extract(extraction_strength, &mut vec4_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                        if let Literal(0.0) = e3 { true } else { !pulling_out_addend }
                    });
                    if let Literal(0.0) = e2 { true } else { !pulling_out_addend }
                });
                if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
            });
            if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
        });
    }

    if vec4_sum.is_empty() && coalesce_sum_literal == [0.0; 4] {
        tracing::trace!("no extractions");
        return None;
    }
    let mut keep_remaining = false;
    let p0 = if float_sum_0.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_0.take_as_owned(), 0.0)
    };
    let p1 = if float_sum_1.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_1.take_as_owned(), 0.0)
    };
    let p2 = if float_sum_2.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_2.take_as_owned(), 0.0)
    };
    let p3 = if float_sum_3.is_empty() {
        Literal(0.0)
    } else {
        keep_remaining = true;
        FloatExpr::sum(float_sum_3.take_as_owned(), 0.0)
    };
    if keep_remaining {
        vec4_sum.push((Vec4Expr::Gather4(p0, p1, p2, p3), 1.0));
    }
    let mut result = Vec4Expr::sum(vec4_sum, coalesce_sum_literal);

    // Since this was a non-trivial transposition of structures,
    // run simplification again on the result.
    result.vec4_simplify(false, false);
    tracing::trace!("Transpose result: {result:?}");
    Some(result)
}

#[tracing::instrument(level = "trace", skip_all, fields(extraction_strength))]
fn vec4_sum_extract(
    extraction_strength: ExtractionStrength,
    vec4_sum: &mut Vec<(Vec4Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 4],
    x: &mut FloatExpr,
    x_coefficient: &mut f32,
    y: &mut FloatExpr,
    y_coefficient: &mut f32,
    z: &mut FloatExpr,
    z_coefficient: &mut f32,
    w: &mut FloatExpr,
    w_coefficient: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut z_is_zero = false;
    let mut w_is_zero = false;
    if let Literal(f) = x {
        coalesce_sum_literals[0] += *f * *x_coefficient;
        *f = 0.0;
        *x_coefficient = 0.0;
    }
    if let Literal(f) = y {
        coalesce_sum_literals[1] += *f * *y_coefficient;
        *f = 0.0;
        *y_coefficient = 0.0;
    }
    if let Literal(f) = z {
        coalesce_sum_literals[2] += *f * *z_coefficient;
        *f = 0.0;
        *z_coefficient = 0.0;
        z_is_zero = true;
    }
    if let Literal(f) = w {
        coalesce_sum_literals[3] += *f * *w_coefficient;
        *f = 0.0;
        *w_coefficient = 0.0;
        w_is_zero = true;
    }

    // Some critical match criteria that we can calculate up front.
    // xyzw all have the same coefficient
    let xyzw = eqs!(x_coefficient, y_coefficient, z_coefficient, w_coefficient);
    // w has the same coefficient as xyz, or is 0 so can accept any coefficient
    let xyz_w = eqs!(x_coefficient, y_coefficient, z_coefficient) && (x_coefficient == w_coefficient || w_is_zero);
    // zw has the same coefficient as xy, or is 0 so can accept any coefficient
    let xy_zw = eqs!(x_coefficient, y_coefficient) && (x_coefficient == z_coefficient || z_is_zero) && (x_coefficient == w_coefficient || w_is_zero);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_zw { return false; }

    // The final coefficient we'll use when extracting
    let coefficient = x_coefficient.clone();

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyzw && eqs!(x, y, z, w) {
        vec4_sum.push((Vec4Expr::Gather1(x.clone()), coefficient));
        return true;
    }
    x.undo_flat_access();
    y.undo_flat_access();
    z.undo_flat_access();
    w.undo_flat_access();
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?}, {w:?})");
    match (x, y, z, w) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if extraction_strength >= WholeGroups && xyzw && eqs!(v0, v1, v2, v3) => {
            vec4_sum.push((v0.clone(), coefficient));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if extraction_strength >= Swizzle && xyzw && eqs!(v0, v1, v2, v3) => {
            // The swizzle will later be simplified, if applicable
            vec4_sum.push((Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3), coefficient));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w && eqs!(v0, v1, v2) => {
            vec4_sum.push((Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()), coefficient));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()), coefficient));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if extraction_strength >= NaturalExtend && xyz_w && eqs!(v0, v1, v2) => {
            vec4_sum.push((Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()), coefficient));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()), coefficient));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if extraction_strength >= NaturalExtend && xy_zw && eqs!(v0, v1) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()), coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            Product(v3, a3),
        ) if xyzw => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = vec4_product_transpose(v0, v1, v2, v3, a) else { return false };
            vec4_sum.push((transposed, coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_product_transpose(v0, v1, v2, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend3to4(transposed, w.clone()), coefficient));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(v0, v1, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), coefficient));
            true
        }
        _ => false,
    }
}

