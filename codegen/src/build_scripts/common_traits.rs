#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]

use crate::algebra::basis::BasisElement;
use crate::algebra::basis::filter::{SigFilter, SigSetFilter};
use crate::algebra::basis::grades::{AntiGrades, Grades};
use crate::ast::datatype::MultiVector;
use crate::ast::impls::{Elaborated, InlineOnly};
use crate::ast::traits::{NameTrait, TraitDef_1_Type_1_Arg};
use crate::build_scripts::common_traits::impls::*;

pub mod conformal;

pub static Zero: Elaborated<ZeroImpl> = ZeroImpl
    .new_trait_named("Zero")
    .blurb("All elements set to zero.");
pub static One: Elaborated<OneImpl> = OneImpl
    .new_trait_named("One")
    .blurb("The scalar element set to one, and all other elements set to zero.");
pub static AntiOne: Elaborated<AntiOneImpl> = AntiOneImpl
    .new_trait_named("AntiOne")
    .blurb("The anti-scalar element set to one, and all other elements set to zero.");
pub static Unit: Elaborated<UnitImpl> = UnitImpl
    .new_trait_named("Unit")
    .blurb("All elements set to one.");
pub static Grade: Elaborated<GradeImpl> = GradeImpl
    .new_trait_named("Grade")
    .blurb("A multivector may have uniform grade, or mixed grade, depending on \
    the grades of its elements. This trait only characterizes uniform grade multivectors.");
pub static AntiGrade: Elaborated<AntiGradeImpl> = AntiGradeImpl
    .new_trait_named("AntiGrade")
    .blurb("The AntiGrade can be described as the missing Grade with respect to an \
    AntiScalar. This trait only characterizes uniform anti-grade multivectors.");

pub static RightDual: Elaborated<RightDualImpl> = RightDualImpl
    .new_trait_named("RightDual")
    .blurb("This dual is the \"Metric Right Dual\". To take this dual of an object, \
    the object is multiplied by the metric, and then we take the right complement. This will turn \
    a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use \
    of the metric may give distinct results from a simple right complement, typically by changing \
    the coefficient on some terms (1, -1, or 0)");

pub static RightAntiDual: Elaborated<RightAntiDualImpl> = RightAntiDualImpl
    .new_trait_named("RightAntiDual")
    .blurb("This dual is the \"AntiMetric Right Dual\". To take this dual of an object, \
    the object is multiplied by the anti-metric, and then we take the right complement. This will \
    turn a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use \
    of the anti-metric may give distinct results from a simple right complement, typically by \
    changing the coefficient on some terms (1, -1, or 0)");

pub static LeftDual: Elaborated<LeftDualImpl> = LeftDualImpl
    .new_trait_named("LeftDual")
    .blurb("This dual is the \"Metric Left Dual\". To take this dual of an object, \
    the object is multiplied by the metric, and then we take the left complement. This will turn \
    a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use \
    of the metric may give distinct results from a simple left complement, typically by changing \
    the coefficient on some terms (1, -1, or 0)");

pub static LeftAntiDual: Elaborated<LeftAntiDualImpl> = LeftAntiDualImpl
    .new_trait_named("LeftAntiDual")
    .blurb("This dual is the \"AntiMetric Left Dual\". To take this dual of an object, \
    the object is multiplied by the anti-metric, and then we take the left complement. This will \
    turn a Scalar into an AntiScalar, a Vector into an AntiVector, so on, and vice versa. The use \
    of the metric may give distinct results from a simple left complement, typically by changing \
    the coefficient on some terms (1, -1, or 0).");

pub static Reverse: Elaborated<ReverseImpl> = ReverseImpl
    .new_trait_named("Reverse")
    .blurb("Reversal is an operation that will negate some BasisElements. This changes \
    the \"sign\" or \"direction\" of a BasisElement. Each BasisElement is a wedge of some \
    generator elements, for example e12 = wedge(e1, e2). The reversal of e12 is reverse(e12) = \
    e21 = -e12 = wedge(e2, e1). When the number of generator element position swaps is odd, then \
    the reverse negates thee sign. Otherwise, the sign stays the same. For example, reverse(e1) = \
    e1, and reverse(e1234) = e1234 = e4321. This is a consequence of the wedge product being \
    anti-commutative.");

pub static AntiReverse: Elaborated<AntiReverseImpl> = AntiReverseImpl
    .new_trait_named("AntiReverse")
    .blurb("The AntiReversal is like Reversal, but with respect to the AntiWedge \
    product instead of the Wedge product. This means we can only find the AntiReverse of an \
    element if we specify an AntiScalar first. By example, if our AntiScalar is e123 in vanilla \
    geometric algebra, then e2 = anti_wedge(e12, e32), and anti_reverse(e2) = anti_wedge(e32, e12) \
    = -e2. Notably, the Reverse of grade 1 vectors does not change sign, but the AntiReverse of \
    grade 1 vectors may change sign (depending on the AntiScalar). When it comes to the \
    AntiReverse, it is AntiGrade 1 Vectors (AntiVectors) that will not change sign. ");
/*
    As usual with \
    Anti-Operations and duality, it is useful to think of the space missing from the object. When \
    an AntiScalar is specified, any geometric object implies not just itself, but its dual. \
    Therefore in our 3D VGA example, you can imagine that a Vector AntiReverses to its negative \
    because its dual BiVector would Reverse to its own negative. Like a right-hand-rule axis \
    (thumb) and spin (fingers) flipping from thumbs up to thumbs down.
*/

pub static AutoMorphism: Elaborated<AutoMorphismImpl> = AutoMorphismImpl
    .new_trait_named("AutoMorphism")
    .blurb("Negate every BasisElement with an odd Grade. Also known as grade involution.");

pub static AntiAutoMorphism: Elaborated<AntiAutoMorphismImpl> = AntiAutoMorphismImpl
    .new_trait_named("AntiAutoMorphism")
    .blurb("Negate every BasisElement with an odd AntiGrade.");

pub static Conjugation: Elaborated<ConjugationImpl> = ConjugationImpl
    .new_trait_named("Conjugation")
    .blurb("This composes the reverse and grade involution (automorphism).");

pub static AntiConjugation: Elaborated<AntiConjugationImpl> = AntiConjugationImpl
    .new_trait_named("AntiConjugation")
    .blurb("This composes anti-reverse and anti-grade involution (anti-automorphism)");

