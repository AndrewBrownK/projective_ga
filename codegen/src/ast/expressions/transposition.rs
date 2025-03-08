
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

fn transpose_vec2_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Product
    let mut vec2_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            pulling_out_factor = vec2_product_extract(&mut vec2_product, &mut coalesce_product_literal, e0, f0, e1, f1);
            if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
        });
        if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
    });

    if vec2_product.is_empty() && coalesce_product_literal == [1.0; 2] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec2_product_extract(
    vec2_product: &mut Vec<(Vec2Expr, f32)>,
    coalesce_product_literals: &mut [f32; 2],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    
    // TODO if f0 == f1 in order to do anything anyway, then might be able to return early
    //  by testing it right here
    
    if e0 == e1 && f0 == f1 {
        vec2_product.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    match (e0, e1) {
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            // The swizzle will later be simplified, if applicable
            vec2_product.push((Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_product.push((Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_product.push((Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1)
        ) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_sum(v0, v1, a) else { return false };
            vec2_product.push((transposed, *f0));
            true
        }
        _ => false,
    }
}

fn transpose_vec2_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 2]
) -> Option<Vec2Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec2Expr::Sum
    let mut vec2_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            pulling_out_addend = vec2_sum_extract(&mut vec2_sum, &mut coalesce_sum_literal, e0, f0, e1, f1);
            if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
        });
        if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
    });

    if vec2_sum.is_empty() && coalesce_sum_literal == [0.0; 2] {
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec2_sum_extract(
    vec2_sum: &mut Vec<(Vec2Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 2],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut pulled_out_literal = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
            pulled_out_literal = true;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
            pulled_out_literal = true;
        }
    }
    if pulled_out_literal {
        return false;
    }
    if e0 == e1 && f0 == f1 {
        vec2_sum.push((Vec2Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    match (e0, e1) {
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            // The swizzle will later be simplified, if applicable
            vec2_sum.push((Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1)
        ) if v0 == v1 && f0 == f1 => {
            vec2_sum.push((Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1)
        ) if f0 == f1 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_product(v0, v1, a) else { return false };
            vec2_sum.push((transposed, *f0));
            true
        }
        _ => false,
    }
}

fn transpose_vec3_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Product
    let mut vec3_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                pulling_out_factor = vec3_product_extract(&mut vec3_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2);
                if let Literal(1.0) = e2 { true } else if let Literal(0.0) = e2 { true } else { !pulling_out_factor }
            });
            if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
        });
        if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
    });

    if vec3_product.is_empty() && coalesce_product_literal == [1.0; 3] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_2.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec3_product_extract(
    vec3_product: &mut Vec<(Vec3Expr, f32)>,
    coalesce_product_literals: &mut [f32; 3],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut e2_is_special_lit = false;
    if let Literal(f) = e0 {
        if *f == 0.0 {
            coalesce_product_literals[0] = 0.0;
            *f0 = 1.0;
        } else if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
        }
    }
    if let Literal(f) = e1 {
        if *f == 0.0 {
            coalesce_product_literals[1] = 0.0;
            *f1 = 1.0;
        } else if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
        }
    }
    if let Literal(f) = e2 {
        if *f == 0.0 {
            coalesce_product_literals[2] = 0.0;
            *f2 = 1.0;
            e2_is_special_lit = true;
        } else if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            e2_is_special_lit = true;
        }
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_product.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    e2.undo_flat_access();
    match (e0, e1, e2) {
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            // The swizzle will later be simplified, if applicable
            vec3_product.push((Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) => {
            vec3_product.push((Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) => {
            vec3_product.push((Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_product.push((Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2)
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_sum(v0, v1, v2, a) else { return false };
            vec3_product.push((transposed, *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_sum(v0, v1, a) else { return false };
            vec3_product.push((Vec3Expr::Extend2to3(transposed, z.clone()), *f0));
            true
        }
        _ => false,
    }
}

fn transpose_vec3_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 3],
) -> Option<Vec3Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec3Expr::Sum
    let mut vec3_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                pulling_out_addend = vec3_sum_extract(&mut vec3_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2);
                if let Literal(0.0) = e2 { true } else { !pulling_out_addend }
            });
            if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
        });
        if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
    });

    if vec3_sum.is_empty() && coalesce_sum_literal == [0.0; 3] {
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec3_sum_extract(
    vec3_sum: &mut Vec<(Vec3Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 3],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut e2_is_special_lit = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
        } else {
            *f2 = 0.0;
            e2_is_special_lit = true;
        }
    }
    if e0 == e1 && e1 == e2 && f0 == f1 && f1 == f2 {
        vec3_sum.push((Vec3Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    e2.undo_flat_access();
    match (e0, e1, e2) {
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && (f1 == f2 || e2_is_special_lit) => {
            // The swizzle will later be simplified, if applicable
            vec3_sum.push((Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) => {
            vec3_sum.push((Vec3Expr::Extend2to3(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z
        ) if v0 == v1 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2)
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 => {
            vec3_sum.push((Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2)
        ) if f0 == f1 && f1 == f2 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_product(v0, v1, v2, a) else { return false };
            vec3_sum.push((transposed, *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
        ) if f0 == f1 && (f1 == f2 || e2_is_special_lit) => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_product(v0, v1, a) else { return false };
            vec3_sum.push((Vec3Expr::Extend2to3(transposed, z.clone()), *f0));
            true
        }
        _ => false,
    }
}

fn transpose_vec4_product(
    float_product_0: &mut Vec<(FloatExpr, f32)>,
    float_product_1: &mut Vec<(FloatExpr, f32)>,
    float_product_2: &mut Vec<(FloatExpr, f32)>,
    float_product_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_product_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Product
    let mut vec4_product = vec![];
    float_product_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_factor = false;
        float_product_1.retain_mut(|(e1, f1)| {
            if pulling_out_factor {
                return true;
            }
            float_product_2.retain_mut(|(e2, f2)| {
                if pulling_out_factor {
                    return true;
                }
                float_product_3.retain_mut(|(e3, f3)| {
                    if pulling_out_factor {
                        return true;
                    }
                    pulling_out_factor = vec4_product_extract(&mut vec4_product, &mut coalesce_product_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                    if let Literal(1.0) = e3 { true } else if let Literal(0.0) = e3 { true } else { !pulling_out_factor }
                });
                if let Literal(1.0) = e2 { true } else if let Literal(0.0) = e2 { true } else { !pulling_out_factor }
            });
            if let Literal(1.0) = e1 { true } else if let Literal(0.0) = e1 { true } else { !pulling_out_factor }
        });
        if let Literal(1.0) = e0 { true } else if let Literal(0.0) = e0 { true } else { !pulling_out_factor }
    });

    if vec4_product.is_empty() && coalesce_product_literal == [1.0; 4] {
        // Revert to flat access, from the extraction-converted group access
        float_product_0.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_1.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_2.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
        float_product_3.iter_mut().for_each(|(e0, _)| { e0.redo_flat_access() });
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec4_product_extract(
    vec4_product: &mut Vec<(Vec4Expr, f32)>,
    coalesce_product_literals: &mut [f32; 4],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut e2_is_special_lit = false;
    let mut e3_is_special_lit = false;
    if let Literal(f) = e0 {
        if *f == 0.0 {
            coalesce_product_literals[0] = 0.0;
            *f0 = 1.0;
        } else if *f != 1.0 {
            coalesce_product_literals[0] *= f32::powf(*f, *f0);
            *f = 1.0;
            *f0 = 0.0;
        }
    }
    if let Literal(f) = e1 {
        if *f == 0.0 {
            coalesce_product_literals[1] = 0.0;
            *f1 = 1.0;
        } else if *f != 1.0 {
            coalesce_product_literals[1] *= f32::powf(*f, *f1);
            *f = 1.0;
            *f1 = 0.0;
        }
    }
    if let Literal(f) = e2 {
        if *f == 0.0 {
            coalesce_product_literals[2] = 0.0;
            *f2 = 1.0;
            e2_is_special_lit = true;
        } else if *f != 1.0 {
            coalesce_product_literals[2] *= f32::powf(*f, *f2);
            *f = 1.0;
            *f2 = 0.0;
            e2_is_special_lit = true;
        }
    }
    if let Literal(f) = e3 {
        if *f == 0.0 {
            coalesce_product_literals[3] = 0.0;
            *f3 = 1.0;
            e3_is_special_lit = true;
        } else if *f != 1.0 {
            coalesce_product_literals[3] *= f32::powf(*f, *f3);
            *f = 1.0;
            *f3 = 0.0;
            e3_is_special_lit = true;
        }
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_product.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    e2.undo_flat_access();
    e3.undo_flat_access();
    match (e0, e1, e2, e3) {
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3),
        ) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 => {
            // The swizzle will later be simplified, if applicable
            vec4_product.push((Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && (f2 == f3 || e3_is_special_lit) => {
            vec4_product.push((Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && (f2 == f3 || e3_is_special_lit)  => {
            vec4_product.push((Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && f1 == f2 && f2 == f3 => {
            vec4_product.push((Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            Sum(v3, a3)
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = transpose_vec4_sum(v0, v1, v2, v3, a) else { return false };
            vec4_product.push((transposed, *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            Sum(v2, a2),
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_sum(v0, v1, v2, a) else { return false };
            vec4_product.push((Vec4Expr::Extend3to4(transposed, w.clone()), *f0));
            true
        }
        (
            Sum(v0, a0),
            Sum(v1, a1),
            z,
            w
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_sum(v0, v1, a) else { return false };
            vec4_product.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), *f0));
            true
        }
        _ => false,
    }
}

fn transpose_vec4_sum(
    float_sum_0: &mut Vec<(FloatExpr, f32)>,
    float_sum_1: &mut Vec<(FloatExpr, f32)>,
    float_sum_2: &mut Vec<(FloatExpr, f32)>,
    float_sum_3: &mut Vec<(FloatExpr, f32)>,
    mut coalesce_sum_literal: [f32; 4],
) -> Option<Vec4Expr> {
    use crate::ast::expressions::FloatExpr::*;
    // See if we can pull out a Vec4Expr::Sum
    let mut vec4_sum = vec![];
    float_sum_0.retain_mut(|(e0, f0)| {
        let mut pulling_out_addend = false;
        float_sum_1.retain_mut(|(e1, f1)| {
            if pulling_out_addend {
                return true;
            }
            float_sum_2.retain_mut(|(e2, f2)| {
                if pulling_out_addend {
                    return true;
                }
                float_sum_3.retain_mut(|(e3, f3)| {
                    if pulling_out_addend {
                        return true;
                    }
                    pulling_out_addend = vec4_sum_extract(&mut vec4_sum, &mut coalesce_sum_literal, e0, f0, e1, f1, e2, f2, e3, f3);
                    if let Literal(0.0) = e3 { true } else { !pulling_out_addend }
                });
                if let Literal(0.0) = e2 { true } else { !pulling_out_addend }
            });
            if let Literal(0.0) = e1 { true } else { !pulling_out_addend }
        });
        if let Literal(0.0) = e0 { true } else { !pulling_out_addend }
    });

    if vec4_sum.is_empty() && coalesce_sum_literal == [0.0; 4] {
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
    result.simplify_nuanced(false, false);
    Some(result)
}

fn vec4_sum_extract(
    vec4_sum: &mut Vec<(Vec4Expr, f32)>,
    coalesce_sum_literals: &mut [f32; 4],
    e0: &mut FloatExpr,
    f0: &mut f32,
    e1: &mut FloatExpr,
    f1: &mut f32,
    e2: &mut FloatExpr,
    f2: &mut f32,
    e3: &mut FloatExpr,
    f3: &mut f32,
) -> bool {
    use crate::ast::expressions::FloatExpr::*;
    let mut e2_is_special_lit = false;
    let mut e3_is_special_lit = false;
    if let Literal(f) = e0 {
        if *f != 0.0 {
            coalesce_sum_literals[0] += *f * *f0;
            *f = 0.0;
            *f0 = 0.0;
        }
    }
    if let Literal(f) = e1 {
        if *f != 0.0 {
            coalesce_sum_literals[1] += *f * *f1;
            *f = 0.0;
            *f1 = 0.0;
        }
    }
    if let Literal(f) = e2 {
        if *f != 0.0 {
            coalesce_sum_literals[2] += *f * *f2;
            *f = 0.0;
            *f2 = 0.0;
        } else {
            *f2 = 0.0;
            e2_is_special_lit = true;
        }
    }
    if let Literal(f) = e3 {
        if *f != 0.0 {
            coalesce_sum_literals[3] += *f * *f3;
            *f = 0.0;
            *f3 = 0.0;
        } else {
            *f3 = 0.0;
            e3_is_special_lit = true;
        }
    }
    if e0 == e1 && e1 == e2 && e2 == e3 && f0 == f1 && f1 == f2 && f2 == f3 {
        vec4_sum.push((Vec4Expr::Gather1(e0.clone()), *f0));
        return true;
    }
    e0.undo_flat_access();
    e1.undo_flat_access();
    e2.undo_flat_access();
    e3.undo_flat_access();
    match (e0, e1, e2, e3) {
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            AccessVec4(box v3, i3)
        ) if v0 == v1 && v1 == v2 && v2 == v3 && f0 == f1 && f1 == f2 && f2 == f3 => {
            // The swizzle will later be simplified, if applicable
            vec4_sum.push((Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, *i3), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            AccessVec4(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && (f2 == f3 || e3_is_special_lit) => {
            vec4_sum.push((Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, *i2, 3))), w.clone()), *f0));
            true
        }
        (
            AccessVec4(box v0, i0),
            AccessVec4(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v0.clone(), *i0, *i1, 2, 3))), z.clone(), w.clone()), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            AccessVec3(box v2, i2),
            w
        ) if v0 == v1 && v1 == v2 && f0 == f1 && f1 == f2 && (f2 == f3 || e3_is_special_lit) => {
            vec4_sum.push((Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, *i2), w.clone()), *f0));
            true
        }
        (
            AccessVec3(box v0, i0),
            AccessVec3(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v0.clone(), *i0, *i1, 2))), z.clone(), w.clone()), *f0));
            true
        }
        (
            AccessVec2(box v0, i0),
            AccessVec2(box v1, i1),
            z,
            w
        ) if v0 == v1 && f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            vec4_sum.push((Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v0.clone(), *i0, *i1), z.clone(), w.clone()), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            Product(v3, a3)
        ) if f0 == f1 && f1 == f2 && f2 == f3 => {
            let a = [*a0, *a1, *a2, *a3];
            let Some(transposed) = transpose_vec4_product(v0, v1, v2, v3, a) else { return false };
            vec4_sum.push((transposed, *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            Product(v2, a2),
            w
        ) if f0 == f1 && f1 == f2 && (f2 == f3 || e3_is_special_lit) => {
            let a = [*a0, *a1, *a2];
            let Some(transposed) = transpose_vec3_product(v0, v1, v2, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend3to4(transposed, w.clone()), *f0));
            true
        }
        (
            Product(v0, a0),
            Product(v1, a1),
            z,
            w
        ) if f0 == f1 && (f1 == f2 || e2_is_special_lit) && (f2 == f3 || e3_is_special_lit) => {
            let a = [*a0, *a1];
            let Some(transposed) = transpose_vec2_product(v0, v1, a) else { return false };
            vec4_sum.push((Vec4Expr::Extend2to4(transposed, z.clone(), w.clone()), *f0));
            true
        }
        _ => false,
    }
}

