#![allow(non_upper_case_globals)]
#![allow(unused)]

use std::sync::atomic::Ordering::Release;
use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug10, Debug11, Debug22, Debug12f, Debug12i, Debug21, DebugTrait};
use crate::build_scripts::common_traits::{AntiAutoMorphism, AntiConstraintViolation, AntiProjectOrthogonallyOnto, AntiProjectViaHorizonOnto, DotProduct, GeometricAntiProduct, SquareRoot};
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
}

#[tokio::test]
async fn single_expression_simplification_debugger() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .event_format(DebuggableCopyPasta::new())
        .init();

    // Debuggable Copy-Pasta: impl AntiProjectOrthogonallyOnto<Motor> for AntiScalar
    let slf = multivec_var("self", &AntiScalar, None);
    let other = multivec_var("other", &Motor, None);
    let anti_wedge_g0 = vec4_var("anti_wedge_g0", Some(Vec4Expr::product(vec![
        (Vec4Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0)), 1.0),
        (Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1), 1.0),
    ], [-1.0, -1.0, -1.0, 1.0])));
    let anti_wedge_g1 = vec4_var("anti_wedge_g1", Some(Vec4Expr::Gather1(FloatExpr::Literal(0.0))));
    let anti_wedge = multivec_var("anti_wedge", &Motor, Some(MultiVectorExpr::new(&Motor, MultiVectorVia::Construct(vec![
        /* e41, e42, e43, e1234 */
        MultiVectorGroupExpr::Vec4(anti_wedge_g0.clone().into()),
        /* e23, e31, e12, scalar */
        MultiVectorGroupExpr::Vec4(anti_wedge_g1.clone().into()),
    ]))));
    anti_wedge.decl.force_inline.store(true, Release);
    anti_wedge_g1.decl.force_inline.store(true, Release);

    let mut the_return: Vec4Expr = Vec4Expr::sum(vec![
        (Vec4Expr::product(vec![
            (Vec4Expr::Gather1(FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 7)), 1.0),
            (Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 1.0),
        ], [1.0, 1.0, 1.0, 1.0]), 1.0),
        (Vec4Expr::product(vec![
            (Vec4Expr::Gather1(FloatExpr::AccessMultiVecFlat(other.clone().into(), 7)), 1.0),
            (Vec4Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 1.0),
        ], [1.0, 1.0, 1.0, 1.0]), 1.0),
        (Vec4Expr::Extend3to4(
            Vec3Expr::Gather1(FloatExpr::Literal(0.0)),
            FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 0), 1.0),
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 4), 1.0),
            ], 1.0)
        ), 1.0),
    ], [0.0, 0.0, 0.0, 0.0]);



    println!("the_return {:?}", DebugExpression::new(true, &the_return));
    the_return.slice_to_floats();
    println!("the_return {:?}", DebugExpression::new(true, &the_return));
    the_return.simplify();
    println!("the_return {:?}", DebugExpression::new(true, &the_return));


    let mut the_return = Vec4Expr::Gather4(
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 0), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 0), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::Literal(0.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 1), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 1), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::Literal(0.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 2), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 2), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::Literal(0.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 3), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 3), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 0), 1.0),
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 4), 1.0),
            ], 1.0), 1.0),
        ], 0.0)
    );

    let first_extraction = (
        FloatExpr::access_vec_4(anti_wedge_g0.clone().into(), 0),
        FloatExpr::access_vec_4(anti_wedge_g0.clone().into(), 1),
        FloatExpr::access_vec_4(anti_wedge_g0.clone().into(), 2),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_4(anti_wedge_g0.clone().into(), 0), 1.0),
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 4), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_4(anti_wedge_g0.clone().into(), 3), 1.0),
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
            ], 1.0), 1.0),
        ], 0.0));




    // let gather4_match = Vec4Expr::Gather4(
    //     FloatExpr::AccessMultiVecFlat(other.clone().into(), 7),
    //     FloatExpr::AccessMultiVecFlat(other.clone().into(), 7),
    //     FloatExpr::AccessMultiVecFlat(other.clone().into(), 7),
    //     FloatExpr::Literal(1.0)
    // );
    // println!("the_return {:?}", DebugExpression::new(true, &gather4_match));
    // the_return.simplify();
    // println!("the_return {:?}", DebugExpression::new(true, &gather4_match));
}

// TODO impl AntiConstraintViolation for MultiVector {
//   - (self.group1().xwzw()[0] * self[e423])

#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    DebugTrait(AntiProjectOrthogonallyOnto)
        .trace_implementation(Level::TRACE, repo, &AntiScalar, &Motor)
        .await;
}