pub static Complement: Elaborated<RightComplementImpl> = RightComplementImpl
    .new_trait_named("Complement")
    .blurb("The Complement of a BasisElement is the missing BasisElement that when \
    wedged together will create the AntiScalar. In geometric algebras with an odd number of \
    dimensions, the LeftComplement and RightComplement are the same, so we just call it the \
    Complement. For example, with an AntiScalar of e123, the complement of e1 can be found \
    equivalently by solving e123 = wedge(e1, x) or e123 = wedge(x, e1). See also \
    DoubleComplement.");

pub static RightComplement: Elaborated<RightComplementImpl> = RightComplementImpl
    .new_trait_named("RightComplement")
    .blurb("The RightComplement of a BasisElement is the missing BasisElement that when \
    wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the \
    right_complement(e1) = e234, because wedge(e1, e234) = e1234. In this example, the \
    right_complement(e234) = -e1, because wedge(e234, -e1) = e1234. See also LeftComplement and \
    DoubleComplement. The LeftComplement can be used to undo a RightComplement.");

pub static LeftComplement: Elaborated<LeftComplementImpl> = LeftComplementImpl
    .new_trait_named("LeftComplement")
    .blurb("The LeftComplement of a BasisElement is the missing BasisElement that when \
    wedged together will create the AntiScalar. For example, with an AntiScalar of e1234, the \
    left_complement(e1) = -e234, because wedge(e234, e1) = e1234. In this example, the \
    left_complement(e234) = e1, because wedge(e1, e234) = e1234. See also RightComplement and \
    DoubleComplement. The RightComplement can be used to undo a LeftComplement.");

pub static DoubleComplement: Elaborated<DoubleComplementImpl> = DoubleComplementImpl
    .new_trait_named("DoubleComplement")
    .blurb("Repeatedly taking a Complement will eventually return the original object. \
    In geometric algebras with an even number of dimensions, double_complement(x) = \
    right_complement(right_complement(x)) = left_complement(left_complement(x)). In geometric \
    algebras with an odd number of dimensions, double_complement(x) = complement(complement(x)). \
    In all cases, x = double_complement(double_complement(x)). ");

pub static Negation: Elaborated<NegationImpl> = NegationImpl
    .new_trait_named("Negation")
    .blurb("Negates all elements.");

pub static Addition: Elaborated<AdditionImpl> = AdditionImpl
    .new_trait_named("Addition")
    .blurb("Add two MultiVectors together, possibly resulting in a different type of \
    MultiVector.");

pub static Subtraction: Elaborated<SubtractionImpl> = SubtractionImpl
    .new_trait_named("Subtraction")
    .blurb("Subtract two MultiVectors, possibly resulting in a different type of \
    MultiVector.");

pub static Wedge: Elaborated<WedgeImpl> = WedgeImpl
    .new_trait_named("Wedge")
    .blurb("The Wedge product (also known as \"Exterior Product\" or Grassmann's \
    \"Progressive Combinatorial Product\") combines BasisElements into higher grade BasisElements. \
    For example, wedge(e1, e2) = e12, and wedge(e1, e23) = e123. The Wedge product is \
    anti-commutative, so wedge(a, b) = -wedge(b, a). A non-scalar element wedged with itself is \
    zero. This behaves something like a union of the subscripts in the BasisElements.");

pub static AntiWedge: Elaborated<AntiWedgeImpl> = AntiWedgeImpl
    .new_trait_named("AntiWedge")
    .blurb("The AntiWedge product is the dual operation to the Wedge product, that \
    depends on a specified AntiScalar. It combines BasisElements by which parts are missing, \
    instead of which parts are present. For example, with an AntiScalar of e1234, \
    anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts \
    in the BasisElements.");

pub static GeometricProduct: Elaborated<GeometricProductImpl> = GeometricProductImpl
    .new_trait_named("GeometricProduct")
    .blurb("The geometric product is what lets us treat geometry like algebra. It is \
    literally multiplication of geometry. It is derived by the wedge product and the algebra's \
    metric. Generator elements (grade 1 BasisElements) will square to a particular \
    value (as specified by the algebra and metric), typically 1, -1, or 0. This is in contrast to \
    the wedge product which always wedge-squares elements to zero. Multiplying uniform grade \
    geometry may result in mixed grade products, topping out at the AntiScalar. \
    See also Sandwich.");

pub static GeometricAntiProduct: Elaborated<GeometricAntiProductImpl> = GeometricAntiProductImpl
    .new_trait_named("GeometricAntiProduct")
    .blurb("The GeometricAntiProduct or sometimes called AntiProduct is the dual to \
    the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade \
    geometry may result in mixed grade anti-products, bottoming out at the Scalar. \
    See also AntiSandwich.");

pub static Sandwich: Elaborated<SandwichImpl> = SandwichImpl
    .new_trait_named("Sandwich")
    .blurb("The so-called \"sandwich product\" squeezes some factor A between another \
    factor B and the reversal of B. This is frequently used to represent geometric \
    transformations, for example reflecting across a plane or rotating around a line.");

pub static AntiSandwich: Elaborated<AntiSandwichImpl> = AntiSandwichImpl
    .new_trait_named("AntiSandwich")
    .blurb("The anti-sandwich is the dual to the sandwich, using the AntiProduct instead \
    of the Product, and the AntiReverse instead of the Reverse. This is also used to represent \
    geometric transformations, for example reflecting across a plane or rotating around a line. \
    The Sandwich and AntiSandwich are not identical for the purposes of transforming geometry, \
    you simply choose which one to use depending on your geometric interpretation and the algebra. \
    For example, in G(3,0,1) you may interpret grade 1 vectors as points or planes, since they \
    are dual to one another. The sandwich product gives euclidean transformations in the grade 1 \
    = planes interpretation, and the AntiSandwich gives euclidean transformations in the grade 1 \
    = points interpretation.");

