
impl AnyExpression {
    pub fn expression_type(&self) -> ExpressionType {
        match self {
            AnyExpression::Int(_) => ExpressionType::Int(Integer),
            AnyExpression::Float(_) => ExpressionType::Float(Float),
            AnyExpression::Vec2(_) => ExpressionType::Vec2(Vec2),
            AnyExpression::Vec3(_) => ExpressionType::Vec3(Vec3),
            AnyExpression::Vec4(_) => ExpressionType::Vec4(Vec4),
            AnyExpression::Class(mv) => ExpressionType::Class(mv.mv_class.clone()),
        }
    }

    pub(crate) fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            AnyExpression::Int(i) => i.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Float(f) => f.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec2(v2) => v2.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec3(v3) => v3.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Vec4(v4) => v4.substitute_variable(old.clone(), new.clone()),
            AnyExpression::Class(c) => c.substitute_variable(old.clone(), new.clone()),
        }
    }
}



/// This helps unify Variable<MultiVector> and MultiVectorExpr
pub fn extract_multivector_expr<Expr: Expression<MultiVector>>(expr: Expr) -> MultiVectorExpr {
    match expr.into_any_expression() {
        AnyExpression::Class(mve) => mve,
        _ => unreachable!("Expression<MultiVector> will always create AnyExpression::Class"),
    }
}

/// This helps unify Variable<Float> and FloatExpr
pub fn extract_float_expr<Expr: Expression<Float>>(expr: Expr) -> FloatExpr {
    match expr.into_any_expression() {
        AnyExpression::Float(f) => f,
        _ => unreachable!("Expression<Float> will always create AnyExpression::Float"),
    }
}

/// This helps unify Variable<Float> and FloatExpr
pub fn extract_integer_expr<Expr: Expression<Integer>>(expr: Expr) -> IntExpr {
    match expr.into_any_expression() {
        AnyExpression::Int(i) => i,
        _ => unreachable!("Expression<Integer> will always create AnyExpression::Int"),
    }
}


impl Variable<MultiVector> {
    // pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
    //     let mv_expr: MultiVectorExpr = self.clone().into();
    //     self.expr_type
    //         .elements()
    //         .into_iter()
    //         .enumerate()
    //         .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(mv_expr.clone(), i), el))
    // }

    pub fn groups(&self) -> impl Iterator<Item = (MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type.groups().into_iter().enumerate().map(move |(g, group)| {
            let g = g;
            match group {
                BasisElementGroup::G1(a) => (MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G1(a)),
                BasisElementGroup::G2(a, b) => (MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G2(a, b)),
                BasisElementGroup::G3(a, b, c) => (MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G3(a, b, c)),
                BasisElementGroup::G4(a, b, c, d) => (MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g)), BasisElementGroup::G4(a, b, c, d)),
            }
        })
    }

    pub fn elements(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        let mv_expr: MultiVectorExpr = self.clone().into();
        self.expr_type
            .groups()
            .into_iter()
            .enumerate()
            .map(move |(g, group)| {
                let mut v = vec![];
                match group {
                    BasisElementGroup::G1(a) => {
                        v.push((FloatExpr::AccessMultiVecGroup(mv_expr.clone(), g), a));
                    }
                    BasisElementGroup::G2(a, b) => {
                        v.push((FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(mv_expr.clone(), g), 1), b));
                    }
                    BasisElementGroup::G3(a, b, c) => {
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g), 1), b));
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(mv_expr.clone(), g), 2), c));
                    }
                    BasisElementGroup::G4(a, b, c, d) => {
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g), 1), b));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g), 2), c));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(mv_expr.clone(), g), 3), d));
                    }
                }
                for (f, _el) in v.iter_mut() {
                    f.simplify();
                }
                v.into_iter()
            })
            .flatten()
    }
}

impl MultiVectorExpr {
    // pub fn elements_flat(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
    //     self.mv_class
    //         .elements()
    //         .into_iter()
    //         .enumerate()
    //         .map(move |(i, el)| (FloatExpr::AccessMultiVecFlat(self.clone(), i), el))
    // }

