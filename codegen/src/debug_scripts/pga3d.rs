#![allow(non_upper_case_globals)]
#![allow(unused)]

use crate::algebra::basis::elements::e0123;
use crate::algebra::multivector::DynamicMultiVector;
use crate::ast::datatype::{Float, MultiVector};
use crate::ast::expressions::FloatExpr;
use crate::ast::traits::{TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImplBuilder};
use crate::ast::Variable;
use crate::build_scripts::common_traits::{AntiReverse, AntiDotProduct, AntiWedge, GeometricProduct, Reverse, RightDual, Sandwich, DotProduct, Wedge};
use crate::{ga, multi_vecs};
use crate::ast::quick_variables::float_var;

fn float_var_expr(n: &str) -> FloatExpr {
    float_var(n).into()
}

// R ⟑ C = MultiVector(
// scalar((c12 * -1)),
// e0(0), e1(0), e2(0), e3(0),
// e01((-c23 + c02)),
// e02((-c31 - c01 + c12)),
// e03((-c12 - c31)),
// e12(0),
// e31((c23 * -1)),
// e23(c31),
// e021(0), e013(0), e032(0), e123(0),
// e0123((c03 + c23))
// )
multi_vecs! { e0123;

    // Base might be either point or plane, depending on our interpretation
    Vector as e0 | e1 | e2 | e3;
    BiVector as e01 | e02 | e03 | e23 | e31 | e12;
    TriVector as e021 | e013 | e032 | e123;
    SimpleRotor as e01 | e12 | e0123;
    Rotor as e01 | e02 | e03 | e23 | e31 | e12 | e0123;

    HalfRotorSandwich1 as e0 | e1 | e2 | e021 | e013 | e032 | e123;
    HalfRotorSandwich2 as scalar | e01 | e02 | e03 | e12 | e31 | e23 | e0123;

    DualNum as scalar | e0123;
    MultiVector as scalar | e0 | e1 | e2 | e3 | e01 | e02 | e03 | e12 | e31 | e23 | e021 | e013 | e032 | e123 | e0123;
}

#[test]
// noinspection DuplicatedCode
fn anti_product_argument() {
    let rga3d = ga! { e0123;
        1 => e1, e2, e3;
        0 => e0;
    };
    use crate::algebra::basis::elements::*;
    let repo = register_multi_vecs(rga3d.clone()).finished();

    let vector = MultiVector::from(&Vector);
    let rotor = MultiVector::from(&SimpleRotor);
    let bivector = MultiVector::from(&BiVector);
    let hrs2 = MultiVector::from(&HalfRotorSandwich2);

    let a = vector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "a");
        float_var_expr(n.as_str())
    });
    let b = vector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "b");
        float_var_expr(n.as_str())
    });
    let c = bivector.construct(|el| {
        let mut n = format!("{el}");
        n = n.replace("e", "c");
        float_var_expr(n.as_str())
    });
    let h = hrs2.construct(|el| {
        let mut n = format!("{el}");
        if n.as_str() == "scalar" {
            n = "h_scalar".to_string()
        } else {
            n = n.replace("e", "h");
        }
        float_var_expr(n.as_str())
    });
    let rotor = rotor.construct(|el| {
        if el == e01 {
            FloatExpr::Literal(1.0)
        } else if el == e12 {
            FloatExpr::Literal(1.0)
        } else if el == e0123 {
            FloatExpr::Literal(1.0)
        } else {
            FloatExpr::Literal(0.0)
        }
    });

    let builder = TraitImplBuilder::new_sandbox(rga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let a_dual = RightDual.deep_inline(&builder, a.clone()).await?;
        println!("A = {a}");
        println!("A* = {a_dual}");

        let b_dual = RightDual.deep_inline(&builder, b.clone()).await?;
        println!("B = {b}");
        println!("B* = {b_dual}");

        println!("C = {c}");
        println!("H = {h}");
        println!("R = {rotor}");

        let r_reverse = Reverse.deep_inline(&builder, rotor.clone()).await?;
        println!("~R = {r_reverse}");

        let r_anti_reverse = AntiReverse.deep_inline(&builder, rotor.clone()).await?;
        println!("R~ = {r_anti_reverse}");

        let a_wedge_b = Wedge.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A ∧ B = {a_wedge_b}");

        let a_anti_wedge_b = AntiWedge.deep_inline(&builder, a.clone(), b.clone()).await;
        println!("A ∨ B = {a_anti_wedge_b:?}");

        let a_dot_b = DotProduct.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A • B = {a_dot_b}");

        let a_anti_dot_b = AntiDotProduct.deep_inline(&builder, a.clone(), b.clone()).await?;
        println!("A ∘ B = {a_anti_dot_b}");

        let r_wd_a_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), a.clone()).await?;
        println!("R ⟑ A ⟑ ~R = {r_wd_a_wd_r}");

        let r_wd_b_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), b.clone()).await?;
        println!("R ⟑ B ⟑ ~R = {r_wd_b_wd_r}");

        // Geometric tests

        // let r_wd_c = GeometricProduct.deep_inline(&builder, rotor.clone(), c.clone()).await?;
        // println!("R ⟑ C = {r_wd_c}");
        // println!("Alright so the above is perfect");

        // let h_wd_r = GeometricProduct.deep_inline(&builder, h.clone(), r_reverse.clone()).await?;
        // println!("H ⟑ ~R = {h_wd_r}");

        // let r_wd_awb_wd_r = GeometricProduct.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        // println!("R ⟑ (A ∧ B) = {r_wd_awb_wd_r}");
        let r_wd_awb_wd_r = Sandwich.deep_inline(&builder, rotor.clone(), a_wedge_b.clone()).await?;
        println!("R ⟑ (A ∧ B) ⟑ ~R = {r_wd_awb_wd_r}");

        let rar_w_rbr = Wedge.deep_inline(&builder, r_wd_a_wd_r.clone(), r_wd_b_wd_r.clone()).await?;
        println!("(R ⟑ A ⟑ ~R) ∧ (R ⟑ B ⟑ ~R) = {rar_w_rbr}");

        Some(())
    });
    result.expect("Entire script must complete")
}