// By all means, call anything you want the "dot product" in some application.
// That is the entire point of this framework, that you can name stuff whatever you want,
// and change the documentation as you see fit! 😀
// However, for the pre-built traits provided by this module, and a limited namespace, we find
// the arguments in this blog post to be compelling. https://terathon.com/blog/poor-foundations-ga.html
pub static DotProduct: Elaborated<DotProductImpl> = DotProductImpl
    .new_trait_named("DotProduct")
    .blurb("This dot product is almost exactly what you would expect from regular \
    vector algebra. It always returns a scalar result. It is determined by the metric of the \
    algebra, so intermediate terms might come out to zero or negative, depending on the generator \
    squares. The \"dot product\" is overloaded to several different meanings depending on which \
    community you discuss with, so if someone or something refers to the \"dot product\" then \
    use care and double check the definition that is intended.");

pub static AntiDotProduct: Elaborated<AntiDotProductImpl> = AntiDotProductImpl
    .new_trait_named("AntiDotProduct")
    .blurb("This is the dual to the dot product, and always returns an AntiScalar.");

pub static ScalarNormSquared: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl
    .new_trait_named("ScalarNormSquared")
    .blurb("This is an intermediate step to the ScalarNorm, prior to taking a \
    square root.");

pub static AntiScalarNormSquared: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl
    .new_trait_named("AntiScalarNormSquared")
    .blurb("This is an intermediate step to the AntiScalarNorm, prior to taking a \
    square root (or AntiSquareRoot, to be technical).");

pub static ScalarNorm: Elaborated<ScalarNormSquaredImpl> = ScalarNormSquaredImpl
    .new_trait_named("ScalarNorm")
    .blurb("The ScalarNorm of a geometric object characterizes the object with a Scalar. \
    The exact meaning of the characterization depends on the algebra and geometric interpretation. \
    As an example, in G(3,0,1) with grade 1 vectors as points, the ScalarNorm is the \"Bulk Norm\" \
    of the object, i.e. the ScalarNorm of the Bulk aspect of the object. In this example, the \
    ScalarNorm characterizes the magnitude of the Bulk (compared to the magnitude of the Weight), \
    and therefore the proximity to the origin (once the WeightNorm is incorporated, or assuming \
    unitization).");

pub static AntiScalarNorm: Elaborated<AntiScalarNormImpl> = AntiScalarNormImpl
    .new_trait_named("AntiScalarNorm")
    .blurb("The AntiScalarNorm is dual to the ScalarNorm. It characterizes a geometric \
    object with an AntiScalar. The exact meaning of the characterization depends on the algebra \
    and geometric interpretation. As an example, in G(3,0,1) with grade 1 vectors as points, \
    the AntiScalarNorm is the \"Weight Norm\" of the object, i.e. the AntiScalarNorm of the Weight \
    aspect of the object. In this example, the AntiScalarNorm characterizes the magnitude of the \
    Weight (compared to the magnitude of the Bulk), and therefore the proximity to the horizon \
    (once the BulkNorm is incorporated).");




#[cfg(feature = "incorrect-wip-traits")]
pub static Square: Elaborated<SquareImpl> = SquareImpl
    .new_trait_named("Square")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiSquare: Elaborated<AntiSquareImpl> = AntiSquareImpl
    .new_trait_named("AntiSquare")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static Powf: Elaborated<PowfImpl> = PowfImpl
    .new_trait_named("Powf")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static Powi: Elaborated<PowiImpl> = PowiImpl
    .new_trait_named("Powi")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiPowf: Elaborated<AntiPowfImpl> = AntiPowfImpl
    .new_trait_named("AntiPowf")
    .blurb("TODO");
#[cfg(feature = "incorrect-wip-traits")]
pub static AntiPowi: Elaborated<AntiPowiImpl> = AntiPowiImpl
    .new_trait_named("AntiPowi")
    .blurb("TODO");






pub static ConstraintViolation: Elaborated<ConstraintViolationImpl> = ConstraintViolationImpl
    .new_trait_named("ConstraintViolation")
    .blurb("Not every combinations of floats is valid geometry. Some types of objects \
    are required to fulfill a constraint in order to be valid geometry. We call this the geometric \
    constraint. If a type of object may possibly violate this constraint, then it will implement \
    this trait. The constraint is violated if a non-zero value is returned. See also \
    ConstraintValid and Fix.");

pub static AntiConstraintViolation: Elaborated<AntiConstraintViolationImpl> = AntiConstraintViolationImpl
    .new_trait_named("AntiConstraintViolation")
    .blurb("Not every combinations of floats is valid geometry. Some types of objects \
    are required to fulfill a constraint in order to be valid geometry. We call this the (anti) \
    geometric constraint. If a type of object may possibly violate this constraint, then it will \
    implement this trait. The constraint is violated if a non-zero value is returned. See also \
    AntiConstraintValid and AntiFix.");

pub static ConstraintValid: Elaborated<ConstraintValidImpl> = ConstraintValidImpl
    .new_trait_named("ConstraintValid")
    .blurb("Implementors of this trait cannot violate the geometric constraint. They \
    always represent valid geometry. This trait does not exist to perform any calculation, it \
    just exists to serve as contrasting information side-by-side with ConstraintViolation. See \
    also ConstraintViolation and Fix.");

pub static AntiConstraintValid: Elaborated<AntiConstraintValidImpl> = AntiConstraintValidImpl
    .new_trait_named("AntiConstraintValid")
    .blurb("Implementors of this trait cannot violate the anti geometric constraint. \
    They always represent valid geometry. This trait does not exist to perform any calculation, it \
    just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See \
    also AntiConstraintViolation and AntiFix.");

pub static Fix: Elaborated<FixImpl> = FixImpl
    .new_trait_named("Fix")
    .blurb("Automatically fix the geometric constraint by adjusting the weight to comply \
    with the bulk, and then bulk normalize the result.");

pub static AntiFix: Elaborated<AntiFixImpl> = AntiFixImpl
    .new_trait_named("AntiFix")
    .blurb("Automatically fix the anti geometric constraint by adjusting the bulk to \
    comply with the weight, and then weight normalize the result.");




pub static Inverse: Elaborated<InverseImpl> = InverseImpl
    .new_trait_named("Inverse")
    .blurb("The inverse with respect to geometric product. Inverse(x) = x^-1. ");

pub static AntiInverse: Elaborated<AntiInverseImpl> = AntiInverseImpl
    .new_trait_named("AntiInverse")
    .blurb("The inverse with respect to the geometric anti-product.");

pub static GeometricQuotient: Elaborated<GeometricQuotientImpl> = GeometricQuotientImpl
    .new_trait_named("GeometricQuotient")
    .blurb("Product of A with Inverse of B.");