    pub fn groups(&self) -> impl Iterator<Item = (MultiVectorGroupExpr, BasisElementGroup)> + '_ {
        self.mv_class.groups().into_iter().enumerate().map(move |(g, group)| {
            match group {
                BasisElementGroup::G1(a) => (MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G1(a)),
                BasisElementGroup::G2(a, b) => (MultiVectorGroupExpr::Vec2(Vec2Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G2(a, b)),
                BasisElementGroup::G3(a, b, c) => (MultiVectorGroupExpr::Vec3(Vec3Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G3(a, b, c)),
                BasisElementGroup::G4(a, b, c, d) => (MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(self.clone(), g)), BasisElementGroup::G4(a, b, c, d)),
            }
        })
    }
    pub fn elements(&self) -> impl Iterator<Item = (FloatExpr, BasisElement)> + '_ {
        self.mv_class
            .groups()
            .into_iter()
            .enumerate()
            .map(move |(g, group)| {
                let mut v = vec![];
                match group {
                    BasisElementGroup::G1(a) => {
                        v.push((FloatExpr::AccessMultiVecGroup(self.clone(), g), a));
                    }
                    BasisElementGroup::G2(a, b) => {
                        v.push((FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(self.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_2(Vec2Expr::AccessMultiVecGroup(self.clone(), g), 1), b));
                    }
                    BasisElementGroup::G3(a, b, c) => {
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(self.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(self.clone(), g), 1), b));
                        v.push((FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(self.clone(), g), 2), c));
                    }
                    BasisElementGroup::G4(a, b, c, d) => {
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(self.clone(), g), 0), a));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(self.clone(), g), 1), b));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(self.clone(), g), 2), c));
                        v.push((FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(self.clone(), g), 3), d));
                    }
                }
                for (f, _el) in v.iter_mut() {
                    f.simplify();
                }
                v.into_iter()
            })
            .flatten()
    }

    pub fn new<MV: Into<MultiVector>>(mv: MV, via: MultiVectorVia) -> Self {
        MultiVectorExpr {
            mv_class: mv.into(),
            expr: Box::new(via),
        }
    }
}


impl IntExpr {
    #[allow(unused)]
    fn take_as_owned(&mut self) -> Self {
        let mut x = IntExpr::Literal(0);
        mem::swap(&mut x, self);
        x
    }

    // /// Check if this expression is zero, assuming it is already simplified
    // fn is_zero(&self) -> bool {
    //     match self {
    //         IntExpr::Literal(0) => true,
    //         _ => false,
    //     }
    // }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            IntExpr::Variable(_) => true,
            IntExpr::Literal(_) => true,
            IntExpr::TraitInvoke10ToInt(_, _) => false,
        }
    }
}

impl FloatExpr {
    pub(crate) fn take_as_owned(&mut self) -> Self {
        let mut x = FloatExpr::Literal(0.0);
        mem::swap(&mut x, self);
        x
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            FloatExpr::Literal(0.0) => true,
            _ => false,
        }
    }

    fn is_one_or_zero(&self) -> bool {
        match self {
            FloatExpr::Literal(0.0) => true,
            FloatExpr::Literal(1.0) => true,
            _ => false,
        }
    }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            FloatExpr::Variable(_) => true,
            FloatExpr::Literal(_) => true,
            FloatExpr::FromInt(e) => e.is_memory_read_and_not_compute(),
            FloatExpr::AccessVec2(v, _i) => v.is_memory_read_and_not_compute(),
            FloatExpr::AccessVec3(v, _i) => v.is_memory_read_and_not_compute(),
            FloatExpr::AccessVec4(v, _i) => v.is_memory_read_and_not_compute(),
            FloatExpr::AccessMultiVecGroup(mve, _i) => mve.is_memory_read_and_not_compute(),
            FloatExpr::AccessMultiVecFlat(mve, _i) => mve.is_memory_read_and_not_compute(),
            FloatExpr::TraitInvoke11ToFloat(_, _) => false,
            FloatExpr::Product(_, _) => false,
            FloatExpr::Sum(_, _) => false,
            FloatExpr::Exp(_, _, _) => false,
        }
    }
}
impl Vec2Expr {
    pub(crate) fn take_as_owned(&mut self) -> Self {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }

    fn take_part_as_owned(&mut self, idx: usize) -> FloatExpr {
        let mut x = Vec2Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        match x {
            Vec2Expr::Variable(_) => FloatExpr::access_vec_2(x, idx),
            Vec2Expr::Gather1(f) => f,
            Vec2Expr::Gather2(f0, f1) => match idx {
                0 => f0, 1 => f1, _ => panic!("{idx} does not fit in Vec2 for take_part_as_owned")
            },
            Vec2Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx {
                            flat_idx = flat_idx + idx;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width();
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec2Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::product(f_factors, f_lit)
            }
            Vec2Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::sum(f_addends, f_lit)
            }
            Vec2Expr::SwizzleVec2(box mut v, x, y) => v.take_part_as_owned([x, y][idx]),
            Vec2Expr::SwizzleVec3(box mut v, x, y) => v.take_part_as_owned([x, y][idx]),
            Vec2Expr::SwizzleVec4(box mut v, x, y) => v.take_part_as_owned([x, y][idx]),
            Vec2Expr::Truncate3to2(box mut v3) => v3.take_part_as_owned(idx),
            Vec2Expr::Truncate4to2(box mut v4) => v4.take_part_as_owned(idx),
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec2Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
    }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            Vec2Expr::Variable(_) => true,
            Vec2Expr::Gather1(x) => x.is_memory_read_and_not_compute(),
            Vec2Expr::Gather2(x, y) => {
                x.is_memory_read_and_not_compute() && y.is_memory_read_and_not_compute()
            }
            Vec2Expr::AccessMultiVecGroup(mve, _i) => mve.is_memory_read_and_not_compute(),
            Vec2Expr::Product(_, _) => false,
            Vec2Expr::Sum(_, _) => false,
            Vec2Expr::SwizzleVec2(_, _, _) => false,
            Vec2Expr::SwizzleVec3(_, _, _) => false,
            Vec2Expr::SwizzleVec4(_, _, _) => false,
            Vec2Expr::Truncate3to2(v) => v.is_memory_read_and_not_compute(),
            Vec2Expr::Truncate4to2(v) => v.is_memory_read_and_not_compute(),
        }
    }
}
impl Vec3Expr {
    pub(crate) fn take_as_owned(&mut self) -> Self {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: usize) -> FloatExpr {
        let mut x = Vec3Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        match x {
            Vec3Expr::Variable(_) => FloatExpr::access_vec_3(x, idx),
            Vec3Expr::Gather1(f) => f,
            Vec3Expr::Gather3(f0, f1, f2) => match idx {
                0 => f0, 1 => f1, 2 => f2, _ => panic!("{idx} does not fit in Vec3 for take_part_as_owned")
            },
            Vec3Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx {
                            flat_idx = flat_idx + idx;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width();
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec3Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::product(f_factors, f_lit)
            }
            Vec3Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::sum(f_addends, f_lit)
            }
            Vec3Expr::SwizzleVec2(mut v, x, y, z) => v.take_part_as_owned([x, y, z][idx]),
            Vec3Expr::SwizzleVec3(box mut v, x, y, z) => v.take_part_as_owned([x, y, z][idx]),
            Vec3Expr::SwizzleVec4(box mut v, x, y, z) => v.take_part_as_owned([x, y, z][idx]),
            Vec3Expr::Truncate4to3(box mut v4) => v4.take_part_as_owned(idx),
            Vec3Expr::Extend2to3(mut v2, f) => match idx {
                0 | 1 => v2.take_part_as_owned(idx),
                2 => f,
                _ => panic!("{idx} does not fit in Vec3 for take_part_as_owned")
            }
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec3Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
    }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            Vec3Expr::Variable(_) => true,
            Vec3Expr::Gather1(x) => x.is_memory_read_and_not_compute(),
            Vec3Expr::Gather3(x, y, z) => {
                x.is_memory_read_and_not_compute() && y.is_memory_read_and_not_compute() && z.is_memory_read_and_not_compute()
            }
            Vec3Expr::AccessMultiVecGroup(mve, _) => mve.is_memory_read_and_not_compute(),
            Vec3Expr::Product(_, _) => false,
            Vec3Expr::Sum(_, _) => false,
            Vec3Expr::SwizzleVec2(_, _, _, _) => false,
            Vec3Expr::SwizzleVec3(_, _, _, _) => false,
            Vec3Expr::SwizzleVec4(_, _, _, _) => false,
            Vec3Expr::Truncate4to3(v) => v.is_memory_read_and_not_compute(),
            // could go one way or the other on this one, I'll allow it for now
            Vec3Expr::Extend2to3(v, z) => v.is_memory_read_and_not_compute() && z.is_memory_read_and_not_compute(),
        }
    }
}
impl Vec4Expr {

