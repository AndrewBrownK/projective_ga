#![allow(non_upper_case_globals)]
#![allow(unused)]

use tracing::Level;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::ast::expressions::Vec4Expr;
use crate::ast::quick_variables::*;
use crate::ast::traits::Register11;
use crate::build_scripts::common_traits::AntiConstraintViolation;
use crate::elements::e1234;
use crate::utility::tracing::DebuggableCopyPasta;

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
    let slf = multivec_var("self", &Motor);
    let other_g0 = float_var("other_g0");
    let anti_reverse_g0 = Vec4Expr::Product(vec![(Vec4Expr::AccessMultiVecGroup(slf.into(), 0), 1.0)], [-1.0, -1.0, -1.0, 1.0]);
    let mut term = Vec4Expr::Product(vec![(anti_reverse_g0, 1.0), (Vec4Expr::Gather1(other_g0.into()), 1.0)], [1.0; 4]);
    println!("{:?}", term);
    term.simplify();
    println!("{:?}", term);
}

// TODO this has outright wrongness in it: impl AntiConstraintViolation for Line
#[tokio::test]
async fn multi_line_simplification_debugger() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        // .with(DebuggableCopyPasta::<Format>::new())
        .init();

    AntiConstraintViolation.trace_implementation(repo, &Line).await;
}