pub static GeometricAntiQuotient: Elaborated<GeometricAntiQuotientImpl> = GeometricAntiQuotientImpl
    .new_trait_named("GeometricAntiQuotient")
    .blurb("AntiProduct of A with AntiInverse of B.");

pub static SquareRoot: Elaborated<SquareRootImpl> = SquareRootImpl
    .new_trait_named("SquareRoot")
    .blurb("Square root of geometry. Multiple different types of geometry might \
    square to one type of geometry, so this is only defined for types that are closed with \
    themselves under the geometric product. ");

pub static AntiSquareRoot: Elaborated<AntiSquareRootImpl> = AntiSquareRootImpl
    .new_trait_named("AntiSquareRoot")
    .blurb("Square root of geometry with respect to the AntiProduct. Multiple different \
    types of geometry might anti-square to one type of geometry, so this is only defined for \
    types that are closed with themselves under the geometric anti-product.");

pub static BulkContraction: Elaborated<BulkContractionImpl> = BulkContractionImpl
    .new_trait_named("BulkContraction")
    .blurb("This is an interior product (contrast with inner product and exterior \
    product). The interior products are derived by Wedging (or AntiWedging) one object with \
    the Dual (or AntiDual) of another object.");

pub static WeightContraction: Elaborated<WeightContractionImpl> = WeightContractionImpl
    .new_trait_named("WeightContraction")
    .blurb("This is an interior product (contrast with inner product and exterior \
    product). The interior products are derived by Wedging (or AntiWedging) one object with \
    the Dual (or AntiDual) of another object.");

pub static BulkExpansion: Elaborated<BulkExpansionImpl> = BulkExpansionImpl
    .new_trait_named("BulkExpansion")
    .blurb("This is an interior product (contrast with inner product and exterior \
    product). The interior products are derived by Wedging (or AntiWedging) one object with \
    the Dual (or AntiDual) of another object.");

pub static WeightExpansion: Elaborated<WeightExpansionImpl> = WeightExpansionImpl
    .new_trait_named("WeightExpansion")
    .blurb("This is an interior product (contrast with inner product and exterior \
    product). The interior products are derived by Wedging (or AntiWedging) one object with \
    the Dual (or AntiDual) of another object.");

pub static ProjectOrthogonallyOnto: Elaborated<ProjectOrthogonallyOntoImpl> = ProjectOrthogonallyOntoImpl
    .new_trait_named("ProjectOrthogonallyOnto")
    .blurb("Typically involves bringing a lower dimensional object to a higher dimensional object.");

pub static AntiProjectOrthogonallyOnto: Elaborated<AntiProjectOrthogonallyOntoImpl> = AntiProjectOrthogonallyOntoImpl
    .new_trait_named("AntiProjectOrthogonallyOnto")
    .blurb("Typically involves bringing a higher dimensional object to a lower dimensional object.");

pub static ProjectViaOriginOnto: Elaborated<ProjectViaOriginOntoImpl> = ProjectViaOriginOntoImpl
    .new_trait_named("ProjectViaOriginOnto")
    .blurb("Central (to origin) Projection.");

pub static AntiProjectViaHorizonOnto: Elaborated<AntiProjectViaHorizonOntoImpl> = AntiProjectViaHorizonOntoImpl
    .new_trait_named("AntiProjectViaHorizonOnto")
    .blurb("Outward (to horizon) AntiProjection.");

pub static RejectOrthogonallyFrom: Elaborated<RejectOrthogonallyFromImpl> = RejectOrthogonallyFromImpl
    .new_trait_named("RejectOrthogonallyFrom")
    .blurb("Counterpart to ProjectOrthogonallyOnto.");

pub static AntiRejectOrthogonallyFrom: Elaborated<AntiRejectOrthogonallyFromImpl> = AntiRejectOrthogonallyFromImpl
    .new_trait_named("AntiRejectOrthogonallyFrom")
    .blurb("Counterpart to AntiProjectOrthogonallyOnto.");

pub static RejectViaOriginFrom: Elaborated<RejectViaOriginFromImpl> = RejectViaOriginFromImpl
    .new_trait_named("RejectViaOriginFrom")
    .blurb("Counterpart to ProjectViaOriginOnto.");

pub static AntiRejectViaHorizonFrom: Elaborated<AntiRejectViaHorizonFromImpl> = AntiRejectViaHorizonFromImpl
    .new_trait_named("AntiRejectViaHorizonFrom")
    .blurb("Counterpart to AntiProjectViaHorizonOnto.");

pub const fn support(origin: BasisElement) -> Elaborated<SupportImpl> {
    SupportImpl { origin }
        .new_trait_named("Support")
        .blurb("The support is the point enclosed by the object closest to the origin.")
}

pub const fn anti_support(origin: BasisElement) -> Elaborated<AntiSupportImpl> {
    AntiSupportImpl { origin }
        .new_trait_named("AntiSupport")
        .blurb("The anti-support is the anti-vector furthest from the origin that encloses the object.")
}

pub const fn unitize<WeightNorm>(weight_norm: WeightNorm) -> Elaborated<UnitizeImpl<WeightNorm>>
    where WeightNorm: TraitDef_1_Type_1_Arg<Output=MultiVector> {
    UnitizeImpl { weight_norm }
        .new_trait_named("Unitize")
        .blurb("Scale the object to have a weight norm of 1.")
}





pub static Into: Elaborated<IntoImpl> = IntoImpl
    .new_trait_named("Into")
    .blurb("Reliably convert one type into another.");

pub static TryInto: Elaborated<TryIntoImpl> = TryIntoImpl
    .new_trait_named("TryInto")
    .blurb("Fallibly convert one type into another.");

#[allow(non_snake_case)]
pub fn SubType<F1, F2, F3>(filter_multivecs_in: F1, filter_elements: F2, filter_multivecs_out: F3) -> InlineOnly<SubTypeImpl<F1, F2, F3>> where
    F1: SigSetFilter + Copy + Send + Sync + 'static,
    F2: SigFilter + Copy + Send + Sync + 'static,
    F3: SigSetFilter + Copy + Send + Sync + 'static {
    InlineOnly::new("SubType", SubTypeImpl { filter_multivecs_in, filter_elements, filter_multivecs_out })
}


