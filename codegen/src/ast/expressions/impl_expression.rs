pub trait Expression<ExprType>: Send + Sized {
    fn into_any_expression(self) -> AnyExpression;

    fn from_any_expression(any: AnyExpression) -> Option<Self>;
    fn expression_type(&self) -> ExprType;
    fn type_from_any(any: &AnyExpression) -> Option<ExprType>;
    fn try_into_variable(&self) -> Option<Variable<ExprType>>;
    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>);
}


impl Expression<Integer> for IntExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Int(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Int(i) => Some(i),
            _ => None,
        }
    }

    fn expression_type(&self) -> Integer {
        Integer
    }

    fn type_from_any(any: &AnyExpression) -> Option<Integer> {
        match any {
            AnyExpression::Int(_) => Some(Integer),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Integer>> {
        match self {
            IntExpr::Variable(v) => Some(Variable {
                expr_type: Integer,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            IntExpr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            IntExpr::Literal(_) => {}
            IntExpr::TraitInvoke10ToInt(_, _) => {}
        }
    }
}
impl Expression<Float> for FloatExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Float(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Float(f) => Some(f),
            _ => None,
        }
    }

    fn expression_type(&self) -> Float {
        Float
    }

    fn type_from_any(any: &AnyExpression) -> Option<Float> {
        match any {
            AnyExpression::Float(_) => Some(Float),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Float>> {
        match self {
            FloatExpr::Variable(v) => Some(Variable {
                expr_type: Float,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            FloatExpr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            FloatExpr::Literal(_) => {}
            FloatExpr::AccessVec2(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessVec3(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessVec4(v, _) => v.substitute_variable(old.clone(), new.clone()),
            FloatExpr::TraitInvoke11ToFloat(_, mvc) => mvc.substitute_variable(old.clone(), new.clone()),
            FloatExpr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            FloatExpr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::Exp(a, b, _) => {
                a.substitute_variable(old.clone(), new.clone());
                if let Some(b) = b {
                    b.substitute_variable(old.clone(), new.clone());
                }
            }
            FloatExpr::FromInt(a) => a.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<Vec2> for Vec2Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec2(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec2(f) => Some(f),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec2 {
        Vec2
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec2> {
        match any {
            AnyExpression::Vec2(_) => Some(Vec2),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec2>> {
        match self {
            Vec2Expr::Variable(v) => Some(Variable {
                expr_type: Vec2,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec2Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec2Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Gather2(f1, f2) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
            }
            Vec2Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec2Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec2Expr::SwizzleVec2(v, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::SwizzleVec3(v, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::SwizzleVec4(v, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Truncate3to2(v) => v.substitute_variable(old.clone(), new.clone()),
            Vec2Expr::Truncate4to2(v) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<Vec3> for Vec3Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec3(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec3(f) => Some(f),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec3 {
        Vec3
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec3> {
        match any {
            AnyExpression::Vec3(_) => Some(Vec3),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec3>> {
        match self {
            Vec3Expr::Variable(v) => Some(Variable {
                expr_type: Vec3,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec3Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec3Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::Gather3(f1, f2, f3) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
                f3.substitute_variable(old.clone(), new.clone());
            }
            Vec3Expr::Extend2to3(v2, f1) => {
                v2.substitute_variable(old.clone(), new.clone());
                f1.substitute_variable(old.clone(), new.clone());
            }
            Vec3Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec3Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec3Expr::SwizzleVec2(v, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::SwizzleVec3(v, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::SwizzleVec4(v, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec3Expr::Truncate4to3(v) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<Vec4> for Vec4Expr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec4(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec4(f) => Some(f),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec4 {
        Vec4
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec4> {
        match any {
            AnyExpression::Vec4(_) => Some(Vec4),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec4>> {
        match self {
            Vec4Expr::Variable(v) => Some(Variable {
                expr_type: Vec4,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self {
            Vec4Expr::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            Vec4Expr::Gather1(f) => f.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::Gather4(f1, f2, f3, f4) => {
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
                f3.substitute_variable(old.clone(), new.clone());
                f4.substitute_variable(old.clone(), new.clone());
            }
            Vec4Expr::Extend2to4(v2, f1, f2) => {
                v2.substitute_variable(old.clone(), new.clone());
                f1.substitute_variable(old.clone(), new.clone());
                f2.substitute_variable(old.clone(), new.clone());
            }
            Vec4Expr::Extend3to4(v2, f1) => {
                v2.substitute_variable(old.clone(), new.clone());
                f1.substitute_variable(old.clone(), new.clone());
            }
            Vec4Expr::AccessMultiVecGroup(mve, _) => mve.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::Product(v, _last_factor) => {
                for (v, _) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec4Expr::Sum(v, _last_addend) => {
                for (v, _factor) in v.iter_mut() {
                    v.substitute_variable(old.clone(), new.clone());
                }
            }
            Vec4Expr::SwizzleVec2(v, _, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::SwizzleVec3(v, _, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
            Vec4Expr::SwizzleVec4(v, _, _, _, _) => v.substitute_variable(old.clone(), new.clone()),
        }
    }
}
impl Expression<MultiVector> for MultiVectorExpr {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Class(self)
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Class(mv) => Some(mv),
            _ => None,
        }
    }

    fn expression_type(&self) -> MultiVector {
        self.mv_class.clone()
    }

    fn type_from_any(any: &AnyExpression) -> Option<MultiVector> {
        match any {
            AnyExpression::Class(mve) => Some(mve.mv_class.clone()),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<MultiVector>> {
        match self.expr.as_ref() {
            MultiVectorVia::Variable(v) => Some(Variable {
                expr_type: self.mv_class,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        match self.expr.as_mut() {
            MultiVectorVia::Variable(var) => {
                if var.decl.as_ref() == old.as_ref() {
                    var.decl = new.clone();
                }
            }
            MultiVectorVia::Construct(stuff) => {
                for stuff in stuff {
                    match stuff {
                        MultiVectorGroupExpr::JustFloat(f) => f.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec2(v) => v.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec3(v) => v.substitute_variable(old.clone(), new.clone()),
                        MultiVectorGroupExpr::Vec4(v) => v.substitute_variable(old.clone(), new.clone()),
                    }
                }
            }
            MultiVectorVia::TraitInvoke11ToClass(_, a) => a.substitute_variable(old.clone(), new.clone()),
            MultiVectorVia::TraitInvoke21ToClass(_, a, _) => a.substitute_variable(old.clone(), new.clone()),
            MultiVectorVia::TraitInvoke22ToClass(_, a, b) => {
                a.substitute_variable(old.clone(), new.clone());
                b.substitute_variable(old.clone(), new.clone());
            }
            MultiVectorVia::TraitInvoke12iToClass(_, a, b) => {
                a.substitute_variable(old.clone(), new.clone());
                b.substitute_variable(old.clone(), new.clone());
            }
            MultiVectorVia::TraitInvoke12fToClass(_, a, b) => {
                a.substitute_variable(old.clone(), new.clone());
                b.substitute_variable(old.clone(), new.clone());
            }
        }
    }
}

impl Expression<Integer> for Variable<Integer> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Int(IntExpr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Int(IntExpr::Variable(v)) => Some(Variable {
                expr_type: Integer,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn expression_type(&self) -> Integer {
        Integer
    }

    fn type_from_any(any: &AnyExpression) -> Option<Integer> {
        match any {
            AnyExpression::Int(IntExpr::Variable(_)) => Some(Integer),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Integer>> {
        Some(self.clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Float> for Variable<Float> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Float(FloatExpr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Float(FloatExpr::Variable(v)) => Some(Variable {
                expr_type: Float,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn expression_type(&self) -> Float {
        Float
    }

    fn type_from_any(any: &AnyExpression) -> Option<Float> {
        match any {
            AnyExpression::Float(FloatExpr::Variable(_)) => Some(Float),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Float>> {
        Some(self.clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec2> for Variable<Vec2> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec2(Vec2Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec2(Vec2Expr::Variable(v)) => Some(Variable {
                expr_type: Vec2,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec2 {
        Vec2
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec2> {
        match any {
            AnyExpression::Vec2(Vec2Expr::Variable(_)) => Some(Vec2),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec2>> {
        Some(self.clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec3> for Variable<Vec3> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec3(Vec3Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec3(Vec3Expr::Variable(v)) => Some(Variable {
                expr_type: Vec3,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec3 {
        Vec3
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec3> {
        match any {
            AnyExpression::Vec3(Vec3Expr::Variable(_)) => Some(Vec3),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec3>> {
        Some(self.clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<Vec4> for Variable<Vec4> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Vec4(Vec4Expr::Variable(RawVariableInvocation { decl: self.decl.clone() }))
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Vec4(Vec4Expr::Variable(v)) => Some(Variable {
                expr_type: Vec4,
                decl: v.decl.clone(),
            }),
            _ => None,
        }
    }

    fn expression_type(&self) -> Vec4 {
        Vec4
    }

    fn type_from_any(any: &AnyExpression) -> Option<Vec4> {
        match any {
            AnyExpression::Vec4(Vec4Expr::Variable(_)) => Some(Vec4),
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<Vec4>> {
        Some(self.clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
impl Expression<MultiVector> for Variable<MultiVector> {
    fn into_any_expression(self) -> AnyExpression {
        AnyExpression::Class(MultiVectorExpr {
            mv_class: self.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: self.decl.clone() })),
        })
    }

    fn from_any_expression(any: AnyExpression) -> Option<Self> {
        match any {
            AnyExpression::Class(MultiVectorExpr { mv_class, expr }) => {
                if let MultiVectorVia::Variable(var) = *expr {
                    Some(Variable {
                        expr_type: mv_class,
                        decl: var.decl.clone(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn expression_type(&self) -> MultiVector {
        self.expr_type.clone()
    }

    fn type_from_any(any: &AnyExpression) -> Option<MultiVector> {
        match any {
            AnyExpression::Class(MultiVectorExpr { mv_class, expr }) => {
                if let MultiVectorVia::Variable(_) = **expr {
                    Some(mv_class.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn try_into_variable(&self) -> Option<Variable<MultiVector>> {
        Some((*self).clone())
    }

    fn substitute_variable(&mut self, old: Arc<RawVariableDeclaration>, new: Arc<RawVariableDeclaration>) {
        if self.decl.as_ref() == old.as_ref() {
            self.decl = new;
        }
    }
}
