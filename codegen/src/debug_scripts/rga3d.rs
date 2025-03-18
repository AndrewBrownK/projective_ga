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

    // Full MultiVector
    MultiVector as scalar, e1234 | e1, e2, e3, e4 | e41, e42, e43 | e23, e31, e12 | e423, e431, e412, e321;

}

#[tokio::test]
async fn single_expression_simplification_debugger() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .event_format(DebuggableCopyPasta::new())
        .init();

    // do stuff
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
    DebugTrait(AntiConstraintViolation)
        .trace_implementation(Level::TRACE, repo, &MultiVector)
        .await;
}