// NOTE: If you find yourself wanting to generate grade selection traits, you are
// probably generating extremely wasteful implementations that perform a lot more
// floating point calculations than necessary. That is why these trait definitions
// are InlineOnly. They will not generate trait definitions, but you can still invoke
// them as if they were traits, and they will always be inlined instead. (With inlining,
// terms that are unused by the final result of each function will be cut out of the AST
// automatically. With grade selection declarations (not inlining strictly always), then
// you'd have the chance to use grade selection in application code which is very undesirable.)

macro_rules! select_grades {
    ($($gr:literal),+ $(,)?) => {
        pub static SelectGrade: [InlineOnly<SelectGradesImpl>; 17] = [$(
            InlineOnly::new(concat!("SelectGrade", $gr), SelectGradesImpl(Grades::new($gr))),
        )+];
        pub static SelectAntiGrade: [InlineOnly<SelectAntiGradesImpl>; 17] = [$(
            InlineOnly::new(concat!("SelectAntiGrade", $gr), SelectAntiGradesImpl(AntiGrades::new($gr))),
        )+];
    };
}
select_grades!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);

pub const fn select_grades(grades: Grades) -> InlineOnly<SelectGradesImpl> {
    InlineOnly::new("SelectMultipleGrades", SelectGradesImpl(grades))
}
pub const fn select_anti_grades(anti_grades: AntiGrades) -> InlineOnly<SelectAntiGradesImpl> {
    InlineOnly::new("SelectMultipleAntiGrades", SelectAntiGradesImpl(anti_grades))
}

pub mod impls {
    use std::collections::{BTreeMap, BTreeSet};

    use async_trait::async_trait;

    use crate::algebra::basis::{BasisElement, BasisSignature};
    use crate::algebra::basis::filter::{SigFilter, SigSetFilter};
    use crate::algebra::basis::grades::{AntiGrades, Grades};
    use crate::algebra::multivector::DynamicMultiVector;
    use crate::ast::datatype::{Integer, MultiVector};
    use crate::ast::expressions::{Expression, FloatExpr, IntExpr};
    use crate::ast::traits::{HasNotReturned, TraitDef_1_Type_1_Arg, TraitDef_2_Types_2_Args, TraitImpl_11, TraitImplBuilder};
    use crate::ast::Variable;
    use crate::build_scripts::common_traits::{AntiConstraintViolation, AntiDotProduct, AntiInverse, AntiReverse, AntiSquareRoot, AntiWedge, ConstraintViolation, DotProduct, GeometricAntiProduct, GeometricProduct, Inverse, Reverse, RightAntiDual, RightComplement, RightDual, SquareRoot, Subtraction, Wedge};
    use crate::elements::scalar;

