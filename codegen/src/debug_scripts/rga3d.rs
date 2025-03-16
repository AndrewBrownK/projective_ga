#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::build_scripts::common_traits::AntiConstraintViolation;
use crate::elements::e1234;
use crate::utility::tracing::DebuggableCopyPasta;
use tracing::Level;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::ast::traits::{Debug11, DebugTrait};

crate::multi_vecs! { e1234;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e1234;
    Origin     as e4;
    Horizon    as e321;
    DualNum    as scalar, e1234;

    // Uniform Grade Flat Objects
    Point      as e1, e2, e3, e4;
    Line       as e41, e42, e43 | e23, e31, e12;
    Plane      as e423, e431, e412, e321;

    // Versors
    Motor      as e41, e42, e43, e1234 | e23, e31, e12, scalar;
    Flector    as e1, e2, e3, e4 | e423, e431, e412, e321;
}

#[test]
fn single_expression_simplification_debugger() {
    // Debuggable Copy-Pasta: impl AntiConstraintViolation for Line
    let slf = multivec_var("self", &Line);
    let anti_reverse = /* AnyExpression */ MultiVectorExpr::new(&Line, MultiVectorVia::Construct(vec![/* e41, e42, e43 */ MultiVectorGroupExpr::Vec3(Vec3Expr::Product(vec![(Vec3Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1.0), ], [-1.0, -1.0, -1.0])), /* e23, e31, e12 */ MultiVectorGroupExpr::Vec3(Vec3Expr::Product(vec![(Vec3Expr::AccessMultiVecGroup(slf.clone().into(), 1), 1.0), ], [-1.0, -1.0, -1.0]))]));
    let geometric_anti_product = /* AnyExpression */ MultiVectorExpr::new(&DualNum, MultiVectorVia::Construct(vec![/* scalar, e1234 */ MultiVectorGroupExpr::Vec2(Vec2Expr::Sum(vec![(Vec2Expr::Gather2(FloatExpr::Sum(vec![(FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 3), 1.0), (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0), ], 1.0), -1.0), (FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 4), 1.0), (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0), ], 1.0), -1.0), (FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 5), 1.0), (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 2), 1.0), ], 1.0), -1.0), ], 0.0), FloatExpr::Literal(0.0)), 1.0), (Vec2Expr::Product(vec![(Vec2Expr::Gather1(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 0)), 1.0), (Vec2Expr::Gather2(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0)), 1.0), ], [1.0, 1.0]), -1.0), (Vec2Expr::Product(vec![(Vec2Expr::Gather1(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 1)), 1.0), (Vec2Expr::Gather2(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 4), FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1)), 1.0), ], [1.0, 1.0]), -1.0), (Vec2Expr::Product(vec![(Vec2Expr::Gather1(FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 2)), 1.0), (Vec2Expr::Gather2(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 5), FloatExpr::AccessMultiVecFlat(slf.clone().into(), 2)), 1.0), ], [1.0, 1.0]), -1.0), ], [0.0, 0.0]))]));
    // This comment is an unused variable that will get removed
    let subtraction = /* AnyExpression */ MultiVectorExpr::new(&Scalar, MultiVectorVia::Construct(vec![/* scalar */ MultiVectorGroupExpr::JustFloat(FloatExpr::AccessMultiVecFlat(geometric_anti_product.clone().into(), 0))]));
    let the_return: MultiVectorExpr = /* AnyExpression */ subtraction.clone().into();
}

// TODO this has outright wrongness in it: impl AntiConstraintViolation for Line
#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    DebugTrait(AntiConstraintViolation).trace_implementation(Level::DEBUG, repo, &Line).await;
}
