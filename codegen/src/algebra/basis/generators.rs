#![allow(non_upper_case_globals)]

use crate::algebra::basis::arithmetic::{GradedProduct, GradedSum, Product};
use crate::algebra::basis::grades::{grade1};
use crate::algebra::basis::substitutes::Substitutions;
use crate::algebra::basis::{BasisElement, BasisSignature};
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};
use crate::utility::const_option::ConstOption;

/// The foundational GeneratorSquares assumes a diagonal metric
/// (with no generator substitutions).
// I don't think we want to <const AntiScalar: BasisElement> here.
// It is fundamentally more annoying to get GeneratorElements to track this type level
// information. I would need a separate struct for each generator, and then add arithmetic
// operations for every combination of these generators. I think I'm better off giving
// GeometricAlgebra a PhantomData and validating the GeneratorSquares and SubstitutionRepository
// against the AntiScalar at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratorSquares {
    pub(crate) negative_anti_scalar: bool,
    active_bases: BasisSignature,
    pub(crate) raw_squares: [i8; 16],
}

#[macro_export]
macro_rules! generator_squares {
    ($( $i8_lit:expr => $( $generator:expr ),+ $(,)? );+ $(;)? ) => {
        {
            use $crate::algebra::basis::generators::*;
            let mut gs = $crate::algebra::basis::generators::GeneratorSquares::empty();
            $($(gs = gs.overwrite([($generator, $i8_lit)]);)+)+
            gs
        }
    };
}

impl GeneratorSquares {
    pub const fn anti_scalar(&self) -> BasisElement {
        let signature = self.active_bases;
        BasisElement {
            coefficient: if self.negative_anti_scalar { -1 } else { 1 },
            signature,
            display_name: ConstOption::None,
        }
    }

    pub fn next_available_basis(&self) -> anyhow::Result<GeneratorElement> {
        let mut emptying_signature = self.active_bases.clone();

        // The way this works, if the active_bases is not empty and starts at e1 (or higher) instead
        // of e0, then it will skip over e0 (etc.) unless it runs out of bases all the way to eF,
        // and then it will loop around and try the lower bases again.
        for basis in GeneratorElement::array().into_iter().chain(GeneratorElement::array()) {
            if emptying_signature.is_empty() {
                return Ok(basis);
            }
            emptying_signature.remove(basis.signature());
        }
        Err(anyhow::format_err!("There are no more available PrimaryBasis for {self:?}."))
    }

    pub fn empty() -> Self {
        Self {
            negative_anti_scalar: false,
            active_bases: BasisSignature::empty(),
            raw_squares: [0i8; 16],
        }
    }

    pub fn new<const N: usize>(generator_squares: [(GeneratorElement, i8); N]) -> Self {
        let mut active_bases = BasisSignature::empty();
        let mut raw_squares = [0i8; 16];
        for (basis, square) in generator_squares {
            if square > 1 || square < -1 {
                panic!("Generator square of {square} is not allowed. Please choose 1, -1, or 0.")
            }
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self {
            negative_anti_scalar: false,
            active_bases,
            raw_squares,
        }
    }

    pub fn append<const N: usize>(self, generator_squares: [(GeneratorElement, i8); N]) -> anyhow::Result<Self> {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            if square > 1 || square < -1 {
                panic!("Generator square of {square} is not allowed. Please choose 1, -1, or 0.")
            }
            let sig = basis.signature();
            if active_bases.contains(sig) {
                return Err(anyhow::format_err!("The PrimaryBasis {basis:?} is already taken on {self:?}"));
            }
            active_bases = active_bases.union(sig);
            raw_squares[(basis as u8) as usize] = square;
        }
        Ok(Self {
            negative_anti_scalar: false,
            active_bases,
            raw_squares,
        })
    }