    #[macro_export]
    macro_rules! trait_impl_1_type_0_args {
        ($trait_impl:ident($builder:ident, $owner:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_10 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $owner: $crate::ast::datatype::MultiVector,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_1_arg {
        ($trait_impl:ident($builder:ident, $slf:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_11 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_2_types_1_arg {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_21 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::datatype::MultiVector,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_2_types_2_args {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_22 for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_2_arg_i32 {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_12i for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::Integer>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }
    #[macro_export]
    macro_rules! trait_impl_1_type_2_arg_f32 {
        ($trait_impl:ident($builder:ident, $slf:ident, $other:ident) -> $output:ident $the_impl:tt) => {
            #[derive(Clone, Copy)]
            pub struct $trait_impl;
            #[async_trait]
            impl $crate::ast::traits::TraitImpl_12f for $trait_impl {
                type Output = $output;
                #[allow(unused_mut)]
                async fn general_implementation<const AntiScalar: $crate::algebra::basis::BasisElement>(
                    self,
                    mut $builder: $crate::ast::traits::TraitImplBuilder<AntiScalar, $crate::ast::traits::HasNotReturned>,
                    $slf: $crate::ast::Variable<$crate::ast::datatype::MultiVector>,
                    $other: $crate::ast::Variable<$crate::ast::datatype::Float>,
                ) -> Option<$crate::ast::traits::TraitImplBuilder<AntiScalar, Self::Output>> {
                    $the_impl
                }
            }
        };
    }

    trait_impl_1_type_0_args!(ZeroImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(0.0)))
    });

    trait_impl_1_type_0_args!(OneImpl(builder, owner) -> MultiVector {
        if !owner.elements().into_iter().any(|el| el.signature() == BasisSignature::scalar) {
            return None;
        }
        builder.return_expr(owner.construct(|element| {
            if element.signature() == BasisSignature::scalar {
                FloatExpr::Literal(1.0)
            } else {
                FloatExpr::Literal(0.0)
            }
        }))
    });

    trait_impl_1_type_0_args!(AntiOneImpl(builder, owner) -> MultiVector {
        if !owner.elements().into_iter().any(|el| el.signature() == AntiScalar.signature()) {
            return None;
        }
        builder.return_expr(owner.construct(|element| {
            if element.signature() == AntiScalar.signature() {
                FloatExpr::Literal(1.0)
            } else {
                FloatExpr::Literal(0.0)
            }
        }))
    });

    trait_impl_1_type_0_args!(UnitImpl(builder, owner) -> MultiVector {
        builder.return_expr(owner.construct(|_| FloatExpr::Literal(1.0)))
    });

    trait_impl_1_type_0_args!(GradeImpl(builder, owner) -> Integer {
        let gr = owner.grade()?;
        builder.return_expr(IntExpr::Literal(gr))
    });

    trait_impl_1_type_0_args!(AntiGradeImpl(builder, owner) -> Integer {
        let ag = owner.anti_grade()?;
        builder.return_expr(IntExpr::Literal(ag))
    });

    // TODO left duals, because https://terathon.com/blog/poor-foundations-ga.html
    //  Also rename scalar product to inner product, also for technical reasons in that blog post

    trait_impl_1_type_1_arg!(RightDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.right_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(RightAntiDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.right_anti_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.left_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftAntiDualImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.left_anti_dual(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiReverseImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.anti_reverse(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AutoMorphismImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements() {
            if el.grade() % 2 == 1 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiAutoMorphismImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements() {
            if el.anti_grade(AntiScalar) % 2 == 1 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ConjugationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements() {
            if (el.grade() + 3) % 4 < 2 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiConjugationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (mut fe, el) in slf.elements() {
            if (el.anti_grade(AntiScalar) + 3) % 4 < 2 {
                fe = fe * -1.0;
            }
            result += (fe, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(RightComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.fix_name_and_sign(el.right_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(LeftComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let (f, el) = builder.ga.fix_name_and_sign(el.left_complement(AntiScalar));
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(DoubleComplementImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            let el = el.right_complement(AntiScalar).right_complement(AntiScalar);
            let (f, el) = builder.ga.fix_name_and_sign(el);
            result += (fe * f, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(NegationImpl(builder, slf) -> MultiVector {
        let mut result = DynamicMultiVector::zero();
        for (fe, el) in slf.elements() {
            result += (fe * -1.0, el);
        }
        let result = result.construct(&builder)?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(AdditionImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            dyn_mv += (a, a_el);
        }
        for (b, b_el) in other.elements() {
            dyn_mv += (b, b_el);
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(SubtractionImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            dyn_mv += (a, a_el);
        }
        for (b, b_el) in other.elements() {
            dyn_mv += (b * -1.0, b_el);
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(WedgeImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let a = a.clone();
                let (f, c) = builder.ga.wedge(a_el, b_el);
                dyn_mv += (a * b * f, c);
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(AntiWedgeImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let a = a.clone();
                let (f, c) = builder.ga.anti_wedge(a_el, b_el);
                dyn_mv += (a * b * f, c);
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(GeometricProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(GeometricAntiProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let sop = builder.ga.anti_product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(SandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricProduct.inline(&mut builder, slf.clone(), other).await?;
        let r = Reverse.invoke(&mut builder, slf).await?;
        let result = GeometricProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(AntiSandwichImpl(builder, slf, other) -> MultiVector {
        // TODO incorrect cycle detection if use all invoke
        let c = GeometricAntiProduct.inline(&mut builder, slf.clone(), other).await?;
        let r = AntiReverse.invoke(&mut builder, slf).await?;
        let result = GeometricAntiProduct.invoke(&mut builder, c, r).await?;
        builder.return_expr(result)
    });

    // TODO this is giving incorrect results, which is kind of a big deal
    //  see impl DotProduct<Plane> for Plane and compare it to page 73
    trait_impl_2_types_2_args!(DotProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let sop = builder.ga.inner_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = p.coefficient;
                    dyn_mv += (a * b * c, p.element);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_2_types_2_args!(AntiDotProductImpl(builder, slf, other) -> MultiVector {
        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in other.elements() {
                let sop = builder.ga.inner_anti_product(a_el, b_el);
                for p in sop.sum {
                    let a = a.clone();
                    let b = b.clone();
                    let c = p.coefficient;
                    dyn_mv += (a * b * c, p.element);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        builder.return_expr(mv)
    });

    trait_impl_1_type_1_arg!(ScalarNormSquaredImpl(builder, slf) -> MultiVector {
        let result = DotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormSquaredImpl(builder, slf) -> MultiVector {
        let result = AntiDotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ScalarNormImpl(builder, slf) -> MultiVector {
        let dot = DotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = SquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiScalarNormImpl(builder, slf) -> MultiVector {
        let dot = AntiDotProduct.invoke(&mut builder, slf.clone(), slf).await?;
        let result = AntiSquareRoot.invoke(&mut builder, dot).await?;
        builder.return_expr(result)
    });





    // TODO troublesome traits that won't make it to 1.0
    #[cfg(feature = "incorrect-wip-traits")]
    include!("wip_traits.rs");





    trait_impl_1_type_1_arg!(ConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&mut builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&mut builder, slf.clone(), r).await?;
        let d = DotProduct.inline(&mut builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&mut builder, p, d).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiConstraintViolationImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&mut builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&mut builder, slf.clone(), r).await?;
        let d = AntiDotProduct.inline(&mut builder, slf.clone(), slf.clone()).await?;
        let result = Subtraction.inline(&mut builder, p, d).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(ConstraintValidImpl(builder, slf) -> MultiVector {
        return match ConstraintViolation.invoke(&mut builder, slf.clone()).await {
            None => builder.return_expr(slf),
            Some(_) => None,
        }
    });
    trait_impl_1_type_1_arg!(AntiConstraintValidImpl(builder, slf) -> MultiVector {
        return match AntiConstraintViolation.invoke(&mut builder, slf.clone()).await {
            None => builder.return_expr(slf),
            Some(_) => None,
        }
    });

    // See section 3.4.3 and 3.6.2 of the book
    // TODO we absolutely need advanced inlining and factorization for Fix and AntiFix
    trait_impl_1_type_1_arg!(FixImpl(builder, slf) -> MultiVector {
        let r = Reverse.inline(&mut builder, slf.clone()).await?;
        let p = GeometricProduct.inline(&mut builder, slf.clone(), r).await?;
        let sqrt = SquareRoot.inline(&mut builder, p).await?;
        let i = Inverse.inline(&mut builder, sqrt).await?;
        let result = GeometricProduct.inline(&mut builder, slf, i).await?;
        builder.return_expr(result)
    });
    trait_impl_1_type_1_arg!(AntiFixImpl(builder, slf) -> MultiVector {
        let r = AntiReverse.inline(&mut builder, slf.clone()).await?;
        let p = GeometricAntiProduct.inline(&mut builder, slf.clone(), r).await?;
        let sqrt = AntiSquareRoot.inline(&mut builder, p).await?;
        let i = AntiInverse.inline(&mut builder, sqrt).await?;
        let result = GeometricAntiProduct.inline(&mut builder, slf, i).await?;
        builder.return_expr(result)
    });




    trait_impl_1_type_1_arg!(InverseImpl(builder, slf) -> MultiVector {
        let scalar_mv = MultiVector::from(builder.mvs.scalar());
        let dot = DotProduct.inline(&mut builder, slf.clone(), slf.clone()).await?;
        let raw_dot = FloatExpr::AccessMultiVecFlat(dot.into(), 0);
        let inverse_squared = scalar_mv.construct_direct([(scalar, FloatExpr::product(vec![(raw_dot, -1.0)], 1.0))]);
        let reverse = Reverse.inline(&mut builder, slf).await?;
        let result = GeometricProduct.inline(&mut builder, reverse, inverse_squared).await?;
        builder.return_expr(result)
    });

    trait_impl_1_type_1_arg!(AntiInverseImpl(builder, slf) -> MultiVector {
        let anti_scalar_mv = MultiVector::from(builder.mvs.anti_scalar());
        let anti_scalar = builder.ga.anti_scalar();
        let dot = AntiDotProduct.inline(&mut builder, slf.clone(), slf.clone()).await?;
        let raw_dot = FloatExpr::AccessMultiVecFlat(dot.into(), 0);
        let inverse_squared = anti_scalar_mv.construct_direct([(anti_scalar, FloatExpr::product(vec![(raw_dot, -1.0)], 1.0))]);
        let anti_reverse = AntiReverse.inline(&mut builder, slf).await?;
        let result = GeometricAntiProduct.inline(&mut builder, anti_reverse, inverse_squared).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(GeometricQuotientImpl(builder, slf, other) -> MultiVector {
        let i = Inverse.inline(&mut builder, other).await?;
        let result = GeometricProduct.inline(&mut builder, slf, i).await?;
        builder.return_expr(result)
    });

    trait_impl_2_types_2_args!(GeometricAntiQuotientImpl(builder, slf, other) -> MultiVector {
        let i = AntiInverse.inline(&mut builder, other).await?;
        let result = GeometricAntiProduct.inline(&mut builder, slf, i).await?;
        builder.return_expr(result)
    });







    trait_impl_1_type_1_arg!(SquareRootImpl(builder, slf) -> MultiVector {
        let scalar_mv = MultiVector::from(builder.mvs.scalar());
        if slf.expr_type == scalar_mv {
            let raw = FloatExpr::AccessMultiVecFlat(slf.into(), 0);
            let sqrt = FloatExpr::product(vec![(raw, 0.5)], 1.0);
            let result = scalar_mv.construct_direct([(scalar, sqrt)]);
            return builder.return_expr(result)
        }
        let mv_elements = scalar_mv.elements();
        if !mv_elements.contains(&scalar) {
            return None
        }

        let mut dyn_mv = DynamicMultiVector::zero();
        for (a, a_el) in slf.elements() {
            for (b, b_el) in slf.elements() {
                let sop = builder.ga.product(a_el, b_el);
                for p in sop.sum {
                    let el = p.element;
                    let f = a.clone() * b.clone() * p.coefficient;
                    dyn_mv += (f, el);
                }
            }
        }
        let mv = dyn_mv.construct(&builder)?;
        if mv.mv_class != slf.expr_type {
            return None
        }

        // TODO support SquareRoot with more types of MultiVector

        // TODO I think these ones stand out to me as telling:
        //  - Scalar squares to Scalar
        //  - AntiScalar squares to Scalar
        //  - VersorOdd squares to VersorOdd
        //  - VersorEven squares to VersorOdd
        //  - AntiDualNum squares to AntiDualNum
        //  - DualNum squares to AntiDualNum

        // TODO other things of note...
        //  - CircleRotor squares to VersorOdd
        //  - DipoleInversion squares to VersorOdd

        None
    });

    trait_impl_1_type_1_arg!(AntiSquareRootImpl(builder, slf) -> MultiVector {
        let anti_scalar_mv = MultiVector::from(builder.mvs.anti_scalar());
        let anti_scalar = builder.ga.anti_scalar();
        if slf.expr_type == anti_scalar_mv {
            let raw = FloatExpr::AccessMultiVecFlat(slf.into(), 0);
            let sqrt = FloatExpr::product(vec![(raw, 0.5)], 1.0);
            let result = anti_scalar_mv.construct_direct([(anti_scalar, sqrt)]);
            return builder.return_expr(result)
        }
        let mv_elements = anti_scalar_mv.elements();
        if !mv_elements.contains(&anti_scalar) {
            return None
        }
        // TODO support AntiSquareRoot with more types of MultiVector

        // TODO I think these ones stand out to me as telling:
        //  - Scalar anti-squares to AntiScalar
        //  - AntiScalar anti-squares to AntiScalar
        //  - VersorOdd anti-squares to VersorEven
        //  - VersorEven anti-squares to VersorEven
        //  - AntiDualNum anti-squares to DualNum
        //  - DualNum anti-squares to DualNum

        // TODO so knowing that, we can try to predict....
        //  - CircleRotor anti-squares to CircleRotor? No.... VersorEven.
        //  - DipoleInversion anti-squares to VersorEven

        None
    });

    trait_impl_2_types_2_args!(BulkExpansionImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&mut builder, other).await?;
        let wedge = Wedge.inline(&mut builder, slf, dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(WeightExpansionImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&mut builder, other).await?;
        let wedge = Wedge.inline(&mut builder, slf, anti_dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(BulkContractionImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&mut builder, other).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, slf, dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(WeightContractionImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&mut builder, other).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, slf, anti_dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(ProjectOrthogonallyOntoImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&mut builder, other.clone()).await?;
        let wedge = Wedge.inline(&mut builder, slf, anti_dual).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, other, wedge).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(AntiProjectOrthogonallyOntoImpl(builder, slf, other) -> MultiVector {
        let anti_dual = RightAntiDual.inline(&mut builder, other.clone()).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, slf, anti_dual).await?;
        let wedge = Wedge.inline(&mut builder, other, anti_wedge).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(ProjectViaOriginOntoImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&mut builder, other.clone()).await?;
        let wedge = Wedge.inline(&mut builder, slf, dual).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, other, wedge).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(AntiProjectViaHorizonOntoImpl(builder, slf, other) -> MultiVector {
        let dual = RightDual.inline(&mut builder, other.clone()).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, slf, dual).await?;
        let wedge = Wedge.inline(&mut builder, other, anti_wedge).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(RejectOrthogonallyFromImpl(builder, slf, other) -> MultiVector {
        let anti_wedge = AntiWedge.inline(&mut builder, slf, other.clone()).await?;
        let anti_dual = RightAntiDual.inline(&mut builder, other).await?;
        let wedge = Wedge.inline(&mut builder, anti_wedge, anti_dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(AntiRejectOrthogonallyFromImpl(builder, slf, other) -> MultiVector {
        let wedge = Wedge.inline(&mut builder, slf, other.clone()).await?;
        let anti_dual = RightAntiDual.inline(&mut builder, other).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, wedge, anti_dual).await?;
        builder.return_expr(anti_wedge)
    });

    trait_impl_2_types_2_args!(RejectViaOriginFromImpl(builder, slf, other) -> MultiVector {
        let anti_wedge = AntiWedge.inline(&mut builder, slf, other.clone()).await?;
        let dual = RightDual.inline(&mut builder, other).await?;
        let wedge = Wedge.inline(&mut builder, anti_wedge, dual).await?;
        builder.return_expr(wedge)
    });

    trait_impl_2_types_2_args!(AntiRejectViaHorizonFromImpl(builder, slf, other) -> MultiVector {
        let wedge = Wedge.inline(&mut builder, slf, other.clone()).await?;
        let dual = RightDual.inline(&mut builder, other).await?;
        let anti_wedge = AntiWedge.inline(&mut builder, wedge, dual).await?;
        builder.return_expr(anti_wedge)
    });


    #[derive(Clone, Copy)]
    pub struct SupportImpl { pub origin: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for SupportImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_origin = DynamicMultiVector::zero();
            dyn_origin += (FloatExpr::Literal(1.0), self.origin);
            let origin = dyn_origin.construct(&builder)?;
            let anti_dual = RightAntiDual.inline(&mut builder, slf).await?;
            let wedge = Wedge.inline(&mut builder, origin, anti_dual).await?;
            builder.return_expr(wedge)
        }
    }

    #[derive(Clone, Copy)]
    pub struct AntiSupportImpl { pub origin: BasisElement }
    #[async_trait]
    impl TraitImpl_11 for AntiSupportImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_origin = DynamicMultiVector::zero();
            dyn_origin += (FloatExpr::Literal(1.0), self.origin);
            let origin = dyn_origin.construct(&builder)?;
            let rc_origin = RightComplement.inline(&mut builder, origin).await?;
            let dual = RightDual.inline(&mut builder, slf).await?;
            let anti_wedge = AntiWedge.inline(&mut builder, rc_origin, dual).await?;
            builder.return_expr(anti_wedge)
        }
    }


    #[derive(Clone, Copy)]
    pub struct UnitizeImpl<WeightNorm> { pub weight_norm: WeightNorm }
    #[async_trait]
    impl<WeightNorm> TraitImpl_11 for UnitizeImpl<WeightNorm>
        where WeightNorm: TraitDef_1_Type_1_Arg<Output=MultiVector> {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            mut builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let weight_norm = self.weight_norm.inline(&mut builder, slf.clone()).await?;
            let inverse = AntiInverse.inline(&mut builder, weight_norm).await?;
            let product = GeometricAntiProduct.inline(&mut builder, slf, inverse).await?;
            builder.return_expr(product)
        }
    }



    // Into is treated kind of special, because we actually want to implement From,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat Into as From instead.
    trait_impl_2_types_1_arg!(IntoImpl(builder, slf, other) -> MultiVector {
        if slf.expression_type() == other {
            return None;
        }
        let other_elements: BTreeSet<_> = other.elements().into_iter().collect();
        let mut these_elements: BTreeMap<_, _> = BTreeMap::new();
        for (f, el) in slf.elements() {
            if !other_elements.contains(&el) {
                return None;
            }
            these_elements.insert(el, f);
        }
        let result = other.construct(|el| these_elements.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        builder.return_expr(result)
    });

    // TryInto is treated kind of special, because we actually want to implement TryFrom,
    // but the TraitImpl_21 pattern assumes the first argument is the owner.
    // So in the code generation we have a special exception to treat TrInto as TryFrom instead.
    trait_impl_2_types_1_arg!(TryIntoImpl(builder, slf, other) -> MultiVector {
        let mut missing_some = false;
        let mut overlapping_some = false;
        let other_elements: BTreeSet<_> = other.elements().into_iter().collect();
        let mut these_elements: BTreeMap<_, _> = BTreeMap::new();
        for (f, el) in slf.elements() {
            if other_elements.contains(&el) {
                overlapping_some = true;
            } else {
                missing_some = true;
            }
            these_elements.insert(el, f);
        }
        if !missing_some || !overlapping_some {
            return None;
        }
        let result = other.construct(|el| these_elements.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        builder.return_expr(result)
    });




    #[derive(Clone, Copy)]
    pub struct SubTypeImpl<F1, F2, F3> {
        pub filter_multivecs_in: F1,
        pub filter_elements: F2,
        pub filter_multivecs_out: F3,
    }
    #[async_trait]
    impl<F1, F2, F3> TraitImpl_11 for SubTypeImpl<F1, F2, F3> where
        F1: SigSetFilter + Copy + Send + Sync + 'static,
        F2: SigFilter + Copy + Send + Sync + 'static,
        F3: SigSetFilter + Copy + Send + Sync + 'static {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let sig_in = slf.expr_type.signatures();
            if !self.filter_multivecs_in.filter_sig_set(&sig_in) {
                return None;
            }
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                if self.filter_elements.filter_sig(a_el.signature()) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            let sig_out = mv.mv_class.signatures();
            if !self.filter_multivecs_out.filter_sig_set(&sig_out) {
                return None;
            }
            builder.return_expr(mv)
        }
    }






    #[derive(Clone, Copy)]
    pub struct SelectGradesImpl(pub Grades);
    #[async_trait]
    impl TraitImpl_11 for SelectGradesImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                if self.0.contains(a_el.grades()) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }

    #[derive(Clone, Copy)]
    pub struct SelectAntiGradesImpl(pub AntiGrades);
    #[async_trait]
    impl TraitImpl_11 for SelectAntiGradesImpl {
        type Output = MultiVector;
        async fn general_implementation<const AntiScalar: BasisElement>(
            self,
            builder: TraitImplBuilder<AntiScalar, HasNotReturned>,
            slf: Variable<MultiVector>,
        ) -> Option<TraitImplBuilder<AntiScalar, Self::Output>> {
            let mut dyn_mv = DynamicMultiVector::zero();
            for (a, a_el) in slf.elements() {
                if self.0.contains(a_el.anti_grades(AntiScalar)) {
                    dyn_mv += (a, a_el);
                }
            }
            let mv = dyn_mv.construct(&builder)?;
            builder.return_expr(mv)
        }
    }
}
