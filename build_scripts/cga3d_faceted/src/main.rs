#![allow(non_upper_case_globals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(adt_const_params)]

use codegen::algebra::multivector::DeclareMultiVecs;
use codegen::build_scripts::common_traits::conformal::{RadiusNorm, RadiusNormSquared};
use codegen::elements::e12345;
use custom_traits::*;

// PRO-TIP: Run with the release profile

codegen::multi_vecs! { e12345;

    // Special Objects
    Scalar     as scalar;
    AntiScalar as e12345;
    Origin     as e4;
    Infinity   as e5;
    FlatOrigin as e45;
    Horizon    as e3215;
    DualNum    as e4, e12345;
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

#[test]
fn lazy_compile_button() {
    // hi
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
    let repo = generate_variants(base_documentation(register_multi_vecs(cga3d))).finished();
    let traits = codegen::register_all! { repo;
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
    codegen::operators! { repo, traits;
        fancy_infix => Div;

        binary
        Add => Addition,
        Sub => Subtraction,
        // BitXor => Wedge,
        Mul => GeometricProduct;

        unary
        Neg => Negation,
        Not => RightDual;
    }
    let traits = traits.finish();

    // At the time of this commit, this wgsl file weighs in at 72.2 MB.
    // I'm going to gitignore it and also comment it out, since
    // I don't think I want to be using it anyway.

    // let mut wgsl = codegen::Wgsl::new();
    // wgsl.write_shader_file(
    //     "libraries/cga3d_faceted/src/",
    //     "cga3d_faceted",
    //     1,
    //     0,
    //     0,
    //     "",
    //     "Latest generation test case",
    //     "https://github.com/AndrewBrownK/projective_ga/",
    //     &[],
    //     repo.clone(),
    //     traits.clone(),
    // );

    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.write_crate(
        "libraries/cga3d_faceted/",
        "cga3d_faceted",
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
    declarations.append_documentation(
        &Origin,
        "\
    The Origin is the RoundPoint where x, y, z, and radius are all zero.
    It is the base element e4.
    Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    ",
    );
    // TODO more documentation
    declarations
}

fn generate_variants(mut declarations: DeclareMultiVecs<e12345>) -> DeclareMultiVecs<e12345> {
    use codegen::algebra::basis::filter::{allow_all_signatures, SigFilter, signatures_containing};
    use codegen::elements::*;

    let origin = signatures_containing(e4);
    let infinity = signatures_containing(e5);
    let flat_origin = origin & infinity;

    let all = allow_all_signatures();
    let is_flat = infinity.all_match();
    let is_not_flat = (!infinity).any_match();
    let tangent_null_cone = origin.any_match() ^ infinity.any_match();
    let intersects_null_cone = origin.any_match() & infinity.any_match();

    codegen::variants! { declarations;

        #docs("This variant of {type} has a radius of zero and is centered on the Origin.")
        Null{type}AtOrigin => (origin & !infinity)  where is_not_flat => tangent_null_cone;

        #docs("This variant of {type} intersects the Origin.")
        {type}OnOrigin => (origin)                  where all => intersects_null_cone;

        #docs("This variant of {type} exists in the Horizon.")
        {type}AtInfinity => (!origin)               where is_flat => tangent_null_cone;

        #docs("This variant of {type} is centered on the Origin.")
        {type}AtOrigin => (origin ^ infinity)       where is_not_flat => intersects_null_cone;

        // TODO I think we want to rename this to AtInfinity
        //  Notably, both the carrier and cocarrier are completely in the horizon
        //  ACTUALLY.... Don't mix it up! MysteryDipole has e45! This makes the CoCarrier at infinity,
        //  but this element itself is actually the FlatOrigin!
        #docs("TODO this is currently a mystery I'm investigating")
        Mystery{type} => (!(origin ^ infinity))     where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a Carrier that intersects the Origin.")
        {type}AligningOrigin => (origin | infinity) where is_not_flat => intersects_null_cone;

        #docs("This variant of {type} has a CoCarrier that intersects the Origin.")
        {type}OrthogonalOrigin => (!flat_origin)    where is_not_flat => intersects_null_cone;

        // TODO and this.... should change the name slightly.
        //  The carrier is at infinity, but the cocarrier is not
        //  and could there be yet another with finite carrier but infinite cocarrier?
        #docs("This variant of {type} exists at the Horizon.")
        {type}AtInfinity => (!origin | flat_origin) where is_not_flat => intersects_null_cone;
    }

    declarations.generate_missing_duals(Some(
        "This variant of {super} is the Dual to {type}. It is common for
        objects of this type to not intersect the null cone, which also prevents them from
        projecting onto the horosphere in the usual manner. When this happens, this
        object has behavioral and operative similarity to a {super},
        but an imaginary radius, and a spacial presence in the shape of a
        {type} with a real radius.",
    ));
    declarations
}

//     pub static Carrier: Elaborated<CarrierImpl> = CarrierImpl.new_trait_named("Carrier").blurb("TODO");
//
//     trait_impl_1_type_1_arg!(CarrierImpl(builder, slf) -> MultiVector {
//         let mut result = DynamicMultiVector::zero();
//         for (mut fe, el) in slf.elements() {
//             let (f, el) = builder.ga.wedge(el, e5);
//             result += (fe * f, el);
//         }
//         let result = result.construct(&builder)?;
//         builder.return_expr(result)
//     });
//
//     pub static CoCarrier: Elaborated<CoCarrierImpl> = CoCarrierImpl.new_trait_named("CoCarrier").blurb("TODO");
//
//     trait_impl_1_type_1_arg!(CoCarrierImpl(builder, slf) -> MultiVector {
//         let slf = RightAntiDual.invoke(&mut builder, slf).await?;
//         let mut result = DynamicMultiVector::zero();
//         for (mut fe, el) in slf.elements() {
//             let (f, el) = builder.ga.wedge(el, e5);
//             result += (fe * f, el);
//         }
//         let result = result.construct(&builder)?;
//         builder.return_expr(result)
//     });



// noinspection DuplicatedCode
pub mod custom_traits {
    #![allow(non_upper_case_globals)]

    use codegen::algebra::basis::BasisElement;
    use codegen::ast::impls::Elaborated;
    use codegen::build_scripts::common_traits::{anti_support, support, unitize};
    use codegen::build_scripts::common_traits::conformal::*;
    use codegen::build_scripts::common_traits::conformal::impls::*;
    use codegen::build_scripts::common_traits::impls::{AntiSupportImpl, SupportImpl, UnitizeImpl};

    const origin: BasisElement = codegen::elements::e4;
    const infinity: BasisElement = codegen::elements::e5;
    const option_infinity: Option<BasisElement> = Some(codegen::elements::e5);

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