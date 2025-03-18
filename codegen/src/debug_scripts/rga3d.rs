#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug22, DebugTrait};
use crate::build_scripts::common_traits::{AntiConstraintViolation, AntiProjectViaHorizonOnto, GeometricAntiProduct};
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


    // Debuggable Copy-Pasta: impl AntiProjectViaHorizonOnto<Point> for Horizon
    let slf = multivec_var("self", &Horizon, None);
    let other = multivec_var("other", &Point, None);
    let right_dual = multivec_var("right_dual", &Plane, Some(MultiVectorExpr::new(&Plane, MultiVectorVia::Construct(vec![
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::Extend3to4(
            Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))),
            FloatExpr::Literal(0.0)
        )),
    ]))));
    let anti_wedge = multivec_var("anti_wedge", &Line, Some(MultiVectorExpr::new(&Line, MultiVectorVia::Construct(vec![
        /* e41, e42, e43 */
        MultiVectorGroupExpr::Vec3(Vec3Expr::Gather1(FloatExpr::Literal(0.0))),
        /* e23, e31, e12 */
        MultiVectorGroupExpr::Vec3(Vec3Expr::product(vec![
            (Vec3Expr::Gather1(FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
            ], -1.0)), 1.0),
            (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(right_dual.clone().into(), 0))), 1.0),
        ], [1.0, 1.0, 1.0])),
    ]))));
    let wedge = multivec_var("wedge", &Plane, Some(MultiVectorExpr::new(&Plane, MultiVectorVia::Construct(vec![
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::sum(vec![
            (Vec4Expr::Gather4(
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 1), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 2), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 0), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 0), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), -1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), -1.0),
                ], 0.0)
            ), 1.0),
            (Vec4Expr::product(vec![
                (Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 1, 2, 0, 0), 1.0),
                (Vec4Expr::Extend3to4(
                    Vec3Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 2, 0, 1),
                    FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3)
                ), 1.0),
            ], [1.0, 1.0, 1.0, 1.0]), -1.0),
        ], [0.0, 0.0, 0.0, 0.0])),
    ]))));
    let the_return: MultiVectorExpr = /* AnyExpression */ wedge.clone().into();



    // Debuggable Copy-Pasta: impl AntiProjectViaHorizonOnto<Point> for Horizon
    let slf = multivec_var("self", &Horizon, None);
    let other = multivec_var("other", &Point, None);
    let anti_wedge = multivec_var("anti_wedge", &Line, Some(MultiVectorExpr::new(&Line, MultiVectorVia::Construct(vec![
        /* e41, e42, e43 */
        MultiVectorGroupExpr::Vec3(Vec3Expr::Gather1(FloatExpr::Literal(0.0))),
        /* e23, e31, e12 */
        MultiVectorGroupExpr::Vec3(Vec3Expr::product(vec![
            (Vec3Expr::Gather1(FloatExpr::product(vec![
                (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
            ], -1.0)), 1.0),
            (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
        ], [1.0, 1.0, 1.0])),
    ]))));
    let the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&Plane, MultiVectorVia::Construct(vec![
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::sum(vec![
            (Vec4Expr::Gather4(
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 1), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 2), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 0), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 0), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), -1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), -1.0),
                ], 0.0)
            ), 1.0),
            (Vec4Expr::product(vec![
                (Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 1, 2, 0, 0), 1.0),
                (Vec4Expr::Extend3to4(
                    Vec3Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 2, 0, 1),
                    FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3)
                ), 1.0),
            ], [1.0, 1.0, 1.0, 1.0]), -1.0),
        ], [0.0, 0.0, 0.0, 0.0])),
    ]));


    // Debuggable Copy-Pasta: impl AntiProjectViaHorizonOnto<Point> for Horizon
    let slf = multivec_var("self", &Horizon, None);
    let other = multivec_var("other", &Point, None);
    let anti_wedge_g0 = vec3_var("anti_wedge_g0", Some(Vec3Expr::Gather1(FloatExpr::Literal(0.0))));
    let anti_wedge_g1 = vec3_var("anti_wedge_g1", Some(Vec3Expr::product(vec![
        (Vec3Expr::Gather1(FloatExpr::product(vec![
            (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
        ], -1.0)), 1.0),
        (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
    ], [1.0, 1.0, 1.0])));
    let the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&Plane, MultiVectorVia::Construct(vec![
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::sum(vec![
            (Vec4Expr::Gather4(
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 1), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 2), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 0), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 0), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), 1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 3), 1.0),
                    ], 1.0), 1.0),
                ], 0.0),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 4), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), -1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 5), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), -1.0),
                ], 0.0)
            ), 1.0),
            (Vec4Expr::product(vec![
                (Vec4Expr::swizzle_vec_4(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0), 1, 2, 0, 0), 1.0),
                (Vec4Expr::Extend3to4(
                    Vec3Expr::swizzle_vec_3(Vec3Expr::AccessMultiVecGroup(anti_wedge.clone().into(), 0), 2, 0, 1),
                    FloatExpr::AccessMultiVecFlat(anti_wedge.clone().into(), 3)
                ), 1.0),
            ], [1.0, 1.0, 1.0, 1.0]), -1.0),
        ], [0.0, 0.0, 0.0, 0.0])),
    ]));



    // Debuggable Copy-Pasta: impl AntiProjectViaHorizonOnto<Point> for Horizon
    let slf = multivec_var("self", &Horizon, None);
    let other = multivec_var("other", &Point, None);
    let anti_wedge_g1 = vec3_var("anti_wedge_g1", Some(Vec3Expr::product(vec![
        (Vec3Expr::Gather1(FloatExpr::product(vec![
            (FloatExpr::AccessMultiVecFlat(slf.clone().into(), 0), 1.0),
        ], -1.0)), 1.0),
        (Vec3Expr::Truncate4to3(Box::new(Vec4Expr::AccessMultiVecGroup(other.clone().into(), 0))), 1.0),
    ], [1.0, 1.0, 1.0])));
    let mut the_return: MultiVectorExpr = /* AnyExpression */ MultiVectorExpr::new(&Plane, MultiVectorVia::Construct(vec![
        /* e423, e431, e412, e321 */
        MultiVectorGroupExpr::Vec4(Vec4Expr::product(vec![
            (Vec4Expr::Gather4(
                FloatExpr::AccessMultiVecFlat(other.clone().into(), 3),
                FloatExpr::AccessMultiVecFlat(other.clone().into(), 3),
                FloatExpr::AccessMultiVecFlat(other.clone().into(), 3),
                FloatExpr::sum(vec![
                    (FloatExpr::product(vec![
                        (FloatExpr::access_vec_3(anti_wedge_g1.clone().into(), 0), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 0), 1.0),
                    ], 1.0), -1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::access_vec_3(anti_wedge_g1.clone().into(), 1), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 1), 1.0),
                    ], 1.0), -1.0),
                    (FloatExpr::product(vec![
                        (FloatExpr::access_vec_3(anti_wedge_g1.clone().into(), 2), 1.0),
                        (FloatExpr::AccessMultiVecFlat(other.clone().into(), 2), 1.0),
                    ], 1.0), -1.0),
                ], 0.0)
            ), 1.0),
            (Vec4Expr::Extend3to4(
                anti_wedge_g1.clone().into(),
                FloatExpr::Literal(1.0)
            ), 1.0),
        ], [1.0, 1.0, 1.0, 1.0])),
    ]));





    println!("the_return {:?}", DebugExpression::new(true, &the_return));
    the_return.simplify();
    println!("the_return {:?}", DebugExpression::new(true, &the_return));
    // Yes the above doesn't change

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
    DebugTrait(AntiProjectViaHorizonOnto)
        .trace_implementation(Level::TRACE, repo, &Horizon, &Point)
        .await;
}