    pub fn overwrite<const N: usize>(self, generator_squares: [(GeneratorElement, i8); N]) -> Self {
        let mut active_bases = self.active_bases;
        let mut raw_squares = self.raw_squares;
        for (basis, square) in generator_squares {
            if square > 1 || square < -1 {
                panic!("Generator square of {square} is not allowed. Please choose 1, -1, or 0.")
            }
            active_bases = active_bases.union(basis.signature());
            raw_squares[(basis as u8) as usize] = square;
        }
        Self {
            negative_anti_scalar: false,
            active_bases,
            raw_squares,
        }
    }

    pub const fn square_generator(&self, basis: GeneratorElement) -> i8 {
        self.raw_squares[(basis as u8) as usize]
    }

    // OKAY SO while implementing scalar/dot product I can definitely see where the confusion
    // arises when different authors argue the dot product should be always grade zero, or always
    // grade difference.
    // So suppose in vanilla GA we have e2 and e123 and we want to rediscover and/or generalize what
    // the dot product could mean between them.
    //
    // The "Grade 0 Approach" seems to argue (as far as I can discern/infer) that the generalization
    // should be "In dot products between grade 1 vectors, each basis ONLY pairs up with itself (for
    // a non-zero result), and if you try to dot product  mismatched (orthogonal) basis elements,
    // then the result is zero. Therefore it is reasonable to say that since e2 and e123 are not
    // identical basis elements, their dot product should be zero. Therefore ultimately the dot
    // product is the scalar product."
    //
    // The "Grade Difference Approach" seems (as far as I can discern/infer) to attempt the
    // generalization differently. The treatment seems to be:
    // e2 ● e123 = ?
    // e2 ● (e1 ∧ e2 ∧ e3) = ?
    // e2 ● -(e2 ∧ e1 ∧ e3) = ?
    // -(e2 ● (e2 ∧ e1 ∧ e3)) = ?
    // -((e2 ● e2) ∧ e1 ∧ e3) = ?     <- concern #1
    // -(1 ∧ e1 ∧ e3) = ?             <- concern #2
    // -e13 = ?
    // e2 ● e123 = -e13
    //
    // And then from there (again, attempting to discover/generalize a dot product from scratch here),
    // it is noticed this result is inside the geometric product, and then the dot product is appointed
    // to be the "grade difference" selection of the geometric product.
    //
    // Concern #1: Is the dot product really supposed to be associative with wedge product?
    //
    // Concern #2: Why do we allow ourselves to collapse (e2 ● e2) = 1 without first encountering
    // interference with (e2 ● e1) = 0? Since we're trying to build upon/generalize the grade 1 dot
    // product to begin with.
    //
    // Maybe the motivation is... "The dot product doesn't mean to only match identical elements,
    // it means to only exclude orthogonal elements"?

    pub fn inner_product(&self, a: BasisElement, b: BasisElement) -> Product {
        if a.coefficient == 0 || b.coefficient == 0 {
            return Product::zero();
        }
        if a.signature != b.signature {
            return Product::zero();
        }
        // Take care of the outer sign
        let mut result = a.coefficient * b.coefficient;
        // Then simply square the generators
        for g in a.signature.into_generator_elements_const().1.into_iter().filter_map(|it| it) {
            result *= self.square_generator(g);
        }
        // And construct a Product for the result
        let coefficient = result as f32;
        use crate::algebra::basis::elements::*;
        Product { coefficient, element: scalar }
    }

    pub fn inner_anti_product(&self, a: BasisElement, b: BasisElement) -> Product {
        let anti_scalar = self.anti_scalar();
        let a = a.left_complement(anti_scalar);
        let b = b.left_complement(anti_scalar);
        let mut p = self.inner_product(a, b);
        p.element = p.element.right_complement(anti_scalar);
        p
    }

