use crate::ast::expressions::{AnyExpression, Expression};
use parking_lot::RwLock;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub mod datatype;
pub mod expressions;
pub mod impls;
mod operations_tracker;
pub mod traits;
pub mod trace;

#[derive(Clone, Debug)]
pub struct Variable<ExprType> {
    pub expr_type: ExprType,
    pub(crate) decl: Arc<RawVariableDeclaration>,
}

impl<ExprType> Variable<ExprType> {
    // For quick testing purposes
    fn quick_var<Expr: Expression<ExprType>>(name: &str, expr_type: ExprType, e: Option<Expr>) -> Self {
        let expr = e.map(|it| Arc::new(RwLock::new(it.into_any_expression())));
        Variable {
            expr_type,
            decl: Arc::new(RawVariableDeclaration {
                comment: None,
                name: (name.to_string(), 0),
                expr,
                force_inline: Arc::new(AtomicBool::new(false)),
            }),
        }
    }
}


/// Quickly create variables for testing purposes. These are not suitable for use
/// in trait implementations or trait definition registration, since the names are not
/// checked for uniqueness. These are just intended for test cases and small demo scripts.
pub mod quick_variables {
    use std::sync::atomic::Ordering::Release;
    use crate::algebra::basis::BasisElement;
    use crate::ast::datatype::{Float, Integer, MultiVector, Vec2, Vec3, Vec4};
    use crate::ast::expressions::{FloatExpr, IntExpr, MultiVectorExpr, Vec2Expr, Vec3Expr, Vec4Expr};
    use crate::ast::Variable;

    pub fn int_var(name: &str, expr: Option<IntExpr>) -> Variable<Integer> {
        Variable::<Integer>::quick_var(name, Integer, expr)
    }
    pub fn float_var(name: &str, expr: Option<FloatExpr>) -> Variable<Float> {
        Variable::<Float>::quick_var(name, Float, expr)
    }
    pub fn vec2_var(name: &str, expr: Option<Vec2Expr>) -> Variable<Vec2> {
        Variable::<Vec2>::quick_var(name, Vec2, expr)
    }
    pub fn vec3_var(name: &str, expr: Option<Vec3Expr>) -> Variable<Vec3> {
        Variable::<Vec3>::quick_var(name, Vec3, expr)
    }
    pub fn vec4_var(name: &str, expr: Option<Vec4Expr>) -> Variable<Vec4> {
        Variable::<Vec4>::quick_var(name, Vec4, expr)
    }
    #[allow(non_upper_case_globals)]
    pub fn multivec_var<const AntiScalar: BasisElement>(
        name: &str,
        mv: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        expr: Option<MultiVectorExpr>
    ) -> Variable<MultiVector> {
        let mv = MultiVector::from(mv);
        Variable::<MultiVector>::quick_var(name, mv, expr)
    }
    pub fn int_var_will_be_inlined(name: &str, expr: IntExpr) -> Variable<Integer> {
        let v = int_var(name, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
    pub fn float_var_will_be_inlined(name: &str, expr: FloatExpr) -> Variable<Float> {
        let v = float_var(name, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
    pub fn vec2_var_will_be_inlined(name: &str, expr: Vec2Expr) -> Variable<Vec2> {
        let v = vec2_var(name, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
    pub fn vec3_var_will_be_inlined(name: &str, expr: Vec3Expr) -> Variable<Vec3> {
        let v = vec3_var(name, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
    pub fn vec4_var_will_be_inlined(name: &str, expr: Vec4Expr) -> Variable<Vec4> {
        let v = vec4_var(name, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
    #[allow(non_upper_case_globals)]
    pub fn multivec_var_will_be_inlined<const AntiScalar: BasisElement>(
        name: &str,
        mv: &'static crate::algebra::multivector::MultiVec<AntiScalar>,
        expr: MultiVectorExpr
    ) -> Variable<MultiVector> {
        let v = multivec_var(name, mv, Some(expr));
        v.decl.force_inline.store(true, Release);
        v
    }
}




impl<ExprType> PartialEq for Variable<ExprType> where ExprType: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.expr_type == other.expr_type && self.decl == other.decl
    }
}
impl<ExprType> Eq for Variable<ExprType> where ExprType: Eq {}
impl<ExprType> PartialOrd for Variable<ExprType> where ExprType: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl<ExprType> Ord for Variable<ExprType> where ExprType: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.expr_type.cmp(&other.expr_type).then_with(|| {
            self.decl.cmp(&other.decl)
        })
    }
}


#[derive(Clone, Debug)]
pub struct RawVariableDeclaration {
    pub(crate) comment: Option<Cow<'static, String>>,
    pub(crate) name: (String, usize),
    pub(crate) expr: Option<Arc<RwLock<AnyExpression>>>,
    pub(crate) force_inline: Arc<AtomicBool>,
}
impl PartialEq for RawVariableDeclaration {
    fn eq(&self, other: &Self) -> bool {
        let result = std::ptr::eq(self, other);
        if !result {
            return self.name == other.name && self.comment == other.comment && match (&self.expr, &other.expr) {
                (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false
            }
        }
        result
    }
}
impl Eq for RawVariableDeclaration {}
impl PartialOrd for RawVariableDeclaration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for RawVariableDeclaration {
    fn cmp(&self, other: &Self) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal
        }
        self.name.cmp(&other.name).then_with(|| {
            self.comment.cmp(&other.comment).then_with(|| {
                let a = &self.expr;
                let b = &other.expr;
                match (a, b) {
                    (None, None) => Ordering::Equal,
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (Some(a), Some(b)) => {
                        if Arc::ptr_eq(a, b) {
                            Ordering::Equal
                        } else {
                            let a = a.read();
                            let b = b.read();
                            a.deref().cmp(b.deref())
                        }
                    }
                }
            })
        })
    }
}



#[derive(Clone)]
pub struct RawVariableInvocation {
    pub(crate) decl: Arc<RawVariableDeclaration>,
}
impl Debug for RawVariableInvocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.decl.name {
            (n, 0) => write!(f, "{n}"),
            (n, i) => write!(f, "{n}_{}", i + 1),
        }
    }
}

impl PartialEq for RawVariableInvocation {
    fn eq(&self, other: &Self) -> bool {
        self.decl == other.decl
    }
}
impl Eq for RawVariableInvocation {}
impl PartialOrd for RawVariableInvocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.decl.cmp(&other.decl))
    }
}
impl Ord for RawVariableInvocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.decl.cmp(&other.decl)
    }
}