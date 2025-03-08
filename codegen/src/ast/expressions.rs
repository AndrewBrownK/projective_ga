use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::Arc;
use float_ord::FloatOrd;
use crate::algebra::basis::BasisElement;
use crate::algebra::multivector::{BasisElementGroup};
use crate::ast::datatype::{ExpressionType, Float, Integer, MultiVector, Vec2, Vec3, Vec4};
use crate::ast::operations_tracker::{TrackOperations, TraitOperationsLookup, VectoredOperationsTracker};
use crate::ast::traits::TraitKey;
use crate::ast::{RawVariableDeclaration, RawVariableInvocation, Variable};
use crate::utility::slice_retain_mut;
use std::collections::HashSet;

pub trait TraitResultType: Clone + Debug + Sized + Send + Sync + 'static {
    type Expr: Expression<Self>;
    #[allow(unused)]
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_10 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_10(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_10 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_11 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_11(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_11 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_12i(trait_name: TraitKey, owner: MultiVectorExpr, other: IntExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_12i is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_12i(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_12i is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_12f(trait_name: TraitKey, owner: MultiVectorExpr, other: FloatExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_12f is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_12f(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_12f is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_21 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_21(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_21 is needed (but not supported)")
    }
    #[allow(unused)]
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        panic!("expr_22 is needed (but not supported) for {trait_name:?}")
    }
    fn inlined_expr_22(_var: Variable<Self>) -> Self::Expr {
        panic!("inlined_expr_22 is needed (but not supported)")
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self>;
    fn select_expr(expr: AnyExpression) -> Option<Self::Expr>;
}
impl TraitResultType for Integer {
    type Expr = IntExpr;
    fn expr_10(trait_name: TraitKey, owner: MultiVector, mv_out: Option<MultiVector>) -> IntExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Integer, found MultiVector");
        IntExpr::TraitInvoke10ToInt(trait_name, owner)
    }

    fn inlined_expr_10(var: Variable<Self>) -> Self::Expr {
        IntExpr::Variable(RawVariableInvocation { decl: var.decl.clone() })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Int(_) => Some(Self),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Int(e) => Some(e),
            _ => None,
        }
    }
}
impl TraitResultType for Float {
    type Expr = FloatExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> FloatExpr {
        assert!(mv_out.is_none(), "Confused Trait output: Expected Float, found MultiVector");
        FloatExpr::TraitInvoke11ToFloat(trait_name, owner)
    }

    fn inlined_expr_11(var: Variable<Self>) -> Self::Expr {
        FloatExpr::Variable(RawVariableInvocation { decl: var.decl.clone() })
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Float(_) => Some(Self),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Float(mut e) => {
                e.simplify();
                Some(e)
            }
            _ => None,
        }
    }
}
impl TraitResultType for MultiVector {
    type Expr = MultiVectorExpr;

    fn expr_11(trait_name: TraitKey, owner: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke11ToClass(trait_name, owner)),
        }
    }
    fn inlined_expr_11(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_12i(trait_name: TraitKey, owner: MultiVectorExpr, other: IntExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke12iToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_12i(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_12f(trait_name: TraitKey, owner: MultiVectorExpr, other: FloatExpr, mv_out: Option<MultiVector>) -> Self::Expr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke12fToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_12f(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_21(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVector, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke21ToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_21(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }
    fn expr_22(trait_name: TraitKey, owner: MultiVectorExpr, other: MultiVectorExpr, mv_out: Option<MultiVector>) -> MultiVectorExpr {
        let mv_class = mv_out.expect("Confused Trait output: Expected MultiVector, but None provided.");
        MultiVectorExpr {
            mv_class,
            expr: Box::new(MultiVectorVia::TraitInvoke22ToClass(trait_name, owner, other)),
        }
    }
    fn inlined_expr_22(var: Variable<Self>) -> Self::Expr {
        MultiVectorExpr {
            mv_class: var.expr_type,
            expr: Box::new(MultiVectorVia::Variable(RawVariableInvocation { decl: var.decl.clone() })),
        }
    }

    fn of_expr(expr: &AnyExpression) -> Option<Self> {
        match expr {
            AnyExpression::Class(mv_expr) => Some(mv_expr.mv_class.clone()),
            _ => None,
        }
    }

    fn select_expr(expr: AnyExpression) -> Option<Self::Expr> {
        match expr {
            AnyExpression::Class(mut e) => {
                e.simplify();
                Some(e)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntExpr {
    Variable(RawVariableInvocation),
    Literal(u32),
    // e.g. Grade
    TraitInvoke10ToInt(TraitKey, MultiVector),
}
#[derive(Clone, Debug)]
pub enum FloatExpr {
    Variable(RawVariableInvocation),
    Literal(f32),
    FromInt(IntExpr),
    AccessVec2(Box<Vec2Expr>, usize),
    AccessVec3(Box<Vec3Expr>, usize),
    AccessVec4(Box<Vec4Expr>, usize),
    AccessMultiVecGroup(MultiVectorExpr, usize),
    AccessMultiVecFlat(MultiVectorExpr, usize),
    // e.g. UnitizedNorm
    TraitInvoke11ToFloat(TraitKey, MultiVectorExpr),
    Product(Vec<(FloatExpr, f32)>, f32),
    Sum(Vec<(FloatExpr, f32)>, f32),
    Exp(Box<FloatExpr>, Option<Box<FloatExpr>>, f32),
    // TODO trig? floor? log? round? trunc? mix? step? smoothstep? fma? fract? modf?
}
#[derive(Clone, Debug)]
pub enum Vec2Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather2(FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, usize),
    Product(Vec<(Vec2Expr, f32)>, [f32; 2]),
    Sum(Vec<(Vec2Expr, f32)>, [f32; 2]),
    SwizzleVec2(Box<Vec2Expr>, usize, usize),
    Truncate3to2(Box<Vec3Expr>),
    Truncate4to2(Box<Vec4Expr>),
}
#[derive(Clone, Debug)]
pub enum Vec3Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather3(FloatExpr, FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, usize),
    Product(Vec<(Vec3Expr, f32)>, [f32; 3]),
    Sum(Vec<(Vec3Expr, f32)>, [f32; 3]),
    SwizzleVec3(Box<Vec3Expr>, usize, usize, usize),
    Truncate4to3(Box<Vec4Expr>),
    Extend2to3(Vec2Expr, FloatExpr),
}
#[derive(Clone, Debug)]
pub enum Vec4Expr {
    Variable(RawVariableInvocation),
    Gather1(FloatExpr),
    Gather4(FloatExpr, FloatExpr, FloatExpr, FloatExpr),
    AccessMultiVecGroup(MultiVectorExpr, usize),
    Product(Vec<(Vec4Expr, f32)>, [f32; 4]),
    Sum(Vec<(Vec4Expr, f32)>, [f32; 4]),
    SwizzleVec4(Box<Vec4Expr>, usize, usize, usize, usize),
    Extend2to4(Vec2Expr, FloatExpr, FloatExpr),
    Extend3to4(Vec3Expr, FloatExpr),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiVectorGroupExpr {
    JustFloat(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr),
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiVectorExpr {
    pub mv_class: MultiVector,
    pub expr: Box<MultiVectorVia>,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiVectorVia {
    Variable(RawVariableInvocation),
    Construct(Vec<MultiVectorGroupExpr>),
    // e.g. Involutions
    TraitInvoke11ToClass(TraitKey, MultiVectorExpr),
    // e.g. Powi
    TraitInvoke12iToClass(TraitKey, MultiVectorExpr, IntExpr),
    // e.g. Powf
    TraitInvoke12fToClass(TraitKey, MultiVectorExpr, FloatExpr),
    // e.g. Into
    TraitInvoke21ToClass(TraitKey, MultiVectorExpr, MultiVector),
    // e.g. Wedge
    TraitInvoke22ToClass(TraitKey, MultiVectorExpr, MultiVectorExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnyExpression {
    Int(IntExpr),
    Float(FloatExpr),
    Vec2(Vec2Expr),
    Vec3(Vec3Expr),
    Vec4(Vec4Expr),
    Class(MultiVectorExpr),
}



// Trait implementations and stuff
include!("expressions/impl_from.rs");
include!("expressions/impl_expression.rs");
include!("expressions/impl_display.rs");
include!("expressions/impl_debug.rs");
include!("expressions/impl_eq_ord.rs");
include!("expressions/impl_operators.rs");
include!("expressions/impl_statistics.rs");

// A few methods
include!("expressions/misc_methods.rs");
include!("expressions/constructors.rs");
include!("expressions/scan_destructurable.rs");

// The extremely tedious stuff
include!("expressions/simplify.rs");
include!("expressions/transposition.rs");