    pub fn apply_metric(&self, a: BasisElement) -> BasisElement {
        let a = a.anon();
        // There is no need for any kind of elements reversal, or counting of swapped elements here.
        // The (diagonal) metric is just a function that turns one BasisElement into another,
        // potentially with a sign change. So for example in Rigid GA you have:
        // G e12 = e12
        // G e123 = e123
        // Even though they are even and odd grades respectively and so would permute differently.

        // Again we EMPHASIZE that GeneratorSquares ASSUMES A DIAGONAL METRIC! Non-diagonal metrics
        // should use SubstitutionRepository to define substitute elements over an underlying
        // GeneratorSquares where the underlying metric is diagonal even if the substitute metric
        // is not.

        // Anyway, all that to say, this implementation here is pretty simple.
        // Just scan through the generators and accumulate their squares on the result.

        // (We copy `a` to preserve any customized display names)
        let mut result = a;
        let mut sign = a.coefficient;
        if sign == 0 {
            return BasisElement::zero();
        }
        let (gs_len, gs) = a.signature.into_generator_elements_const();
        let mut g_idx = 0;
        while g_idx < gs_len {
            let Some(g) = gs[g_idx] else {
                g_idx += 1;
                continue;
            };
            sign *= self.square_generator(g);
            g_idx += 1;
        }
        if sign == 0 {
            return BasisElement::zero();
        }
        result.coefficient = sign;
        result
    }

    pub fn apply_anti_metric(&self, a: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        let a = a.right_complement(anti_scalar);
        let a = self.apply_metric(a);
        a.left_complement(anti_scalar)
    }

    pub fn right_dual(&self, a: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        self.apply_metric(a).right_complement(anti_scalar)
    }

    pub fn left_dual(&self, a: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        self.apply_metric(a).left_complement(anti_scalar)
    }

    pub fn right_anti_dual(&self, a: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        self.apply_anti_metric(a).right_complement(anti_scalar)
    }

    pub fn left_anti_dual(&self, a: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        self.apply_anti_metric(a).left_complement(anti_scalar)
    }

    /// True Geometric Product, in other words without using substituted bases.
    /// If your BasisElements are substituting bases, you'll need to convert to
    /// the underlying bases before you can properly use this function.
    pub const fn geometric_product(&self, a: BasisElement, b: BasisElement) -> BasisElement {
        // Implementation may look a bit strange because it is const compatible
        let s = a;
        let o = b;

        let (a_len, a) = s.signature.into_grade_1_signatures_const();
        let (b_len, b) = o.signature.into_grade_1_signatures_const();
        let mut sign = s.coefficient * o.coefficient;

        let mut result_elements: [Option<BasisSignature>; 16] = [None; 16];
        let mut a_idx = 0;
        let mut b_idx = 0;
        let mut r_idx = 0;
        while a_idx < a_len || b_idx < b_len {
            if a_idx >= a_len {
                result_elements[r_idx] = b[b_idx];
                r_idx += 1;
                b_idx += 1;
                continue;
            }
            if b_idx >= b_len {
                result_elements[r_idx] = a[a_idx];
                r_idx += 1;
                a_idx += 1;
                continue;
            }
            let a_ = a[a_idx].unwrap();
            let b_ = b[b_idx].unwrap();
            match BasisSignature::const_cmp(&a_, &b_) {
                Ordering::Less => {
                    result_elements[r_idx] = Some(a_);
                    r_idx += 1;
                    a_idx += 1;
                }
                Ordering::Equal => {
                    let mut g_idx = 0;
                    let (gs_len, gs) = a_.into_generator_elements_const();
                    while g_idx < gs_len {
                        let Some(g) = gs[g_idx] else {
                            g_idx += 1;
                            continue;
                        };
                        sign *= self.square_generator(g);
                        g_idx += 1;
                    }
                    if sign == 0 {
                        return BasisElement::zero();
                    }
                    // Must move b_ at least to the right of a_.
                    // Which negates the sign each step.
                    let swaps = ((a_len - a_idx) - 1) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    a_idx += 1;
                    b_idx += 1;
                }
                Ordering::Greater => {
                    // Must move b_ all the way to left of a_.
                    // Which negates the sign each step.
                    result_elements[r_idx] = Some(b_);
                    r_idx += 1;
                    let swaps = (a_len - a_idx) as u32;
                    sign *= i8::pow(-1, swaps % 2);
                    b_idx += 1;
                }
            }
        }
        let mut result_sig = 0u16;
        let mut i = 0;
        while i < r_idx {
            let primary_basis = result_elements[i].unwrap();
            i += 1;
            let additional_sig = primary_basis.bits();
            result_sig = result_sig | additional_sig;
        }
        if sign == 0 {
            result_sig = 0u16;
        }
        let signature = BasisSignature::from_bits_retain(result_sig);
        BasisElement {
            coefficient: sign,
            signature,
            display_name: ConstOption::None,
        }
    }