    pub(crate) fn take_as_owned(&mut self) -> Self {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: usize) -> FloatExpr {
        let mut x = Vec4Expr::Gather1(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        match x {
            Vec4Expr::Variable(_) => FloatExpr::access_vec_4(x, idx),
            Vec4Expr::Gather1(f) => f,
            Vec4Expr::Gather4(f0, f1, f2, f3) => match idx {
                0 => f0, 1 => f1, 2 => f2, 3 => f3, _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            },
            Vec4Expr::AccessMultiVecGroup(mve, g_idx) => match *mve.expr {
                MultiVectorVia::Construct(mut groups) => groups[g_idx].take_part_as_owned(idx),
                _ => {
                    let mut flat_idx = 0;
                    for (scan_g_idx, (_, g)) in mve.groups().enumerate() {
                        if scan_g_idx == g_idx {
                            flat_idx = flat_idx + idx;
                            break
                        }
                        flat_idx = flat_idx + g.simd_width();
                    }
                    FloatExpr::AccessMultiVecFlat(mve, flat_idx)
                },
            }
            Vec4Expr::Product(v_factors, v_lits) => {
                let mut f_factors = vec![];
                for mut v_factor in v_factors {
                    f_factors.push((v_factor.0.take_part_as_owned(idx), v_factor.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::product(f_factors, f_lit)
            }
            Vec4Expr::Sum(v_addends, v_lits) => {
                let mut f_addends = vec![];
                for mut v_addend in v_addends {
                    f_addends.push((v_addend.0.take_part_as_owned(idx), v_addend.1));
                }
                let f_lit = v_lits[idx];
                FloatExpr::sum(f_addends, f_lit)
            }
            Vec4Expr::SwizzleVec2(mut v, x, y, z, w) => v.take_part_as_owned([x, y, z, w][idx]),
            Vec4Expr::SwizzleVec3(mut v, x, y, z, w) => v.take_part_as_owned([x, y, z, w][idx]),
            Vec4Expr::SwizzleVec4(box mut v, x, y, z, w) => v.take_part_as_owned([x, y, z, w][idx]),
            Vec4Expr::Extend2to4(mut v2, z, w) => match idx {
                0 | 1 => v2.take_part_as_owned(idx),
                2 => z,
                3 => w,
                _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            }
            Vec4Expr::Extend3to4(mut v3, f) => match idx {
                0 | 1 | 2 => v3.take_part_as_owned(idx),
                3 => f,
                _ => panic!("{idx} does not fit in Vec4 for take_part_as_owned")
            }
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            Vec4Expr::Gather1(f) => f.is_zero(),
            _ => false,
        }
    }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            Vec4Expr::Variable(_) => true,
            Vec4Expr::Gather1(x) => x.is_memory_read_and_not_compute(),
            Vec4Expr::Gather4(x, y, z, w) => {
                x.is_memory_read_and_not_compute() && y.is_memory_read_and_not_compute() && z.is_memory_read_and_not_compute() && w.is_memory_read_and_not_compute()
            }
            Vec4Expr::AccessMultiVecGroup(mve, _) => mve.is_memory_read_and_not_compute(),
            // TODO maybe should count negation of one simple term as simple
            //  see impl AntiConstraintViolation for AntiFlector
            Vec4Expr::Product(_, _) => false,
            Vec4Expr::Sum(_, _) => false,
            Vec4Expr::SwizzleVec2(_, _, _, _, _) => false,
            Vec4Expr::SwizzleVec3(_, _, _, _, _) => false,
            Vec4Expr::SwizzleVec4(_, _, _, _, _) => false,
            // could go one way or the other on these, I'll allow it for now
            Vec4Expr::Extend2to4(v, z, w) => v.is_memory_read_and_not_compute() && z.is_memory_read_and_not_compute() && w.is_memory_read_and_not_compute(),
            Vec4Expr::Extend3to4(v, w) => v.is_memory_read_and_not_compute() && w.is_memory_read_and_not_compute(),
        }
    }
}
impl MultiVectorGroupExpr {
    fn take_as_owned(&mut self) -> Self {
        let mut x = MultiVectorGroupExpr::JustFloat(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        x
    }
    fn take_part_as_owned(&mut self, idx: usize) -> FloatExpr {
        let mut x = MultiVectorGroupExpr::JustFloat(FloatExpr::Literal(0.0));
        mem::swap(&mut x, self);
        match x {
            MultiVectorGroupExpr::JustFloat(f) => f,
            MultiVectorGroupExpr::Vec2(mut v) => v.take_part_as_owned(idx),
            MultiVectorGroupExpr::Vec3(mut v) => v.take_part_as_owned(idx),
            MultiVectorGroupExpr::Vec4(mut v) => v.take_part_as_owned(idx),
        }
    }

    fn width(&self) -> usize {
        match self {
            MultiVectorGroupExpr::JustFloat(_) => 1,
            MultiVectorGroupExpr::Vec2(_) => 2,
            MultiVectorGroupExpr::Vec3(_) => 3,
            MultiVectorGroupExpr::Vec4(_) => 4,
        }
    }

    /// Check if this expression is zero, assuming it is already simplified
    fn is_zero(&self) -> bool {
        match self {
            MultiVectorGroupExpr::JustFloat(f) => f.is_zero(),
            MultiVectorGroupExpr::Vec2(v) => v.is_zero(),
            MultiVectorGroupExpr::Vec3(v) => v.is_zero(),
            MultiVectorGroupExpr::Vec4(v) => v.is_zero(),
        }
    }
}
impl MultiVectorExpr {
    fn take_as_owned(&mut self) -> Self {
        let mut x = MultiVectorExpr {
            mv_class: self.mv_class,
            expr: Box::new(MultiVectorVia::Construct(vec![]))
        };
        mem::swap(&mut x, self);
        x
    }

    /// Check if this expression is zero, assuming it is already simplified
    pub(crate) fn is_zero(&self) -> bool {
        match self.expr.as_ref() {
            MultiVectorVia::Construct(gs) => gs.iter().all(|it| it.is_zero()),
            _ => false,
        }
    }

    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match &*self.expr {
            MultiVectorVia::Variable(_) => true,
            MultiVectorVia::Construct(v) => v.iter().all(|it| match it {
                MultiVectorGroupExpr::JustFloat(f) => f.is_memory_read_and_not_compute(),
                MultiVectorGroupExpr::Vec2(v) => v.is_memory_read_and_not_compute(),
                MultiVectorGroupExpr::Vec3(v) => v.is_memory_read_and_not_compute(),
                MultiVectorGroupExpr::Vec4(v) => v.is_memory_read_and_not_compute(),
            }),
            MultiVectorVia::TraitInvoke11ToClass(_, _) => false,
            MultiVectorVia::TraitInvoke12iToClass(_, _, _) => false,
            MultiVectorVia::TraitInvoke12fToClass(_, _, _) => false,
            MultiVectorVia::TraitInvoke21ToClass(_, _, _) => false,
            MultiVectorVia::TraitInvoke22ToClass(_, _, _) => false,
        }
    }
}

impl AnyExpression {
    pub(crate) fn is_memory_read_and_not_compute(&self) -> bool {
        match self {
            AnyExpression::Int(e) => e.is_memory_read_and_not_compute(),
            AnyExpression::Float(e) => e.is_memory_read_and_not_compute(),
            AnyExpression::Vec2(e) => e.is_memory_read_and_not_compute(),
            AnyExpression::Vec3(e) => e.is_memory_read_and_not_compute(),
            AnyExpression::Vec4(e) => e.is_memory_read_and_not_compute(),
            AnyExpression::Class(e) => e.is_memory_read_and_not_compute(),
        }
    }
}


trait TakeAsOwned {
    fn take_as_owned(&mut self) -> Self;
}
impl<T> TakeAsOwned for Vec<T> {
    fn take_as_owned(&mut self) -> Self {
        let mut x = vec![];
        mem::swap(&mut x, self);
        x
    }
}