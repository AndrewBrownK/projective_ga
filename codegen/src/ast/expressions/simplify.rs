use std::sync::atomic::Ordering::Acquire;

// TODO clean up commented out println!()
//  (after you're sure you no longer need them)



// TODO seeming mistakes, seen in diffs:
//  .
//  impl AntiInverse for Flector {
//  // e1, e2, e3, e4
//  Simd32x4::from(other_g0) * self.group0() * Simd32x4::from(-1.0),
//  self.group0() / Simd32x4::from(other_g0 * -1.0),
//  .
//  impl AntiConstraintViolation for Flector {
//  .
//  impl AntiInverse for Motor {
//  // e41, e42, e43, e1234
//  Simd32x4::from(other_g0) * self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
//  self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) / Simd32x4::from(other_g0),
//  // e23, e31, e12, scalar
//  Simd32x4::from(other_g0) * self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
//  self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) / Simd32x4::from(other_g0),
//  .
//



// TODO not strict correctness mistakes, but could still use improvement, seen in diffs:
//  .
//  impl std::ops::Sub<AntiScalar> for DualNum {
//  DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([0.0, other[e1234] * -1.0]) + self.group0())
//  DualNum::from_groups(/* scalar, e1234 */ self.group0() + (Simd32x2::from([1.0, other[e1234]]) * Simd32x2::from([0.0, -1.0])))
//  .
//  impl AntiProjectOrthogonallyOnto<Flector> for AntiScalar {
//  let anti_wedge_g1_xyz = Simd32x3::from(self[e1234]) * other.group0().xyz();
//  let anti_wedge_g1 = Simd32x4::from(self[e1234]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]);

macro_rules! max {
    ($x:expr) => { $x };
    ($x:expr, $y:expr) => { usize::max($x, $y) };
    ($x:expr, $($rest:expr),+) => { usize::max($x, max!($($rest),+)) };
}
macro_rules! min {
    ($x:expr) => { $x };
    ($x:expr, $y:expr) => { usize::min($x, $y) };
    ($x:expr, $($rest:expr),+) => { usize::min($x, min!($($rest),+)) };
}



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
    pub(crate) fn simplify(&mut self) {
        match self {
            AnyExpression::Int(e) => e.simplify(),
            AnyExpression::Float(e) => e.simplify(),
            AnyExpression::Vec2(e) => e.simplify(),
            AnyExpression::Vec3(e) => e.simplify(),
            AnyExpression::Vec4(e) => e.simplify(),
            AnyExpression::Class(e) => e.simplify(),
        }
    }
    pub(crate) fn transposing_simplify(&mut self) {
        match self {
            AnyExpression::Int(e) => e.transposing_simplify(),
            AnyExpression::Float(e) => e.transposing_simplify(),
            AnyExpression::Vec2(e) => e.transposing_simplify(),
            AnyExpression::Vec3(e) => e.transposing_simplify(),
            AnyExpression::Vec4(e) => e.transposing_simplify(),
            AnyExpression::Class(e) => e.transposing_simplify(),
        }
    }
    pub(crate) fn deep_simplify(&mut self) {
        match self {
            AnyExpression::Int(e) => e.deep_simplify(),
            AnyExpression::Float(e) => e.deep_simplify(),
            AnyExpression::Vec2(e) => e.deep_simplify(),
            AnyExpression::Vec3(e) => e.deep_simplify(),
            AnyExpression::Vec4(e) => e.deep_simplify(),
            AnyExpression::Class(e) => e.deep_simplify(),
        }
    }
}

