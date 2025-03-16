#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{
    FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr,
};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug22, DebugTrait};
use crate::build_scripts::common_traits::{AntiConstraintViolation, GeometricAntiProduct};
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

#[test]
fn single_expression_simplification_debugger() {

    // Debuggable Copy-Pasta: impl GeometricAntiProduct<Flector> for DualNum
    let slf = multivec_var("self", &DualNum);
    let other = multivec_var("other", &Flector);


    let mut e2t4 = Vec4Expr::Extend2to4(
        Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1, 1),
        FloatExpr::Product(vec![
            (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
        ], 1.0),
        FloatExpr::Product(vec![
            (FloatExpr::Sum(vec![
                (FloatExpr::Product(vec![
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
                    (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                ], 1.0), 1.0),
                (FloatExpr::Product(vec![
                    (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
                    (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                ], 1.0), 1.0), ], 0.0), 1.0),
        ], 1.0));
    println!("{e2t4:?}");
    e2t4.simplify();
    println!("{e2t4:?}");




    // Debuggable Copy-Pasta: impl GeometricAntiProduct<Flector> for DualNum
    let slf = multivec_var("self", &DualNum);
    let other = multivec_var("other", &Flector);
    let the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&Flector, MultiVectorVia::Construct(vec![
        /* e1, e2, e3, e4 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Extend3to4(Vec3Expr::Sum(vec![(Vec3Expr::Product(vec![(Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0)), 1.0), (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1))), 1.0), ], [1.0, 1.0, 1.0]), 1.0), (Vec3Expr::Product(vec![(Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1)), 1.0), (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0), ], [1.0, 1.0, 1.0]), 1.0), ], [0.0, 0.0, 0.0]), FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0), (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0), ], 1.0))),
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Product(vec![
            (Vec4Expr::Extend2to4(
                Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1, 1),
                FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0), ], 1.0),
                FloatExpr::Product(vec![(FloatExpr::Sum(vec![(FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0), (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0), ], 1.0), 1.0), (FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0), (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0), ], 1.0), 1.0), ], 0.0), 1.0), ], 1.0)), 1.0), (Vec4Expr::Extend3to4(Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1))), FloatExpr::Literal(1.0)), 1.0), ], [1.0, 1.0, 1.0, 1.0]))]));


    // Debuggable Copy-Pasta: impl GeometricAntiProduct<Flector> for DualNum
    let slf = multivec_var("self", &DualNum);
    let other = multivec_var("other", &Flector);
    let the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&Flector, MultiVectorVia::Construct(vec![
        /* e1, e2, e3, e4 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Extend3to4(
            Vec3Expr::Sum(vec![
                (Vec3Expr::Product(vec![
                    (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0)), 1.0),
                    (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1))), 1.0),
                ], [1.0, 1.0, 1.0]), 1.0),
                (Vec3Expr::Product(vec![
                    (Vec3Expr::Gather1(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1)), 1.0),
                    (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
                ], [1.0, 1.0, 1.0]), 1.0),
            ], [0.0, 0.0, 0.0]),
            FloatExpr::Product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
                (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
            ], 1.0))),
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Product(vec![
            (Vec4Expr::Extend2to4(
                Vec2Expr::swizzle_vec_2(Vec2Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1, 1),
                FloatExpr::Product(vec![(FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0), ], 1.0),
                FloatExpr::Product(vec![
                    (FloatExpr::Sum(vec![
                        (FloatExpr::Product(vec![
                            (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
                            (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                        ], 1.0), 1.0),
                        (FloatExpr::Product(vec![
                            (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 1), 1.0),
                            (FloatExpr::AccessMultiVecFlat(other.clone().into(), 7), 1.0),
                        ], 1.0), 1.0),
                    ], 0.0), 1.0),
                ], 1.0)), 1.0),
            (Vec4Expr::Extend3to4(
                Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 1))),
                FloatExpr::Literal(1.0)), 1.0),
        ], [1.0, 1.0, 1.0, 1.0]))
    ]));

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
    DebugTrait(GeometricAntiProduct)
        .trace_implementation(Level::TRACE, repo, &DualNum, &Flector)
        .await;
}
