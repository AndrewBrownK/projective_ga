#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::ast::datatype::{ExpressionType, Float, MultiVector};
use crate::ast::expressions::Vec4Expr;
use crate::ast::Variable;
use crate::elements::e1234;

crate::multi_vecs! { e1234;
    // Versors
    Motor      as e41, e42, e43, e1234 | e23, e31, e12, scalar;
}

#[test]
fn simplification_debugger() {
    let slf = Variable::<MultiVector>::quick_var("self", MultiVector::from(&Motor));
    let other_g0 = Variable::<Float>::quick_var("other_g0", Float);
    let anti_reverse_g0 = Vec4Expr::Product(vec![(Vec4Expr::AccessMultiVecGroup(slf.into(), 0), 1.0)], [-1.0, -1.0, -1.0, 1.0]);
    let mut term = Vec4Expr::Product(vec![(anti_reverse_g0, 1.0), (Vec4Expr::Gather1(other_g0.into()), 1.0)], [1.0; 4]);
    println!("{:?}", term);
    term.simplify();
    println!("{:?}", term);
}

#[test]
fn test_stuff() {
    let rga3d = crate::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = register_multi_vecs(rga3d).finished();
    let traits = crate::register_all! { repo;
        Wedge AntiWedge
    };
    let traits = traits.finish();

    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let impls = traits.get_impls().await;
        for i in impls {
            if let ExpressionType::Class(mv) = i.owner {
                if mv.name() == "Motor" {
                    let o = &i.other_type_params;
                    println!("{o:?}");
                    let r = &i.return_expr;
                    println!("{r:?}");
                }
            }
        }
        Some(())
    });
    result.expect("Entire script must complete")
}