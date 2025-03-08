use std::sync::atomic::Ordering::Acquire;

trait SortVecDespiteF32 {
    fn sort_with_f32(&mut self);
}
impl<Expr: Ord> SortVecDespiteF32 for Vec<(Expr, f32)> {
    fn sort_with_f32(&mut self) {
        self.sort_by(|(a_expr, a_f32), (b_expr, b_f32)| {
            // Sort higher coefficients/exponents (additions and multiplications) first,
            // then lower coefficients/exponents (subtractions and divisions) last
            FloatOrd(*a_f32).cmp(&FloatOrd(*b_f32))
                .reverse()
                // Then deterministically sort the expressions themselves
                .then_with(|| a_expr.cmp(b_expr))
        });
    }
}

impl AnyExpression {
    pub(crate) fn final_simplify(&mut self) {

        // TODO impl AntiInverse for FlatPoint
        //  Simd32x4::from(1.0 / (self[e45] * self[e45])) * Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45] * -1.0]),
        //  if my suspicions are correct, this occurs because
        //  of the two-phase final simplification, where simd can't be transposed after
        //  we convert to flat access. When the multi-line simplification routine in
        //  traits.rs TraitImplBuilder.into_trait_xx() inlines more expressions, it might
        //  create more situations where we want transposition (like above), but without
        //  the ability to do transposition anymore (because of flat access). The solution
        //  to allow transposition with flat access. This is SUPER FUCKING TEDIOUS but on the
        //  bright side, I'll be able to eliminate the "double simplifies" described below,
        //  and simplification will be a one-and-done operation again.

        // First do the transposing simplify.
        // Then do the flat access conversion.
        // Can't do both at once because flat access interferes with transposition,
        // and simplification takes place depth-first.
        //
        // That is the ONLY reason we are consecutive-simplifying though.
        // The INTENTION behind simplification methods is (in the non-nuanced case), you only
        // have to invoke it once, and it will simplify as much as possible. The programmer
        // shouldn't have to second guess and try layering up consecutive simplifications just
        // to get the basic job done.
        match self {
            AnyExpression::Int(_) => {}
            AnyExpression::Float(e) => {
                e.simplify_nuanced(false, true, false);
                e.simplify_nuanced(false, false, true);
            },
            AnyExpression::Vec2(e) => {
                e.simplify_nuanced(false, true, false);
                e.simplify_nuanced(false, false, true);
            },
            AnyExpression::Vec3(e) => {
                e.simplify_nuanced(false, true, false);
                e.simplify_nuanced(false, false, true);
            },
            AnyExpression::Vec4(e) => {
                e.simplify_nuanced(false, true, false);
                e.simplify_nuanced(false, false, true);
            },
            AnyExpression::Class(e) => {
                e.simplify_nuanced(false, true, false);
                e.simplify_nuanced(false, false, true);
            },
        }
    }
}

impl IntExpr {
    #[allow(unused)]
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }
    #[allow(unused)]
    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            IntExpr::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Int(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            IntExpr::Literal(_) => {}
            IntExpr::TraitInvoke10ToInt(_t, owner) => {
                // Owner is a type, not an expression
                // if !insides_already_done {
                //     owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                // }
            }
        }
    }
}

