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

    // Debuggable Copy-Pasta: impl ProjectOrthogonallyOnto<DualNum> for DualNum
    let slf = multivec_var("self", &Point, None);
    let other = multivec_var("other", &Point, None);


    // before vec4_sum_transpose
    let mut the_return = Vec4Expr::Gather4(
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 0), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
            ], 1.0), -1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
            ], 1.0), -1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3), 1.0),
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 2), 1.0),
            ], 1.0), -1.0),
        ], 0.0),
        FloatExpr::Literal(0.0)
    );

    // Starting extractions




    // vec3_product_extract 1:
    let vec3_product_extract_1 = Vec3Expr::Gather1(FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 3));

    // vec3_product_extract 2:
    let vec3_product_extract_2 = Vec3Expr::Truncate4to3(Box::new(
        Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 0, 1, 2, 3)
    ));

    // vec4_sum_extract 1:
    let vec4_sum_extract_1 = Vec4Expr::Extend3to4(
        Vec3Expr::product(vec![
            (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3)), 1.0),
            (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
        ], [1.0, 1.0, 1.0]),
        FloatExpr::Literal(0.0)
    );





    // vec3_product_extract 3:
    let vec3_product_extract_3 = Vec3Expr::Gather1(FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 3));

    // vec3_product_extract 4:
    let vec3_product_extract_4 = Vec3Expr::Truncate4to3(Box::new(
        Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 0, 1, 2, 3)
    ));

    // vec4_sum_extract 2:
    let vec4_sum_extract_2 = Vec4Expr::Extend3to4(
        Vec3Expr::product(vec![
            (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(other.clone().into(), 3)), 1.0),
            (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0))), 1.0),
        ], [1.0, 1.0, 1.0]),
        FloatExpr::Literal(0.0)
    );




    // End result should look like this:
    /*
        // e41, e42, e43, e1234
        ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
     */


    // vec4_sum_transpose before simplification
    let mut the_half_fixed_result = Vec4Expr::sum(vec![
        (Vec4Expr::Extend3to4(
            Vec3Expr::product(vec![
                (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3)), 1.0),
                (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
            ], [1.0, 1.0, 1.0]),
            FloatExpr::Literal(0.0) ), 0.0),
        (Vec4Expr::Extend3to4(
            Vec3Expr::product(vec![
                (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(other.clone().into(), 3)), 1.0),
                (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0))), 1.0),
            ], [1.0, 1.0, 1.0]),
            FloatExpr::Literal(0.0) ), -1.0),
        (Vec4Expr::Gather4(
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::Literal(0.0), 0.0),
            ], 0.0) ), 1.0),
    ], [0.0, 0.0, 0.0, 0.0]);


    // vec4_sum_transpose still a problem
    let mut the_return =  Vec4Expr::sum(vec![
        (Vec4Expr::Extend3to4(
            Vec3Expr::product(vec![
                (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 3)), 1.0),
                (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
            ], [1.0, 1.0, 1.0]),
            FloatExpr::Literal(0.0) ), 0.0),
        (Vec4Expr::Extend3to4(
            Vec3Expr::product(vec![
                (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(other.clone().into(), 3)), 1.0),
                (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0))), 1.0),
            ], [1.0, 1.0, 1.0]),
            FloatExpr::Literal(0.0) ), -1.0),
        (Vec4Expr::Gather4(
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
                (FloatExpr::Literal(0.0), 0.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
                (FloatExpr::Literal(0.0), 0.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::product(vec![ ], 1.0), 1.0),
                (FloatExpr::Literal(0.0), 0.0),
            ], 0.0),
            FloatExpr::sum(vec![
                (FloatExpr::Literal(0.0), 0.0),
            ], 0.0) ), 1.0),
    ], [0.0, 0.0, 0.0, 0.0]);


    // the_return.slice_to_floats();
    // println!("{:?}", DebugExpression::new(true, &the_return));
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
        .trace_implementation(Level::TRACE, repo, &Point, &Point)
        .await;
}

