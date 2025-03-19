#![allow(non_upper_case_globals)]
#![allow(unused)]

use std::sync::atomic::Ordering::Release;
use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug10, Debug11, Debug22, Debug12f, Debug12i, Debug21, DebugTrait};
use crate::build_scripts::common_traits::{AntiAutoMorphism, AntiConstraintViolation, AntiProjectOrthogonallyOnto, AntiProjectViaHorizonOnto, DotProduct, GeometricAntiProduct, GeometricProduct, ProjectOrthogonallyOnto, SquareRoot, Subtraction};
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

    // Debuggable Copy-Pasta: impl AntiConstraintViolation for Flector
    let slf = multivec_var("self", &Flector, None);
    let anti_reverse = multivec_var("anti_reverse", &Flector, Some(MultiVectorExpr::new(&Flector, MultiVectorVia::Construct(vec![
        /* e1, e2, e3, e4 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Gather4(
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
            ], -1.0),
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
            ], -1.0),
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 2), 1.0),
            ], -1.0),
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
            ], -1.0)
        )),
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 1)),
    ]))));
    let geometric_anti_product = multivec_var("geometric_anti_product", &DualNum, Some(MultiVectorExpr::new(&DualNum, MultiVectorVia::Construct(vec![
        /* scalar, e1234 */
        MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 4), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 5), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 6), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 2), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 7), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 0), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 4), 1.0),
                ], 1.0), -1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 1), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 5), 1.0),
                ], 1.0), -1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 2), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 6), 1.0),
                ], 1.0), -1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 3), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 7), 1.0),
                ], 1.0), -1.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 4), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 4), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 5), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 5), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 6), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 6), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::product(vec![
                    (FloatExpr::AccessMultiVecFlat(anti_reverse.clone().into(), 3), 1.0),
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
                ], 1.0), -1.0),
            ], 0.0)
        )),
    ]))));
    let anti_dot_product = multivec_var("anti_dot_product", &AntiScalar, Some(MultiVectorExpr::new(&AntiScalar, MultiVectorVia::Construct(vec![
        /* e1234 */
        MultiVectorGroupExpr::JustFloat(FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 2.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 4), 2.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 5), 2.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 6), 2.0),
            ], 1.0), 1.0),
        ], 0.0)),
    ]))));
    let subtraction = multivec_var("subtraction", &DualNum, Some(MultiVectorExpr::new(&DualNum, MultiVectorVia::Construct(vec![
        /* scalar, e1234 */
        MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(
            FloatExpr::AccessMultiVecFlat(geometric_anti_product.clone().into(), 0),
            FloatExpr::sum(vec![
                (FloatExpr::AccessMultiVecFlat(geometric_anti_product.clone().into(), 1), 1.0),
                (FloatExpr::AccessMultiVecFlat(anti_dot_product.clone().into(), 0), -1.0),
            ], 0.0)
        )),
    ]))));
    let mut the_return: MultiVectorExpr = /* AnyExpression */ subtraction.clone().into();


    the_return.deep_simplify();
    println!("{:?}", DebugExpression::new(true, &the_return));

    let mut the_return = MultiVectorExpr::new(&DualNum, MultiVectorVia::Construct(vec![
        /* scalar, e1234 */
        MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(
            FloatExpr::AccessMultiVecFlat(geometric_anti_product.clone().into(), 0),
            FloatExpr::sum(vec![
                (FloatExpr::AccessMultiVecFlat(geometric_anti_product.clone().into(), 1), 1.0),
                (FloatExpr::AccessMultiVecFlat(anti_dot_product.clone().into(), 0), -1.0),
            ], 0.0)
        )),
    ]));


    // Debuggable Copy-Pasta: impl AntiConstraintViolation for Flector
    let slf = multivec_var("self", &Flector, None);
    let mut the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&DualNum, MultiVectorVia::Construct(vec![
        /* scalar, e1234 */
        MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(
            FloatExpr::Literal(0.0),
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 2.0),
            ], -2.0)
        )),
    ]));




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
    DebugTrait(AntiConstraintViolation).trace_implementation(Level::TRACE, repo, &Flector).await;
}

