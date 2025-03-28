
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

pub trait NotInsaneSignum {
    fn ni_signum(&self) -> i32;
}
impl NotInsaneSignum for f32 {
    fn ni_signum(&self) -> i32 {
        match *self {
            0.0 => 0,
            _ => self.signum() as i32
        }
    }
}

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

fn closest_to_zero(arr: &[f32]) -> f32 {
    arr.iter().copied().fold(arr[0], |acc, x| {
        if x.abs() < acc.abs() { x } else { acc }
    })
}


// TODO incorporate new swizzles

#[tracing::instrument(level = "trace", skip_all)]
fn vec2_product_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut vec2_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            float_product_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                vec2_product_extract(extraction_strength, &mut vec2_product, &mut coalesce_product_literal, e0, f0, e1, f1);
                if *f1 == 0.0 { *e1 = Literal(1.0); }
                e1.is_one_or_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(1.0); }
            e0.is_one_or_zero() || f0.abs() > 0.0
        });
    }

    if vec2_product.is_empty() && coalesce_product_literal == [1.0; 2] {
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec2_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
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
    // xy have compatible powers
    let xy = eqs!(x_power.ni_signum(), y_power.ni_signum());

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy { return; }

    // The final power we will use when extracting
    let power = closest_to_zero(&[*x_power, *y_power]);
    macro_rules! do_extract {
        ($v2:expr) => {
            let v2 = $v2;
            tracing::trace!("Extracting: {:?}", v2);
            vec2_product.push((v2, power));
            x_power.sub_assign(power);
            y_power.sub_assign(power);
            return;
        }
    }

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xy && eqs!(x, y) {
        do_extract!(Vec2Expr::Gather1(x.clone()));
    }
    tracing::trace!("attempting match on ({x:?}, {y:?})");
    match (x, y) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1)
        ) if extraction_strength >= WholeGroups && xy && eqs!(v0, v1) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if extraction_strength >= Swizzle && xy && eqs!(v0, v1) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            do_extract!(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            do_extract!(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))));
        }
        (
            Sum(v0, a0),
            Sum(v1, a1)
        ) if xy => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            do_extract!(transposed);
        }
        _ => {}
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec2_sum_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            float_sum_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                vec2_sum_extract(extraction_strength, &mut vec2_sum, &mut coalesce_sum_literal, e0, f0, e1, f1);
                if *f1 == 0.0 { *e1 = Literal(0.0); }
                e1.is_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(0.0); }
            e0.is_zero() || f0.abs() > 0.0
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec2_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
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
    // xy have compatible coefficients
    let xy = eqs!(x_coefficient.ni_signum(), y_coefficient.ni_signum());

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy { return; }

    // The final coefficient we'll use when extracting
    let coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient]);
    macro_rules! do_extract {
        ($v2:expr) => {
            let v2 = $v2;
            tracing::trace!("Extracting: {:?}", v2);
            vec2_sum.push((v2, coefficient));
            x_coefficient.sub_assign(coefficient);
            y_coefficient.sub_assign(coefficient);
            return;
        }
    }

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xy && eqs!(x, y) {
        do_extract!(Vec2Expr::Gather1(x.clone()));
    }
    tracing::trace!("attempting match on ({x:?}, {y:?})");
    match (x, y) {
        (
            AccessVec2(box v0, 0),
            AccessVec2(box v1, 1)
        ) if extraction_strength >= WholeGroups && xy && eqs!(v0, v1) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if extraction_strength >= Swizzle && xy && eqs!(v0, v1) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            do_extract!(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if extraction_strength >= TruncateAndExtend && xy && eqs!(v0, v1) => {
            do_extract!(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))));
        }
        (
            Product(v0, a0),
            Product(v1, a1)
        ) if xy => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            do_extract!(transposed);
        }
        _ => {}
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec3_product_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut vec3_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            float_product_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                float_product_2.retain_mut(|(e2, f2)| {
                    if *f0 == 0.0 || *f1 == 0.0 { return true; }
                    vec3_product_extract(extraction_strength, &mut vec3_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2);
                    if *f2 == 0.0 { *e2 = Literal(1.0); }
                    e2.is_one_or_zero() || f2.abs() > 0.0
                });
                if *f1 == 0.0 { *e1 = Literal(1.0); }
                e1.is_one_or_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(1.0); }
            e0.is_one_or_zero() || f0.abs() > 0.0
        });
    }

    if vec3_product.is_empty() && coalesce_product_literal == [1.0; 3] {
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec3_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
    use crate::ast::expressions::FloatExpr::*;
    let x_is_zero_or_one = if let Literal(f) = x {
        coalesce_product_literals[0] *= f32::powf(*f, *x_power);
        *f = 1.0;
        *x_power = 1.0;
        true
    } else { *x_power == 0.0 };
    let y_is_zero_or_one = if let Literal(f) = y {
        coalesce_product_literals[1] *= f32::powf(*f, *y_power);
        *f = 1.0;
        *y_power = 1.0;
        true
    } else { *y_power == 0.0 };
    let z_is_zero_or_one = if let Literal(f) = z {
        coalesce_product_literals[2] *= f32::powf(*f, *z_power);
        *f = 1.0;
        *z_power = 1.0;
        true
    } else { *z_power == 0.0 };
    let xy_is_zero_or_one = x_is_zero_or_one && y_is_zero_or_one;
    // let xy_is_zero = eqs!(0.0, coalesce_product_literals[0], coalesce_product_literals[1]);
    let z_is_zero = coalesce_product_literals[2] == 0.0;

    // Some critical match criteria that we can calculate up front.
    // xyz have compatible powers
    let xyz = eqs!(x_power.ni_signum(), y_power.ni_signum(), z_power.ni_signum());
    // xyz have compatible powers if broken up
    let xy_z = xyz || xy_is_zero_or_one || (eqs!(x_power.ni_signum(), y_power.ni_signum()) && z_is_zero_or_one);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_z { return; }

    // The final power we will use when extracting
    let xyz_power = closest_to_zero(&[*x_power, *y_power, *z_power]);
    let xy_power = closest_to_zero(&[*x_power, *y_power]);
    macro_rules! do_extract {
        ($v3:expr) => {
            let v3 = $v3;
            tracing::trace!("Extracting: {:?}", v3);
            if xyz {
                vec3_product.push((v3, xyz_power));
                x_power.sub_assign(xyz_power);
                y_power.sub_assign(xyz_power);
                z_power.sub_assign(xyz_power);
            } else if xy_z && z_is_zero_or_one {
                vec3_product.push((v3, xy_power));
                x_power.sub_assign(xy_power);
                y_power.sub_assign(xy_power);
            } else if xy_z && xy_is_zero_or_one {
                vec3_product.push((v3, *z_power));
                z_power.sub_assign(*z_power);
            } else {
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            }
            return;
        }
    }

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyz && eqs!(x, y, z) {
        do_extract!(Vec3Expr::Gather1(x.clone()));
    }
    if extraction_strength >= Gather1 && xy_z && eqs!(x, y) && z_is_zero {
        do_extract!(Vec3Expr::Gather1(x.clone()));
    }
    // TODO should we do this?
    // if extraction_strength >= Gather1 && xy_z && xy_is_zero {
    //     do_extract!(Vec3Expr::Gather1(z.clone()));
    // }
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?})");
    match (x, y, z) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2)
        ) if extraction_strength >= WholeGroups && xyz && eqs!(v0, v1, v2) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if extraction_strength >= Swizzle && xyz && eqs!(v0, v1, v2) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if extraction_strength >= TruncateAndExtend && xy_z && eqs!(v0, v1) => {
            do_extract!(Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()));
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if extraction_strength >= NaturalExtend && xy_z && eqs!(v0, v1) => {
            do_extract!(Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if extraction_strength >= TruncateAndExtend && xyz && eqs!(v0, v1, v2) => {
            do_extract!(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))));
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2)
        ) if xyz => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_sum_transpose(Some(extraction_strength), v0, v1, v2, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(1.0), 1.0)); *a2 = 0.0; }
            do_extract!(transposed);
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
        ) if extraction_strength >= TruncateAndExtend && xy_z => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            do_extract!(Vec3Expr::Extend2to3(transposed, z.clone()));
        }
        _ => {}
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec3_sum_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            float_sum_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                float_sum_2.retain_mut(|(e2, f2)| {
                    if *f0 == 0.0 || *f1 == 0.0 { return true; }
                    vec3_sum_extract(extraction_strength, &mut vec3_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2);
                    if *f2 == 0.0 { *e2 = Literal(0.0); }
                    e2.is_zero() || f2.abs() > 0.0
                });
                if *f1 == 0.0 { *e1 = Literal(0.0); }
                e1.is_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(0.0); }
            e0.is_zero() || f0.abs() > 0.0
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec3_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
    use crate::ast::expressions::FloatExpr::*;
    let x_is_zero = if let Literal(f) = x {
        coalesce_sum_literals[0] += *f * *x_coefficient;
        *f = 0.0;
        *x_coefficient = 0.0;
        true
    } else { *x_coefficient == 0.0 };
    let y_is_zero = if let Literal(f) = y {
        coalesce_sum_literals[1] += *f * *y_coefficient;
        *f = 0.0;
        *y_coefficient = 0.0;
        true
    } else { *y_coefficient == 0.0 };
    let z_is_zero = if let Literal(f) = z {
        coalesce_sum_literals[2] += *f * *z_coefficient;
        *f = 0.0;
        *z_coefficient = 0.0;
        true
    } else { *z_coefficient == 0.0 };
    let xy_is_zero = x_is_zero && y_is_zero;

    // Some critical match criteria that we can calculate up front.
    // xyz have compatible coefficients
    let xyz = eqs!(x_coefficient, y_coefficient, z_coefficient);
    // xyz have compatible coefficients if broken up
    let xy_z = xyz || xy_is_zero || (eqs!(x_coefficient, y_coefficient) && z_is_zero);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_z { return; }

    // The final coefficient we'll use when extracting
    let xyz_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient, *z_coefficient]);
    let xy_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient]);
    macro_rules! do_extract {
        ($v3:expr) => {
            let v3 = $v3;
            tracing::trace!("Extracting: {:?}", v3);
            if xyz {
                vec3_sum.push((v3, xyz_coefficient));
                x_coefficient.sub_assign(xyz_coefficient);
                y_coefficient.sub_assign(xyz_coefficient);
                z_coefficient.sub_assign(xyz_coefficient);
            } else if xy_z && z_is_zero {
                vec3_sum.push((v3, xy_coefficient));
                x_coefficient.sub_assign(xy_coefficient);
                y_coefficient.sub_assign(xy_coefficient);
            } else if xy_z && xy_is_zero {
                vec3_sum.push((v3, *z_coefficient));
                z_coefficient.sub_assign(*z_coefficient);
            } else {
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            }
            return;
        }
    }

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyz && eqs!(x, y, z) {
        do_extract!(Vec3Expr::Gather1(x.clone()));
    }
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?})");
    match (x, y, z) {
        (
            AccessVec3(box v0, 0),
            AccessVec3(box v1, 1),
            AccessVec3(box v2, 2),
        ) if extraction_strength >= WholeGroups && xyz && eqs!(v0, v1, v2) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if extraction_strength >= Swizzle && xyz && eqs!(v0, v1, v2) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if extraction_strength >= TruncateAndExtend && xy_z && eqs!(v0, v1) => {
            do_extract!(Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()));
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if extraction_strength >= NaturalExtend && xy_z && eqs!(v0, v1) => {
            do_extract!(Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
        ) if extraction_strength >= TruncateAndExtend && xyz && eqs!(v0, v1, v2) => {
            do_extract!(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))));
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
        ) if xyz => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_product_transpose(Some(extraction_strength), v0, v1, v2, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(0.0), 1.0)); *a2 = 0.0; }
            do_extract!(transposed);
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
        ) if extraction_strength >= TruncateAndExtend && xy_z => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            do_extract!(Vec3Expr::Extend2to3(transposed, z.clone()));
        }
        _ => {}
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec4_product_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    // TODO impl GeometricAntiQuotient<Plane> for AntiScalar
    //  (Simd32x4::from([
    //      other[e423] * other[e423] * self[e1234],
    //      other[e431] * other[e431] * self[e1234],
    //      other[e412] * other[e412] * self[e1234],
    //      other[e423] * other[e423] * self[e1234],
    //  ])

    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut vec4_product = vec![];
    for extraction_strength in ExtractionStrength::ASCENDING_STRENGTH.into_iter() {
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_product_0.retain_mut(|(e0, f0)| {
            float_product_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                float_product_2.retain_mut(|(e2, f2)| {
                    if *f0 == 0.0 || *f1 == 0.0 { return true; }
                    float_product_3.retain_mut(|(e3, f3)| {
                        if *f0 == 0.0 || *f1 == 0.0 || *f2 == 0.0 { return true; }
                        vec4_product_extract(extraction_strength, &mut vec4_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                        if *f3 == 0.0 { *e3 = Literal(1.0); }
                        e3.is_one_or_zero() || f3.abs() > 0.0
                    });
                    if *f2 == 0.0 { *e2 = Literal(1.0); }
                    e2.is_one_or_zero() || f2.abs() > 0.0
                });
                if *f1 == 0.0 { *e1 = Literal(1.0); }
                e1.is_one_or_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(1.0); }
            e0.is_one_or_zero() || f0.abs() > 0.0
        });
    }

    if vec4_product.is_empty() && coalesce_product_literal == [1.0; 4] {
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec4_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
    use crate::ast::expressions::FloatExpr::*;
    let x_is_zero_or_one = if let Literal(f) = x {
        coalesce_product_literals[0] *= f32::powf(*f, *x_power);
        *f = 1.0;
        *x_power = 1.0;
        true
    } else { *x_power == 0.0 };
    let y_is_zero_or_one = if let Literal(f) = y {
        coalesce_product_literals[1] *= f32::powf(*f, *y_power);
        *f = 1.0;
        *y_power = 1.0;
        true
    } else { *y_power == 0.0 };
    let z_is_zero_or_one = if let Literal(f) = z {
        coalesce_product_literals[2] *= f32::powf(*f, *z_power);
        *f = 1.0;
        *z_power = 1.0;
        true
    } else { *z_power == 0.0 };
    let w_is_zero_or_one = if let Literal(f) = w {
        coalesce_product_literals[3] *= f32::powf(*f, *w_power);
        *f = 1.0;
        *w_power = 1.0;
        true
    } else { *w_power == 0.0 };
    let xy_is_zero_or_one = x_is_zero_or_one && y_is_zero_or_one;
    let zw_is_zero_or_one = z_is_zero_or_one && w_is_zero_or_one;
    let xyz_is_zero_or_one = x_is_zero_or_one && y_is_zero_or_one && z_is_zero_or_one;
    let xy_is_zero = eqs!(0.0, coalesce_product_literals[0], coalesce_product_literals[1]);
    let z_is_zero = coalesce_product_literals[2] == 0.0;
    let w_is_zero = coalesce_product_literals[3] == 0.0;
    let xyz_is_zero = xy_is_zero && z_is_zero;

    // TODO what about leading Vec3Expr::Gather1(FloatExpr::Literal(0.0)) or Vec2Expr::Gather1(FloatExpr::Literal(0.0))?

    // Some critical match criteria that we can calculate up front.
    // xyzw have compatible powers
    let xyzw = eqs!(x_power.ni_signum(), y_power.ni_signum(), z_power.ni_signum(), w_power.ni_signum());
    // xyzw have compatible powers if broken up
    let xyz = eqs!(x_power.ni_signum(), y_power.ni_signum(), z_power.ni_signum());
    let xyz_w = xyzw || xyz_is_zero || (xyz && w_is_zero_or_one);
    // xyzw have compatible powers if broken up
    let xy = eqs!(x_power.ni_signum(), y_power.ni_signum());
    let xyw = eqs!(x_power.ni_signum(), y_power.ni_signum(), w_power.ni_signum());
    let xy_z = xyz || (xy && z_is_zero_or_one);
    let xy_w = xyw || (xy && w_is_zero_or_one);
    let zw = eqs!(z_power.ni_signum(), w_power.ni_signum()) || (z_is_zero_or_one ^ w_is_zero_or_one);
    let xy_zw = xyz_w || (xy_is_zero && zw) || (xy && xy_z && xy_w);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_zw { return; }

    // The final power we will use when extracting
    let xyzw_power = closest_to_zero(&[*x_power, *y_power, *z_power, *w_power]);
    let xyz_power = closest_to_zero(&[*x_power, *y_power, *z_power]);
    let xyw_power = closest_to_zero(&[*x_power, *y_power, *w_power]);
    let xy_power = closest_to_zero(&[*x_power, *y_power]);
    let zw_power = closest_to_zero(&[*z_power, *w_power]);
    macro_rules! do_extract {
        ($v4:expr) => {
            let v4 = $v4;
            tracing::trace!("Extracting: {:?}", v4);
            if xyzw {
                vec4_product.push((v4, xyzw_power));
                x_power.sub_assign(xyzw_power);
                y_power.sub_assign(xyzw_power);
                z_power.sub_assign(xyzw_power);
                w_power.sub_assign(xyzw_power);
            } else if xyz_w && w_is_zero_or_one {
                vec4_product.push((v4, xyz_power));
                x_power.sub_assign(xyz_power);
                y_power.sub_assign(xyz_power);
                z_power.sub_assign(xyz_power);
            } else if xyz_w && xyz_is_zero_or_one {
                vec4_product.push((v4, *w_power));
                w_power.sub_assign(*w_power);
            } else if xy_zw && zw_is_zero_or_one {
                vec4_product.push((v4, xy_power));
                x_power.sub_assign(xy_power);
                y_power.sub_assign(xy_power);
            } else if xy_zw && xy_is_zero_or_one && zw {
                vec4_product.push((v4, zw_power));
                z_power.sub_assign(zw_power);
                w_power.sub_assign(zw_power);
            } else if xy_zw && xyz && w_is_zero_or_one {
                // already covered: xyz_w && w_is_zero_or_one
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            } else if xy_zw && xyw && z_is_zero_or_one {
                vec4_product.push((v4, xyw_power));
                x_power.sub_assign(xyw_power);
                y_power.sub_assign(xyw_power);
                w_power.sub_assign(xyw_power);
            } else if xy_zw && xy_is_zero_or_one && z_is_zero_or_one {
                // already covered: xyz_w && xyz_is_zero_or_one
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            } else if xy_zw && xy_is_zero_or_one && w_is_zero_or_one {
                vec4_product.push((v4, *z_power));
                z_power.sub_assign(*z_power);
            } else {
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            }
            return;
        }
    }

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
        do_extract!(Vec4Expr::Gather1(x.clone()));
    }
    if extraction_strength >= Gather1 && xyz_w && eqs!(x, y, z) && w_is_zero {
        do_extract!(Vec4Expr::Gather1(x.clone()));
    }
    if extraction_strength >= Gather1 && xy_zw && eqs!(x, y) && z_is_zero && w_is_zero {
        do_extract!(Vec4Expr::Gather1(x.clone()));
    }
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?}, {w:?})");
    match (x, y, z, w) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if extraction_strength >= WholeGroups && xyzw && eqs!(v0, v1, v2, v3) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if extraction_strength >= Swizzle && xyzw && eqs!(v0, v1, v2, v3) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w && eqs!(v0, v1, v2) => {
            do_extract!(Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if extraction_strength >= NaturalExtend && xyz_w && eqs!(v0, v1, v2) => {
            do_extract!(Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()));
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if extraction_strength >= NaturalExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()));
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            Sum(v3, a3),
        ) if xyzw => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = vec4_sum_transpose(Some(extraction_strength), v0, v1, v2, v3, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(1.0), 1.0)); *a2 = 0.0; }
            if v3.is_empty() { v3.push((Literal(1.0), 1.0)); *a3 = 0.0; }
            do_extract!(transposed);
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_sum_transpose(Some(extraction_strength), v0, v1, v2, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(1.0), 1.0)); *a2 = 0.0; }
            do_extract!(Vec4Expr::Extend3to4(transposed, w.clone()));
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_sum_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(1.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(1.0), 1.0)); *a1 = 0.0; }
            do_extract!(Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()));
        }
        _ => {}
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn vec4_sum_transpose(
    max_extraction_strength: Option<ExtractionStrength>,
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
        if let Some(mes) = &max_extraction_strength {
            if &extraction_strength > mes {
                break
            }
        }
        tracing::trace!("attempting extraction at strength {extraction_strength:?}");
        float_sum_0.retain_mut(|(e0, f0)| {
            float_sum_1.retain_mut(|(e1, f1)| {
                if *f0 == 0.0 { return true; }
                float_sum_2.retain_mut(|(e2, f2)| {
                    if *f0 == 0.0 || *f1 == 0.0 { return true; }
                    float_sum_3.retain_mut(|(e3, f3)| {
                        if *f0 == 0.0 || *f1 == 0.0 || *f2 == 0.0 { return true; }
                        vec4_sum_extract(extraction_strength, &mut vec4_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                        if *f3 == 0.0 { *e3 = Literal(0.0); }
                        e3.is_zero() || f3.abs() > 0.0
                    });
                    if *f2 == 0.0 { *e2 = Literal(0.0); }
                    e2.is_zero() || f2.abs() > 0.0
                });
                if *f1 == 0.0 { *e1 = Literal(0.0); }
                e1.is_zero() || f1.abs() > 0.0
            });
            if *f0 == 0.0 { *e0 = Literal(0.0); }
            e0.is_zero() || f0.abs() > 0.0
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
    tracing::trace!("Transpose Result (before simplification):\n{:?}", DebugExpression::new(true, &result));
    result.vec4_simplify(false, false, false);
    tracing::trace!("Transpose Result (after simplification):\n{:?}", DebugExpression::new(true, &result));
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
) {
    use crate::ast::expressions::FloatExpr::*;
    let x_is_zero = if let Literal(f) = x {
        coalesce_sum_literals[0] += *f * *x_coefficient;
        *f = 0.0;
        *x_coefficient = 0.0;
        true
    } else { *x_coefficient == 0.0 };
    let y_is_zero = if let Literal(f) = y {
        coalesce_sum_literals[1] += *f * *y_coefficient;
        *f = 0.0;
        *y_coefficient = 0.0;
        true
    } else { *y_coefficient == 0.0 };
    let z_is_zero = if let Literal(f) = z {
        coalesce_sum_literals[2] += *f * *z_coefficient;
        *f = 0.0;
        *z_coefficient = 0.0;
        true
    } else { *z_coefficient == 0.0 };
    let w_is_zero = if let Literal(f) = w {
        coalesce_sum_literals[3] += *f * *w_coefficient;
        *f = 0.0;
        *w_coefficient = 0.0;
        true
    } else { *w_coefficient == 0.0 };
    let xy_is_zero = x_is_zero && y_is_zero;
    let xyz_is_zero = xy_is_zero && z_is_zero;
    // TODO use this
    let xyw_is_zero = xy_is_zero && w_is_zero;
    let zw_is_zero = z_is_zero && w_is_zero;

    // Some critical match criteria that we can calculate up front.
    // xyzw have compatible coefficients
    let xyzw = eqs!(x_coefficient.ni_signum(), y_coefficient.ni_signum(), z_coefficient.ni_signum(), w_coefficient.ni_signum());
    // xyzw have compatible coefficients if broken up
    let xyz = eqs!(x_coefficient.ni_signum(), y_coefficient.ni_signum(), z_coefficient.ni_signum());
    let xyz_w = xyzw || xyz_is_zero || (xyz && w_is_zero);
    // xyzw have compatible coefficients if broken up
    let xy = eqs!(x_coefficient.ni_signum(), y_coefficient.ni_signum());
    let xyw = eqs!(x_coefficient.ni_signum(), y_coefficient.ni_signum(), w_coefficient.ni_signum());
    let xy_z = xyz || (xy && z_is_zero);
    let xy_w = xyw || (xy && w_is_zero);
    let zw = eqs!(z_coefficient.ni_signum(), w_coefficient.ni_signum()) || (z_is_zero ^ w_is_zero);
    let xy_zw = xyz_w || (xy_is_zero && zw) || (xy && xy_z && xy_w);

    // Early return if no possible extractions
    // (Further uses of this variable are left intact to reinforce the requirement to the reader)
    if !xy_zw { return; }

    // The final coefficient we'll use when extracting
    let xyzw_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient, *z_coefficient, *w_coefficient]);
    let xyz_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient, *z_coefficient]);
    let xyw_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient, *w_coefficient]);
    let xy_coefficient = closest_to_zero(&[*x_coefficient, *y_coefficient]);
    let zw_coefficient = closest_to_zero(&[*z_coefficient, *w_coefficient]);
    macro_rules! do_extract {
        ($v4:expr) => {
            let v4 = $v4;
            tracing::trace!("Extracting: {:?}", v4);
            tracing::trace!(xyzw, xyz, xyz_w, xy, xyw, xy_z, xy_w, zw, xy_zw, x_is_zero, y_is_zero, z_is_zero, w_is_zero);
            if xyzw {
                vec4_sum.push((v4, xyzw_coefficient));
                x_coefficient.sub_assign(xyzw_coefficient);
                y_coefficient.sub_assign(xyzw_coefficient);
                z_coefficient.sub_assign(xyzw_coefficient);
                w_coefficient.sub_assign(xyzw_coefficient);
            } else if xyz_w && w_is_zero {
                vec4_sum.push((v4, xyz_coefficient));
                x_coefficient.sub_assign(xyz_coefficient);
                y_coefficient.sub_assign(xyz_coefficient);
                z_coefficient.sub_assign(xyz_coefficient);
            } else if xyz_w && xyz_is_zero {
                vec4_sum.push((v4, *w_coefficient));
                w_coefficient.sub_assign(*w_coefficient);
            } else if xy_zw && zw_is_zero {
                vec4_sum.push((v4, xy_coefficient));
                x_coefficient.sub_assign(xy_coefficient);
                y_coefficient.sub_assign(xy_coefficient);
            } else if xy_zw && xy_is_zero && zw {
                vec4_sum.push((v4, zw_coefficient));
                z_coefficient.sub_assign(zw_coefficient);
                w_coefficient.sub_assign(zw_coefficient);
            } else if xy_zw && xyz && w_is_zero {
                // already covered: xyz_w && w_is_zero
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            } else if xy_zw && xyw && z_is_zero {
                vec4_sum.push((v4, xyw_coefficient));
                x_coefficient.sub_assign(xyw_coefficient);
                y_coefficient.sub_assign(xyw_coefficient);
                w_coefficient.sub_assign(xyw_coefficient);
            } else if xy_zw && xy_is_zero && z_is_zero {
                // already covered: xyz_w && xyz_is_zero
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            } else if xy_zw && xy_is_zero && w_is_zero {
                vec4_sum.push((v4, *z_coefficient));
                z_coefficient.sub_assign(*z_coefficient);
            } else {
                panic!("Extraction logic is flawed - Failed to match extraction condition")
            }
            return;
        }
    }

    //
    // Begin extractions!
    //

    if extraction_strength >= Gather1 && xyzw && eqs!(x, y, z, w) {
        do_extract!(Vec4Expr::Gather1(x.clone()));
    }
    tracing::trace!("attempting match on ({x:?}, {y:?}, {z:?}, {w:?})");
    match (x, y, z, w) {
        (
            AccessVec4(box v0, 0),
            AccessVec4(box v1, 1),
            AccessVec4(box v2, 2),
            AccessVec4(box v3, 3),
        ) if extraction_strength >= WholeGroups && xyzw && eqs!(v0, v1, v2, v3) => {
            do_extract!(v0.clone());
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if extraction_strength >= Swizzle && xyzw && eqs!(v0, v1, v2, v3) => {
            // The swizzle will later be simplified, if applicable
            do_extract!(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w && eqs!(v0, v1, v2) => {
            do_extract!(Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()));
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if extraction_strength >= NaturalExtend && xyz_w && eqs!(v0, v1, v2) => {
            do_extract!(Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()));
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()));
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if extraction_strength >= NaturalExtend && xy_zw && eqs!(v0, v1) => {
            do_extract!(Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()));
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            Product(v3, a3),
        ) if xyzw => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = vec4_product_transpose(Some(extraction_strength), v0, v1, v2, v3, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(0.0), 1.0)); *a2 = 0.0; }
            if v3.is_empty() { v3.push((Literal(0.0), 1.0)); *a3 = 0.0; }
            do_extract!(transposed);
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            w
        ) if extraction_strength >= TruncateAndExtend && xyz_w => {
            // TODO maybe we should multiply these as by coalesce_sum_literals?
            let a = [*a0, *a1, *a2];
            let Some(transposed) = vec3_product_transpose(Some(extraction_strength), v0, v1, v2, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            if v2.is_empty() { v2.push((Literal(0.0), 1.0)); *a2 = 0.0; }
            do_extract!(Vec4Expr::Extend3to4(transposed, w.clone()));
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
            w
        ) if extraction_strength >= TruncateAndExtend && xy_zw => {
            let a = [*a0, *a1];
            let Some(transposed) = vec2_product_transpose(Some(extraction_strength), v0, v1, a) else { return; };
            if v0.is_empty() { v0.push((Literal(0.0), 1.0)); *a0 = 0.0; }
            if v1.is_empty() { v1.push((Literal(0.0), 1.0)); *a1 = 0.0; }
            do_extract!(Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()));
        }
        _ => {}
    }
}

