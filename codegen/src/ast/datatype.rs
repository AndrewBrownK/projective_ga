use crate::algebra::basis::grades::{AntiGrades, Grades};
use crate::algebra::basis::{BasisElement, BasisSignature};
use crate::algebra::multivector::{BasisElementGroup, MultiVec};
use crate::ast::expressions::{DebugExpression, FloatExpr, MultiVectorExpr, MultiVectorGroupExpr, MultiVectorVia, Vec2Expr, Vec3Expr, Vec4Expr};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Integer;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Float;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Vec2;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Vec3;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Vec4;

const DUMMY_ANTI_SCALAR: BasisElement = BasisElement::const_from(BasisSignature::from_bits_retain(u16::MAX)).with_name("DUMMY_ANTI_SCALAR", false);

// Should we infect a generic AntiScalar parameter across the AST?
//
// So... the way this works is.... we have TraitImpl_10/11/21/22. Right now these accept a
// generic AntiScalar at the method level, instead of the trait level. The reason that this is
// nice is that we can find the TraitImpl_10/11/21/22 implementation without having to specify
// an AntiScalar, thus allowing the compiler to find and use the NameTrait, and declare the
// static Elaborated<Traits> without depending on an AntiScalar.
//
// To be clear, it wouldn't be so bad to move the AntiScalar to the trait-level instead of
// method-level on TraitImpl_10/11/21/22, but ONLY IF the detection/use of NameTrait weren't
// impacted, and no AntiScalar had to be specified on the static Elaborated<Traits>.
//
// Then again... there's something to be said for the "purity" of a trait definition that works
// for all possible AntiScalars, instead of allowing you to define it such that it only works
// with specific AntiScalars. This is especially true since we allow such huge flexibility of
// AntiScalar specification, it would be stupid for something to work on e1234 but not e0123.
//
// So with that in mind, I think the preferred approach (with respect to TraitImpl_10/11/21/22) is
// to leave the AntiScalar at the method-level and not the trait-level. This brings us to our next
// problem then. When implementing TraitImpl_10/11/21/22, you specify the associated type "Output".
// Obviously, "MultiVector" (our struct that we are commenting on right here) is a common output
// type. And MultiVector holds a reference to a MultiVec, but MultiVecs cannot be specified without
// and AntiScalar (generic or otherwise). So if we infect MultiVector with an AntiScalar too, then
// we have no way to specify the associated output type of TraitImpl_10/11/21/22 without also
// infecting those traits. In other words, we have to erase the AntiScalar right here.
//
// Solution: unsafe pointer casting to and from MultiVec<DUMMY_ANTI_SCALAR>
#[derive(Clone, Copy, Debug, Hash)]
pub struct MultiVector {
    anti_scalar: BasisElement,
    multi_vec: &'static MultiVec<DUMMY_ANTI_SCALAR>,
}
impl PartialOrd for MultiVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for MultiVector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.anti_scalar.cmp(&other.anti_scalar).then_with(|| {
            self.multi_vec.name.cmp(&other.multi_vec.name).then_with(|| {
                self.multi_vec
                    .grades
                    .cmp(&other.multi_vec.grades)
                    .then_with(|| self.multi_vec.elements().cmp(&other.multi_vec.elements()))
            })
        })
    }
}

impl PartialEq for MultiVector {
    fn eq(&self, other: &Self) -> bool {
        self.anti_scalar == other.anti_scalar && std::ptr::eq(self.multi_vec, other.multi_vec)
    }
}
impl Eq for MultiVector {}
impl Display for MultiVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let anti_scalar = self.anti_scalar;
        write!(f, "<{anti_scalar}> ")?;
        let s = self.multi_vec.macro_expression();
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ExpressionType {
    Int(Integer),
    Float(Float),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Class(MultiVector),
}
impl Debug for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionType::Int(_) => write!(f, "Int"),
            ExpressionType::Float(_) => write!(f, "Float"),
            ExpressionType::Vec2(_) => write!(f, "Vec2"),
            ExpressionType::Vec3(_) => write!(f, "Vec3"),
            ExpressionType::Vec4(_) => write!(f, "Vec4"),
            ExpressionType::Class(c) => {
                let n = c.multi_vec.name;
                write!(f, "{n}")
            }
        }
    }
}


// SAFETY:
// MultiVector should not use methods on the multi_vec field that depend on
// a correct AntiScalar type. Currently, this is only enforced by manual
// inspection and diligence. If you want a method that changes behavior depending
// on an accurate AntiScalar, then use the anti_scalar field instead, or accept an AntiScalar
// type parameter for the function and use the implementation of From or Into to convert
// multi_vec to the correct type first. If rust had dependent types, then we could trivially do
// this using the anti_scalar field. But if rust had dependent types, we probably wouldn't be
// using DUMMY_ANTI_SCALAR to begin with.
impl MultiVector {
    pub fn construct<F: FnMut(BasisElement) -> FloatExpr>(&self, mut f: F) -> MultiVectorExpr {
        let mut outer = vec![];
        let groups = self.multi_vec.groups();
        let mut i = 0;
        while i < groups.len() {
            let thing = groups.get(i).clone().into_vec();
            outer.push(match thing.len() {
                1 => MultiVectorGroupExpr::JustFloat(f(thing[0])),
                2 => MultiVectorGroupExpr::Vec2(Vec2Expr::Gather2(f(thing[0]), f(thing[1]))),
                3 => MultiVectorGroupExpr::Vec3(Vec3Expr::Gather3(f(thing[0]), f(thing[1]), f(thing[2]))),
                4 => MultiVectorGroupExpr::Vec4(Vec4Expr::Gather4(f(thing[0]), f(thing[1]), f(thing[2]), f(thing[3]))),
                _ => unreachable!("BasisElementGroup has len 1-4"),
            });
            i += 1;
        }
        let mut result = MultiVectorExpr {
            mv_class: self.clone(),
            expr: Box::new(MultiVectorVia::Construct(outer)),
        };
        tracing::trace!("Raw MultiVector::Construct result:\n{:?}", DebugExpression::new(true, &result));
        result.simplify();
        tracing::trace!("Simplified MultiVector::Construct result:\n{:?}", DebugExpression::new(true, &result));
        result
    }

