#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug11, DebugTrait};
use crate::build_scripts::common_traits::{AntiInverse, AntiProjectOrthogonallyOnto, AntiProjectViaHorizonOnto, ConstraintViolation, GeometricAntiQuotient, GeometricQuotient, ProjectViaOriginOnto};
use crate::elements::e1234;
use crate::utility::tracing::DebuggableCopyPasta;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

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

    // Full MultiVector
    MultiVector as scalar, e1234 | e1, e2, e3, e4 | e41, e42, e43 | e23, e31, e12 | e423, e431, e412, e321;

}

#[tokio::test]
async fn single_expression_simplification_debugger() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .event_format(DebuggableCopyPasta::new())
        .init();

    // Debuggable Copy-Pasta: impl AntiProjectOrthogonallyOnto<MultiVector> for AntiScalar
    let slf = multivec_var("self", &AntiScalar, None);
    let other = multivec_var("other", &MultiVector, None);
    let anti_wedge_g1_w = float_var("anti_wedge_g1_w", Some(FloatExpr::product(vec![
        (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 4), 3), 1.0),
        (FloatExpr::AccessMultiVecGroup(slf.clone().into(), 0), 1.0),
    ], -1.0)));







    // TODO this is the problem, this doesn't simplify to a flat product
    let mut the_return = FloatExpr::sum(vec![
        (FloatExpr::Literal(0.0), 0.0),
        (FloatExpr::product(vec![
            (anti_wedge_g1_w.clone().into(), 1.0),
            (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1), 0), 1.0),
        ], 1.0), -1.0),
    ], 0.0);



    tracing::trace!("{:?}", DebugExpression::new(true, &the_return));
    // the_return.transposing_simplify();
    // the_return.vec3_simplify(true, false, false);
    the_return.simplify();
    tracing::trace!("{:?}", DebugExpression::new(true, &the_return));
}

#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();

    DebugTrait(AntiInverse).trace_implementation(Level::TRACE, repo, &Point).await;
}

