#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]

use codegen::algebra::multivector::DeclareMultiVecs;
use codegen::ast::traits::{
    TraitDef_1_Type_1_Arg,
    TraitDef_1_Type_2_Args_f32,
    TraitDef_1_Type_2_Args_i32,
    TraitDef_2_Types_2_Args,
};
use codegen::build_scripts::common_traits::conformal::{RadiusNorm, RadiusNormSquared};
use codegen::build_scripts::common_traits::GeometricAntiProduct;
use codegen::elements::e12345;
use custom_traits::*;

// PRO-TIP: Run with the release profile

codegen::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
    DualNum    as e5, e12345;
    // DualNum4   as e4, e12345;
    // DualNum321 as e321, e12345;
    // TripleNum    as e4, e5, e12345;
    // QuadNum    as e4, e5, e321, e12345;

    // Uniform Grade Flat Objects
    FlatPoint  as e15, e25, e35, e45;
    Line       as e415, e425, e435 | e235, e315, e125;
    Plane      as e4235, e4315, e4125, e3215;

    // Uniform Grade Round Objects
    RoundPoint as e1, e2, e3, e4 | e5;
    Dipole     as e41, e42, e43 | e23, e31, e12, e45 | e15, e25, e35;
    Circle     as e423, e431, e412 | e415, e425, e435, e321 | e235, e315, e125;
    Sphere     as e4235, e4315, e4125, e3215 | e1234;

    // Versors
    Motor      as e415, e425, e435, e12345 | e235, e315, e125, e5;
    Flector    as e15, e25, e35, e45 | e4235, e4315, e4125, e3215;
    // 2 reflections
    CircleRotor as e423, e431, e412 | e415, e425, e435, e321 | e235, e315, e125, e12345;
    // 3 reflections
    DipoleInversion as e41, e42, e43 | e23, e31, e12, e45 | e15, e25, e35, e1234 | e4235, e4315, e4125, e3215;
    // 4 reflections
    VersorEven as e423, e431, e412, e12345 | e415, e425, e435, e321 | e235, e315, e125, e5 | e1, e2, e3, e4;
    // 5 reflections
    VersorOdd  as e41, e42, e43, scalar | e23, e31, e12, e45 | e15, e25, e35, e1234 | e4235, e4315, e4125, e3215;
}


/// Lengyel styled CGA of 5 dimensions representing 3 dimensions
fn main() {
    let cga3d = codegen::ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    let repo = base_documentation(register_multi_vecs(cga3d)).finished();
    let traits = codegen::register_all! { e12345 repo;
        Zero One AntiOne Unit
        Grade AntiGrade Into TryInto
        RightDual RightAntiDual Reverse AntiReverse
        Wedge AntiWedge
        GeometricProduct GeometricAntiProduct
        Sandwich AntiSandwich
        DotProduct AntiDotProduct
        Inverse AntiInverse
        GeometricQuotient GeometricAntiQuotient

        BulkExpansion BulkContraction WeightExpansion WeightContraction
        Fix AntiFix
        ConstraintViolation AntiConstraintViolation
        ConstraintValid AntiConstraintValid
        AutoMorphism AntiAutoMorphism
        Conjugation ConformalConjugate

        Complement DoubleComplement
        Carrier CoCarrier

        RoundBulk
        RoundWeight
        FlatBulk
        FlatWeight

        RoundNormSquared
            RoundBulkNormSquared
            RoundWeightNormSquared
            UnitizedRoundNormSquared
        FlatNormSquared
            FlatBulkNormSquared
            FlatWeightNormSquared
            UnitizedFlatNormSquared

        RoundNorm
            RoundBulkNorm
            RoundWeightNorm
            UnitizedRoundNorm
        FlatNorm
            FlatBulkNorm
            FlatWeightNorm
            UnitizedFlatNorm

        RadiusNormSquared
        UnitizedRadiusNormSquared
        CenterNormSquared
        UnitizedCenterNormSquared

        RadiusNorm
        UnitizedRadiusNorm
        CenterNorm
        UnitizedCenterNorm

        ProjectOrthogonallyOnto AntiProjectOrthogonallyOnto
        ProjectViaOriginOnto AntiProjectViaHorizonOnto
        RejectOrthogonallyFrom AntiRejectOrthogonallyFrom
        RejectViaOriginFrom AntiRejectViaHorizonFrom
        Support AntiSupport
        Unitize
    };
    codegen::operators! { e12345 repo, traits;
        fancy_infix => Div;

        binary
        Add => Addition,
        Sub => Subtraction,
        BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Neg => Negation,
        Not => RightDual;
    }
    let traits = traits.finish();

    let slang = codegen::Slang::new();
    slang.write_src(
        "libraries/cga3d/src",
        "cga3d",
        repo.clone(),
        traits.clone()
    );

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.wgsl = false;
    rust.glsl = false;
    rust.write_crate(
        "libraries/cga3d/",
        "cga3d",
        1,
        0,
        0,
        "",
        "Latest generated test case",
        "https://github.com/AndrewBrownK/projective_ga/",
        &[],
        repo,
        traits,
    );
}

