#![allow(non_upper_case_globals)]
#![allow(unused)]

use std::sync::atomic::Ordering::Release;
use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug10, Debug11, Debug22, Debug12f, Debug12i, Debug21, DebugTrait};
use crate::build_scripts::common_traits::{AntiAutoMorphism, AntiConstraintViolation, AntiProjectOrthogonallyOnto, AntiProjectViaHorizonOnto, DotProduct, GeometricAntiProduct, GeometricProduct, SquareRoot, Subtraction};
use crate::elements::e1234;
use crate::utility::tracing::DebuggableCopyPasta;
use tracing::Level;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::layer::SubscriberExt;
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

    // Debuggable Copy-Pasta: impl GeometricProduct<Motor> for MultiVector
    let slf = multivec_var("self", &MultiVector, None);
    let other = multivec_var("other", &Motor, None);
    let mut the_return: Vec4Expr = Vec4Expr::Extend3to4(
        Vec3Expr::product(vec![
            (Vec3Expr::swizzle_vec_3(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 0, 1, 0), 1.0),
            (Vec3Expr::Extend2to3(
                Vec2Expr::Truncate4to2(Box::new(Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 4), 3, 3, 2, 3))),
                FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3)
            ), 1.0),
        ], [1.0, 1.0, 1.0]),
        FloatExpr::Literal(1.0)
    );

    the_return.slice_to_floats();
    println!("{:?}", DebugExpression::new(true, &the_return));
    the_return.simplify();
    println!("{:?}", DebugExpression::new(true, &the_return));
}

#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    DebugTrait(GeometricProduct)
        .trace_implementation(Level::TRACE, repo, &MultiVector, &Motor)
        .await;
}