    pub fn construct_direct<const N: usize>(&self, arr: [(BasisElement, FloatExpr); N]) -> MultiVectorExpr {
        let mut vals: BTreeMap<BasisElement, FloatExpr> = arr.into_iter().collect();
        let result = self.construct(|el| vals.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        if !vals.is_empty() {
            let k: Vec<_> = vals.keys().collect();
            panic!("Attempted direct construction of {self} using mismatched elements {k:?}");
        }
        result
    }

    pub fn grade(&self) -> Option<u32> {
        let mut grade = None;
        for el in self.multi_vec.elements() {
            let elg = el.grade();
            match grade {
                None => grade = Some(elg),
                Some(gr) => {
                    if elg != gr {
                        return None;
                    }
                }
            }
        }
        grade
    }

    pub fn anti_grade(&self) -> Option<u32> {
        let gr = self.grade()?;
        Some(self.anti_scalar.grade() - gr)
    }

    pub fn grades(&self) -> Grades {
        let mut grades = Grades::none;
        for el in self.multi_vec.elements() {
            grades |= el.grades();
        }
        grades
    }
    pub fn anti_grades(&self) -> AntiGrades {
        let mut anti_grades = AntiGrades::none;
        for el in self.multi_vec.elements() {
            anti_grades |= el.anti_grades(self.anti_scalar);
        }
        anti_grades
    }

    pub fn elements(&self) -> Vec<BasisElement> {
        self.multi_vec.elements()
    }

    pub fn signatures(&self) -> BTreeSet<BasisSignature> {
        let mut set = BTreeSet::new();
        for el in self.multi_vec.elements() {
            set.insert(el.signature());
        }
        set
    }

    pub fn groups(&self) -> Vec<BasisElementGroup> {
        let mut v = vec![];
        let groups = self.multi_vec.groups();
        let mut i = 0;
        while i < groups.len() {
            let g = groups[i];
            v.push(g);
            i += 1;
        }
        v
    }

    pub fn name(&self) -> &'static str {
        self.multi_vec.name
    }
}

#[allow(non_upper_case_globals)]
impl<const AntiScalar: BasisElement> From<&'static MultiVec<AntiScalar>> for MultiVector {
    fn from(value: &'static MultiVec<AntiScalar>) -> Self {
        // SAFETY:
        // We erase the real AntiScalar in order to prevent generic type parameter infection.
        // We record the correct AntiScalar in order to safely (use unsafe to) restore the correct
        // type in Option<&'static MultiVec<AntiScalar>>::from(MultiVector).
        let multi_vec = unsafe {
            let correct_pointer: *const MultiVec<AntiScalar> = value as *const MultiVec<AntiScalar>;
            let obscure_pointer: *const MultiVec<DUMMY_ANTI_SCALAR> = correct_pointer.cast::<MultiVec<DUMMY_ANTI_SCALAR>>();
            &*(obscure_pointer)
        };
        Self {
            anti_scalar: AntiScalar,
            multi_vec,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<const AntiScalar: BasisElement> From<MultiVector> for Option<&'static MultiVec<AntiScalar>> {
    fn from(value: MultiVector) -> Self {
        if value.anti_scalar != AntiScalar {
            return None;
        }
        // SAFETY:
        // The original AntiScalar that this MultiVector was created with was stored in
        // anti_scalar. So by checking that the type parameter of this method matches that
        // recorded type, we can know we are casting to the correct type.
        let mv = unsafe {
            let obscure_pointer: *const MultiVec<DUMMY_ANTI_SCALAR> = value.multi_vec as *const MultiVec<DUMMY_ANTI_SCALAR>;
            let correct_pointer: *const MultiVec<AntiScalar> = obscure_pointer.cast::<MultiVec<AntiScalar>>();
            &*(correct_pointer)
        };
        Some(mv)
    }
}

pub trait ClassesFromRegistry {
    fn include_class(&self, mvc: &MultiVector) -> bool;
}
pub struct NoParam;
impl ClassesFromRegistry for NoParam {
    fn include_class(&self, _: &MultiVector) -> bool {
        false
    }
}
pub struct AnyClasses;
impl ClassesFromRegistry for AnyClasses {
    fn include_class(&self, _: &MultiVector) -> bool {
        true
    }
}

/// Good for manual implementations
pub struct Specifically(pub MultiVector);

impl ClassesFromRegistry for Specifically {
    fn include_class(&self, mvc: &MultiVector) -> bool {
        mvc == &self.0.clone()
    }
}
