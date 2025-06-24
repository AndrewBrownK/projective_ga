#![allow(non_upper_case_globals)]
#![feature(const_trait_impl)]

use codegen::algebra::multivector::DeclareMultiVecs;
use codegen::elements::e1234;
use custom_traits::*;

// PRO-TIP: Run with the release profile

codegen::multi_vecs! { e1234;

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

fn main() {
    let rga3d = codegen::ga! { e1234;
        1 => e1, e2, e3;
        0 => e4
    };
    let repo = base_documentation(register_multi_vecs(rga3d)).finished();
    // TODO https://rigidgeometricalgebra.org/wiki/index.php?title=Transwedge_products
    let traits = codegen::register_all! { e1234 repo;
        Zero One AntiOne Unit
        Into TryInto
        Grade AntiGrade
        RightDual RightAntiDual
        RightComplement LeftComplement DoubleComplement
        Reverse AntiReverse
        Wedge AntiWedge
        GeometricProduct GeometricAntiProduct
        Sandwich AntiSandwich
        DotProduct AntiDotProduct
        Inverse AntiInverse
        GeometricQuotient GeometricAntiQuotient
        Fix AntiFix
        ConstraintViolation AntiConstraintViolation
        ConstraintValid AntiConstraintValid
        SquareRoot AntiSquareRoot
        AutoMorphism AntiAutoMorphism
        Conjugation
        ProjectOrthogonallyOnto AntiProjectOrthogonallyOnto
        ProjectViaOriginOnto AntiProjectViaHorizonOnto
        RejectOrthogonallyFrom AntiRejectOrthogonallyFrom
        RejectViaOriginFrom AntiRejectViaHorizonFrom
        Support AntiSupport
        Bulk Weight
        WeightNorm WeightNormSquared
        BulkNorm BulkNormSquared
        Norm NormSquared
        UnitizedNorm
        Unitize
    };
    codegen::operators! { e1234 repo, traits;
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
        "libraries/rga3d/src",
        "rga3d",
        repo.clone(),
        traits.clone()
    );


    let mut rust = codegen::Rust::new(true).all_features();
    rust.sql = false;
    rust.wgsl = false;
    rust.glsl = false;
    rust.write_crate(
        "libraries/rga3d/",
        "rga3d",
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

fn base_documentation(mut declarations: DeclareMultiVecs<e1234>) -> DeclareMultiVecs<e1234> {
    declarations.append_documentation(
        &Origin,
        "\
    The Origin is the RoundPoint where x, y, z, and radius are all zero.
    It is the base element e4.
    Not to be confused with FlatOrigin, which is a Dipole connecting Origin and Infinity.
    ",
    );
    // TODO more documentation
    // declarations.generate_missing_duals(None);
    declarations
}

pub mod custom_traits {
    use codegen::algebra::basis::BasisElement;
    use codegen::ast::impls::Elaborated;
    use codegen::build_scripts::common_traits::conformal::impls::{FlatBulkImpl, FlatBulkNormImpl, FlatBulkNormSquaredImpl, FlatNormImpl, FlatNormSquaredImpl, FlatWeightImpl, FlatWeightNormImpl, FlatWeightNormSquaredImpl, UnitizedFlatNormImpl, UnitizedFlatNormSquaredImpl};
    use codegen::build_scripts::common_traits::conformal::{flat_bulk, flat_bulk_norm, flat_bulk_norm_squared, flat_norm, flat_norm_squared, flat_weight, flat_weight_norm, flat_weight_norm_squared, unitized_flat_norm, unitized_flat_norm_squared};
    use codegen::build_scripts::common_traits::impls::{AntiSupportImpl, SupportImpl, UnitizeImpl};
    use codegen::build_scripts::common_traits::{anti_support, support, unitize};

    const origin: BasisElement = codegen::elements::e4;

    //
    pub static Support: Elaborated<SupportImpl> = support(origin);
    pub static AntiSupport: Elaborated<AntiSupportImpl> = anti_support(origin);

    pub static Bulk: Elaborated<FlatBulkImpl> = flat_bulk(origin, None).rename("Bulk");
    pub static Weight: Elaborated<FlatWeightImpl> = flat_weight(origin, None).rename("Weight");
    pub static BulkNorm: Elaborated<FlatBulkNormImpl> = flat_bulk_norm(origin, None).rename("BulkNorm");
    pub static BulkNormSquared: Elaborated<FlatBulkNormSquaredImpl> = flat_bulk_norm_squared(origin, None).rename("BulkNormSquared");
    pub static Norm: Elaborated<FlatNormImpl> = flat_norm(origin, None).rename("Norm");
    pub static NormSquared: Elaborated<FlatNormSquaredImpl> = flat_norm_squared(origin, None).rename("NormSquared");
    pub static WeightNorm: Elaborated<FlatWeightNormImpl> = flat_weight_norm(origin, None).rename("WeightNorm");
    pub static WeightNormSquared: Elaborated<FlatWeightNormSquaredImpl> = flat_weight_norm_squared(origin, None).rename("WeightNormSquared");
    pub static UnitizedNorm: Elaborated<UnitizedFlatNormImpl> = unitized_flat_norm(origin, None).rename("UnitizedNorm");
    pub static UnitizedNormSquared: Elaborated<UnitizedFlatNormSquaredImpl> = unitized_flat_norm_squared(origin, None).rename("UnitizedNormSquared");

    pub static Unitize: Elaborated<UnitizeImpl<Elaborated<FlatWeightNormImpl>>> = unitize(WeightNorm);
}