fn base_documentation(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    // declarations.append_documentation(
    //     &Origin,
    //     "\
    // The Origin is the RoundPoint where x, y, z, and radius are all zero.
    // It is the base element e4.
    // Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    // ",
    // );
    // TODO more documentation
    declarations.generate_missing_duals(None);
    declarations
}

// noinspection DuplicatedCode
pub mod custom_traits {
    #![allow(non_upper_case_globals)]

    use codegen::algebra::basis::BasisElement;
    use codegen::ast::impls::Elaborated;
    use codegen::build_scripts::common_traits::conformal::*;
    use codegen::build_scripts::common_traits::conformal::impls::*;
    use codegen::build_scripts::common_traits::impls::{AntiSupportImpl, SupportImpl, UnitizeImpl};
    use codegen::build_scripts::common_traits::{anti_support, support, unitize};

    const origin: BasisElement = codegen::elements::e4;
    const infinity: BasisElement = codegen::elements::e5;
    const option_infinity: Option<BasisElement> = Some(codegen::elements::e5);

    pub static Carrier: Elaborated<CarrierImpl> = carrier(infinity);
    pub static CoCarrier: Elaborated<CoCarrierImpl> = co_carrier(infinity);
    pub static ConformalConjugate: Elaborated<ConformalConjugateImpl> = conformal_conjugate(infinity);
    pub static RoundBulk: Elaborated<RoundBulkImpl> = round_bulk(origin, infinity);
    pub static RoundWeight: Elaborated<RoundWeightImpl> = round_weight(origin, infinity);
    pub static FlatBulk: Elaborated<FlatBulkImpl> = flat_bulk(origin, option_infinity);
    pub static FlatWeight: Elaborated<FlatWeightImpl> = flat_weight(origin, option_infinity);
    pub static CenterNorm: Elaborated<CenterNormImpl> = center_norm(origin, infinity);
    pub static CenterNormSquared: Elaborated<CenterNormSquaredImpl> = center_norm_squared(origin, infinity);
    pub static FlatBulkNorm: Elaborated<FlatBulkNormImpl> = flat_bulk_norm(origin, option_infinity);
    pub static FlatBulkNormSquared: Elaborated<FlatBulkNormSquaredImpl> = flat_bulk_norm_squared(origin, option_infinity);
    pub static FlatNorm: Elaborated<FlatNormImpl> = flat_norm(origin, option_infinity);
    pub static FlatNormSquared: Elaborated<FlatNormSquaredImpl> = flat_norm_squared(origin, option_infinity);
    pub static FlatWeightNorm: Elaborated<FlatWeightNormImpl> = flat_weight_norm(origin, option_infinity);
    pub static FlatWeightNormSquared: Elaborated<FlatWeightNormSquaredImpl> = flat_weight_norm_squared(origin, option_infinity);
    pub static RoundBulkNorm: Elaborated<RoundBulkNormImpl> = round_bulk_norm(origin, infinity);
    pub static RoundBulkNormSquared: Elaborated<RoundBulkNormSquaredImpl> = round_bulk_norm_squared(origin, infinity);
    pub static RoundNorm: Elaborated<RoundNormImpl> = round_norm(origin, infinity);
    pub static RoundNormSquared: Elaborated<RoundNormSquaredImpl> = round_norm_squared(origin, infinity);
    pub static RoundWeightNorm: Elaborated<RoundWeightNormImpl> = round_weight_norm(origin, infinity);
    pub static RoundWeightNormSquared: Elaborated<RoundWeightNormSquaredImpl> = round_weight_norm_squared(origin, infinity);
    pub static UnitizedCenterNorm: Elaborated<UnitizedCenterNormImpl> = unitized_center_norm(origin, infinity);
    pub static UnitizedCenterNormSquared: Elaborated<UnitizedCenterNormSquaredImpl> = unitized_center_norm_squared(origin, infinity);
    pub static UnitizedFlatNorm: Elaborated<UnitizedFlatNormImpl> = unitized_flat_norm(origin, option_infinity);
    pub static UnitizedFlatNormSquared: Elaborated<UnitizedFlatNormSquaredImpl> = unitized_flat_norm_squared(origin, option_infinity);
    pub static UnitizedRadiusNorm: Elaborated<UnitizedRadiusNormImpl> = unitized_radius_norm(origin, infinity);
    pub static UnitizedRadiusNormSquared: Elaborated<UnitizedRadiusNormSquaredImpl> = unitized_radius_norm_squared(origin, infinity);
    pub static UnitizedRoundNorm: Elaborated<UnitizedRoundNormImpl> = unitized_round_norm(origin, infinity);
    pub static UnitizedRoundNormSquared: Elaborated<UnitizedRoundNormSquaredImpl> = unitized_round_norm_squared(origin, infinity);