    /// True Geometric AntiProduct, in other words without using substituted bases.
    /// If your BasisElements are substituting bases, you'll need to convert to
    /// the underlying bases before you can properly use this function.
    pub fn geometric_anti_product(&self, a: BasisElement, b: BasisElement) -> BasisElement {
        let anti_scalar = self.anti_scalar();
        let a = a.right_complement(anti_scalar);
        let b = b.right_complement(anti_scalar);
        let c = self.geometric_product(a, b);
        c.left_complement(anti_scalar)
    }
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum GeneratorElement {
    e0 = 0,
    e1 = 1,
    e2 = 2,
    e3 = 3,
    e4 = 4,
    e5 = 5,
    e6 = 6,
    e7 = 7,
    e8 = 8,
    e9 = 9,
    eA = 10,
    eB = 11,
    eC = 12,
    eD = 13,
    eE = 14,
    eF = 15,
}

// List some primary basis elements
pub const e0: GeneratorElement = GeneratorElement::e0;
pub const e1: GeneratorElement = GeneratorElement::e1;
pub const e2: GeneratorElement = GeneratorElement::e2;
pub const e3: GeneratorElement = GeneratorElement::e3;
pub const e4: GeneratorElement = GeneratorElement::e4;
pub const e5: GeneratorElement = GeneratorElement::e5;
pub const e6: GeneratorElement = GeneratorElement::e6;
pub const e7: GeneratorElement = GeneratorElement::e7;
pub const e8: GeneratorElement = GeneratorElement::e8;
pub const e9: GeneratorElement = GeneratorElement::e9;
pub const eA: GeneratorElement = GeneratorElement::eA;
pub const eB: GeneratorElement = GeneratorElement::eB;
pub const eC: GeneratorElement = GeneratorElement::eC;
pub const eD: GeneratorElement = GeneratorElement::eD;
pub const eE: GeneratorElement = GeneratorElement::eE;
pub const eF: GeneratorElement = GeneratorElement::eF;

// There is room for people to be weird and make these
// generators square to unconventional values, but that
// risk of breaking semantics is worth providing the
// convenience of these already being defined.
pub const e_inf: GeneratorElement = GeneratorElement::eD;
pub const e_plus: GeneratorElement = GeneratorElement::eE;
pub const e_minus: GeneratorElement = GeneratorElement::eF;
pub const eI: GeneratorElement = GeneratorElement::eD;
pub const eP: GeneratorElement = GeneratorElement::eE;
pub const eM: GeneratorElement = GeneratorElement::eF;

impl GeneratorElement {
    pub const fn array() -> [Self; 16] {
        [
            GeneratorElement::e0,
            GeneratorElement::e1,
            GeneratorElement::e2,
            GeneratorElement::e3,
            GeneratorElement::e4,
            GeneratorElement::e5,
            GeneratorElement::e6,
            GeneratorElement::e7,
            GeneratorElement::e8,
            GeneratorElement::e9,
            GeneratorElement::eA,
            GeneratorElement::eB,
            GeneratorElement::eC,
            GeneratorElement::eD,
            GeneratorElement::eE,
            GeneratorElement::eF,
        ]
    }