impl IntExpr {
    #[allow(unused)]
    pub(crate) fn simplify(&mut self) {
        self.int_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.int_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.int_simplify(false, false, true);
    }
    #[allow(unused)]
    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn int_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            IntExpr::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Int(mut new_self) = inlined_expr {
                            new_self.int_simplify(false, transpose_simd, force_inline_all_variables);
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
        self.float_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.float_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.float_simplify(false, false, true);
    }

    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn float_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            FloatExpr::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Float(mut new_self) = inlined_expr {
                            new_self.float_simplify(false, transpose_simd, force_inline_all_variables);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            FloatExpr::Literal(_) => {}
            FloatExpr::FromInt(a) => {
                let span = tracing::trace_span!("match_FromInt");
                let _span_entered = span.enter();
                if !insides_already_done {
                    a.int_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                let span = tracing::trace_span!("match_AccessVec2");
                let _span_entered = span.enter();
                if !insides_already_done {
                    av2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match av2.as_mut() {
                    Vec2Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec2Expr::Gather2(fe0, fe1) => {
                        *self = [fe0, fe1][*idx_in_vec].take_as_owned();
                    }
                    Vec2Expr::Truncate3to2(box v) => {
                        *self = FloatExpr::access_vec_3(v.take_as_owned(), *idx_in_vec);
                        self.float_simplify(false, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec2Expr::Truncate4to2(box v) => {
                        *self = FloatExpr::access_vec_4(v.take_as_owned(), *idx_in_vec);
                        self.float_simplify(false, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec2Expr::AccessMultiVecGroup(mve, target_group_idx) => {
                        let mut flat_idx = 0;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                                return
                            }
                            flat_idx = flat_idx + g.simd_width();
                        }
                    }
                    Vec2Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_2(factor.take_as_owned(), *idx_in_vec), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec2Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_2(addend.take_as_owned(), *idx_in_vec), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec2Expr::SwizzleVec2(box v2, i0, i1) => {
                        *self = FloatExpr::access_vec_2(v2.take_as_owned(), [*i0, *i1][*idx_in_vec]);
                    }
                    Vec2Expr::SwizzleVec3(box v3, i0, i1) => {
                        *self = FloatExpr::access_vec_3(v3.take_as_owned(), [*i0, *i1][*idx_in_vec]);
                    }
                    Vec2Expr::SwizzleVec4(box v4, i0, i1) => {
                        *self = FloatExpr::access_vec_4(v4.take_as_owned(), [*i0, *i1][*idx_in_vec]);
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec3(av3, idx_in_vec) => {
                let span = tracing::trace_span!("match_AccessVec3");
                let _span_entered = span.enter();
                if !insides_already_done {
                    av3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match av3.as_mut() {
                    Vec3Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec3Expr::Gather3(fe0, fe1, fe2) => {
                        *self = [fe0, fe1, fe2][*idx_in_vec].take_as_owned();
                    }
                    Vec3Expr::Truncate4to3(box v) => {
                        *self = FloatExpr::access_vec_4(v.take_as_owned(), *idx_in_vec);
                        self.float_simplify(false, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec3Expr::Extend2to3(xy, z) => {
                        match *idx_in_vec {
                            0 | 1 => {
                                *self = FloatExpr::access_vec_2(xy.take_as_owned(), *idx_in_vec);
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            2 => {
                                *self = z.take_as_owned();
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec3Expr::AccessMultiVecGroup(mve, target_group_idx) => {
                        let mut flat_idx = 0;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                                return
                            }
                            flat_idx = flat_idx + g.simd_width();
                        }
                    }
                    Vec3Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_3(factor.take_as_owned(), *idx_in_vec), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec3Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_3(addend.take_as_owned(), *idx_in_vec), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec3Expr::SwizzleVec2(v2, i0, i1, i2) => {
                        *self = FloatExpr::access_vec_2(v2.take_as_owned(), [*i0, *i1, *i2][*idx_in_vec]);
                    }
                    Vec3Expr::SwizzleVec3(box v3, i0, i1, i2) => {
                        *self = FloatExpr::access_vec_3(v3.take_as_owned(), [*i0, *i1, *i2][*idx_in_vec]);
                    }
                    Vec3Expr::SwizzleVec4(box v4, i0, i1, i2) => {
                        *self = FloatExpr::access_vec_4(v4.take_as_owned(), [*i0, *i1, *i2][*idx_in_vec]);
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessVec4(av4, idx_in_vec) => {
                let span = tracing::trace_span!("match_AccessVec4");
                let _span_entered = span.enter();
                if !insides_already_done {
                    av4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match av4.as_mut() {
                    Vec4Expr::Gather1(fe) => {
                        *self = fe.take_as_owned();
                    }
                    Vec4Expr::Gather4(fe0, fe1, fe2, fe3) => {
                        *self = [fe0, fe1, fe2, fe3][*idx_in_vec].take_as_owned();
                    }
                    Vec4Expr::Extend2to4(xy, z, w) => {
                        match *idx_in_vec {
                            0 | 1 => {
                                *self = FloatExpr::access_vec_2(xy.take_as_owned(), *idx_in_vec);
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            2 => {
                                *self = z.take_as_owned();
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            3 => {
                                *self = w.take_as_owned();
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec4Expr::Extend3to4(xyz, w) => {
                        match *idx_in_vec {
                            0 | 1 | 2 => {
                                *self = FloatExpr::access_vec_3(xyz.take_as_owned(), *idx_in_vec);
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            3 => {
                                *self = w.take_as_owned();
                                self.float_simplify(false, transpose_simd, force_inline_all_variables);
                                return
                            }
                            _ => {}
                        }
                    }
                    Vec4Expr::AccessMultiVecGroup(mve, target_group_idx) => {
                        let mut flat_idx = 0;
                        for (scanning_group_idx, g) in mve.mv_class.groups().into_iter().enumerate() {
                            if scanning_group_idx == (*target_group_idx) {
                                *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx + *idx_in_vec);
                                // tracing::trace!("Replaced AccessMultiVecGroup with AccessMultiVecFlat: {:?}", self);
                                return
                            }
                            flat_idx = flat_idx + g.simd_width();
                        }
                    }
                    Vec4Expr::Product(factors, literal) => {
                        let mut new_factors = vec![];
                        for (factor, exponent) in factors {
                            new_factors.push((FloatExpr::access_vec_4(factor.take_as_owned(), *idx_in_vec), *exponent));
                        }
                        *self = FloatExpr::product(new_factors, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec4Expr::Sum(addends, literal) => {
                        let mut new_addends = vec![];
                        for (addend, factor) in addends {
                            new_addends.push((FloatExpr::access_vec_4(addend.take_as_owned(), *idx_in_vec), *factor));
                        }
                        *self = FloatExpr::sum(new_addends, literal[*idx_in_vec]);
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                        return
                    }
                    Vec4Expr::SwizzleVec2(v2, i0, i1, i2, i3) => {
                        *self = FloatExpr::access_vec_2(v2.take_as_owned(), [*i0, *i1, *i2, *i3][*idx_in_vec]);
                    }
                    Vec4Expr::SwizzleVec3(v3, i0, i1, i2, i3) => {
                        *self = FloatExpr::access_vec_3(v3.take_as_owned(), [*i0, *i1, *i2, *i3][*idx_in_vec]);
                    }
                    Vec4Expr::SwizzleVec4(box v4, i0, i1, i2, i3) => {
                        *self = FloatExpr::access_vec_4(v4.take_as_owned(), [*i0, *i1, *i2, *i3][*idx_in_vec]);
                    }
                    _ => {}
                }
            }
            FloatExpr::AccessMultiVecGroup(mve, idx) => {
                let span = tracing::trace_span!("match_AccessMultiVecGroup");
                let _span_entered = span.enter();
                if !insides_already_done {
                    mve.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                let idx = *idx;
                let mv = mve.mv_class;
                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx] {
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

                let mut flat_idx = 0;
                for (i, g) in mv.groups().into_iter().enumerate() {
                    if i == idx {
                        *self = FloatExpr::AccessMultiVecFlat(mve.take_as_owned(), flat_idx);
                        return
                    }
                    flat_idx = flat_idx + g.simd_width();
                }
            }
            FloatExpr::AccessMultiVecFlat(mve, idx) => {
                let span = tracing::trace_span!("match_AccessMultiVecFlat");
                let _span_entered = span.enter();
                if !insides_already_done {
                    mve.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                        let i = i as usize;
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
                                    *self = v2.take_part_as_owned(i);
                                    return
                                }
                                scan_idx += 2;
                            }
                            MultiVectorGroupExpr::Vec3(v3) => {
                                if i < 3 {
                                    *self = v3.take_part_as_owned(i);
                                    return
                                }
                                scan_idx += 3;
                            }
                            MultiVectorGroupExpr::Vec4(v4) => {
                                if i < 4 {
                                    *self = v4.take_part_as_owned(i);
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
                let span = tracing::trace_span!("match_TraitInvoke11ToFloat");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
            FloatExpr::Product(product, last_factor) => {
                let span = tracing::trace_span!("match_Product");
                let _span_entered = span.enter();
                // TODO smells like nested products in impl AntiConstraintViolation for DualNum

                if product.is_empty() {
                    panic!("Please use FloatExpr::product so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    for (factor, _exponent) in product.iter_mut() {
                        factor.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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

                // a^x * a^y   ->   a^(x+y)
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
                    self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                let span = tracing::trace_span!("match_Sum");
                let _span_entered = span.enter();
                if sum.is_empty() {
                    panic!("Please use FloatExpr::sum so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    for (addend, _factor) in sum.iter_mut() {
                        addend.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if sum.len() == 1 && *last_addend == 0.0 {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = FloatExpr::product(vec![(addend, 1.0)], factor);
                        new_self.float_simplify(true, transpose_simd, force_inline_all_variables);
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
                let span = tracing::trace_span!("match_Exp");
                let _span_entered = span.enter();
                if !insides_already_done {
                    base_expression.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if let Some(d) = exponent_expression {
                    if !insides_already_done {
                        d.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                        self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                    self.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    return
                }
            }
        }
    }
}



impl Vec2Expr {
    pub(crate) fn simplify(&mut self) {
        self.vec2_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.vec2_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.slice_to_floats();
        self.vec2_simplify(false, false, true);
    }
    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn vec2_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            Vec2Expr::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec2(mut new_self) = inlined_expr {
                            new_self.vec2_simplify(false, transpose_simd, force_inline_all_variables);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec2Expr::Gather1(ref mut f) => {
                let span = tracing::trace_span!("match_Gather1");
                let _span_entered = span.enter();
                if !insides_already_done {
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                // Do I really want to do more here?
            }
            Vec2Expr::Gather2(ref mut f0, ref mut f1) => {
                let span = tracing::trace_span!("match_Gather2");
                let _span_entered = span.enter();
                use crate::ast::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if f0 == f1 {
                    *self = Vec2Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1) {
                    (AccessVec4(box ref mut v4_a, x), AccessVec4(box ref mut v4_b, y)) if eqs!(v4_a, v4_b) => {
                        *self = if *x == 0 && *y == 1 {
                            Vec2Expr::Truncate4to2(Box::new(v4_a.take_as_owned()))
                        } else if *x < 2 && *y < 2 {
                            Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate4to2(Box::new(v4_a.take_as_owned())), *x, *y)
                        } else {
                            Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x, *y, 2, 3)))
                        };
                        return;
                    }
                    (AccessVec3(box ref mut v3_a, x), AccessVec3(box ref mut v3_b, y)) if eqs!(v3_a, v3_b) => {
                        *self = if *x == 0 && *y == 1 {
                            Vec2Expr::Truncate3to2(Box::new(v3_a.take_as_owned()))
                        } else if *x < 2 && *y < 2 {
                            Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate3to2(Box::new(v3_a.take_as_owned())), *x, *y)
                        } else {
                            Vec2Expr::Truncate3to2(Box::new(Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x, *y, 2)))
                        };
                        return;
                    }
                    (AccessVec2(box ref mut v2_a, x), AccessVec2(box ref mut v2_b, y)) if eqs!(v2_a, v2_b) => {
                        *self = if *x == 0 && *y == 1 {
                            v2_a.take_as_owned()
                        } else {
                            Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x, *y)
                        };
                        return;
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                    ) if eqs!(x_mve, y_mve) && min!(*x_idx, *y_idx) + 3 >= max!(*x_idx, *y_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 2) => Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                (_, 4) => Vec2Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y),
                                (_, 3) => Vec2Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y),
                                (false, 2) => Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec2Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)),
                                _ => return
                            };
                            self.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            return
                        }
                    }
                    (Product(ref mut x_product, x_lit), Product(ref mut y_product, y_lit)) if transpose_simd => {
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_product_transpose(None, x_product, y_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit)) if transpose_simd => {
                        let lits = [1.0, *y_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = vec2_product_transpose(None, &mut x, y_product, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y) if transpose_simd => {
                        let lits = [*x_lit, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = vec2_product_transpose(None, x_product, &mut y, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), Sum(ref mut y_sum, y_lit)) if transpose_simd => {
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_sum_transpose(None, x_sum, y_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit)) if transpose_simd => {
                        let lits = [0.0, *y_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        if let Some(transposed) = vec2_sum_transpose(None, &mut x, y_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y) if transpose_simd => {
                        let lits = [*x_lit, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = vec2_sum_transpose(None, x_sum, &mut y, lits) {
                            *self = transposed;
                        }
                    }
                    _ => {}
                }
            }
            Vec2Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                let span = tracing::trace_span!("match_AccessMultiVecGroup");
                let _span_entered = span.enter();
                if !insides_already_done {
                    mve.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx] {
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
                let span = tracing::trace_span!("match_Product");
                let _span_entered = span.enter();
                // TODO not sure if this should be fixed here in simplification, or in code emission:
                //  impl AntiProjectOrthogonallyOnto<Scalar> for DualNum
                //  Before:    DualNum::from_groups(/* scalar, e1234 */ Simd32x2::powi(Simd32x2::from(other[scalar]), 2) * self.group0())
                //  After:     DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * Simd32x2::from(other[scalar]) * self.group0())
                //  Preferred: DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * other[scalar]) * self.group0())

                // TODO impl AntiConstraintViolation for Flector {
                //  push the last_factor inside the vec in this situation:
                //  (Simd32x2::from([0.0, self[e4] * self[e4] + self[e423] * self[e423] + self[e431] * self[e431] + self[e412] * self[e412]]) * Simd32x2::from([0.0, -1.0]))

                // TODO impl AntiConstraintViolation for Flector {
                //  this thing is torture.
                //  It's like... I'll either have to un-transpose, then do exponent (for product) or coefficient (for sum) cancellations, then re-transpose
                //  Or I'll have to just do a shallow un-transpose here in the simplification
                //  What a fricken nightmare
                //  I think... maybe it can be an iterated shallow un-transpose... Fuck it. Maybe this should be done outside simplification.

                // TODO is this possible to fix?
                //  impl AntiConstraintViolation for Line {
                //  DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
                //     -2.0 * (self[e41] * self[e23]) - 2.0 * (self[e42] * self[e31]) - 2.0 * (self[e43] * self[e12]),
                //     -2.0 * self[e41] * self[e41] - 2.0 * self[e42] * self[e42] - 2.0 * self[e43] * self[e43],
                //  ]))

                // TODO impl Wedge<Plane> for Flector {
                //  Simd32x2::from([0.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]),

                if product.is_empty() {
                    panic!("Please use Vec2Expr::product so you can find out where you constructed something wrong");
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 2] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }

                let mut gather1 = vec![];
                let mut gather2_x = vec![];
                let mut gather2_y = vec![];

                let mut start_idx = 0;
                while start_idx < product.len() {
                    let mut iter_idx = 0;
                    let mut flatten = vec![];
                    product.retain_mut(|(factor, exponent)| {
                        if iter_idx < start_idx {
                            iter_idx += 1;
                            return true
                        }
                        match factor {
                            Vec2Expr::Gather1(f) => {
                                match f {
                                    FloatExpr::Literal(f) => {
                                        let powf = f32::powf(*f, *exponent);
                                        last_factor[0] *= powf;
                                        last_factor[1] *= powf;
                                    }
                                    _ => gather1.push((f.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec2Expr::Gather2(x, y) => {
                                match x {
                                    FloatExpr::Literal(x) => last_factor[0] *= f32::powf(*x, *exponent),
                                    _ => gather2_x.push((x.take_as_owned(), *exponent)),
                                }
                                match y {
                                    FloatExpr::Literal(y) => last_factor[1] *= f32::powf(*y, *exponent),
                                    _ => gather2_y.push((y.take_as_owned(), *exponent)),
                                }
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
                        }
                    });
                    start_idx = product.len();
                    product.append(&mut flatten);
                }

                if *last_factor == [0.0; 2] {
                    *self = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }

                let x = last_factor[0];
                let y = last_factor[1];
                if x == 0.0 { gather2_x.clear(); }
                if y == 0.0 { gather2_y.clear(); }

                macro_rules! default_coefficient {
                    ($i:expr) => {
                        FloatExpr::Literal(if last_factor[$i] == 0.0 { 0.0 } else { 1.0 })
                    };
                }
                macro_rules! swap_take {
                    ($var:ident, $replacement:expr) => {
                        {
                            let mut x = $replacement;
                            mem::swap(&mut x, &mut $var);
                            x
                        }
                    };
                }

                if eqs!(x, y) && !gather1.is_empty() {
                    let gather1 = swap_take!(gather1, vec![]);
                    let mut f = FloatExpr::product(gather1, x);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    product.push((Vec2Expr::Gather1(f), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                }

                let is_any_gather2 = !gather2_x.is_empty() || !gather2_y.is_empty();
                let is_only_gather2 = is_any_gather2 && product.is_empty() && gather1.is_empty();

                let mut x_is_zeroed_without_last_factor = false;
                let mut y_is_zeroed_without_last_factor = false;

                if !gather1.is_empty() {
                    let mut f = FloatExpr::product(gather1, 1.0);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    product.push((Vec2Expr::Gather1(f), 1.0));
                }
                if is_any_gather2 {
                    let mut x = if gather2_x.is_empty() { default_coefficient!(0) } else { FloatExpr::Product(gather2_x, 1.0) };
                    let mut y = if gather2_y.is_empty() { default_coefficient!(1) } else { FloatExpr::Product(gather2_y, 1.0) };
                    x.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    y.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let FloatExpr::Literal(0.0) = &x {
                        x_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &y {
                        y_is_zeroed_without_last_factor = true;
                    }
                    if is_only_gather2 {
                        product.push((Vec2Expr::Gather2(x * last_factor[0], y * last_factor[1]), 1.0));
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                    } else {
                        product.push((Vec2Expr::Gather2(x, y), 1.0));
                    }
                }
                if (last_factor[0] == 1.0 || x_is_zeroed_without_last_factor) &&
                    (last_factor[1] == 1.0 || y_is_zeroed_without_last_factor) {
                    *last_factor = [1.0; 2];
                }

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
                    *self = if eqs!(f0, f1) {
                        Vec2Expr::Gather1(f0)
                    } else {
                        Vec2Expr::Gather2(f0, f1)
                    };
                }
            }
            Vec2Expr::Sum(ref mut sum, last_addend) => {
                // TODO impl std::ops::Sub<Scalar> for MultiVector {
                //  self.group0() + (Simd32x2::from([other[scalar], 0.0]) * Simd32x2::from([-1.0, 0.0])),

                let span = tracing::trace_span!("match_Sum");
                let _span_entered = span.enter();
                if sum.is_empty() {
                    panic!("Please use Vec2Expr::sum so you can find out where you constructed something wrong");
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 2] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec2Expr::product(vec![(addend, 1.0)], [factor, factor]);
                        new_self.vec2_simplify(true, transpose_simd, force_inline_all_variables);
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
            Vec2Expr::SwizzleVec2(box v2, i0, i1) => {
                let span = tracing::trace_span!("match_SwizzleVec2");
                let _span_entered = span.enter();
                if *i0 > 1 || *i1 > 1 {
                    panic!("Please use Vec2Expr::swizzle_vec_2 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if *i0 == 0 && *i1 == 1 {
                    *self = v2.take_as_owned();
                    return;
                }
                if eqs!(*i0, *i1) {
                    *self = Vec2Expr::Gather1(FloatExpr::AccessVec2(Box::new(v2.take_as_owned()), *i0));
                    self.vec2_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v2 {
                    Vec2Expr::Gather1(f0) => {
                        *self = Vec2Expr::Gather1(f0.take_as_owned());
                    }
                    Vec2Expr::Gather2(f0, f1) => {
                        let fs = [f0, f1];
                        *self = Vec2Expr::Gather2(fs[*i0].clone(), fs[*i1].clone());
                    }
                    _ => {}
                }
            }
            Vec2Expr::SwizzleVec3(box v3, i0, i1) => {
                let span = tracing::trace_span!("match_SwizzleVec3");
                let _span_entered = span.enter();
                if *i0 > 2 || *i1 > 2 {
                    panic!("Please use Vec2Expr::swizzle_vec_3 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1) {
                    *self = Vec2Expr::Gather1(FloatExpr::AccessVec3(Box::new(v3.take_as_owned()), *i0));
                    self.vec2_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v3 {
                    Vec3Expr::Gather1(f0) => {
                        *self = Vec2Expr::Gather1(f0.take_as_owned());
                    }
                    Vec3Expr::Gather3(f0, f1, f2) => {
                        let fs = [f0, f1, f2];
                        *self = Vec2Expr::Gather2(fs[*i0].clone(), fs[*i1].clone());
                    }
                    _ => {}
                }
            }
            Vec2Expr::SwizzleVec4(box v4, i0, i1) => {
                let span = tracing::trace_span!("match_SwizzleVec4");
                let _span_entered = span.enter();
                if *i0 > 3 || *i1 > 3 {
                    panic!("Please use Vec2Expr::swizzle_vec_4 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1) {
                    *self = Vec2Expr::Gather1(FloatExpr::AccessVec4(Box::new(v4.take_as_owned()), *i0));
                    self.vec2_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v4 {
                    Vec4Expr::Gather1(f0) => {
                        *self = Vec2Expr::Gather1(f0.take_as_owned());
                    }
                    Vec4Expr::Gather4(f0, f1, f2, f3) => {
                        let fs = [f0, f1, f2, f3];
                        *self = Vec2Expr::Gather2(fs[*i0].clone(), fs[*i1].clone());
                    }
                    _ => {}
                }
            }
            Vec2Expr::Truncate3to2(box v3) => {
                let span = tracing::trace_span!("match_Truncate3to2");
                let _span_entered = span.enter();
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                        *self = Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate3to2(Box::new(inner_v3.take_as_owned())), *i0, *i1);
                    }
                    Vec3Expr::Extend2to3(v2, _) => {
                        *self = v2.take_as_owned();
                        return
                    }
                    Vec3Expr::Truncate4to3(box v4) => {
                        *self = Vec2Expr::Truncate4to2(Box::new(v4.take_as_owned()));
                        return
                    }
                    _ => {}
                }
            }
            Vec2Expr::Truncate4to2(box v4) => {
                let span = tracing::trace_span!("match_Truncate4to2");
                let _span_entered = span.enter();
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match v4 {
                    Vec4Expr::Gather1(x) => {
                        *self = Vec2Expr::Gather1(x.take_as_owned());
                    }
                    Vec4Expr::Gather4(x,y, _, _) => {
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
                        *self = Vec2Expr::swizzle_vec_2(Vec2Expr::Truncate4to2(Box::new(inner_v4.take_as_owned())), *i0, *i1);
                    }
                    Vec4Expr::Extend2to4(v2, _, _) => {
                        *self = v2.take_as_owned();
                        return
                    }
                    Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(v4), _) => {
                        // So in total, self matches Truncate4to2(Extend3to4(Truncate4to3(...)))
                        // You would normally think this rare/impossible to occur,
                        // since truncations are driven towards the leaves of the AST and
                        // extensions are pulled towards the root of the AST.
                        // So the sneaky place where this shows up is variable inlining.
                        *self = Vec2Expr::Truncate4to2(Box::new(v4.take_as_owned()));
                        return
                    }
                    Vec4Expr::Extend3to4(v2, _) => {
                        *self = Vec2Expr::Truncate3to2(Box::new(v2.take_as_owned()));
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
        self.vec3_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.vec3_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.slice_to_floats();
        self.vec3_simplify(false, false, true);
    }
    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn vec3_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            Vec3Expr::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec3(mut new_self) = inlined_expr {
                            new_self.vec3_simplify(false, transpose_simd, force_inline_all_variables);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec3Expr::Gather1(ref mut f) => {
                let span = tracing::trace_span!("match_Gather1");
                let _span_entered = span.enter();
                if !insides_already_done {
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                // Do I really want to do more here?
            }
            Vec3Expr::Gather3(ref mut f0, ref mut f1, ref mut f2) => {
                let span = tracing::trace_span!("match_Gather3");
                let _span_entered = span.enter();
                use crate::ast::expressions::FloatExpr::*;
                if !insides_already_done {
                    f0.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f2.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if f0 == f1 && f0 == f2 {
                    *self = Vec3Expr::Gather1(f0.take_as_owned());
                    return;
                }
                match (f0, f1, f2) {
                    (
                        AccessVec4(box ref mut v4_a, x),
                        AccessVec4(box ref mut v4_b, y),
                        AccessVec4(box ref mut v4_c, z)
                    ) if eqs!(*v4_a, *v4_b, *v4_c) => {
                        if v4_a == v4_b && v4_a == v4_c {
                            *self = if *x == 0 && *y == 1 && *z == 2 {
                                Vec3Expr::Truncate4to3(Box::new(v4_a.take_as_owned()))
                            } else if *x < 3 && *y < 3 && *z < 3 {
                                Vec3Expr::swizzle_vec_3(Vec3Expr::Truncate4to3(Box::new(v4_a.take_as_owned())), *x, *y, *z)
                            } else {
                                Vec3Expr::Truncate4to3(Box::new(Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x, *y, *z, 3)))
                            };
                            return;
                        }
                    }
                    (
                        AccessVec3(box ref mut v3_a, x),
                        AccessVec3(box ref mut v3_b, y),
                        AccessVec3(box ref mut v3_c, z)
                    ) if eqs!(*v3_a, *v3_b, *v3_c) => {
                        *self = if *x == 0 && *y == 1 && *z == 2 {
                            v3_a.take_as_owned()
                        } else {
                            Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x, *y, *z)
                        };
                        return;
                    }
                    (AccessVec2(box v2_a, x), AccessVec2(box v2_b, y), z) if eqs!(*v2_a, *v2_b) => {
                        let mut v3 = Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x, *y);
                        v3.vec2_simplify(true, transpose_simd, force_inline_all_variables);
                        *self = Vec3Expr::Extend2to3(v3, z.take_as_owned());
                        return;
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                    ) if eqs!(x_mve, y_mve, z_mve) && min!(*x_idx, *y_idx, *z_idx) + 3 >= max!(*x_idx, *y_idx, *z_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx, *z_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx, *z_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            let z = *z_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 3) => Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                (_, 4) => Vec3Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z),
                                (false, 3) => Vec3Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z),
                                (false, 2) => Vec3Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec3Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)),
                                _ => return
                            };
                            self.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            return
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        z,
                    ) if eqs!(x_mve, y_mve) && min!(*x_idx, *y_idx) + 3 >= max!(*x_idx, *y_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 2) => Vec3Expr::Extend2to3(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), z.take_as_owned()),
                                (_, 4) => Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned()),
                                (_, 3) => Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned()),
                                (false, 2) => Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned()),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec3Expr::Extend2to3(Vec2Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)), z.take_as_owned()),
                                _ => return
                            };
                            self.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            return
                        }
                    }
                    (Literal(x), Literal(y), z) if *x == *y => {
                        *self = Vec3Expr::Extend2to3(Vec2Expr::Gather1(Literal(*x)), z.take_as_owned())
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = vec3_product_transpose(None, x_product, y_product, z_product, lits) {
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
                        if let Some(transposed) = vec3_product_transpose(None, &mut x, y_product, z_product, lits) {
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
                        if let Some(transposed) = vec3_product_transpose(None, x_product, &mut y, z_product, lits) {
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
                        if let Some(transposed) = vec3_product_transpose(None, x_product, y_product, &mut zv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_product_transpose(None, x_product, y_product, lits) {
                            *self = Vec3Expr::Extend2to3(transposed, z.take_as_owned());
                            return
                        }
                    }
                    (x, y, Product(ref mut z_product, z_lit)) if transpose_simd => {
                        let lits = [1.0, 1.0, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = vec3_product_transpose(None, &mut x, &mut y, z_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit), z) if transpose_simd => {
                        let lits = [1.0, *y_lit, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec3_product_transpose(None, &mut x, y_product, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y, z) if transpose_simd => {
                        let lits = [*x_lit, 1.0, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec3_product_transpose(None, x_product, &mut y, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (
                        Sum(ref mut x_sum, x_lit),
                        Sum(ref mut y_sum, y_lit),
                        Sum(ref mut z_sum, z_lit)
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = vec3_sum_transpose(None, x_sum, y_sum, z_sum, lits) {
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
                        if let Some(transposed) = vec3_sum_transpose(None, &mut x, y_sum, z_sum, lits) {
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
                        if let Some(transposed) = vec3_sum_transpose(None, x_sum, &mut y, z_sum, lits) {
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
                        if let Some(transposed) = vec3_sum_transpose(None, x_sum, y_sum, &mut zv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_sum_transpose(None, x_sum, y_sum, lits) {
                            *self = Vec3Expr::Extend2to3(transposed, z.take_as_owned());
                            return
                        }
                    }
                    (x, y, Sum(ref mut z_sum, z_lit)) if transpose_simd => {
                        let lits = [0.0, 0.0, *z_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        if let Some(transposed) = vec3_sum_transpose(None, &mut x, &mut y, z_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit), z) if transpose_simd => {
                        let lits = [0.0, *y_lit, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec3_sum_transpose(None, &mut x, y_sum, &mut z, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y, z) if transpose_simd => {
                        let lits = [*x_lit, 0.0, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec3_sum_transpose(None, x_sum, &mut y, &mut z, lits) {
                            *self = transposed;
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
                let span = tracing::trace_span!("match_Extend2to3");
                let _span_entered = span.enter();
                if !insides_already_done {
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match (v2, f1) {
                    (Vec2Expr::Gather1(x), z) if x.is_memory_read_and_not_compute() => {
                        *self = Vec3Expr::Gather3(x.clone(), x.take_as_owned(), z.take_as_owned());
                        self.vec3_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    (Vec2Expr::Gather2(x, y), z) => {
                        *self = Vec3Expr::Gather3(x.take_as_owned(), y.take_as_owned(), z.take_as_owned());
                        self.vec3_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    _ => {}
                }
            }
            Vec3Expr::AccessMultiVecGroup(ref mut mve, ref mut idx) => {
                let span = tracing::trace_span!("match_AccessMultiVecGroup");
                let _span_entered = span.enter();
                if !insides_already_done {
                    mve.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx] {
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
                let span = tracing::trace_span!("match_Product");
                let _span_entered = span.enter();
                if product.is_empty() {
                    panic!("Please use Vec3Expr::product so you can find out where you constructed something wrong");
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 3] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }

                let mut gather1 = vec![];
                let mut extend2to3_xy = vec![];
                let mut gather3_x = vec![];
                let mut gather3_y = vec![];
                let mut gather3_z = vec![];

                let mut start_idx = 0;
                while start_idx < product.len() {
                    let mut iter_idx = 0;
                    let mut flatten = vec![];
                    product.retain_mut(|(factor, exponent)| {
                        if iter_idx < start_idx {
                            iter_idx += 1;
                            return true
                        }
                        match factor {
                            Vec3Expr::Gather1(f) => {
                                match f {
                                    FloatExpr::Literal(f) => {
                                        let powf = f32::powf(*f, *exponent);
                                        last_factor[0] *= powf;
                                        last_factor[1] *= powf;
                                        last_factor[2] *= powf;
                                    }
                                    _ => gather1.push((f.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec3Expr::Gather3(x, y, z) => {
                                match x {
                                    FloatExpr::Literal(x) => last_factor[0] *= f32::powf(*x, *exponent),
                                    _ => gather3_x.push((x.take_as_owned(), *exponent)),
                                }
                                match y {
                                    FloatExpr::Literal(y) => last_factor[1] *= f32::powf(*y, *exponent),
                                    _ => gather3_y.push((y.take_as_owned(), *exponent)),
                                }
                                match z {
                                    FloatExpr::Literal(z) => last_factor[2] *= f32::powf(*z, *exponent),
                                    _ => gather3_z.push((z.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec3Expr::Extend2to3(xy, z) => {
                                // Do not need to further match on xy because
                                // xy + z would simplify to Gather3 when we care about it.
                                extend2to3_xy.push((xy.take_as_owned(), *exponent));
                                match z {
                                    FloatExpr::Literal(z) => last_factor[2] *= f32::powf(*z, *exponent),
                                    _ => gather3_z.push((z.take_as_owned(), *exponent)),
                                }
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
                        }
                    });
                    start_idx = product.len();
                    product.append(&mut flatten);
                }

                if *last_factor == [0.0; 3] {
                    *self = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }

                let x = last_factor[0];
                let y = last_factor[1];
                let z = last_factor[2];
                if x == 0.0 { gather3_x.clear(); }
                if y == 0.0 { gather3_y.clear(); }
                if z == 0.0 { gather3_z.clear(); }
                if x == 0.0 && y == 0.0 { extend2to3_xy.clear(); }

                macro_rules! default_coefficient {
                    ($i:expr) => {
                        FloatExpr::Literal(if last_factor[$i] == 0.0 { 0.0 } else { 1.0 })
                    };
                }
                macro_rules! swap_take {
                    ($var:ident, $replacement:expr) => {
                        {
                            let mut x = $replacement;
                            mem::swap(&mut x, &mut $var);
                            x
                        }
                    };
                }
                macro_rules! mul_coefficient {
                    ($float_expr:expr, $k:expr) => {
                        if $k != 1.0 {
                            match &mut $float_expr {
                                FloatExpr::Product(_, c) => {
                                    c.mul_assign($k);
                                }
                                _ => {
                                    let f = $float_expr.take_as_owned();
                                    $float_expr = FloatExpr::Product(vec![(f, 1.0)], $k);
                                    $float_expr.float_simplify(true, transpose_simd, force_inline_all_variables);
                                }
                            }
                        }
                    }
                }
                if eqs!(x, y, z) && !gather1.is_empty() {
                    let gather1 = swap_take!(gather1, vec![]);
                    let mut f = FloatExpr::product(gather1, x);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    product.push((Vec3Expr::Gather1(f), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                    last_factor[2] = 1.0;
                } else if eqs!(x, y) && !extend2to3_xy.is_empty() {
                    extend2to3_xy.push((Vec2Expr::Gather1(FloatExpr::Literal(x)), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                }

                let mut leftover_z = default_coefficient!(2);

                match (gather3_x.is_empty(), gather3_y.is_empty(), gather3_z.is_empty()) {
                    (false, false, true) => {
                        let x = swap_take!(gather3_x, vec![]);
                        let y = swap_take!(gather3_y, vec![]);
                        let gather2 = Vec2Expr::Gather2(FloatExpr::product(x, 1.0), FloatExpr::product(y, 1.0));
                        extend2to3_xy.push((gather2, 1.0));
                    }
                    (true, true, false) if !extend2to3_xy.is_empty() => {
                        let z = swap_take!(gather3_z, vec![]);
                        leftover_z = FloatExpr::product(z, 1.0);
                        leftover_z.float_simplify(true, transpose_simd, force_inline_all_variables);
                    }
                    _ => {}
                }

                let is_any_gather3 = !gather3_x.is_empty() || !gather3_y.is_empty() || !gather3_z.is_empty();
                let mut is_only_2to3 = !extend2to3_xy.is_empty() && product.is_empty() && gather1.is_empty() && gather3_x.is_empty() && gather3_y.is_empty();
                let mut is_only_gather3 = is_any_gather3 && product.is_empty() && gather1.is_empty() && extend2to3_xy.is_empty();

                let mut x_is_zeroed_without_last_factor = false;
                let mut y_is_zeroed_without_last_factor = false;
                let mut z_is_zeroed_without_last_factor = false;

                if !gather1.is_empty() {
                    let mut f = FloatExpr::product(gather1, 1.0);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if z == 0.0 {
                        extend2to3_xy.push((Vec2Expr::Gather1(f), 1.0));
                        is_only_2to3 = !extend2to3_xy.is_empty() && product.is_empty() && gather3_x.is_empty() && gather3_y.is_empty();
                        is_only_gather3 = is_any_gather3 && product.is_empty() && extend2to3_xy.is_empty();
                    } else {
                        product.push((Vec3Expr::Gather1(f), 1.0));
                    }
                }
                if !extend2to3_xy.is_empty() {
                    let mut xy_coefficient = [1.0; 2];
                    if is_only_2to3 {
                        xy_coefficient = [last_factor[0], last_factor[1]];
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                        mul_coefficient!(leftover_z, last_factor[2]);
                        last_factor[2] = 1.0;
                    }
                    let mut vec2_products = Vec2Expr::product(extend2to3_xy, xy_coefficient);
                    vec2_products.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let Vec2Expr::Gather1(FloatExpr::Literal(0.0)) = &vec2_products {
                        x_is_zeroed_without_last_factor = true;
                        y_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &leftover_z {
                        z_is_zeroed_without_last_factor = true;
                    }
                    product.push((Vec3Expr::Extend2to3(vec2_products, leftover_z), 1.0));
                }
                if is_any_gather3 {
                    let mut x = if gather3_x.is_empty() { default_coefficient!(0) } else { FloatExpr::Product(gather3_x, 1.0) };
                    let mut y = if gather3_y.is_empty() { default_coefficient!(1) } else { FloatExpr::Product(gather3_y, 1.0) };
                    let mut z = if gather3_z.is_empty() { default_coefficient!(2) } else { FloatExpr::Product(gather3_z, 1.0) };
                    x.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    y.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    z.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let FloatExpr::Literal(0.0) = &x {
                        x_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &y {
                        y_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &z {
                        z_is_zeroed_without_last_factor = true;
                    }
                    if is_only_gather3 {
                        product.push((Vec3Expr::Gather3(
                            x * last_factor[0],
                            y * last_factor[1],
                            z * last_factor[2],
                        ), 1.0));
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                        last_factor[2] = 1.0;
                    } else {
                        product.push((Vec3Expr::Gather3(x, y, z), 1.0));
                    }
                }
                if (last_factor[0] == 1.0 || x_is_zeroed_without_last_factor) &&
                    (last_factor[1] == 1.0 || y_is_zeroed_without_last_factor) &&
                    (last_factor[2] == 1.0 || z_is_zeroed_without_last_factor) {
                    *last_factor = [1.0; 3];
                }

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

                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    *self = if eqs!(f0, f1, f2) {
                        Vec3Expr::Gather1(f0)
                    } else {
                        Vec3Expr::Gather3(f0, f1, f2)
                    };
                }
            }
            Vec3Expr::Sum(ref mut sum, last_addend) => {
                let span = tracing::trace_span!("match_Sum");
                let _span_entered = span.enter();
                if sum.is_empty() {
                    panic!("Please use Vec3Expr::sum so you can find out where you constructed something wrong");
                }
                for (addend, _factor) in sum.iter_mut() {
                    if !insides_already_done {
                        addend.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 3] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec3Expr::product(vec![(addend, 1.0)], [factor, factor, factor]);
                        new_self.vec3_simplify(true, transpose_simd, force_inline_all_variables);
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
                            self.vec3_simplify(false, transpose_simd, force_inline_all_variables);
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
            Vec3Expr::SwizzleVec2(v2, i0, i1, i2) => {
                let span = tracing::trace_span!("match_SwizzleVec2");
                let _span_entered = span.enter();
                if *i0 > 1 || *i1 > 1 || *i2 > 1 {
                    panic!("Please use Vec3Expr::swizzle_vec_2 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1, *i2) {
                    *self = Vec3Expr::Gather1(FloatExpr::AccessVec2(Box::new(v2.take_as_owned()), *i0));
                    self.vec3_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v2 {
                    Vec2Expr::Gather1(f0) => {
                        *self = Vec3Expr::Gather1(f0.take_as_owned());
                    }
                    Vec2Expr::Gather2(f0, f1) if eqs!(*i0, *i1, *i2) => {
                        let fs = [f0, f1];
                        *self = Vec3Expr::Gather1(fs[*i0].clone());
                    }
                    Vec2Expr::Gather2(f0, f1) => {
                        let fs = [f0, f1];
                        *self = Vec3Expr::Gather3(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone());
                    }
                    _ => {}
                }
            }
            Vec3Expr::SwizzleVec3(box v3, i0, i1, i2) => {
                let span = tracing::trace_span!("match_SwizzleVec3");
                let _span_entered = span.enter();
                if *i0 > 2 || *i1 > 2 || *i2 > 2 {
                    panic!("Please use Vec3Expr::swizzle_vec_3 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if *i0 == 0 && *i1 == 1 && *i2 == 2 {
                    *self = v3.take_as_owned();
                    return;
                }
                if eqs!(*i0, *i1, *i2) {
                    *self = Vec3Expr::Gather1(FloatExpr::AccessVec3(Box::new(v3.take_as_owned()), *i0));
                    self.vec3_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v3 {
                    Vec3Expr::Gather1(f0) => {
                        *self = Vec3Expr::Gather1(f0.take_as_owned());
                    }
                    Vec3Expr::Gather3(f0, f1, f2) => {
                        let fs = [f0, f1, f2];
                        *self = Vec3Expr::Gather3(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone());
                    }
                    Vec3Expr::Extend2to3(v, z) if *i0 == 0 && *i1 == 1 => {
                        *self = Vec3Expr::Extend2to3(v.take_as_owned(), z.take_as_owned());
                    }
                    Vec3Expr::Extend2to3(v, z) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec3Expr::Extend2to3(Vec2Expr::swizzle_vec_2(v.take_as_owned(), *i0, *i1), z.take_as_owned());
                    }
                    _ => {}
                }
            }
            Vec3Expr::SwizzleVec4(box v4, i0, i1, i2) => {
                let span = tracing::trace_span!("match_SwizzleVec4");
                let _span_entered = span.enter();
                if *i0 > 3 || *i1 > 3 || *i2 >32 {
                    panic!("Please use Vec3Expr::swizzle_vec_4 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1, *i2) {
                    *self = Vec3Expr::Gather1(FloatExpr::AccessVec4(Box::new(v4.take_as_owned()), *i0));
                    self.vec3_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v4 {
                    Vec4Expr::Gather1(f0) => {
                        *self = Vec3Expr::Gather1(f0.take_as_owned());
                    }
                    Vec4Expr::Gather4(f0, f1, f2, f3) if eqs!(*i0, *i1, *i2) && *i0 < 3 => {
                        let fs = [f0, f1, f2, f3];
                        *self = Vec3Expr::Gather1(fs[*i0].clone());
                    }
                    Vec4Expr::Gather4(f0, f1, f2, f3) => {
                        let fs = [f0, f1, f2, f3];
                        *self = Vec3Expr::Gather3(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone());
                    }
                    _ => {}
                }
            }
            Vec3Expr::Truncate4to3(box v4) => {
                let span = tracing::trace_span!("match_Truncate4to3");
                let _span_entered = span.enter();
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
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
                        *self = Vec3Expr::swizzle_vec_3(Vec3Expr::Truncate4to3(Box::new(inner_v4.take_as_owned())), *i0, *i1, *i2);
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
        self.vec4_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.vec4_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.slice_to_floats();
        self.vec4_simplify(false, false, true);
    }
    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn vec4_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            Vec4Expr::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                // TODO convert all the strong_count uses to into_inner instead
                //  Arc::into_inner(decl)
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Vec4(mut new_self) = inlined_expr {
                            new_self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            Vec4Expr::Gather1(f) => {
                let span = tracing::trace_span!("match_Gather1");
                let _span_entered = span.enter();
                if !insides_already_done {
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                // Do I really want to do more here?
            }
            Vec4Expr::Gather4(f0, f1, f2, f3) => {
                // TODO rare instance of latest simplification updates making code less optimal
                //  impl AntiProjectOrthogonallyOnto<Flector> for DualNum {
                //  impl AntiProjectOrthogonallyOnto<Motor> for AntiScalar {
                //  From:
                //  (anti_wedge_g0 * Simd32x4::from(other[scalar]))
                //      + Simd32x3::from(0.0).with_w(-(anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12])),
                //  To:
                //  Simd32x4::from([
                //      other[scalar],
                //      other[scalar],
                //      other[scalar],
                //      (anti_wedge_g0[3] * other[scalar]) - (anti_wedge_g0[0] * other[e23]) - (anti_wedge_g0[1] * other[e31]) - (anti_wedge_g0[2] * other[e12]),
                //  ]) * anti_wedge_g0.xyz().with_w(1.0),

                // TODO maybe extend3to4 would be useful here
                //  impl AntiProjectOrthogonallyOnto<Point> for Horizon {
                //  impl AntiProjectOrthogonallyOnto<Flector> for Line {


                let span = tracing::trace_span!("match_Gather4");
                let _span_entered = span.enter();
                use crate::ast::expressions::FloatExpr::*;
                // println!("simplify Vec4Expr::Gather4 BEFORE: {f0:?} {f1:?} {f2:?} {f3:?}");
                if !insides_already_done {
                    f0.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f2.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f3.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                // println!("simplify Vec4Expr::Gather4 AFTER: {f0:?} {f1:?} {f2:?} {f3:?}");
                if eqs!(f0, f1, f2, f3) {
                    *self = Vec4Expr::Gather1(f0.take_as_owned());
                    return;
                }
                tracing::trace!("attempting match on ({f0:?}, {f1:?}, {f2:?}, {f3:?})");
                match (f0, f1, f2, f3) {
                    (AccessVec4(box v4_a, x), AccessVec4(box v4_b, y), AccessVec4(box v4_c, z), AccessVec4(box v4_d, w)) if eqs!(v4_a, v4_b, v4_c, v4_d) => {
                        *self = if *x == 0 && *y == 1 && *z == 2 && *w == 3 {
                            v4_a.take_as_owned()
                        } else {
                            Vec4Expr::swizzle_vec_4(v4_a.take_as_owned(), *x, *y, *z, *w)
                        };
                        return;
                    }
                    (AccessVec3(box v3_a, x), AccessVec3(box v3_b, y), AccessVec3(box v3_c, z), w) if eqs!(v3_a, v3_b, v3_c) => {
                        let mut v3 = Vec3Expr::swizzle_vec_3(v3_a.take_as_owned(), *x, *y, *z);
                        v3.vec3_simplify(true, transpose_simd, force_inline_all_variables);
                        *self = Vec4Expr::Extend3to4(v3, w.take_as_owned());
                        return;
                    }
                    (AccessVec2(box v2_a, x), AccessVec2(box v2_b, y), z, w) if eqs!(v2_a, v2_b) => {
                        let mut v3 = Vec2Expr::swizzle_vec_2(v2_a.take_as_owned(), *x, *y);
                        v3.vec2_simplify(true, transpose_simd, force_inline_all_variables);
                        *self = Vec4Expr::Extend2to4(v3, z.take_as_owned(), w.take_as_owned());
                        return;
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                        AccessMultiVecFlat(w_mve, w_idx),
                    ) if eqs!(x_mve, y_mve, z_mve, w_mve) && min!(*x_idx, *y_idx, *z_idx, *w_idx) + 3 >= max!(*x_idx, *y_idx, *z_idx, *w_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx, *z_idx, *w_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx, *z_idx, *w_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx) && (*x_idx + 3 == *w_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            let z = *z_idx - flat_idx;
                            let w = *w_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 4) => Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx),
                                (false, 4) => Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z, w),
                                (false, 3) => Vec4Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z, w),
                                (false, 2) => Vec4Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z, w),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec4Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)),
                                _ => return
                            };
                            self.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            tracing::trace!("Found matching group: {:?}", self);
                            return
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        AccessMultiVecFlat(z_mve, z_idx),
                        w,
                    ) if eqs!(x_mve, y_mve, z_mve) && min!(*x_idx, *y_idx, *z_idx) + 3 >= max!(*x_idx, *y_idx, *z_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx, *z_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx, *z_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = (*x_idx + 1 == *y_idx) && (*x_idx + 2 == *z_idx);
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            let z = *z_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 3) => Vec4Expr::Extend3to4(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), w.take_as_owned()),
                                (_, 4) => Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z), w.take_as_owned()),
                                (false, 3) => Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z), w.take_as_owned()),
                                (false, 2) => Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y, z), w.take_as_owned()),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec4Expr::Extend3to4(Vec3Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)), w.take_as_owned()),
                                _ => return
                            };
                            tracing::trace!("Extend3to4 result: {self:?}");
                            self.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            tracing::trace!("Extend3to4 simplified result: {self:?}");
                            return
                        }
                    }
                    (
                        AccessMultiVecFlat(x_mve, x_idx),
                        AccessMultiVecFlat(y_mve, y_idx),
                        z,
                        w,
                    ) if eqs!(x_mve, y_mve) && min!(*x_idx, *y_idx) + 3 >= max!(*x_idx, *y_idx) => {
                        let max_flat_idx = max!(*x_idx, *y_idx);
                        let min_flat_idx = min!(*x_idx, *y_idx);
                        let required_width = (max_flat_idx - min_flat_idx) + 1;
                        let no_swizzle = *x_idx + 1 == *y_idx;
                        let mut group_idx = 0;
                        let mut flat_idx = 0;
                        for group in x_mve.mv_class.groups().into_iter() {
                            let group_is_too_late = flat_idx > min_flat_idx;
                            if group_is_too_late {
                                tracing::trace!(group_is_too_late);
                                return
                            }
                            let group_is_too_early = (flat_idx + (group.simd_width() - 1)) < max_flat_idx;
                            let group_is_too_narrow = group.simd_width() < required_width;
                            if group_is_too_early || group_is_too_narrow {
                                tracing::trace!(group_is_too_early, group_is_too_narrow);
                                group_idx = group_idx + 1;
                                flat_idx = flat_idx + group.simd_width();
                                continue;
                            }
                            let x = *x_idx - flat_idx;
                            let y = *y_idx - flat_idx;
                            *self = match (no_swizzle, group.simd_width()) {
                                (true, 2) => Vec4Expr::Extend2to4(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), z.take_as_owned(), w.take_as_owned()),
                                (_, 4) => Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned(), w.take_as_owned()),
                                (_, 3) => Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned(), w.take_as_owned()),
                                (false, 2) => Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(x_mve.take_as_owned(), group_idx), x, y), z.take_as_owned(), w.take_as_owned()),
                                (false, 1) if !x_mve.is_memory_read_and_not_compute() => Vec4Expr::Extend2to4(Vec2Expr::Gather1(AccessMultiVecFlat(x_mve.take_as_owned(), *x_idx)), z.take_as_owned(), w.take_as_owned()),
                                _ => return
                            };
                            self.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                            return
                        }
                    }
                    (Literal(x), Literal(y), Literal(z), w) if eqs!(x, y, z) => {
                        *self = Vec4Expr::Extend3to4(Vec3Expr::Gather1(Literal(*x)), w.take_as_owned());
                    }
                    (Literal(x), Literal(y), z, w) if eqs!(x, y) => {
                        *self = Vec4Expr::Extend2to4(Vec2Expr::Gather1(Literal(*x)), z.take_as_owned(), w.take_as_owned())
                    }
                    (
                        Product(ref mut x_product, x_lit),
                        Product(ref mut y_product, y_lit),
                        Product(ref mut z_product, z_lit),
                        Product(ref mut w_product, w_lit),
                    ) if transpose_simd => {
                        let lits = [*x_lit, *y_lit, *z_lit, *w_lit];
                        if let Some(transposed) = vec4_product_transpose(None, x_product, y_product, z_product, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, x_product, y_product, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, &mut y, z_product, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, y_product, &mut z, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, y_product, z_product, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = vec3_product_transpose(None, x_product, y_product, z_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, &mut y, z_product, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, y_product, &mut zv, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_product_transpose(None, x_product, y_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, y_product, z_product, &mut w, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, &mut y, &mut z, w_product, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, x_product, &mut y, z_product, &mut w, lits) {
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
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, y_product, &mut z, w_product, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, z, Product(ref mut w_product, w_lit)) if transpose_simd => {
                        // println!("simplify (will try transpose) Vec4Expr::Gather4(x, y, z, Product(w_product, w_lit))");
                        let lits = [1.0, 1.0, 1.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, &mut y, &mut z, w_product, lits) {
                            // println!("yes transposed");
                            *self = transposed;
                        } else {
                            // println!("no transposed");
                        }
                    }
                    (x, y, Product(ref mut z_product, z_lit), w) if transpose_simd => {
                        let lits = [1.0, 1.0, *z_lit, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, &mut y, z_product, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Product(ref mut y_product, y_lit), z, w) if transpose_simd => {
                        let lits = [1.0, *y_lit, 1.0, 1.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_product_transpose(None, &mut x, y_product, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (Product(ref mut x_product, x_lit), y, z, w) if transpose_simd => {
                        let lits = [*x_lit, 1.0, 1.0, 1.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_product_transpose(None, x_product, &mut y, &mut z, &mut w, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, y_sum, z_sum, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, x_sum, y_sum, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, &mut y, z_sum, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, y_sum, &mut z, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, y_sum, z_sum, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit, *z_lit];
                        if let Some(transposed) = vec3_sum_transpose(None, x_sum, y_sum, z_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, &mut y, z_sum, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, y_sum, &mut zv, &mut wv, lits) {
                            *self = transposed;
                            return
                        }
                        let lits = [*x_lit, *y_lit];
                        if let Some(transposed) = vec2_sum_transpose(None, x_sum, y_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, y_sum, z_sum, &mut w, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, &mut y, &mut z, w_sum, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, &mut y, z_sum, &mut w, lits) {
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
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, y_sum, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, z, Sum(ref mut w_sum, w_lit)) if transpose_simd => {
                        let lits = [0.0, 0.0, 0.0, *w_lit];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, &mut y, &mut z, w_sum, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, Sum(ref mut z_sum, z_lit), w) if transpose_simd => {
                        let lits = [0.0, 0.0, *z_lit, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, &mut y, z_sum, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (x, Sum(ref mut y_sum, y_lit), z, w) if transpose_simd => {
                        let lits = [0.0, *y_lit, 0.0, 0.0];
                        let mut x = vec![(x.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_sum_transpose(None, &mut x, y_sum, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (Sum(ref mut x_sum, x_lit), y, z, w) if transpose_simd => {
                        let lits = [*x_lit, 0.0, 0.0, 0.0];
                        let mut y = vec![(y.clone(), 1.0)];
                        let mut z = vec![(z.clone(), 1.0)];
                        let mut w = vec![(w.clone(), 1.0)];
                        if let Some(transposed) = vec4_sum_transpose(None, x_sum, &mut y, &mut z, &mut w, lits) {
                            *self = transposed;
                        }
                    }
                    (x, y, z, w) if eqs!(x, y, z) => {
                        *self = Vec4Expr::Extend3to4(
                            Vec3Expr::Gather1(x.take_as_owned()),
                            w.take_as_owned(),
                        );
                        return
                    }
                    (x, y, z, w) if eqs!(x, y) => {
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
                let span = tracing::trace_span!("match_Extend2to4");
                let _span_entered = span.enter();
                if !insides_already_done {
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f2.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                match (v2, f1, f2) {
                    (Vec2Expr::Gather1(x), z, w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    (Vec2Expr::Gather2(x, y), z, w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    _ => {}
                }
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                // TODO impl AntiProjectViaHorizonOnto<Flector> for DualNum {
                //  // e41, e42, e43, e1234
                //  Before:
                //  (other.group0().wwwx() * Simd32x3::from(0.0).with_w(anti_wedge_g1_xyz[0]))
                //      + Simd32x3::from(0.0).with_w((anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]))
                //      - (Simd32x3::from(0.0).with_w(anti_wedge_g0_w).wwwx() * other.group0().xyz().with_w(other[e423])),
                //  After:
                //  Simd32x4::from([
                //      anti_wedge_g0_w * other[e1],
                //      anti_wedge_g0_w * other[e2],
                //      anti_wedge_g0_w * other[e3],
                //      (anti_wedge_g1_xyz[0] * other[e1]) + (anti_wedge_g1_xyz[1] * other[e2]) + (anti_wedge_g1_xyz[2] * other[e3]) - (anti_wedge_g0_w * other[e321]),
                //  ]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),

                let span = tracing::trace_span!("match_Extend3to4");
                let _span_entered = span.enter();
                // println!("simplify Vec4Expr::Extend3to4 BEFORE: {v3:?} {f1:?}");
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    f1.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                // println!("simplify Vec4Expr::Extend3to4 AFTER: {v3:?} {f1:?}");
                match (v3, f1) {
                    (Vec3Expr::Gather1(x), w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.clone(), x.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    (Vec3Expr::Gather3(x, y, z), w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    (Vec3Expr::Extend2to3(Vec2Expr::Gather2(x, y), z), w) => {
                        *self = Vec4Expr::Gather4(x.take_as_owned(), y.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    (Vec3Expr::Extend2to3(Vec2Expr::Gather1(x), z), w) if x.is_memory_read_and_not_compute() => {
                        *self = Vec4Expr::Gather4(x.clone(), x.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                        self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
                        return
                    }
                    _ => {}
                }
            }
            Vec4Expr::AccessMultiVecGroup(mve, idx) => {
                let span = tracing::trace_span!("match_AccessMultiVecGroup");
                let _span_entered = span.enter();
                if !insides_already_done {
                    mve.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                let idx = *idx;

                // Not actually unused, it is potentially used by panic a few lines later
                // Please be smarter cargo
                #[allow(unused_variables)]
                let mv = mve.mv_class;

                if let MultiVectorVia::Construct(groups) = mve.expr.as_mut() {
                    let size = match &mut groups[idx] {
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

                // TODO impl GeometricProduct<Motor> for MultiVector {
                //  + (self.group4().ww().with_zw(self[e2], other[e1234] * self[e321]) * other.group1().xyx().with_w(1.0))
                //  + (self.group4().ww().with_zw(self[e2], other[scalar] * self[e321]) * other.group0().xyx().with_w(1.0))

                let span = tracing::trace_span!("match_Product");
                let _span_entered = span.enter();
                if product.is_empty() {
                    panic!("Please use Vec4Expr::product so you can find out where you constructed something wrong");
                }
                for (factor, _exponent) in product.iter_mut() {
                    if !insides_already_done {
                        factor.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if product.len() == 1 && *last_factor == [1.0; 4] {
                    if product[0].1 == 1.0 {
                        let (factor, _exponent) = product.remove(0);
                        *self = factor;
                        return;
                    }
                }

                let mut gather1 = vec![];
                let mut extend3to4_xyz = vec![];
                let mut extend2to4_xy = vec![];
                let mut gather4_x = vec![];
                let mut gather4_y = vec![];
                let mut gather4_z = vec![];
                let mut gather4_w = vec![];

                let mut start_idx = 0;
                while start_idx < product.len() {
                    let mut iter_idx = 0;
                    let mut flatten = vec![];
                    product.retain_mut(|(factor, exponent)| {
                        if iter_idx < start_idx {
                            iter_idx += 1;
                            return true
                        }
                        match factor {
                            Vec4Expr::Gather1(f) => {
                                match f {
                                    FloatExpr::Literal(f) => {
                                        let powf = f32::powf(*f, *exponent);
                                        last_factor[0] *= powf;
                                        last_factor[1] *= powf;
                                        last_factor[2] *= powf;
                                        last_factor[3] *= powf;
                                    }
                                    _ => gather1.push((f.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec4Expr::Gather4(x, y, z, w) => {
                                match x {
                                    FloatExpr::Literal(x) => last_factor[0] *= f32::powf(*x, *exponent),
                                    _ => gather4_x.push((x.take_as_owned(), *exponent)),
                                }
                                match y {
                                    FloatExpr::Literal(y) => last_factor[1] *= f32::powf(*y, *exponent),
                                    _ => gather4_y.push((y.take_as_owned(), *exponent)),
                                }
                                match z {
                                    FloatExpr::Literal(z) => last_factor[2] *= f32::powf(*z, *exponent),
                                    _ => gather4_z.push((z.take_as_owned(), *exponent)),
                                }
                                match w {
                                    FloatExpr::Literal(w) => last_factor[3] *= f32::powf(*w, *exponent),
                                    _ => gather4_w.push((w.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec4Expr::Extend3to4(xyz, w) => {
                                // Do not need to further match on xyz because
                                // xyz + w would simplify to Gather4 when we care about it.
                                extend3to4_xyz.push((xyz.take_as_owned(), *exponent));
                                match w {
                                    FloatExpr::Literal(w) => last_factor[3] *= f32::powf(*w, *exponent),
                                    _ => gather4_w.push((w.take_as_owned(), *exponent)),
                                }
                                false
                            }
                            Vec4Expr::Extend2to4(xy, z, w) => {
                                // Do not need to further match on xy because
                                // xy + z + w would simplify to Gather4 when we care about it.
                                extend2to4_xy.push((xy.take_as_owned(), *exponent));
                                match z {
                                    FloatExpr::Literal(z) => last_factor[2] *= f32::powf(*z, *exponent),
                                    _ => gather4_z.push((z.take_as_owned(), *exponent)),
                                }
                                match w {
                                    FloatExpr::Literal(w) => last_factor[3] *= f32::powf(*w, *exponent),
                                    _ => gather4_w.push((w.take_as_owned(), *exponent)),
                                }
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
                        }
                    });
                    start_idx = product.len();
                    product.append(&mut flatten);
                }

                if *last_factor == [0.0; 4] {
                    *self = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
                    return
                }

                let x = last_factor[0];
                let y = last_factor[1];
                let z = last_factor[2];
                let w = last_factor[3];
                if x == 0.0 { gather4_x.clear(); }
                if y == 0.0 { gather4_y.clear(); }
                if z == 0.0 { gather4_z.clear(); }
                if w == 0.0 { gather4_w.clear(); }
                if x == 0.0 && y == 0.0 { extend2to4_xy.clear(); }
                if x == 0.0 && y == 0.0 && z == 0.0 { extend3to4_xyz.clear(); }

                macro_rules! default_coefficient {
                    ($i:expr) => {
                        FloatExpr::Literal(if last_factor[$i] == 0.0 { 0.0 } else { 1.0 })
                    };
                }
                macro_rules! swap_take {
                    ($var:ident, $replacement:expr) => {
                        {
                            let mut x = $replacement;
                            mem::swap(&mut x, &mut $var);
                            x
                        }
                    };
                }
                macro_rules! mul_coefficient {
                    ($float_expr:expr, $k:expr) => {
                        if $k != 1.0 {
                            match &mut $float_expr {
                                FloatExpr::Product(_, c) => {
                                    c.mul_assign($k);
                                }
                                _ => {
                                    let f = $float_expr.take_as_owned();
                                    $float_expr = FloatExpr::Product(vec![(f, 1.0)], $k);
                                    $float_expr.float_simplify(true, transpose_simd, force_inline_all_variables);
                                }
                            }
                        }
                    }
                }

                if eqs!(x, y, z, w) && !gather1.is_empty() {
                    let gather1 = swap_take!(gather1, vec![]);
                    let mut f = FloatExpr::product(gather1, x);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    product.push((Vec4Expr::Gather1(f), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                    last_factor[2] = 1.0;
                    last_factor[3] = 1.0;
                } else if eqs!(x, y, z) && !extend3to4_xyz.is_empty() {
                    extend3to4_xyz.push((Vec3Expr::Gather1(FloatExpr::Literal(x)), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                    last_factor[2] = 1.0;
                } else if eqs!(x, y) && !extend2to4_xy.is_empty() {
                    extend2to4_xy.push((Vec2Expr::Gather1(FloatExpr::Literal(x)), 1.0));
                    last_factor[0] = 1.0;
                    last_factor[1] = 1.0;
                }

                let mut leftover_z = default_coefficient!(2);
                let mut leftover_w = default_coefficient!(3);

                match (gather4_x.is_empty(), gather4_y.is_empty(), gather4_z.is_empty(), gather4_w.is_empty()) {
                    (false, false, true, true) => {
                        let x = swap_take!(gather4_x, vec![]);
                        let y = swap_take!(gather4_y, vec![]);
                        let gather2 = Vec2Expr::Gather2(FloatExpr::product(x, 1.0), FloatExpr::product(y, 1.0));
                        extend2to4_xy.push((gather2, 1.0));
                    }
                    (false, false, false, true) => {
                        let x = swap_take!(gather4_x, vec![]);
                        let y = swap_take!(gather4_y, vec![]);
                        let z = swap_take!(gather4_z, vec![]);
                        let gather3 = Vec3Expr::Gather3(FloatExpr::product(x, 1.0), FloatExpr::product(y, 1.0), FloatExpr::product(z, 1.0));
                        extend3to4_xyz.push((gather3, 1.0));
                    }
                    (true, true, false, false) if !extend2to4_xy.is_empty() => {
                        let z = swap_take!(gather4_z, vec![]);
                        let w = swap_take!(gather4_w, vec![]);
                        leftover_z = FloatExpr::product(z, 1.0);
                        leftover_w = FloatExpr::product(w, 1.0);
                        leftover_z.float_simplify(true, transpose_simd, force_inline_all_variables);
                        leftover_w.float_simplify(true, transpose_simd, force_inline_all_variables);
                    }
                    (true, true, true, false) if !extend3to4_xyz.is_empty() => {
                        let w = swap_take!(gather4_w, vec![]);
                        leftover_w = FloatExpr::product(w, 1.0);
                        leftover_w.float_simplify(true, transpose_simd, force_inline_all_variables);
                    }
                    _ => {}
                }

                let is_any_gather4 = !gather4_x.is_empty() || !gather4_y.is_empty() || !gather4_z.is_empty() || !gather4_w.is_empty();
                let mut is_only_2to4 = !extend2to4_xy.is_empty() && product.is_empty() && gather1.is_empty() && extend3to4_xyz.is_empty() && gather4_x.is_empty() && gather4_y.is_empty();
                let mut is_only_3to4 = !extend3to4_xyz.is_empty() && product.is_empty() && gather1.is_empty() && extend2to4_xy.is_empty() && gather4_x.is_empty() && gather4_y.is_empty() && gather4_z.is_empty();
                let mut is_only_gather4 = is_any_gather4 && product.is_empty() && gather1.is_empty() && extend2to4_xy.is_empty() && extend3to4_xyz.is_empty();

                let mut x_is_zeroed_without_last_factor = false;
                let mut y_is_zeroed_without_last_factor = false;
                let mut z_is_zeroed_without_last_factor = false;
                let mut w_is_zeroed_without_last_factor = false;

                if !gather1.is_empty() {
                    let mut f = FloatExpr::product(gather1, 1.0);
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if z == 0.0 && w == 0.0 {
                        extend2to4_xy.push((Vec2Expr::Gather1(f), 1.0));
                        is_only_2to4 = !extend2to4_xy.is_empty() && product.is_empty() && extend3to4_xyz.is_empty() && gather4_x.is_empty() && gather4_y.is_empty();
                        is_only_3to4 = !extend3to4_xyz.is_empty() && product.is_empty() && extend2to4_xy.is_empty() && gather4_x.is_empty() && gather4_y.is_empty() && gather4_z.is_empty();
                        is_only_gather4 = is_any_gather4 && product.is_empty() && extend2to4_xy.is_empty() && extend3to4_xyz.is_empty();
                    } else if w == 0.0 {
                        extend3to4_xyz.push((Vec3Expr::Gather1(f), 1.0));
                        is_only_2to4 = !extend2to4_xy.is_empty() && product.is_empty() && extend3to4_xyz.is_empty() && gather4_x.is_empty() && gather4_y.is_empty();
                        is_only_3to4 = !extend3to4_xyz.is_empty() && product.is_empty() && extend2to4_xy.is_empty() && gather4_x.is_empty() && gather4_y.is_empty() && gather4_z.is_empty();
                        is_only_gather4 = is_any_gather4 && product.is_empty() && extend2to4_xy.is_empty() && extend3to4_xyz.is_empty();
                    } else {
                        product.push((Vec4Expr::Gather1(f), 1.0));
                    }
                }
                if !extend2to4_xy.is_empty() {
                    let mut xy_coefficient = [1.0; 2];
                    if is_only_2to4 {
                        xy_coefficient = [last_factor[0], last_factor[1]];
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                        mul_coefficient!(leftover_z, last_factor[2]);
                        mul_coefficient!(leftover_w, last_factor[3]);
                        last_factor[2] = 1.0;
                        last_factor[3] = 1.0;
                    }
                    let mut vec2_products = Vec2Expr::product(extend2to4_xy, xy_coefficient);
                    vec2_products.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let Vec2Expr::Gather1(FloatExpr::Literal(0.0)) = &vec2_products {
                        x_is_zeroed_without_last_factor = true;
                        y_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &leftover_z {
                        z_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &leftover_w {
                        w_is_zeroed_without_last_factor = true;
                    }
                    if !extend3to4_xyz.is_empty() {
                        extend3to4_xyz.push((Vec3Expr::Extend2to3(vec2_products, leftover_z), 1.0));
                        is_only_3to4 = product.is_empty() && gather4_x.is_empty() && gather4_y.is_empty() && gather4_z.is_empty();
                    } else {
                        let leftover_w = swap_take!(leftover_w, default_coefficient!(3));
                        product.push((Vec4Expr::Extend2to4(vec2_products, leftover_z, leftover_w), 1.0));
                    }
                }
                if !extend3to4_xyz.is_empty() {
                    let mut xyz_coefficient = [1.0; 3];
                    if is_only_3to4 {
                        xyz_coefficient = [last_factor[0], last_factor[1], last_factor[2]];
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                        last_factor[2] = 1.0;
                        mul_coefficient!(leftover_w, last_factor[3]);
                        last_factor[3] = 1.0;
                    }
                    let mut vec3_products = Vec3Expr::product(extend3to4_xyz, xyz_coefficient);
                    vec3_products.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let Vec3Expr::Gather1(FloatExpr::Literal(0.0)) = &vec3_products {
                        x_is_zeroed_without_last_factor = true;
                        y_is_zeroed_without_last_factor = true;
                        z_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &leftover_w {
                        w_is_zeroed_without_last_factor = true;
                    }
                    product.push((Vec4Expr::Extend3to4(vec3_products, leftover_w), 1.0));
                }
                if is_any_gather4 {
                    let mut x = if gather4_x.is_empty() { default_coefficient!(0) } else { FloatExpr::Product(gather4_x, 1.0) };
                    let mut y = if gather4_y.is_empty() { default_coefficient!(1) } else { FloatExpr::Product(gather4_y, 1.0) };
                    let mut z = if gather4_z.is_empty() { default_coefficient!(2) } else { FloatExpr::Product(gather4_z, 1.0) };
                    let mut w = if gather4_w.is_empty() { default_coefficient!(3) } else { FloatExpr::Product(gather4_w, 1.0) };
                    x.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    y.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    z.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    w.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    if let FloatExpr::Literal(0.0) = &x {
                        x_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &y {
                        y_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &z {
                        z_is_zeroed_without_last_factor = true;
                    }
                    if let FloatExpr::Literal(0.0) = &w {
                        w_is_zeroed_without_last_factor = true;
                    }
                    if is_only_gather4 {
                        product.push((Vec4Expr::Gather4(
                            x * last_factor[0],
                            y * last_factor[1],
                            z * last_factor[2],
                            w * last_factor[3],
                        ), 1.0));
                        last_factor[0] = 1.0;
                        last_factor[1] = 1.0;
                        last_factor[2] = 1.0;
                        last_factor[3] = 1.0;
                    } else {
                        product.push((Vec4Expr::Gather4(x, y, z, w), 1.0));
                    }
                }
                if (last_factor[0] == 1.0 || x_is_zeroed_without_last_factor) &&
                    (last_factor[1] == 1.0 || y_is_zeroed_without_last_factor) &&
                    (last_factor[2] == 1.0 || z_is_zeroed_without_last_factor) &&
                    (last_factor[3] == 1.0 || w_is_zeroed_without_last_factor) {
                    *last_factor = [1.0; 4];
                }

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

                if product.is_empty() {
                    let f0 = FloatExpr::Literal(last_factor[0]);
                    let f1 = FloatExpr::Literal(last_factor[1]);
                    let f2 = FloatExpr::Literal(last_factor[2]);
                    let f3 = FloatExpr::Literal(last_factor[3]);
                    *self = if eqs!(f0, f1, f2, f3) {
                        Vec4Expr::Gather1(f0)
                    } else {
                        Vec4Expr::Gather4(f0, f1, f2, f3)
                    };
                }
            }
            Vec4Expr::Sum(sum, last_addend) => {
                let span = tracing::trace_span!("match_Sum");
                let _span_entered = span.enter();
                if sum.is_empty() {
                    panic!("Please use Vec4Expr::sum so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    for (addend, _factor) in sum.iter_mut() {
                        addend.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                if sum.len() == 1 && *last_addend == [0.0; 4] {
                    let (addend, factor) = sum.remove(0);
                    return if factor == 1.0 {
                        *self = addend;
                    } else {
                        let mut new_self = Vec4Expr::product(vec![(addend, 1.0)], [factor, factor, factor, factor]);
                        new_self.vec4_simplify(true, transpose_simd, force_inline_all_variables);
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
                            self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
                            return
                        }
                        ((Vec4Expr::Extend2to4(va, za, wa), a), (Vec4Expr::Extend2to4(vb, zb, wb), b)) => {
                            *self = Vec4Expr::Extend2to4(
                                Vec2Expr::sum(vec![(va.take_as_owned(), *a), (vb.take_as_owned(), *b)], [last_addend[0], last_addend[1]]),
                                FloatExpr::sum(vec![(za.take_as_owned(), *a), (zb.take_as_owned(), *b)], last_addend[2]),
                                FloatExpr::sum(vec![(wa.take_as_owned(), *a), (wb.take_as_owned(), *b)], last_addend[3])
                            );
                            // Significant restructure, so re-simplify
                            self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
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
            Vec4Expr::SwizzleVec2(v2, i0, i1, i2, i3) => {
                let span = tracing::trace_span!("match_SwizzleVec2");
                let _span_entered = span.enter();
                if *i0 > 1 || *i1 > 1 || *i2 > 1 {
                    panic!("Please use Vec4Expr::swizzle_vec_2 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1, *i2, *i3) {
                    *self = Vec4Expr::Gather1(FloatExpr::AccessVec2(Box::new(v2.take_as_owned()), *i0));
                    self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v2 {
                    Vec2Expr::Gather1(f0) => {
                        *self = Vec4Expr::Gather1(f0.take_as_owned());
                    }
                    Vec2Expr::Gather2(f0, f1) if eqs!(*i0, *i1, *i2, *i3) => {
                        let fs = [f0, f1];
                        *self = Vec4Expr::Gather1(fs[*i0].take_as_owned());
                    }
                    Vec2Expr::Gather2(f0, f1) => {
                        let fs = [f0, f1];
                        *self = Vec4Expr::Gather4(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone(), fs[*i3].clone());
                    }
                    _ => {}
                }
            }
            Vec4Expr::SwizzleVec3(v3, i0, i1, i2, i3) => {
                let span = tracing::trace_span!("match_SwizzleVec3");
                let _span_entered = span.enter();
                if *i0 > 2 || *i1 > 2 || *i2 > 2 {
                    panic!("Please use Vec4Expr::swizzle_vec_3 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if eqs!(*i0, *i1, *i2, *i3) {
                    *self = Vec4Expr::Gather1(FloatExpr::AccessVec3(Box::new(v3.take_as_owned()), *i0));
                    self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v3 {
                    Vec3Expr::Gather1(f0) => {
                        *self = Vec4Expr::Gather1(f0.take_as_owned());
                    }
                    Vec3Expr::Gather3(f0, f1, f2) if eqs!(*i0, *i1, *i2, *i3)  => {
                        let fs = [f0, f1, f2];
                        *self = Vec4Expr::Gather1(fs[*i0].take_as_owned());
                    }
                    Vec3Expr::Gather3(f0, f1, f2) => {
                        let fs = [f0, f1, f2];
                        *self = Vec4Expr::Gather4(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone(), fs[*i3].clone());
                    }
                    _ => {}
                }
            }
            Vec4Expr::SwizzleVec4(v4, i0, i1, i2, i3) => {
                let span = tracing::trace_span!("match_SwizzleVec4");
                let _span_entered = span.enter();
                if *i0 > 3 || *i1 > 3 || *i2 > 3 {
                    panic!("Please use Vec4Expr::swizzle_vec_4 so you can find out where you constructed something wrong");
                }
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if *i0 == 0 && *i1 == 1 && *i2 == 2 && *i3 == 3 {
                    *self = v4.take_as_owned();
                    return;
                }
                if eqs!(*i0, *i1, *i2, *i3) {
                    *self = Vec4Expr::Gather1(FloatExpr::AccessVec4(Box::new(v4.take_as_owned()), *i0));
                    self.vec4_simplify(false, transpose_simd, force_inline_all_variables);
                    return;
                }
                match v4 {
                    box Vec4Expr::Gather1(f0) => {
                        *self = Vec4Expr::Gather1(f0.take_as_owned());
                    }
                    box Vec4Expr::Gather4(f0, f1, f2, f3) => {
                        let fs = [f0, f1, f2, f3];
                        *self = Vec4Expr::Gather4(fs[*i0].clone(), fs[*i1].clone(), fs[*i2].clone(), fs[*i3].clone());
                    }
                    box Vec4Expr::Extend2to4(v, z, w) if *i0 == 0 && *i1 == 1 => {
                        *self = Vec4Expr::Extend2to4(v.take_as_owned(), z.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend2to4(v, z, w) if *i0 < 2 && *i1 < 2 => {
                        *self = Vec4Expr::Extend2to4(Vec2Expr::swizzle_vec_2(v.take_as_owned(), *i0, *i1), z.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend3to4(v, w) if *i0 == 0 && *i1 == 1 && *i2 == 2 => {
                        *self = Vec4Expr::Extend3to4(v.take_as_owned(), w.take_as_owned());
                    }
                    box Vec4Expr::Extend3to4(v, w) if *i0 < 3 && *i1 < 3 && *i2 < 3 => {
                        *self = Vec4Expr::Extend3to4(Vec3Expr::swizzle_vec_3(v.take_as_owned(), *i0, *i1, *i2), w.take_as_owned());
                    }
                    _ => {}
                }
            }
        }
    }
}
impl MultiVectorGroupExpr {
    fn group_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => {
                if !insides_already_done {
                    f.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if let FloatExpr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = f {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx].take_as_owned();
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
                    v2.vec2_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if let Vec2Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v2 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec3(v3) => {
                if !insides_already_done {
                    v3.vec3_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if let Vec3Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v3 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx].take_as_owned();
                    }
                }
            }
            MultiVectorGroupExpr::Vec4(v4) => {
                if !insides_already_done {
                    v4.vec4_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
                if let Vec4Expr::AccessMultiVecGroup(MultiVectorExpr { expr, mv_class: _ }, idx) = v4 {
                    if let MultiVectorVia::Construct(v) = expr.as_mut() {
                        *self = v[*idx].take_as_owned();
                    }
                }
            }
        }
    }
}
impl MultiVectorExpr {
    pub(crate) fn simplify(&mut self) {
        self.multivec_simplify(false, false, false);
    }
    #[allow(unused)]
    pub(crate) fn transposing_simplify(&mut self) {
        self.multivec_simplify(false, true, false);
    }
    #[allow(unused)]
    pub(crate) fn deep_simplify(&mut self) {
        self.slice_to_floats();
        self.multivec_simplify(false, false, true);
    }
    #[tracing::instrument(level = "debug", skip_all, fields(iad = insides_already_done, ts = transpose_simd, fiav = force_inline_all_variables))]
    fn multivec_simplify(&mut self, insides_already_done: bool, transpose_simd: bool, force_inline_all_variables: bool) {
        match &mut *self.expr {
            MultiVectorVia::Variable(v) => {
                let span = tracing::trace_span!("match_Variable");
                let _span_entered = span.enter();
                let decl = &v.decl;
                if force_inline_all_variables || 1 == Arc::strong_count(decl) || decl.force_inline.load(Acquire) {
                    if let Some(lock) = decl.expr.as_ref() {
                        let guard = lock.read();
                        let inlined_expr = guard.deref().clone();
                        drop(guard);
                        if let AnyExpression::Class(mut new_self) = inlined_expr {
                            new_self.multivec_simplify(false, transpose_simd, force_inline_all_variables);
                            *self = new_self;
                            return
                        }
                    }
                }
            }
            MultiVectorVia::Construct(groups) => {
                let span = tracing::trace_span!("match_Construct");
                let _span_entered = span.enter();
                if !insides_already_done {
                    for group in groups.iter_mut() {
                        group.group_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    }
                }
                let mut flat_idx_offset = 0;
                let result = groups.iter_mut().enumerate().fold(None, |a, (b_idx, b)| {
                    let group_width = b.width();
                    let mv_b = match b {
                        MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecFlat(mv, flat_idx))
                        if *flat_idx == flat_idx_offset && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv, idx))
                        if *idx == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv, idx))
                        if *idx == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv, idx))
                        if *idx == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv, idx))
                        if *idx == b_idx && mv.mv_class == self.mv_class => Some(mv),
                        _ => None,
                    };
                    flat_idx_offset += group_width;
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
                    *self = result.take_as_owned();
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_t, owner) => {
                let span = tracing::trace_span!("match_TraitInvoke11ToClass");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
            MultiVectorVia::TraitInvoke21ToClass(_t, owner, _other) => {
                let span = tracing::trace_span!("match_TraitInvoke21ToClass");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
            MultiVectorVia::TraitInvoke22ToClass(_t, owner, other) => {
                let span = tracing::trace_span!("match_TraitInvoke22ToClass");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    other.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
            MultiVectorVia::TraitInvoke12iToClass(_t, owner, other) => {
                let span = tracing::trace_span!("match_TraitInvoke12iToClass");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    other.int_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
            MultiVectorVia::TraitInvoke12fToClass(_t, owner, other) => {
                let span = tracing::trace_span!("match_TraitInvoke12fToClass");
                let _span_entered = span.enter();
                if !insides_already_done {
                    owner.multivec_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                    other.float_simplify(insides_already_done, transpose_simd, force_inline_all_variables);
                }
            }
        }
    }
}