    pub static Support: Elaborated<SupportImpl> = support(origin);
    pub static AntiSupport: Elaborated<AntiSupportImpl> = anti_support(origin);
    pub static Unitize: Elaborated<UnitizeImpl<Elaborated<RoundWeightNormImpl>>> = unitize(RoundWeightNorm);
}



#[test]
#[cfg(feature = "incorrect-wip-traits")]
fn test_powf() {
    let cga3d = codegen::ga! { e12345;
        1 => e1, e2, e3, eP;
        -1 => eM;
        where
        e4 => 0.5 * (eM - eP);
        e5 => eP + eM;
    };
    fn float_var_expr(n: &str) -> FloatExpr {
        Variable::quick_var(n, Float).into()
    }
    let repo = base_documentation(register_multi_vecs(cga3d.clone())).finished();
    let builder = TraitImplBuilder::new_sandbox(cga3d.clone(), repo);
    let rt = tokio::runtime::Runtime::new().expect("tokio works");
    let result: Option<()> = rt.block_on(async move {
        let mvs = [
            MultiVector::from(&AntiScalar),
            MultiVector::from(&DualNum4),
            MultiVector::from(&DualNum5),
            MultiVector::from(&DualNum321),
            MultiVector::from(&TripleNum),
            MultiVector::from(&QuadNum),
        ];

        for mv in mvs {
            let base_mv = mv.construct(|el| {
                let mut n = format!("{el}");
                n = n.replace("e", "a");
                float_var_expr(n.as_str())
            });

            let accurate_squared = GeometricAntiProduct.deep_inline(&builder, base_mv.clone(), base_mv.clone()).await?;
            let accurate_cubed = GeometricAntiProduct.deep_inline(&builder, accurate_squared.clone(), base_mv.clone()).await?;
            let accurate_4th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_cubed.clone(), base_mv.clone()).await?;
            let accurate_5th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_4th_pow.clone(), base_mv.clone()).await?;
            let accurate_6th_pow = GeometricAntiProduct.deep_inline(&builder, accurate_5th_pow.clone(), base_mv.clone()).await?;
            println!("\nBase:             {base_mv}");
            println!("Accurate Square:  {accurate_squared}");
            println!("Accurate Cube:    {accurate_cubed}");
            println!("Accurate 4th pow: {accurate_4th_pow}");
            println!("Accurate 5th pow: {accurate_5th_pow}");
            println!("Accurate 6th pow: {accurate_6th_pow}");

            // let trait_squared = AntiSquare.deep_inline(&builder, base_mv.clone()).await?;
            // if accurate_squared != trait_squared {
            //     println!("Wrong Trait Square: {trait_squared}");
            // }
            // let powf_squared = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(2.0)).await?;
            // if accurate_squared != powf_squared {
            //     println!("Wrong Powf Square: {powf_squared}");
            // }
            // let powf_cubed = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(3.0)).await?;
            // if accurate_cubed != powf_cubed {
            //     println!("Wrong Powf Cube: {powf_cubed}");
            // }
            //
            // let powf_root = AntiPowf.deep_inline(&builder, base_mv.clone(), FloatExpr::Literal(0.5)).await?;
            // let root_squared = GeometricAntiProduct.deep_inline(&builder, powf_root.clone(), powf_root.clone()).await?;
            // if base_mv != root_squared {
            //     println!("Wrong Root Squared: {root_squared}");
            //     println!("Wrong Powf Root:    {powf_root}");
            // }
        }
        Some(())
    });
    result.expect("Entire script must complete")
}