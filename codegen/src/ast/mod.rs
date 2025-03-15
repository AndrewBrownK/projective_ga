use crate::ast::datatype::{Float, MultiVector};
use crate::ast::expressions::{AnyExpression, Vec4Expr};
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
    decl: Arc<RawVariableDeclaration>,
}

impl<ExprType> Variable<ExprType> {
    // For quick testing purposes
    fn quick_var(name: &str, e: ExprType) -> Self {
        Variable {
            expr_type: e,
            decl: Arc::new(RawVariableDeclaration {
                comment: None,
                name: (name.to_string(), 0),
                expr: None,
                force_inline: Arc::new(AtomicBool::new(false)),
            }),
        }
    }
}


/// Quickly create variables for testing purposes. These are not suitable for use
/// in trait implementations or trait definition registration, since the names are not
/// checked for uniqueness. These are just intended for test cases and small demo scripts.
pub mod quick_variables {
    use crate::algebra::basis::BasisElement;
    use crate::ast::datatype::{Float, Integer, MultiVector, Vec2, Vec3, Vec4};
    use crate::ast::Variable;

    pub fn int_var(name: &str) -> Variable<Integer> {
        Variable::<Integer>::quick_var(name, Integer)
    }
    pub fn float_var(name: &str) -> Variable<Float> {
        Variable::<Float>::quick_var(name, Float)
    }
    pub fn vec2_var(name: &str) -> Variable<Vec2> {
        Variable::<Vec2>::quick_var(name, Vec2)
    }
    pub fn vec3_var(name: &str) -> Variable<Vec3> {
        Variable::<Vec3>::quick_var(name, Vec3)
    }
    pub fn vec4_var(name: &str) -> Variable<Vec4> {
        Variable::<Vec4>::quick_var(name, Vec4)
    }
    pub fn multivec_var<const AntiScalar: BasisElement>(
        name: &str, mv: &'static crate::algebra::multivector::MultiVec<AntiScalar>
    ) -> Variable<MultiVector> {
        let mv = MultiVector::from(mv);
        Variable::<MultiVector>::quick_var(name, mv)
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