    const fn bits(self) -> u16 {
        1u16 << self as u8
    }

    pub const fn signature(self) -> BasisSignature {
        BasisSignature::from_bits_retain(self.bits())
    }

    pub const fn element(self) -> BasisElement {
        BasisElement {
            coefficient: 1,
            signature: self.signature(),
            display_name: ConstOption::None,
        }
    }

    // const fn const_cmp(&self, other: &GeneratorElement) -> Ordering {
    //     let a = *self as u8;
    //     let b = *other as u8;
    //     if a < b {
    //         return Ordering::Less;
    //     }
    //     if a > b {
    //         return Ordering::Greater;
    //     }
    //     Ordering::Equal
    // }
}

impl Mul<f32> for GeneratorElement {
    type Output = GradedProduct<{ grade1 }>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut gp = self.into();
        gp *= rhs;
        gp
    }
}
impl Mul<GeneratorElement> for f32 {
    type Output = GradedProduct<{ grade1 }>;

    fn mul(self, rhs: GeneratorElement) -> Self::Output {
        let mut gp = rhs.into();
        gp *= self;
        gp
    }
}
impl Add<GeneratorElement> for GeneratorElement {
    type Output = GradedSum<{ grade1 }>;

    fn add(self, rhs: GeneratorElement) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = self * 1.0f32;
        let b: GradedProduct<{ grade1 }> = rhs * 1.0f32;
        let c: GradedSum<{ grade1 }> = a + b;
        c
    }
}
impl Add<GradedProduct<{ grade1 }>> for GeneratorElement {
    type Output = GradedSum<{ grade1 }>;

    fn add(self, rhs: GradedProduct<{ grade1 }>) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = self * 1.0f32;
        let c: GradedSum<{ grade1 }> = a + rhs;
        c
    }
}
impl Add<GeneratorElement> for GradedProduct<{ grade1 }> {
    type Output = GradedSum<{ grade1 }>;

    fn add(self, rhs: GeneratorElement) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = rhs * 1.0f32;
        let c: GradedSum<{ grade1 }> = self + a;
        c
    }
}
impl Sub<GeneratorElement> for GeneratorElement {
    type Output = GradedSum<{ grade1 }>;

    fn sub(self, rhs: GeneratorElement) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = self * 1.0f32;
        let b: GradedProduct<{ grade1 }> = rhs * -1.0f32;
        let c: GradedSum<{ grade1 }> = a + b;
        c
    }
}
impl Sub<GradedProduct<{ grade1 }>> for GeneratorElement {
    type Output = GradedSum<{ grade1 }>;

    fn sub(self, rhs: GradedProduct<{ grade1 }>) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = self * 1.0f32;
        let c: GradedSum<{ grade1 }> = a - rhs;
        c
    }
}
impl Sub<GeneratorElement> for GradedProduct<{ grade1 }> {
    type Output = GradedSum<{ grade1 }>;

    fn sub(self, rhs: GeneratorElement) -> Self::Output {
        let a: GradedProduct<{ grade1 }> = 1.0f32 * rhs;
        let c: GradedSum<{ grade1 }> = self - a;
        c
    }
}

impl Add<Vec<(GeneratorElement, GradedSum<{ grade1 }>)>> for GeneratorSquares {
    type Output = Substitutions;

    fn add(self, rhs: Vec<(GeneratorElement, GradedSum<{ grade1 }>)>) -> Self::Output {
        Substitutions::new(self, rhs)
    }
}

impl Add<GeneratorSquares> for Vec<(GeneratorElement, GradedSum<{ grade1 }>)> {
    type Output = Substitutions;

    fn add(self, rhs: GeneratorSquares) -> Self::Output {
        Substitutions::new(rhs, self)
    }
}
