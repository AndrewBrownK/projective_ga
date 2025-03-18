
impl TrackOperations for AnyExpression {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            AnyExpression::Int(a) => a.count_operations(lookup),
            AnyExpression::Float(a) => a.count_operations(lookup),
            AnyExpression::Vec2(a) => a.count_operations(lookup),
            AnyExpression::Vec3(a) => a.count_operations(lookup),
            AnyExpression::Vec4(a) => a.count_operations(lookup),
            AnyExpression::Class(a) => a.count_operations(lookup),
        }
    }
}

impl TrackOperations for IntExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            IntExpr::Variable(_) => VectoredOperationsTracker::zero(),
            IntExpr::Literal(_) => VectoredOperationsTracker::zero(),
            IntExpr::TraitInvoke10ToInt(t, m) => {
                let mut result = lookup.trait_10_ops(t, m);
                result.basis_element_struct_access = false;
                result
            },
        }
    }
}
impl TrackOperations for FloatExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            FloatExpr::Variable(_) => VectoredOperationsTracker::zero(),
            FloatExpr::Literal(_) => VectoredOperationsTracker::zero(),
            FloatExpr::AccessVec2(v, _) => v.count_operations(lookup),
            FloatExpr::AccessVec3(v, _) => v.count_operations(lookup),
            FloatExpr::AccessVec4(v, _) => v.count_operations(lookup),
            FloatExpr::AccessMultiVecGroup(m, _) => {
                let mut result = m.count_operations(lookup);
                result.basis_element_struct_access = true;
                if let box MultiVectorVia::Variable(raw) = &m.expr {
                    let n = raw.decl.name.clone();
                    result.mv_vars_access_grouped.insert(n);
                }
                result
            }
            FloatExpr::AccessMultiVecFlat(m, _) => {
                let mut result = m.count_operations(lookup);
                result.basis_element_struct_access = true;
                if let box MultiVectorVia::Variable(raw) = &m.expr {
                    let n = raw.decl.name.clone();
                    result.mv_vars_access_flat.insert(n);
                }
                result
            }
            FloatExpr::TraitInvoke11ToFloat(t, m) => {
                let mut result = lookup.trait_11_ops(t, &m.mv_class);
                result.basis_element_struct_access = false;
                result += m.count_operations(lookup);
                result
            },
            FloatExpr::Product(v, last_factor) => {
                // TODO update this branch to better reflect the code generation (and simd products)
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.floats.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.floats.div += 1;
                        }
                        _ => {
                            result.floats.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != 1.0 {
                    result.floats.mul += 1;
                }
                result
            }
            FloatExpr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.floats.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.floats.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != 0.0 {
                    result.floats.add_sub += 1;
                }
                result
            }
            FloatExpr::Exp(a, b, c) => {
                let mut result = a.count_operations(lookup);
                if *c != 1.0 && b.is_some() {
                    result.floats.mul += 1;
                }
                if *c != 1.0 || b.is_some() {
                    result.floats.pow += 1;
                }
                if let Some(b) = b {
                    result += b.count_operations(lookup);
                }
                result
            }
            FloatExpr::FromInt(a) => a.count_operations(lookup)
        }
    }
}
impl TrackOperations for Vec2Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec2Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec2Expr::Gather1(f) => f.count_operations(lookup),
            Vec2Expr::Gather2(f0, f1) => f0.count_operations(lookup) + f1.count_operations(lookup),
            Vec2Expr::AccessMultiVecGroup(m, _) => {
                let mut result = m.count_operations(lookup);
                if let box MultiVectorVia::Variable(raw) = &m.expr {
                    let n = raw.decl.name.clone();
                    result.mv_vars_access_grouped.insert(n);
                }
                result
            },
            Vec2Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd2.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd2.div += 1;
                        }
                        _ => {
                            result.simd2.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 2] {
                    result.simd2.mul += 1;
                }
                result
            }
            Vec2Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd2.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd2.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 2] {
                    result.simd2.add_sub += 1;
                }
                result
            }
            Vec2Expr::SwizzleVec2(box v, _, _) => v.count_operations(lookup),
            Vec2Expr::SwizzleVec3(box v, _, _) => v.count_operations(lookup),
            Vec2Expr::SwizzleVec4(box v, _, _) => v.count_operations(lookup),
            Vec2Expr::Truncate3to2(box v) => v.count_operations(lookup),
            Vec2Expr::Truncate4to2(box v) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for Vec3Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec3Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec3Expr::Gather1(f) => f.count_operations(lookup),
            Vec3Expr::Gather3(f0, f1, f2) => f0.count_operations(lookup) + f1.count_operations(lookup) + f2.count_operations(lookup),
            Vec3Expr::Extend2to3(v2, f1) => {
                v2.count_operations(lookup) + f1.count_operations(lookup)
            }
            Vec3Expr::AccessMultiVecGroup(m, _) => {
                let mut result = m.count_operations(lookup);
                if let box MultiVectorVia::Variable(raw) = &m.expr {
                    let n = raw.decl.name.clone();
                    result.mv_vars_access_grouped.insert(n);
                }
                result
            },
            Vec3Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd3.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd3.div += 1;
                        }
                        _ => {
                            result.simd3.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 3] {
                    result.simd3.mul += 1;
                }
                result
            }
            Vec3Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd3.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd3.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 3] {
                    result.simd3.add_sub += 1;
                }
                result
            }
            Vec3Expr::SwizzleVec2(v, _, _, _) => v.count_operations(lookup),
            Vec3Expr::SwizzleVec3(v, _, _, _) => v.count_operations(lookup),
            Vec3Expr::SwizzleVec4(v, _, _, _) => v.count_operations(lookup),
            Vec3Expr::Truncate4to3(v) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for Vec4Expr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            Vec4Expr::Variable(_) => VectoredOperationsTracker::zero(),
            Vec4Expr::Gather1(f) => f.count_operations(lookup),
            Vec4Expr::Gather4(f0, f1, f2, f3) => f0.count_operations(lookup) + f1.count_operations(lookup) + f2.count_operations(lookup) + f3.count_operations(lookup),
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                v2.count_operations(lookup) + f1.count_operations(lookup) + f2.count_operations(lookup)
            }
            Vec4Expr::Extend3to4(v3, f1) => {
                v3.count_operations(lookup) + f1.count_operations(lookup)
            }
            Vec4Expr::AccessMultiVecGroup(m, _) => {
                let mut result = m.count_operations(lookup);
                if let box MultiVectorVia::Variable(raw) = &m.expr {
                    let n = raw.decl.name.clone();
                    result.mv_vars_access_grouped.insert(n);
                }
                result
            },
            Vec4Expr::Product(v, last_factor) => {
                let mut result = VectoredOperationsTracker::zero();
                for (i, (f, exp)) in v.iter().enumerate() {
                    result += f.count_operations(lookup);
                    match exp {
                        1.0 => {
                            if i > 0 {
                                result.simd4.mul += 1;
                            }
                        }
                        -1.0 => {
                            result.simd4.div += 1;
                        }
                        _ => {
                            result.simd4.pow += 1;
                        }
                    }
                }
                if !v.is_empty() && *last_factor != [1.0; 4] {
                    result.simd4.mul += 1;
                }
                result
            }
            Vec4Expr::Sum(v, lits) => {
                let mut result = VectoredOperationsTracker::zero();
                for (f, factor) in v.iter() {
                    if *factor != 1.0 && *factor != -1.0 {
                        result.simd4.mul += 1;
                    }
                    result += f.count_operations(lookup);
                }
                if v.len() > 1 {
                    result.simd4.add_sub += v.len() - 1;
                }
                if !v.is_empty() && *lits != [0.0; 4] {
                    result.simd4.add_sub += 1;
                }
                result
            }
            Vec4Expr::SwizzleVec2(v, _, _, _, _) => v.count_operations(lookup),
            Vec4Expr::SwizzleVec3(v, _, _, _, _) => v.count_operations(lookup),
            Vec4Expr::SwizzleVec4(v, _, _, _, _) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for MultiVectorGroupExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => f.count_operations(lookup),
            MultiVectorGroupExpr::Vec2(v) => v.count_operations(lookup),
            MultiVectorGroupExpr::Vec3(v) => v.count_operations(lookup),
            MultiVectorGroupExpr::Vec4(v) => v.count_operations(lookup),
        }
    }
}
impl TrackOperations for MultiVectorExpr {
    fn count_operations(&self, lookup: &TraitOperationsLookup) -> VectoredOperationsTracker {
        match self.expr.as_ref() {
            MultiVectorVia::Variable(_) => VectoredOperationsTracker::zero(),
            MultiVectorVia::Construct(v) => {
                let mut result = VectoredOperationsTracker::zero();
                for f in v.iter() {
                    result += f.count_operations(lookup);
                }
                result
            }
            MultiVectorVia::TraitInvoke11ToClass(t, m) => {
                let mut result = lookup.trait_11_ops(t, &m.mv_class);
                result.basis_element_struct_access = false;
                result += m.count_operations(lookup);
                result
            },
            MultiVectorVia::TraitInvoke21ToClass(t, a, b) => {
                let mut result = lookup.trait_21_ops(t, &a.mv_class, &b);
                result.basis_element_struct_access = false;
                result += a.count_operations(lookup);
                result
            },
            MultiVectorVia::TraitInvoke22ToClass(t, a, b) => {
                let mut result = lookup.trait_22_ops(t, &a.mv_class, &b.mv_class);
                result.basis_element_struct_access = false;
                result += a.count_operations(lookup);
                result += b.count_operations(lookup);
                result
            },
            MultiVectorVia::TraitInvoke12iToClass(t, a, b) => {
                let mut result = lookup.trait_12i_ops(t, &a.mv_class);
                result.basis_element_struct_access = false;
                result += a.count_operations(lookup);
                result += b.count_operations(lookup);
                result
            },
            MultiVectorVia::TraitInvoke12fToClass(t, a, b) => {
                let mut result = lookup.trait_12f_ops(t, &a.mv_class);
                result.basis_element_struct_access = false;
                result += a.count_operations(lookup);
                result += b.count_operations(lookup);
                result
            },
        }
    }
}
