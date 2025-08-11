#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug22, DebugTrait};
use crate::build_scripts::common_traits::{AntiInverse, AntiProjectOrthogonallyOnto, AntiProjectViaHorizonOnto, AntiWedge, ConstraintViolation, GeometricAntiQuotient, GeometricQuotient, ProjectViaOriginOnto, Wedge};
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


    // Debuggable Copy-Pasta: impl Wedge<Line> for Flector
    let slf = multivec_var("self", &Flector, None);
    let other = multivec_var("other", &Line, None);


    let mut the_return = Vec4Expr::Gather4(
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 0), 1), 1.0), // other[e42]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 2), 1.0), // self[e3]
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 1), 0), 1.0), // other[e23]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 3), 1.0), // self[e4]
            ], 1.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 0), 2), 1.0), // other[e43]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 0), 1.0), // self[e1]
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 1), 1), 1.0), // other[e31]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 3), 1.0), // self[e4]
            ], 1.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 0), 0), 1.0), // other[e41]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1), 1.0), // self[e2]
            ], 1.0), 1.0),
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 1), 2), 1.0), // other[e12]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 3), 1.0), // self[e4]
            ], 1.0), 1.0),
        ], 0.0),
        FloatExpr::sum(vec![
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 1), 0), 1.0), // other[e23]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 0), 1.0), // self[e1]
            ], 1.0), -1.0),
            (FloatExpr::product(vec![
                (FloatExpr::access_vec_3(Vec3Expr::AccessMultiVecGroup(other.clone().into(), 1), 1), 1.0), // other[e31]
                (FloatExpr::access_vec_4(Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 0), 1), 1.0), // self[e2]
            ], 1.0), -1.0),
        ], 0.0)
    );


    tracing::trace!("{:?}", DebugExpression::new(true, &the_return));
    the_return.transposing_simplify();
    // the_return.vec3_simplify(true, false, false);
    // the_return.simplify();
    tracing::trace!("{:?}", DebugExpression::new(true, &the_return));
}

#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();

    // AntiWedge<Horizon> for Flector
    DebugTrait(AntiWedge).trace_implementation(Level::TRACE, repo, &Horizon, &Line).await;

    // DebugTrait(Wedge).trace_implementation(Level::TRACE, repo, &Flector, &Line).await;
}

/*
        Simd32x4::from([
            (other[e42] * self[e3]) + (other[e23] * self[e4]),
            (other[e43] * self[e1]) + (other[e31] * self[e4]),
            (other[e41] * self[e2]) + (other[e12] * self[e4]),
            -(other[e23] * self[e1]) - (other[e31] * self[e2]),
        ]) - (self.group0().yzxz() * other.group0().zxy().with_w(other[e12])),


        // Should be this instead:

        // e423, e431, e412, e321
        ( Simd32x3::from(self[e4]) * other.group1()
        + other.group0().yzx() * self.group0().zxy()
        - self.group0().yzx() * other.group0().zxy()
        ).with_w(
        - (other[e31] * self[e2])
        - (other[e12] * self[e3])
        - (other[e23] * self[e1]))
        )
 */