impl FloatExpr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }

    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            FloatExpr::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Float(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            FloatExpr::Literal(_) => {}
            FloatExpr::FromInt(a) => {
                if !insides_already_done {
                    a.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match a {
                    IntExpr::Variable(_) => {}
                    IntExpr::Literal(i) => {
                        *self = FloatExpr::Literal(*i as f32);
                        return
                    }
                    IntExpr::TraitInvoke10ToInt(_, _) => {}
                }
            },
            FloatExpr::AccessVec2(av2, idx_in_vec) => {
                if !insides_already_done {
                    av2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match av2.as_mut() {
                    Vec2Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec2Expr::Gather2(fe0, fe1) => {
                        *self = [fe0, fe1][*idx_in_vec as usize].take_as_owned();
                    }
                    Vec2Expr::Truncate3to2(box v) => {
                        *self = FloatExpr::access_vec_3(v.take_as_owned(), *idx_in_vec as usize);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec2Expr::Truncate4to2(box v) => {
                        *self = FloatExpr::access_vec_4(v.take_as_owned(), *idx_in_vec as usize);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec2Expr::AccessMultiVecGroup(mve, target_group_idx) if prefer_flat_access => {
                        let mut flat_idx = 0u16;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx as usize) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + (*idx_in_vec as u16));
                                return
                            }
                            flat_idx = flat_idx + (g.simd_width() as u16);
                        }
                    }
                    Vec2Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_2(factor.take_as_owned(), *idx_in_vec as usize), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec2Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_2(addend.take_as_owned(), *idx_in_vec as usize), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec3(av3, idx_in_vec) => {
                if !insides_already_done {
                    av3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match av3.as_mut() {
                    Vec3Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec3Expr::Gather3(fe0, fe1, fe2) => {
                        *self = [fe0, fe1, fe2][*idx_in_vec as usize].take_as_owned();
                    }
                    Vec3Expr::Truncate4to3(box v) => {
                        *self = FloatExpr::access_vec_4(v.take_as_owned(), *idx_in_vec as usize);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec3Expr::Extend2to3(xy, z) => {
                        match *idx_in_vec {
                            0 | 1 => {
                                *self = FloatExpr::access_vec_2(xy.take_as_owned(), *idx_in_vec as usize);
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            2 => {
                                *self = z.take_as_owned();
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec3Expr::AccessMultiVecGroup(mve, target_group_idx) if prefer_flat_access => {
                        let mut flat_idx = 0u16;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx as usize) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + (*idx_in_vec as u16));
                                return
                            }
                            flat_idx = flat_idx + (g.simd_width() as u16);
                        }
                    }
                    Vec3Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_3(factor.take_as_owned(), *idx_in_vec as usize), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec3Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_3(addend.take_as_owned(), *idx_in_vec as usize), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec4(av4, idx_in_vec) => {
                if !insides_already_done {
                    av4.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match av4.as_mut() {
                    Vec4Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec4Expr::Gather4(fe0, fe1, fe2, fe3) => {
                        *self = [fe0, fe1, fe2, fe3][*idx_in_vec as usize].take_as_owned();
                    }
                    Vec4Expr::Extend2to4(xy, z, w) => {
                        match *idx_in_vec {
                            0 | 1 => {
                                *self = FloatExpr::access_vec_2(xy.take_as_owned(), *idx_in_vec as usize);
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            2 => {
                                *self = z.take_as_owned();
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            3 => {
                                *self = w.take_as_owned();
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec4Expr::Extend3to4(xyz, w) => {
                        match *idx_in_vec {
                            0 | 1 | 2 => {
                                *self = FloatExpr::access_vec_3(xyz.take_as_owned(), *idx_in_vec as usize);
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            3 => {
                                *self = w.take_as_owned();
                                self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec4Expr::AccessMultiVecGroup(mve, target_group_idx) if prefer_flat_access => {
                        let mut flat_idx = 0u16;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx as usize) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + (*idx_in_vec as u16));
                                return
                            }
                            flat_idx = flat_idx + (g.simd_width() as u16);
                        }
                    }
                    Vec4Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_4(factor.take_as_owned(), *idx_in_vec as usize), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    Vec4Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_4(addend.take_as_owned(), *idx_in_vec as usize), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec as usize]);
                        self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessMultiVecGroup(mve, idx) => {
                if !insides_already_done {
                    mve.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(f) => {
                            *self = f.take_as_owned();
                            1
                        }
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 1 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 1. {mv}"
                        )
                    }
                    return
                }

                if prefer_flat_access {
                    let mut flat_idx = 0u16;
                    for (i, g) in mv.groups().into_iter().enumerate() {
                        if i == (idx as usize) {
                            *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx);
                            return
                        }
                        flat_idx = flat_idx + (g.simd_width() as u16);
                    }
                }
            }
            FloatExpr::AccessMultiVecFlat(mve, idx) => {
                if !insides_already_done {
                    mve.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let mut scan_idx = 0;
                    let mut scan_group = 0;
                    while scan_group < groups.len() {
                        let i = (*idx as i32) - scan_idx;
                        if i < 0 {
                            // This can happen if the index is valid but does not simplify
                            break;
                        }
                        match &mut groups[scan_group] {
                            MultiVectorGroupExpr::JustFloat(f) => {
                                if i == 0 {
                                    *self = f.take_as_owned();
                                    return;
                                }
                                scan_idx += 1;
                            }
                            MultiVectorGroupExpr::Vec2(v2) => {
                                if i < 2 {
                                    *self = v2.take_part_as_owned(i as u8);
                                    return
                                }
                                scan_idx += 2;
                            }
                            MultiVectorGroupExpr::Vec3(v3) => {
                                if i < 3 {
                                    *self = v3.take_part_as_owned(i as u8);
                                    return
                                }
                                scan_idx += 3;
                            }
                            MultiVectorGroupExpr::Vec4(v4) => {
                                if i < 4 {
                                    *self = v4.take_part_as_owned(i as u8);
                                    return
                                }
                                scan_idx += 4;
                            }
                        }
                        scan_group += 1;
                    }
                }
            }
            FloatExpr::TraitInvoke11ToFloat(_t, owner) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
            FloatExpr::Product(product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                if !insides_already_done {
                    for (factor, _exponent) in product.iter_mut() {
                        factor.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if product.len() == 1 && *last_factor == 1.0 {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut sums_to_distribute = vec![];
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    FloatExpr::Literal(f) => {
                        *last_factor *= f32::powf(*f, *exponent);
                        false
                    }
                    FloatExpr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        *last_factor *= *another_factor;
                        false
                    }
                    FloatExpr::Sum(ref mut s, another_addend) => {
                        sums_to_distribute.push((s.take_as_owned(), *another_addend));
                        false
                    }
                    FloatExpr::Exp(box base_expr, None, exponent_literal) => {
                        flatten.push((base_expr.take_as_owned(), *exponent_literal * *exponent));
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    FloatExpr::Literal(f) => {
                        *last_factor *= f32::powf(*f, *exponent);
                        false
                    }
                    _ => true,
                });
                if *last_factor == 0.0 {
                    *self = FloatExpr::Literal(0.0);
                    return
                }
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if !sums_to_distribute.is_empty() {
                    // Start the sum/product transposition by distributing non-sum factors to the first sum factor
                    let (first_sum, first_sum_lits) = &mut sums_to_distribute[0];
                    if product.is_empty() {
                        product.push((FloatExpr::Literal(1.0), 1.0));
                    }
                    for (addend, factor) in first_sum.iter_mut() {
                        addend.mul_assign(FloatExpr::product(product.clone(), *factor * *last_factor));
                        *factor = 1.0;
                    }
                    first_sum.push((FloatExpr::product(product.take_as_owned(), *last_factor), *first_sum_lits));
                    *first_sum_lits = 0.0;

                    // Then finish the sum/product transposition by distributing sum factors on one another
                    let mut result_sum = first_sum.take_as_owned();
                    let mut idx = 1;
                    while idx < sums_to_distribute.len() {
                        let next_sum = &mut sums_to_distribute[idx];
                        idx += 1;

                        let mut result_replacer = vec![];
                        next_sum.0.push((FloatExpr::Literal(next_sum.1), 1.0));
                        let next_sum = &mut next_sum.0;
                        for result_addend in result_sum.iter_mut() {
                            for next_addend in next_sum.iter_mut() {
                                result_replacer.push((result_addend.0.take_as_owned() * next_addend.0.take_as_owned(), result_addend.1 * next_addend.1));
                            }
                        }
                        result_sum = result_replacer;
                    }
                    *self = FloatExpr::sum(result_sum, 0.0);
                    // Transposition is a non-trivial structural change, so we need to re-simplify
                    self.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    return
                }

                if product.len() == 1 && product[0].1 == 1.0 {
                    if let FloatExpr::Sum(sum, last_addend) = &mut product[0].0 {
                        *last_addend *= *last_factor;
                        for (_addend, sum_factor) in sum.iter_mut() {
                            *sum_factor *= *last_factor;
                        }
                        *last_factor = 1.0;
                    }
                    if *last_factor == 1.0 {
                        // _exponent is 1.0 (we just checked it a few lines ago)
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    *self = FloatExpr::Literal(*last_factor);
                }
            }
            FloatExpr::Sum(sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                if !insides_already_done {
                    for (addend, _factor) in sum.iter_mut() {
                        addend.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if sum.len() == 1 && *last_addend == 0.0 {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = FloatExpr::product(vec![(addend, 1.0)], factor);
                        new_self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        *self = new_self;
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    FloatExpr::Literal(f) => {
                        *last_addend += *f * *factor;
                        false
                    }
                    FloatExpr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        *last_addend += *another_addend;
                        false
                    }
                    FloatExpr::Product(p, last_factor) => {
                        *factor *= *last_factor;
                        *last_factor = 1.0;
                        if p.len() == 1 {
                            if p[0].1 == 1.0 {
                                *addend = p.remove(0).0;
                            }
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    FloatExpr::Literal(f) => {
                        *last_addend += *f * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == 0.0 {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        *self = FloatExpr::product(vec![(addend, 1.0)], factor);
                    };
                }
                if sum.is_empty() {
                    *self = FloatExpr::Literal(*last_addend);
                }
            }
            FloatExpr::Exp(base_expression, exponent_expression, exponent_literal) => {
                if !insides_already_done {
                    base_expression.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let Some(d) = exponent_expression {
                    if !insides_already_done {
                        d.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                    if let box FloatExpr::Literal(l) = d {
                        *exponent_literal *= *l;
                        *exponent_expression = None;
                    }
                }

                match (exponent_expression.as_deref_mut(), base_expression.deref_mut()) {
                    (Some(outer_exponent), FloatExpr::Exp(box inner_base, Some(box inner_exponent), inner_literal)) => {
                        *exponent_expression = Some(Box::new(outer_exponent.take_as_owned() * inner_exponent.take_as_owned()));
                        exponent_literal.mul_assign(*inner_literal);
                        *base_expression = Box::new(inner_base.take_as_owned());
                    }
                    (None, FloatExpr::Exp(box inner_base, Some(box inner_exponent), inner_literal)) => {
                        *exponent_expression = Some(Box::new(inner_exponent.take_as_owned()));
                        exponent_literal.mul_assign(*inner_literal);
                        *base_expression = Box::new(inner_base.take_as_owned());
                    }
                    (_outer_exponent, FloatExpr::Exp(box inner_base, None, inner_literal)) => {
                        exponent_literal.mul_assign(*inner_literal);
                        *base_expression = Box::new(inner_base.take_as_owned());
                    }
                    (Some(outer_exponent), FloatExpr::Product(factors, factor_literal)) if factors.len() == 1 && *factor_literal == 1.0 => {
                        // Pull the inside exponent out
                        let (factor, factor_exponent) = &mut factors[0];
                        outer_exponent.mul_assign(*factor_exponent);
                        *base_expression = Box::new(factor.take_as_owned());
                    }
                    (None, FloatExpr::Product(factors, factor_literal)) if factors.len() == 1 => {
                        // Push the outside exponent in
                        let (factor, factor_exponent) = &mut factors[0];
                        let new_factor_exponent = *factor_exponent * *exponent_literal;
                        let new_factor_literal = f32::powf(*factor_literal, *exponent_literal);
                        *self = FloatExpr::product(vec![(factor.take_as_owned(), new_factor_exponent)], new_factor_literal);
                        self.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
                if exponent_expression.is_none() {
                    if *exponent_literal == 1.0 {
                        *self = base_expression.take_as_owned();
                        return
                    }
                    if *exponent_literal == 0.0 {
                        *self = FloatExpr::Literal(1.0);
                        return
                    }
                    *self = FloatExpr::product(vec![(base_expression.take_as_owned(), *exponent_literal)], 1.0);
                    self.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    return
                }
            }
        }
    }
}



impl Vec2Expr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            Vec2Expr::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec2(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec2Expr::Gather1(ref mut f) => {
                if !insides_already_done {
                    f.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                // Do I really want to do more here?
            }
            Vec2Expr::Gather2(ref mut f0, ref mut f1) => {
                use crate::ast::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if f0 == f1 {
                    *self = Vec2Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1) {
                    (AccessVec4(box ref mut v4_a, x), AccessVec4(box ref mut v4_b, y)) => {
                        if v4_a == v4_b {
                            *self = if *x == 0 && *y == 1 {
                                Vec2Expr::Truncate4to2(Box::new(v4_a.take_as_owned()))
                            } else if *x < 2 && *y < 2 {
                                Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate4to2(Box::new(v4_a.take_as_owned())), *x as usize, *y as usize)
                            } else {
                                Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x as usize, *y as usize, 2, 3)))
                            };
                            return;
                        }
                    }
                    (AccessVec3(box ref mut v3_a, x), AccessVec3(box ref mut v3_b, y)) => {
                        if v3_a == v3_b {
                            *self = if *x == 0 && *y == 1 {
                                Vec2Expr::Truncate3to2(Box::new(v3_a.take_as_owned()))
                            } else if *x < 2 && *y < 2 {
                                Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate3to2(Box::new(v3_a.take_as_owned())), *x as usize, *y as usize)
                            } else {
                                Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x as usize, *y as usize, 2)))
                            };
                            return;
                        }
                    }
                    (AccessVec2(box ref mut v2_a, x), AccessVec2(box ref mut v2_b, y)) if v2_a == v2_b => {
                        *self = if *x == 0 && *y == 1 {
                            v2_a.take_as_owned()
                        } else {
                            Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x as usize, *y as usize)
                        };
                        return;
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                    ) if x_mve == y_mve => {
                        let max_idx = u16::max(*x_idx, *y_idx);
                        let min_idx = u16::min(*x_idx, *y_idx);
                        if (min_idx + 1) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() >= 2 {
                                let mv_group = if group.simd_width() == 2 {
                                    Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx)
                                } else if group.simd_width() == 3 {
                                    Vec2Expr::Truncate3to2(Box::new(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx)))
                                } else {
                                    Vec2Expr::Truncate4to2(Box::new(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx)))
                                };
                                *self = if no_swizzle {
                                    mv_group
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    Vec2Expr::swizzle_vec_2(mv_group, x as usize, y as usize)
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (Product(ref mut x_product, x_lit), Product(ref mut y_product, y_lit)) if transpose_simd => {
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_product(x_product, y_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit)) if transpose_simd => {
                        let lits = [1.0, *y_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec2_product(&mut x, y_product, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y) if transpose_simd => {
                        let lits = [*x_lit, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec2_product(x_product, &mut y, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), Sum(ref mut y_sum, y_lit)) if transpose_simd => {
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_sum(x_sum, y_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit)) if transpose_simd => {
                        let lits = [0.0, *y_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec2_sum(&mut x, y_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y) if transpose_simd => {
                        let lits = [*x_lit, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec2_sum(x_sum, &mut y, lits) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec2Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                if !insides_already_done {
                    mve.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(v2) => {
                            *self = v2.take_as_owned();
                            2
                        }
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 2 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 2. {mv}"
                        )
                    }
                }
            }
            Vec2Expr::Product(ref mut product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 2] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        false
                    }
                    Vec2Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        false
                    }
                    _ => true,
                });
                if *last_factor == [0.0; 2] {
                    *self = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);
                // TODO impl Carrier for AntiDualNum
                //  DualNum::from_groups(Simd32x2::from([self[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]))
                if product.len() == 1 && *last_factor == [1.0; 2] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    *self = if f0 == f1 { Vec2Expr::Gather1(f0) } else { Vec2Expr::Gather2(f0, f1) };
                }
            }
            Vec2Expr::Sum(ref mut sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 2] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec2Expr::product(vec![(addend, 1.0)], [factor, factor]);
                        new_self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        *self = new_self;
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        false
                    }
                    Vec2Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        false
                    }
                    Vec2Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 2];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 2] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec2Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        false
                    }
                    Vec2Expr::Gather2(FloatExpr::Literal(f0), FloatExpr::Literal(f1)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 2] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor];
                        *self = Vec2Expr::product(vec![(addend, 1.0)], gather);
                    };
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    *self = if f0 == f1 { Vec2Expr::Gather1(f0) } else { Vec2Expr::Gather2(f0, f1) };
                }
            }
            Vec2Expr::SwizzleVec2(v2, i0, i1) => {
                if *i0 > 1 || *i1 > 1 {
                    panic!("Problem!");
                }
                if !insides_already_done {
                    v2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if *i0 == 0 && *i1 == 1 {
                    *self = v2.take_as_owned();
                    return;
                }
                match v2 {
                    box Vec2Expr::Gather1(f0) => {
                        *self = Vec2Expr::Gather1(f0.take_as_owned());
                    }
                    box Vec2Expr::Gather2(f0, f1) => {
                        let fs = [f0, f1];
                        *self = Vec2Expr::Gather2(fs[*i0 as usize].clone(), fs[*i1 as usize].clone());
                    }
                    _ => {}
                }
            }
            Vec2Expr::Truncate3to2(box v3) => {
                if !insides_already_done {
                    v3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match v3 {
                    Vec3Expr::Gather1(x) => {
                        *self = Vec2Expr::Gather1(x.take_as_owned());
                    }
                    Vec3Expr::Gather3(x,y, _) => {
                        *self = Vec2Expr::Gather2(x.take_as_owned(), y.take_as_owned());
                    }
                    Vec3Expr::SwizzleVec3(box Vec3Expr::Extend2to3(_xy, z), i0, i1, _i2)
                    if *i0 == 2 && *i1 == 2 => {
                        *self = Vec2Expr::Gather1(z.take_as_owned());
                    }
                    Vec3Expr::SwizzleVec3(box inner_v3, i0, i1, _) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate3to2(Box::new(inner_v3.take_as_owned())), *i0 as usize, *i1 as usize);
                    }
                    Vec3Expr::Extend2to3(v2, _) => {
                        *self = v2.take_as_owned();
                        return
                    }
                    _ => {}
                }
            }
            Vec2Expr::Truncate4to2(box v4) => {
                if !insides_already_done {
                    v4.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match v4 {
                    Vec4Expr::Gather1(x) => {
                        *self = Vec2Expr::Gather1(x.take_as_owned());
                    }
                    Vec4Expr::Gather4(x,y, z, _) => {
                        *self = Vec2Expr::Gather2(x.take_as_owned(), y.take_as_owned());
                    }
                    Vec4Expr::SwizzleVec4(box Vec4Expr::Extend2to4(_xy, z, w), i0, i1, i2, _i3)
                    if *i0 >= 2 && *i1 >= 2 && *i2 >= 2 => {
                        *self = Vec2Expr::Gather2(
                            if *i0 == 2 { z.clone() } else { w.clone() },
                            if *i1 == 2 { z.clone() } else { w.clone() },
                        );
                    }
                    Vec4Expr::SwizzleVec4(box Vec4Expr::Extend3to4(_xyz, w), i0, i1, i2, _i3)
                    if *i0 == 3 && *i1 == 3 && *i2 == 3 => {
                        *self = Vec2Expr::Gather1(w.take_as_owned());
                    }
                    Vec4Expr::SwizzleVec4(box inner_v4, i0, i1, _, _) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate4to2(Box::new(inner_v4.take_as_owned())), *i0 as usize, *i1 as usize);
                    }
                    Vec4Expr::Extend2to4(v2, _, _) => {
                        *self = v2.take_as_owned();
                        return
                    }
                    _ => {}
                }
            }
        }
    }
}
impl Vec3Expr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            Vec3Expr::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec3(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec3Expr::Gather1(ref mut f) => {
                if !insides_already_done {
                    f.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                // Do I really want to do more here?
            }
            Vec3Expr::Gather3(ref mut f0, ref mut f1, ref mut f2) => {
                use crate::ast::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if f0 == f1 && f0 == f2 {
                    *self = Vec3Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1, f2) {
                    (AccessVec4(box ref mut v4_a, x), AccessVec4(box ref mut v4_b, y), AccessVec4(box ref mut v4_c, z)) => {
                        if v4_a == v4_b && v4_a == v4_c {
                            *self = if *x == 0 && *y == 1 && *z == 2 {
                                Vec3Expr::Truncate4to3(Box::new(v4_a.take_as_owned()))
                            } else if *x < 3 && *y < 3 && *z < 3 {
                                Vec3Expr::swizzle_vec_3(Vec3Expr::Truncate4to3(Box::new(v4_a.take_as_owned())), *x as usize, *y as usize, *z as usize)
                            } else {
                                Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x as usize, *y as usize, *z as usize, 3)))
                            };
                            return;
                        }
                    }
                    (AccessVec3(box ref mut v3_a, x), AccessVec3(box ref mut v3_b, y), AccessVec3(box ref mut v3_c, z)) => {
                        if v3_a == v3_b && v3_a == v3_c {
                            *self = if *x == 0 && *y == 1 && *z == 2 {
                                v3_a.take_as_owned()
                            } else {
                                Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x as usize, *y as usize, *z as usize)
                            };
                            return;
                        }
                    }
                    (AccessVec2(box v2_a, x), AccessVec2(box v2_b, y), z) => {
                        if v2_a == v2_b {
                            let mut v3 = Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x as usize, *y as usize);
                            v3.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                            *self = Vec3Expr::Extend2to3(v3, z.take_as_owned());
                            return;
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                    ) if x_mve == y_mve && y_mve == z_mve => {
                        let max_idx = u16::max(*x_idx, u16::max(*y_idx, *z_idx));
                        let min_idx = u16::min(*x_idx, u16::min(*y_idx, *z_idx));
                        if (min_idx + 2) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() >= 3 {
                                let mv_group = if group.simd_width() == 3 {
                                    Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx)
                                } else {
                                    Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx)))
                                };
                                *self = if no_swizzle {
                                    mv_group
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    let z = (*z_idx - min_idx) as u8;
                                    Vec3Expr::swizzle_vec_3(mv_group, x as usize, y as usize, z as usize)
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = transpose_vec3_product(x_product, y_product, z_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit)
                    ) if transpose_simd => {
                        let lits = [1.0, *y_lit, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(&mut x, y_product, z_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        y,
                        Product(ref mut z_product, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, 1.0, *z_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(x_product, &mut y, z_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        z
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 1.0];
                        let mut zv = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(x_product, y_product, &mut zv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_product(x_product, y_product, lits) {
                            *self = Vec3Expr::Extend2to3(transposed, z.take_as_owned());
                            return
                        }
                    }
                    (x, y, Product(ref mut z_product, z_lit)) if transpose_simd => {
                        let lits = [1.0, 1.0, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(&mut x, &mut y, z_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit), z) if transpose_simd => {
                        let lits = [1.0, *y_lit, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(&mut x, y_product, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y, z) if transpose_simd => {
                        let lits = [*x_lit, 1.0, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_product(x_product, &mut y, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = transpose_vec3_sum(x_sum, y_sum, z_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit)
                    ) if transpose_simd => {
                        let lits = [0.0, *y_lit, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(&mut x, y_sum, z_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        y,
                        Sum(ref mut z_sum, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, 0.0, *z_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(x_sum, &mut y, z_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        z
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 0.0];
                        let mut zv = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(x_sum, y_sum, &mut zv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_sum(x_sum, y_sum, lits) {
                            *self = Vec3Expr::Extend2to3(transposed, z.take_as_owned());
                            return
                        }
                    }
                    (x, y, Sum(ref mut z_sum, z_lit)) if transpose_simd => {
                        let lits = [0.0, 0.0, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(&mut x, &mut y, z_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit), z) if transpose_simd => {
                        let lits = [0.0, *y_lit, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(&mut x, y_sum, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y, z) if transpose_simd => {
                        let lits = [*x_lit, 0.0, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec3_sum(x_sum, &mut y, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (Literal(x), Literal(y), z) if *x == *y => {
                        *self = Vec3Expr::Extend2to3(Vec2Expr::Gather1(Literal(*x)), z.take_as_owned())
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        z,
                    ) if x_mve == y_mve => {
                        let max_idx = u16::max(*x_idx, *y_idx);
                        let min_idx = u16::min(*x_idx, *y_idx);
                        if (min_idx + 1) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() >= 2 {
                                let mv_group = match group.simd_width() {
                                    2 => Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                    3 => Vec2Expr::Truncate3to2(Box::new(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx))),
                                    4 => Vec2Expr::Truncate4to2(Box::new(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx))),
                                    _ => unreachable!("max simd width is 4")
                                };
                                *self = if no_swizzle {
                                    Vec3Expr::Extend2to3(mv_group, z.take_as_owned())
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    Vec3Expr::Extend2to3(
                                        Vec2Expr::swizzle_vec_2(mv_group, x as usize, y as usize),
                                        z.take_as_owned(),
                                    )
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (x, y, z) if x == y => {
                        *self = Vec3Expr::Extend2to3(
                            Vec2Expr::Gather1(x.take_as_owned()),
                            z.take_as_owned(),
                        );
                        return
                    }
                    _ => {}
                }
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                if !insides_already_done {
                    v2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match (v2, f1) {
                    (Vec2Expr::Gather1(x), z) if x.is_memory_read_and_not_compute() => {
                        *self = Vec3Expr::Gather3(x.clone(), x.take_as_owned(), z.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    (Vec2Expr::Gather2(x, y), z) => {
                        *self = Vec3Expr::Gather3(x.take_as_owned(), y.take_as_owned(), z.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            Vec3Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                if !insides_already_done {
                    mve.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(v3) => {
                            *self = v3.take_as_owned();
                            3
                        }
                        MultiVectorGroupExpr::Vec4(_) => 4,
                    };
                    if size != 3 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 3. {mv}"
                        )
                    }
                }
            }
            Vec3Expr::Product(ref mut product, last_factor) => {
                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 3] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        false
                    }
                    Vec3Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        last_factor[2] *= another_factor[2];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        false
                    }
                    _ => true,
                });
                if *last_factor == [0.0; 3] {
                    *self = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && *last_factor == [1.0; 3] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.len() == 1 && *last_factor == [0.0, 0.0, 1.0] {
                    let (factor, exp) = &mut product[0];
                    if *exp == 1.0 {
                        if let Vec3Expr::Extend2to3(_xy, z) = factor {
                            *self = Vec3Expr::Extend2to3(Vec2Expr::Gather1(FloatExpr::Literal(0.0)), z.take_as_owned());
                            return
                        }
                    }
                }

                if !product.is_empty() && last_factor[2] == 0.0 {
                    let mut new_factors = vec![];
                    for (existing_factor, existing_exponent) in product {
                        new_factors.push((Vec2Expr::Truncate3to2(Box::new(existing_factor.take_as_owned())), *existing_exponent));
                    }
                    *self = Vec3Expr::Extend2to3(Vec2Expr::product(new_factors, [last_factor[0], last_factor[1]]), FloatExpr::Literal(0.0));
                    self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                    return
                }

                // Vec extensions get pulled to the outside of arithmetic
                if product.len() == 2 {
                    let (a, b) = product.split_at_mut(1);
                    match (&mut a[0], &mut b[0]) {
                        ((Vec3Expr::Extend2to3(va, za), a), (Vec3Expr::Extend2to3(vb, zb), b)) => {
                            *self = Vec3Expr::Extend2to3(
                                Vec2Expr::product(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_factor[0], last_factor[1]]),
                                FloatExpr::product(vec![(za.take_as_owned(), *a), (zb.take_as_owned(), *b)], last_factor[2]),
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        _ => {}
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    *self = if f0 == f1 && f1 == f2 { Vec3Expr::Gather1(f0) } else { Vec3Expr::Gather3(f0, f1, f2) };
                }
            }
            Vec3Expr::Sum(ref mut sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 3] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec3Expr::product(vec![(addend, 1.0)], [factor, factor, factor]);
                        new_self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        *self = new_self;
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        false
                    }
                    Vec3Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        last_addend[2] += another_addend[2];
                        false
                    }
                    Vec3Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] && last_factor[1] == last_factor[2] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 3];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 3] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec3Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        false
                    }
                    Vec3Expr::Gather3(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 3] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor, factor];
                        *self = Vec3Expr::product(vec![(addend, 1.0)], gather);
                    };
                }

                // Vec extensions get pulled to the outside of arithmetic
                if sum.len() == 2 {
                    let (a, b) = sum.split_at_mut(1);
                    match (&mut a[0], &mut b[0]) {
                        ((Vec3Expr::Extend2to3(va, za), a), (Vec3Expr::Extend2to3(vb, zb), b)) => {
                            *self = Vec3Expr::Extend2to3(
                                Vec2Expr::sum(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_addend[0], last_addend[1]]),
                                FloatExpr::sum(vec![(za.take_as_owned(), *a), (zb.take_as_owned(), *b)], last_addend[2]),
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        _ => {}
                    }
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    let f2 = FloatExpr::Literal(last_addend[2]);
                    *self = if f0 == f1 && f1 == f2 { Vec3Expr::Gather1(f0) } else { Vec3Expr::Gather3(f0, f1, f2) };
                }
            }
            Vec3Expr::SwizzleVec3(v3, i0, i1, i2) => {
                if *i0 > 2 || *i1 > 2 || *i2 > 2 {
                    panic!("Problem!");
                }
                if !insides_already_done {
                    v3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if *i0 == 0 && *i1 == 1 && *i2 == 2 {
                    *self = v3.take_as_owned();
                    return;
                }
                match v3 {
                    box Vec3Expr::Gather1(f0) => {
                        *self = Vec3Expr::Gather1(f0.take_as_owned());
                    }
                    box Vec3Expr::Gather3(f0, f1, f2) => {
                        let fs = [f0, f1, f2];
                        *self = Vec3Expr::Gather3(fs[*i0 as usize].clone(), fs[*i1 as usize].clone(), fs[*i2 as usize].clone());
                    }
                    box Vec3Expr::Extend2to3(v, z) if *i0 == 0 && *i1 == 1 => {
                        *self = Vec3Expr::Extend2to3(v.take_as_owned(), z.take_as_owned());
                    }
                    box Vec3Expr::Extend2to3(v, z) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v.take_as_owned(), *i0 as usize, *i1 as usize), z.take_as_owned());
                    }
                    _ => {}
                }
            }
            Vec3Expr::Truncate4to3(box v4) => {
                if !insides_already_done {
                    v4.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match v4 {
                    Vec4Expr::Gather1(x) => {
                        *self = Vec3Expr::Gather1(x.take_as_owned());
                    }
                    Vec4Expr::Gather4(x, y, z, _) => {
                        *self = Vec3Expr::Gather3(x.take_as_owned(), y.take_as_owned(), z.take_as_owned());
                    }
                    Vec4Expr::SwizzleVec4(box Vec4Expr::Extend2to4(_xy, z, w), i0, i1, i2, _i3)
                    if *i0 >= 2 && *i1 >= 2 && *i2 >= 2 => {
                        *self = Vec3Expr::Gather3(
                            if *i0 == 2 { z.clone() } else { w.clone() },
                            if *i1 == 2 { z.clone() } else { w.clone() },
                            if *i2 == 2 { z.clone() } else { w.clone() },
                        );
                    }
                    Vec4Expr::SwizzleVec4(box Vec4Expr::Extend3to4(_xyz, w), i0, i1, i2, _i3)
                    if *i0 == 3 && *i1 == 3 && *i2 == 3 => {
                        // Cases like this may seem really weird or unlikely and you ask yourself "Does that really even happen?"
                        // and the answer is almost always yes, most simplification patterns are based on actual code output, in
                        // this case impl AntiProjectOrthogonallyOnto<AntiMotor> for AntiDipoleInversion
                        *self = Vec3Expr::Gather1(w.take_as_owned());
                    }
                    Vec4Expr::SwizzleVec4(box inner_v4, i0, i1, i2, _) if *i0 < 3 && *i1 < 3 && *i2 < 3 => {
                        *self = Vec3Expr::swizzle_vec_3(Vec3Expr::Truncate4to3(Box::new(inner_v4.take_as_owned())), *i0 as usize, *i1 as usize, *i2 as usize);
                    }
                    Vec4Expr::Extend2to4(xy, z, _w) => {
                        *self = Vec3Expr::Extend2to3(xy.take_as_owned(), z.take_as_owned());
                        return
                    }
                    Vec4Expr::Extend3to4(v3, _) => {
                        *self = v3.take_as_owned();
                        return
                    }
                    _ => {}
                }
            }
        }
    }
}
impl Vec4Expr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }

    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            Vec4Expr::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec4(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec4Expr::Gather1(f) => {
                if !insides_already_done {
                    f.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                // Do I really want to do more here?
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                use crate::ast::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if f0 == f1 && f0 == f2 && f0 == f3 {
                    *self = Vec4Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1, f2, f3) {
                    (AccessVec4(box v4_a, x), AccessVec4(box v4_b, y), AccessVec4(box v4_c, z), AccessVec4(box v4_d, w)) => {
                        if v4_a == v4_b && v4_a == v4_c && v4_a == v4_d {
                            *self = if *x == 0 && *y == 1 && *z == 2 && *w == 3 {
                                v4_a.take_as_owned()
                            } else {
                                Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x as usize, *y as usize, *z as usize, *w as usize)
                            };
                            return;
                        }
                    }
                    (AccessVec3(box v3_a, x), AccessVec3(box v3_b, y), AccessVec3(box v3_c, z), w) => {
                        if v3_a == v3_b && v3_a == v3_c {
                            let mut v3 = Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x as usize, *y as usize, *z as usize);
                            v3.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                            *self = Vec4Expr::Extend3to4(v3, w.take_as_owned());
                            return;
                        }
                    }
                    (AccessVec2(box v2_a, x), AccessVec2(box v2_b, y), z, w) => {
                        if v2_a == v2_b {
                            let mut v3 = Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x as usize, *y as usize);
                            v3.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                            *self = Vec4Expr::Extend2to4(v3, z.take_as_owned(), w.take_as_owned());
                            return;
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                        AccessMultiVecFlat(w_mve, w_idx),
                    ) if x_mve == y_mve && y_mve == z_mve && z_mve == w_mve => {
                        let max_idx = u16::max(*x_idx, u16::max(*y_idx, u16::max(*z_idx, *w_idx)));
                        let min_idx = u16::min(*x_idx, u16::min(*y_idx, u16::min(*z_idx, *w_idx)));
                        if (min_idx + 3) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx) && (*x_idx + 3 == *w_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() == 4 {
                                let mv_group = Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx);
                                *self = if no_swizzle {
                                    mv_group
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    let z = (*z_idx - min_idx) as u8;
                                    let w = (*w_idx - min_idx) as u8;
                                    Vec4Expr::swizzle_vec_4(mv_group, x as usize, y as usize, z as usize, w as usize)
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit),
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit, *w_lit];
                        if let Some(transposed) = transpose_vec4_product(x_product, y_product, z_product, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Product(ref mut x_product, y_lit),
                        Product(ref mut y_product, z_lit),
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [1.0, *y_lit, *z_lit, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, x_product, y_product, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        y,
                        Product(ref mut z_product, z_lit),
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, 1.0, *z_lit, *w_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, &mut y, z_product, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        z,
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 1.0, *w_lit];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, y_product, &mut z, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit, 1.0];
                        let mut wv = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, y_product, z_product, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = transpose_vec3_product(x_product, y_product, z_product, lits) {
                            *self = Vec4Expr::Extend3to4(transposed, w.take_as_owned());
                            return
                        }
                    }
                    (
                        x,
                        y,
                        Product(ref mut z_product, z_lit),
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [1.0, 1.0, *z_lit, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, &mut y, z_product, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        z,
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 1.0, 1.0];
                        let mut zv = vec![(z.clone(), 1.0)];
                        let mut wv = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, y_product, &mut zv, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_product(x_product, y_product, lits) {
                            *self = Vec4Expr::Extend2to4(transposed, z.take_as_owned(), w.take_as_owned());
                            return
                        }
                    }
                    (
                        x,
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [1.0, *y_lit, *z_lit, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, y_product, z_product, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        y,
                        z,
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, 1.0, 1.0, *w_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, &mut y, &mut z, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        y,
                        Product(ref mut z_product, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, 1.0, *z_lit, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, &mut y, z_product, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Product(ref mut y_product, y_lit),
                        z,
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [1.0, *y_lit, 1.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, y_product, &mut z, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, z, Product(ref mut w_product, w_lit)) if transpose_simd => {
                        let lits = [1.0, 1.0, 1.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, &mut y, &mut z, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, Product(ref mut z_product, z_lit), w) if transpose_simd => {
                        let lits = [1.0, 1.0, *z_lit, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, &mut y, z_product, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit), z, w) if transpose_simd => {
                        let lits = [1.0, *y_lit, 1.0, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(&mut x, y_product, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y, z, w) if transpose_simd => {
                        let lits = [*x_lit, 1.0, 1.0, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_product(x_product, &mut y, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit),
                        Sum(ref mut w_sum, w_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit, *w_lit];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, y_sum, z_sum, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Sum(ref mut x_sum, y_lit),
                        Sum(ref mut y_sum, z_lit),
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [0.0, *y_lit, *z_lit, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, x_sum, y_sum, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        y,
                        Sum(ref mut z_sum, z_lit),
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, 0.0, *z_lit, *w_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, &mut y, z_sum, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        z,
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 0.0, *w_lit];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, y_sum, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit, 0.0];
                        let mut wv = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, y_sum, z_sum, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = transpose_vec3_sum(x_sum, y_sum, z_sum, lits) {
                            *self = Vec4Expr::Extend3to4(transposed, w.take_as_owned());
                            return
                        }
                    }
                    (
                        x,
                        y,
                        Sum(ref mut z_sum, z_lit),
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [0.0, 0.0, *z_lit, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, &mut y, z_sum, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        z,
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, 0.0, 0.0];
                        let mut zv = vec![(z.clone(), 1.0)];
                        let mut wv = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, y_sum, &mut zv, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = transpose_vec2_sum(x_sum, y_sum, lits) {
                            *self = Vec4Expr::Extend2to4(transposed, z.take_as_owned(), w.take_as_owned());
                            return
                        }
                    }
                    (
                        x,
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [0.0, *y_lit, *z_lit, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, y_sum, z_sum, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        y,
                        z,
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, 0.0, 0.0, *w_lit];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, &mut y, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        y,
                        Sum(ref mut z_sum, z_lit),
                        w,
                    ) if transpose_simd => {
                        let lits = [*x_lit, 0.0, *z_lit, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, &mut y, z_sum, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        x,
                        Sum(ref mut y_sum, y_lit),
                        z,
                        Sum(ref mut w_sum, w_lit),
                    ) if transpose_simd => {
                        let lits = [0.0, *y_lit, 0.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, y_sum, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, z, Sum(ref mut w_sum, w_lit)) if transpose_simd => {
                        let lits = [0.0, 0.0, 0.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, &mut y, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, Sum(ref mut z_sum, z_lit), w) if transpose_simd => {
                        let lits = [0.0, 0.0, *z_lit, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, &mut y, z_sum, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit), z, w) if transpose_simd => {
                        let lits = [0.0, *y_lit, 0.0, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(&mut x, y_sum, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y, z, w) if transpose_simd => {
                        let lits = [*x_lit, 0.0, 0.0, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = transpose_vec4_sum(x_sum, &mut y, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (Literal(x), Literal(y), Literal(z), w) if *x == *y && *y == *z => {
                        *self = Vec4Expr::Extend3to4(Vec3Expr::Gather1(Literal(*x)), w.take_as_owned())
                    }
                    (Literal(x), Literal(y), z, w) if *x == *y => {
                        *self = Vec4Expr::Extend2to4(Vec2Expr::Gather1(Literal(*x)), z.take_as_owned(), w.take_as_owned())
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                        w,
                    ) if x_mve == y_mve && y_mve == z_mve => {
                        let max_idx = u16::max(*x_idx, u16::max(*y_idx, *z_idx));
                        let min_idx = u16::min(*x_idx, u16::min(*y_idx, *z_idx));
                        if (min_idx + 2) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() >= 3 {
                                let mv_group = match group.simd_width() {
                                    3 => Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                    4 => Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx))),
                                    _ => unreachable!("max simd width is 4")
                                };
                                *self = if no_swizzle {
                                    Vec4Expr::Extend3to4(mv_group, w.take_as_owned())
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    let z = (*z_idx - min_idx) as u8;
                                    Vec4Expr::Extend3to4(
                                        Vec3Expr::swizzle_vec_3(mv_group, x as usize, y as usize, z as usize),
                                        w.take_as_owned()
                                    )
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        z,
                        w,
                    ) if x_mve == y_mve => {
                        let max_idx = u16::max(*x_idx, *y_idx);
                        let min_idx = u16::min(*x_idx, *y_idx);
                        if (min_idx + 1) < max_idx {
                            // indexes are too far apart
                            return
                        }
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            if flat_idx > min_idx {
                                return
                            }
                            if min_idx == flat_idx && group.simd_width() >= 2 {
                                let mv_group = match group.simd_width() {
                                    2 => Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                    3 => Vec2Expr::Truncate3to2(Box::new(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx))),
                                    4 => Vec2Expr::Truncate4to2(Box::new(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx))),
                                    _ => unreachable!("max simd width is 4")
                                };
                                *self = if no_swizzle {
                                    Vec4Expr::Extend2to4(mv_group, z.take_as_owned(), w.take_as_owned())
                                } else {
                                    let x = (*x_idx - min_idx) as u8;
                                    let y = (*y_idx - min_idx) as u8;
                                    Vec4Expr::Extend2to4(
                                        Vec2Expr::swizzle_vec_2(mv_group, x as usize, y as usize),
                                        z.take_as_owned(),
                                        w.take_as_owned(),
                                    )
                                };
                                return
                            }
                            group_idx = group_idx + 1;
                            flat_idx = flat_idx + group.simd_width() as u16;
                        }
                    }
                    (x, y, z, w) if x == y && y == z => {
                        *self = Vec4Expr::Extend3to4(
                            Vec3Expr::Gather1(x.take_as_owned()),
                            w.take_as_owned(),
                        );
                        return
                    }
                    (x, y, z, w) if x == y => {
                        *self = Vec4Expr::Extend2to4(
                            Vec2Expr::Gather1(x.take_as_owned()),
                            z.take_as_owned(),
                            w.take_as_owned(),
                        );
                        return
                    }
                    _ => {}
                }
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                if !insides_already_done {
                    v2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match (v2, f1, f2) {
                    (Vec2Expr::Gather1(x), z, w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    (Vec2Expr::Gather2(x, y), z, w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                if !insides_already_done {
                    v3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    f1.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                match (v3, f1) {
                    (Vec3Expr::Gather1(x), w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.clone(), x.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    (Vec3Expr::Gather3(x, y, z), w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    (Vec3Expr::Extend2to3(Vec2Expr::Gather2(x, y), z), w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    (Vec3Expr::Extend2to3(Vec2Expr::Gather1(x), z), w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        return
                    }
                    _ => {}
                }
            }
            Vec4Expr::AccessMultiVecGroup(mve, idx) => {
                if !insides_already_done {
                    mve.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx as usize] {
                        MultiVectorGroupExpr::JustFloat(_) => 1,
                        MultiVectorGroupExpr::Vec2(_) => 2,
                        MultiVectorGroupExpr::Vec3(_) => 3,
                        MultiVectorGroupExpr::Vec4(v4) => {
                            *self = v4.take_as_owned();
                            4
                        }
                    };
                    if size != 4 {
                        panic!(
                            "Invalid expression detected: MultiVector group {idx} has size \
                        {size}, but is used in a place where we expect size 4. {mv}"
                        )
                    }
                }
            }
            Vec4Expr::Product(product, last_factor) => {

                // TODO impl AntiInverse for FlatPoint
                //  Simd32x4::from(1.0 / (self[e45] * self[e45])) * Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45] * -1.0]),
                //  We can/should convert that into the following
                //  self.group0() * Simd32x4::from(-1.0 / (self[e45] * self[e45]))
                //  or
                //  self.group0() * Simd32x4::from(-1.0) / Simd32x4::from(self[e45] * self[e45])

                if product.is_empty() {
                    panic!("Problem")
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 4] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                let mut flatten = vec![];
                product.retain_mut(|(factor, exponent)| match factor {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        last_factor[3] *= powf;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        last_factor[3] *= f32::powf(*f3, *exponent);
                        false
                    }
                    Vec4Expr::Product(ref mut p, another_factor) => {
                        for (_, e) in p.iter_mut() {
                            *e = *e * *exponent;
                        }
                        flatten.append(p);
                        last_factor[0] *= another_factor[0];
                        last_factor[1] *= another_factor[1];
                        last_factor[2] *= another_factor[2];
                        last_factor[3] *= another_factor[3];
                        false
                    }
                    _ => true,
                });
                flatten.retain(|(factor, exponent)| match factor {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        let powf = f32::powf(*f, *exponent);
                        last_factor[0] *= powf;
                        last_factor[1] *= powf;
                        last_factor[2] *= powf;
                        last_factor[3] *= powf;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_factor[0] *= f32::powf(*f0, *exponent);
                        last_factor[1] *= f32::powf(*f1, *exponent);
                        last_factor[2] *= f32::powf(*f2, *exponent);
                        last_factor[3] *= f32::powf(*f3, *exponent);
                        false
                    }
                    _ => true,
                });
                if *last_factor == [0.0; 4] {
                    *self = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }
                product.append(&mut flatten);
                product.sort_with_f32();

                let mut partition = 1;
                while partition <= product.len() {
                    let (front, back) = product.split_at_mut(partition);
                    let (front_expr, front_exponent) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_exponent)| {
                        if front_expr == back_expr {
                            *front_exponent += *back_exponent;
                            false
                        } else {
                            true
                        }
                    });
                    product.truncate(partition + kept_length);
                    partition += 1;
                }
                product.retain(|(_, e)| *e != 0.0);

                if product.len() == 1 && *last_factor == [1.0; 4] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }
                if product.len() == 1 && *last_factor == [0.0, 0.0, 1.0, 1.0] {
                    let (factor, exp) = &mut product[0];
                    if *exp == 1.0 {
                        if let Vec4Expr::Extend2to4(_xy, z, w) = factor {
                            *self = Vec4Expr::Extend2to4(Vec2Expr::Gather1(FloatExpr::Literal(0.0)), z.take_as_owned(), w.take_as_owned());
                            return
                        }
                    }
                }
                if product.len() == 1 && *last_factor == [0.0, 0.0, 0.0, 1.0] {
                    let (factor, exp) = &mut product[0];
                    if *exp == 1.0 {
                        if let Vec4Expr::Extend3to4(_xyz, w) = factor {
                            *self = Vec4Expr::Extend3to4(Vec3Expr::Gather1(FloatExpr::Literal(0.0)), w.take_as_owned());
                            return
                        }
                    }
                }

                if !product.is_empty() && last_factor[2] == 0.0 && last_factor[3] == 0.0 {
                    let mut new_factors = vec![];
                    for (existing_factor, existing_exponent) in product {
                        new_factors.push((Vec2Expr::Truncate4to2(Box::new(existing_factor.take_as_owned())), *existing_exponent));
                    }
                    *self = Vec4Expr::Extend2to4(Vec2Expr::product(new_factors, [last_factor[0], last_factor[1]]), FloatExpr::Literal(0.0), FloatExpr::Literal(0.0));
                    self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                    return
                }
                if !product.is_empty() && last_factor[3] == 0.0 {
                    let mut new_factors = vec![];
                    for (existing_factor, existing_exponent) in product {
                        new_factors.push((Vec3Expr::Truncate4to3(Box::new(existing_factor.take_as_owned())), *existing_exponent));
                    }
                    *self = Vec4Expr::Extend3to4(Vec3Expr::product(new_factors, [last_factor[0], last_factor[1], last_factor[2]]), FloatExpr::Literal(0.0));
                    self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                    return
                }

                // Vec extensions get pulled to the outside of arithmetic
                if product.len() == 2 {
                    let (a, b) = product.split_at_mut(1);
                    match (&mut a[0], &mut b[0]) {
                        ((Vec4Expr::Extend3to4(va, wa), a), (Vec4Expr::Extend3to4(vb, wb), b)) => {
                            *self = Vec4Expr::Extend3to4(
                                Vec3Expr::product(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_factor[0], last_factor[1], last_factor[2]]),
                                FloatExpr::product(vec![(wa.take_as_owned(), *a), (wb.take_as_owned(), *b)], last_factor[3])
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        ((Vec4Expr::Extend2to4(va, za, wa), a), (Vec4Expr::Extend2to4(vb, zb, wb), b)) => {
                            *self = Vec4Expr::Extend2to4(
                                Vec2Expr::product(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_factor[0], last_factor[1]]),
                                FloatExpr::product(vec![(za.take_as_owned(), *a), (zb.take_as_owned(), *b)], last_factor[2]),
                                FloatExpr::product(vec![(wa.take_as_owned(), *a), (wb.take_as_owned(), *b)], last_factor[3])
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        _ => {}
                    }
                }
                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    let f3 = FloatExpr::Literal(last_factor[3]);
                    *self = if f0 == f1 && f1 == f2 && f2 == f3 {
                        Vec4Expr::Gather1(f0)
                    } else {
                        Vec4Expr::Gather4(f0, f1, f2, f3)
                    };
                }
            }
            Vec4Expr::Sum(sum, last_addend) => {
                if sum.is_empty() {
                    panic!("Problem")
                }
                if !insides_already_done {
                    for (addend, _factor) in sum.iter_mut() {
                        addend.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 4] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec4Expr::product(vec![(addend, 1.0)], [factor, factor, factor, factor]);
                        new_self.simplify_nuanced(true, transpose_simd, prefer_flat_access);
                        *self = new_self;
                    };
                }
                let mut flatten = vec![];
                sum.retain_mut(|(addend, factor)| match addend {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        last_addend[3] += *f * *factor;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        last_addend[3] += *f3 * *factor;
                        false
                    }
                    Vec4Expr::Sum(s, another_addend) => {
                        for (_, f) in s.iter_mut() {
                            *f = *f * *factor;
                        }
                        flatten.append(s);
                        last_addend[0] += another_addend[0];
                        last_addend[1] += another_addend[1];
                        last_addend[2] += another_addend[2];
                        last_addend[3] += another_addend[3];
                        false
                    }
                    Vec4Expr::Product(p, last_factor) => {
                        if last_factor[0] == last_factor[1] && last_factor[1] == last_factor[2] && last_factor[2] == last_factor[3] {
                            *factor *= last_factor[0];
                            *last_factor = [1.0; 4];
                        }
                        if p.len() == 1 && p[0].1 == 1.0 && *last_factor == [1.0; 4] {
                            *addend = p.remove(0).0;
                        }
                        true
                    }
                    _ => true,
                });
                flatten.retain(|(addend, factor)| match addend {
                    Vec4Expr::Gather1(FloatExpr::Literal(f)) => {
                        last_addend[0] += *f * *factor;
                        last_addend[1] += *f * *factor;
                        last_addend[2] += *f * *factor;
                        last_addend[3] += *f * *factor;
                        false
                    }
                    Vec4Expr::Gather4(FloatExpr::Literal(f0), FloatExpr::Literal(f1), FloatExpr::Literal(f2), FloatExpr::Literal(f3)) => {
                        last_addend[0] += *f0 * *factor;
                        last_addend[1] += *f1 * *factor;
                        last_addend[2] += *f2 * *factor;
                        last_addend[3] += *f3 * *factor;
                        false
                    }
                    _ => true,
                });
                sum.append(&mut flatten);
                sum.sort_with_f32();

                let mut partition = 1;
                while partition <= sum.len() {
                    let (front, back) = sum.split_at_mut(partition);
                    let (front_expr, front_factor) = &mut front[partition - 1];
                    let kept_length = slice_retain_mut(back, |(back_expr, back_factor)| {
                        if front_expr == back_expr {
                            *front_factor += *back_factor;
                            false
                        } else {
                            true
                        }
                    });
                    sum.truncate(partition + kept_length);
                    partition += 1;
                }
                sum.retain(|(_, f)| *f != 0.0);

                if sum.len() == 1 && *last_addend == [0.0; 4] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let gather = [factor, factor, factor, factor];
                        *self = Vec4Expr::product(vec![(addend, 1.0)], gather);
                    };
                }

                // Vec extensions get pulled to the outside of arithmetic
                if sum.len() == 2 {
                    let (a, b) = sum.split_at_mut(1);
                    match (&mut a[0], &mut b[0]) {
                        ((Vec4Expr::Extend3to4(va, wa), a), (Vec4Expr::Extend3to4(vb, wb), b)) => {
                            *self = Vec4Expr::Extend3to4(
                                Vec3Expr::sum(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_addend[0], last_addend[1], last_addend[2]]),
                                FloatExpr::sum(vec![(wa.take_as_owned(), *a), (wb.take_as_owned(), *b)], last_addend[3])
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        ((Vec4Expr::Extend2to4(va, za, wa), a), (Vec4Expr::Extend2to4(vb, zb, wb), b)) => {
                            *self = Vec4Expr::Extend2to4(
                                Vec2Expr::sum(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_addend[0], last_addend[1]]),
                                FloatExpr::sum(vec![(za.take_as_owned(), *a), (zb.take_as_owned(), *b)], last_addend[2]),
                                FloatExpr::sum(vec![(wa.take_as_owned(), *a), (wb.take_as_owned(), *b)], last_addend[3])
                            );
                            // Significant restructure, so re-simplify
                            self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            return
                        }
                        _ => {}
                    }
                }
                if sum.is_empty() {
                    let f0 = FloatExpr::Literal(last_addend[0]);
                    let f1 = FloatExpr::Literal(last_addend[1]);
                    let f2 = FloatExpr::Literal(last_addend[2]);
                    let f3 = FloatExpr::Literal(last_addend[3]);
                    *self = if f0 == f1 && f1 == f2 && f2 == f3 {
                        Vec4Expr::Gather1(f0)
                    } else {
                        Vec4Expr::Gather4(f0, f1, f2, f3)
                    };
                }
            }
            Vec4Expr::SwizzleVec4(v4, i0, i1, i2, i3) => {
                if *i0 > 3 || *i1 > 3 || *i2 > 3 {
                    panic!("Problem!");
                }
                if !insides_already_done {
                    v4.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if *i0 == 0 && *i1 == 1 && *i2 == 2 && *i3 == 3 {
                    *self = v4.take_as_owned();
                    return;
                }
                match v4 {
                    box Vec4Expr::Gather1(f0) => {
                        *self = Vec4Expr::Gather1(f0.take_as_owned());
                    }
                    box Vec4Expr::Gather4(f0, f1, f2, f3) => {
                        let fs = [f0, f1, f2, f3];
                        *self = Vec4Expr::Gather4(fs[*i0 as usize].clone(), fs[*i1 as usize].clone(), fs[*i2 as usize].clone(), fs[*i3 as usize].clone());
                    }
                    box Vec4Expr::Extend2to4(v, z, w) if *i0 == 0 && *i1 == 1 => {
                        *self = Vec4Expr::Extend2to4(v.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend2to4(v, z, w) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v.take_as_owned(), *i0 as usize, *i1 as usize), z.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend3to4(v, w) if *i0 == 0 && *i1 == 1 && *i2 == 2 => {
                        *self = Vec4Expr::Extend3to4(v.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend3to4(v, w) if *i0 < 3 && *i1 < 3 && *i2 < 3 => {
                        *self = Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v.take_as_owned(), *i0 as usize, *i1 as usize, *i2 as usize), w.take_as_owned());
                    }
                    _ => {}
                }
            }
        }
    }
}
impl MultiVectorGroupExpr {
    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => {
                if !insides_already_done {
                    f.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let FloatExpr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                        return;
                    }
                }
                if let FloatExpr::AccessMultiVecFlat(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        let mut target = *idx;
                        for ge in v.iter_mut() {
                            match (target, ge) {
                                (0, MultiVectorGroupExpr::JustFloat(fe)) => {
                                    *self = MultiVectorGroupExpr::JustFloat(fe.take_as_owned());
                                    return;
                                }
                                (_, MultiVectorGroupExpr::JustFloat(_)) => {
                                    target -= 1;
                                }
                                (_, MultiVectorGroupExpr::Vec2(_)) => {
                                    if target < 2 {
                                        return;
                                    }
                                    target -= 2;
                                }
                                (_, MultiVectorGroupExpr::Vec3(_)) => {
                                    if target < 3 {
                                        return;
                                    }
                                    target -= 3;
                                }
                                (_, MultiVectorGroupExpr::Vec4(_)) => {
                                    if target < 4 {
                                        return;
                                    }
                                    target -= 4;
                                }
                            }
                        }
                    }
                }
            }
            MultiVectorGroupExpr::Vec2(v2) => {
                if !insides_already_done {
                    v2.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v2 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec3(v3) => {
                if !insides_already_done {
                    v3.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v3 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec4(v4) => {
                if !insides_already_done {
                    v4.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
                if let Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v4 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx as usize].take_as_owned();
                    }
                }
            }
        }
    }
}
impl MultiVectorExpr {
    pub(crate) fn simplify(&mut self) {
        self.simplify_nuanced(false, false, false);
    }
    fn simplify_nuanced(&mut self, insides_already_done: bool, transpose_simd: bool, prefer_flat_access: bool) {
        match &mut *self.expr {
            MultiVectorVia::Variable(v) => {
                let decl = &v.decl;
                if 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Class(mut new_self) = inlined_expr {
                            new_self.simplify_nuanced(false, transpose_simd, prefer_flat_access);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            MultiVectorVia::Construct(groups) => {
                if !insides_already_done {
                    for group in groups.iter_mut() {
                        group.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    }
                }
                let result = groups.iter_mut().enumerate().fold(None, |a, (b_idx, b)| {
                    let mv_b = match b {
                        MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv, idx))
                        if *idx as usize == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv, idx))
                        if *idx as usize == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv, idx))
                        if *idx as usize == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv, idx))
                        if *idx as usize == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        _ => None,
                    };
                    if b_idx == 0 {
                        return mv_b;
                    }
                    let a = a?;
                    let b = mv_b?;
                    if a == b {
                        Some(a)
                    } else {
                        None
                    }
                });
                if let Some(result) = result {
                    // Any chance of take_as_owned for MultiVectorExpr? Not trivial.
                    *self = result.clone();
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_t, owner) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(_t, owner, _other) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
            MultiVectorVia::TraitInvoke22ToClass(_t, owner, other) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    other.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(_t, owner, other) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    other.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(_t, owner, other) => {
                if !insides_already_done {
                    owner.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                    other.simplify_nuanced(insides_already_done, transpose_simd, prefer_flat_access);
                }
            }
        }
    }
}

