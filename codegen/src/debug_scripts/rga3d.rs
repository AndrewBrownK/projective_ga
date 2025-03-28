#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::expressions::{DebugExpression, Vec4Expr};
use crate::ast::quick_variables::*;
use crate::ast::traits::{Debug22, DebugTrait};
use crate::build_scripts::common_traits::{GeometricAntiQuotient, GeometricQuotient};
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

    // Debuggable Copy-Pasta: impl ConstraintViolation for Flector
    let slf = multivec_var("self", &Flector, None);

    let mut the_return = Vec4Expr::product(vec![
        (Vec4Expr::AccessMultiVecGroup(slf.clone().into(), 1), 1.0),
    ], [-1.0, -1.0, -1.0, -1.0]);


    println!("{:?}", DebugExpression::new(true, &the_return));
    the_return.slice_to_floats();
    println!("{:?}", DebugExpression::new(true, &the_return));
    the_return.transposing_simplify();
    println!("{:?}", DebugExpression::new(true, &the_return));

    // THe problem was in destructure_variable_if_applicable
}

#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    DebugTrait(GeometricQuotient).trace_implementation(Level::DEBUG, repo, &Origin, &Flector).await;
}

