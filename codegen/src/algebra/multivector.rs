#![allow(non_upper_case_globals)]

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::ops::AddAssign;
use std::sync::Arc;

use const_panic::concat_panic;
use parking_lot::Mutex;

use crate::algebra::basis::{BasisElement, BasisSignature};
use crate::algebra::basis::filter::{SigFilter, SigSetFilter};
use crate::algebra::basis::grades::Grades;
use crate::algebra::GeometricAlgebra;
use crate::ast::datatype::{ExpressionType, Float, MultiVector, Vec2, Vec3, Vec4};
use crate::ast::expressions::{FloatExpr, MultiVectorExpr};
use crate::ast::traits::{HasNotReturned, RawTraitDefinition, RawTraitImplementation, TraitImplBuilder};
use crate::utility::ConstVec;

// We COULD use { qty_groups(AntiScalar) } everywhere to specify the size of
// ConstVec<BasisElement, N>. And this would make the arrays only as small as necessary. However,
// then we have to infect everything with the constraint:
// where [(); { qty_groups(AntiScalar) }]: Sized
// I wouldn't mind handling that constraint to some extents.... except the infection goes too far.
// It's one thing to add the constraint to MultiVec methods, it is another thing to infect the
// TraitImplBuilder API. So instead we are going to hard code the qty so that we don't need
// a size constraint.
// Assuming in all cases the average elements per group is at least 3 (and for margin error +1).
// 16 dimensions: 2^16 = 65536 -> (n / 3) + 1 = 21846
// 12 dimensions: 2^12 = 4096 -> (n / 3) + 1 = 1366
//  8 dimensions: 2^8  = 256 -> (n / 3) + 1 = 86

#[cfg(feature = "very-large-basis-elements")]
pub const QTY_GROUPS: usize = 21846;
#[cfg(all(feature = "large-basis-elements", not(feature = "very-large-basis-elements")))]
pub const QTY_GROUPS: usize = 1366;
#[cfg(not(feature = "large-basis-elements"))]
pub const QTY_GROUPS: usize = 86;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BasisElementGroup {
    G1(BasisElement),
    G2(BasisElement, BasisElement),
    G3(BasisElement, BasisElement, BasisElement),
    G4(BasisElement, BasisElement, BasisElement, BasisElement),
}
impl Display for BasisElementGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BasisElementGroup::G1(a) => {
                write!(f, "{a}")?;
            }
            BasisElementGroup::G2(a, b) => {
                write!(f, "[{a}, {b}]")?;
            }
            BasisElementGroup::G3(a, b, c) => {
                write!(f, "[{a}, {b}, {c}]")?;
            }
            BasisElementGroup::G4(a, b, c, d) => {
                write!(f, "[{a}, {b}, {c}, {d}]")?;
            }
        }
        Ok(())
    }
}
impl PartialOrd for BasisElementGroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.into_vec().partial_cmp(&other.into_vec())
    }
}
impl Ord for BasisElementGroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_vec().cmp(&other.into_vec())
    }
}
#[derive(Copy, Clone)]
pub struct BasisElementGroupIter(usize, BasisElement, [Option<BasisElement>; 3]);
impl IntoIterator for BasisElementGroup {
    type Item = BasisElement;
    type IntoIter = BasisElementGroupIter;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            BasisElementGroup::G1(a) => BasisElementGroupIter(0, a, [None; 3]),
            BasisElementGroup::G2(a, b) => BasisElementGroupIter(0, a, [Some(b), None, None]),
            BasisElementGroup::G3(a, b, c) => BasisElementGroupIter(0, a, [Some(b), Some(c), None]),
            BasisElementGroup::G4(a, b, c, d) => BasisElementGroupIter(0, a, [Some(b), Some(c), Some(d)]),
        }
    }
}
impl Iterator for BasisElementGroupIter {
    type Item = BasisElement;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.0 {
            0 => Some(self.1),
            1 => self.2[0],
            2 => self.2[1],
            3 => self.2[2],
            _ => None,
        };
        self.0 += 1;
        result
    }
}
impl BasisElementGroup {
    pub const fn can_push(&self) -> bool {
        match self {
            BasisElementGroup::G4(_, _, _, _) => false,
            _ => true,
        }
    }

    pub const fn push(&mut self, el: BasisElement) {
        *self = match self {
            BasisElementGroup::G1(a) => BasisElementGroup::G2(*a, el),
            BasisElementGroup::G2(a, b) => BasisElementGroup::G3(*a, *b, el),
            BasisElementGroup::G3(a, b, c) => BasisElementGroup::G4(*a, *b, *c, el),
            _ => panic!("Please check can_push() before using push()"),
        }
    }

    pub const fn const_clone(&self) -> Self {
        match self {
            BasisElementGroup::G1(a) => BasisElementGroup::G1(*a),
            BasisElementGroup::G2(a, b) => BasisElementGroup::G2(*a, *b),
            BasisElementGroup::G3(a, b, c) => BasisElementGroup::G3(*a, *b, *c),
            BasisElementGroup::G4(a, b, c, d) => BasisElementGroup::G4(*a, *b, *c, *d),
        }
    }

    pub const fn from_vec(v: ConstVec<BasisElement, 4>) -> Self {
        match v.len() {
            1 => BasisElementGroup::G1(*v.get(0)),
            2 => BasisElementGroup::G2(*v.get(0), *v.get(1)),
            3 => BasisElementGroup::G3(*v.get(0), *v.get(1), *v.get(2)),
            4 => BasisElementGroup::G4(*v.get(0), *v.get(1), *v.get(2), *v.get(3)),
            0 => panic!("Cannot create empty BasisElementGroup"),
            _ => panic!("Unreachable: ConstVec has type level size"),
        }
    }

    pub const fn into_vec(self) -> ConstVec<BasisElement, 4> {
        match self {
            BasisElementGroup::G1(a) => {
                let mut v = ConstVec::new();
                v.push(a);
                v
            }
            BasisElementGroup::G2(a, b) => {
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v
            }
            BasisElementGroup::G3(a, b, c) => {
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v.push(c);
                v
            }
            BasisElementGroup::G4(a, b, c, d) => {
                let mut v = ConstVec::new();
                v.push(a);
                v.push(b);
                v.push(c);
                v.push(d);
                v
            }
        }
    }

    pub(crate) fn expr_type(&self) -> ExpressionType {
        match self {
            BasisElementGroup::G1(_) => ExpressionType::Float(Float),
            BasisElementGroup::G2(_, _) => ExpressionType::Vec2(Vec2),
            BasisElementGroup::G3(_, _, _) => ExpressionType::Vec3(Vec3),
            BasisElementGroup::G4(_, _, _, _) => ExpressionType::Vec4(Vec4),
        }
    }

    pub(crate) fn simd_width(&self) -> usize {
        match self {
            BasisElementGroup::G1(_) => 1,
            BasisElementGroup::G2(_, _) => 2,
            BasisElementGroup::G3(_, _, _) => 3,
            BasisElementGroup::G4(_, _, _, _) => 4,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MultiVec<const AntiScalar: BasisElement> {
    pub name: &'static str,
    pub grades: Grades,
    element_groups: ConstVec<BasisElementGroup, QTY_GROUPS>,
}

impl<const AntiScalar: BasisElement> Debug for MultiVec<AntiScalar> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "MultiVec {{ name: \"{n}\", element_groups: [")?;
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i);
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}[")?;
            let mut j = 0;
            let gr = group.clone().into_vec();
            while j < gr.len() {
                let el = gr.get(j);
                let comma = if j == 0 { "" } else { ", " };
                write!(f, "{comma}{el}")?;
                j += 1;
            }
            write!(f, "]")?;
            i += 1;
        }
        write!(f, "] }}")
    }
}
impl<const AntiScalar: BasisElement> Display for MultiVec<AntiScalar> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.name;
        write!(f, "{n}(")?;
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i);
            let comma = if i == 0 { "" } else { ", " };
            write!(f, "{comma}(")?;
            let mut j = 0;
            let gr = group.clone().into_vec();
            while j < gr.len() {
                let comma = if j == 0 { "" } else { ", " };
                let el = gr.get(j);
                write!(f, "{comma}{el}")?;
                j += 1;
            }
            write!(f, ")")?;
            i += 1;
        }
        write!(f, ")")
    }
}

impl<const AntiScalar: BasisElement> MultiVec<AntiScalar> {
    pub fn macro_expression(&self) -> String {
        use std::fmt::Write;
        let n = self.name;
        let mut f = format!("{n} as ");
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i).into_vec();
            if i > 0 {
                write!(f, " | ").unwrap();
            }
            let mut j = 0;
            while j < group.len() {
                let el = group.get(j);
                if j > 0 {
                    write!(f, ", ").unwrap();
                }
                write!(f, "{}", el).unwrap();
                j += 1;
            }
            i += 1;
        }
        write!(f, ";").unwrap();
        f
    }

    pub fn elements(&self) -> Vec<BasisElement> {
        let mut v = vec![];
        let mut i = 0;
        while i < self.element_groups.len() {
            let group = self.element_groups.get(i).clone().into_vec();
            let mut j = 0;
            while j < group.len() {
                v.push(*group.get(j));
                j += 1;
            }
            i += 1;
        }
        v
    }

    pub fn groups(&self) -> &ConstVec<BasisElementGroup, QTY_GROUPS> {
        &self.element_groups
    }

    pub fn new<E: IntoIterator<Item = BasisElement>>(name: &'static str, elements: E) -> Self {
        let mut elements = elements.into_iter().collect::<Vec<_>>();
        elements.sort();
        let mut active_grade = elements.get(0).map(|it| it.grade()).unwrap_or(0);
        let mut grouped = ConstVec::<BasisElementGroup, QTY_GROUPS>::new();

        let mut group_as_vec = ConstVec::<BasisElement, 4>::new();

        let mut i = 0;
        while i < elements.len() {
            let element = elements[i];
            let element_grade = element.grade();
            if element_grade == active_grade {
                // Same grade, same group
                group_as_vec.push(element);
            } else {
                // New grade, new group.
                // If you want extra-compact grouping, then use new_by_groups with manually
                // specified groups instead. Or I guess we could add more heuristics here
                // at some point, but not feeling rushed for it.
                grouped.push(BasisElementGroup::from_vec(group_as_vec));
                group_as_vec = ConstVec::<BasisElement, 4>::new();
                active_grade = element_grade;

                group_as_vec.push(element);
            }

            if group_as_vec.len() >= 4 {
                grouped.push(BasisElementGroup::from_vec(group_as_vec));
                group_as_vec = ConstVec::<BasisElement, 4>::new();
            }
            i += 1;
        }
        if group_as_vec.len() > 0 {
            grouped.push(BasisElementGroup::from_vec(group_as_vec));
        }
        Self::new_by_groups(name, grouped)
    }
}

// At the time of this comment, the compiler can ICE on MultiVec::new_by_groups in const eval.
//  It's really hard to understand why, but this wrapper function seems to mitigate it.
pub const fn multivec_by_groups<const AntiScalar: BasisElement>(name: &'static str, element_groups: ConstVec<BasisElementGroup, QTY_GROUPS>) -> MultiVec<AntiScalar> {
    MultiVec::new_by_groups(name, element_groups)
}

impl<const AntiScalar: BasisElement> MultiVec<AntiScalar> {
    pub const fn new_by_groups(name: &'static str, element_groups: ConstVec<BasisElementGroup, QTY_GROUPS>) -> Self {
        if ((AntiScalar.grade() / 3) + 1) as usize > QTY_GROUPS {
            panic!(
                "If you want to create an 9-12 dimensional GA, then enable the \
            \"large-basis-elements\" feature. If you want to create a 13-16 dimensional GA, then \
            enable the \"very-large-basis-elements\" feature."
            )
        }

        let mut used_signatures = ConstVec::<BasisElement, QTY_GROUPS>::new();
        let mut grades = Grades::none;
        let mut i = 0;
        while i < element_groups.len() {
            let group = element_groups.get(i);
            let group_vec = group.const_clone().into_vec();
            let mut j = 0;
            while j < group_vec.len() {
                let el = group_vec.get(j);
                let el_sig = el.signature();
                if !AntiScalar.signature().contains(el_sig) {
                    concat_panic!(
                        "MultiVector belonging to AntiScalar(",
                        AntiScalar,
                        ") \
                        is defined to include ",
                        el,
                        " which does not fit. "
                    );
                }
                let mut k = 0;
                while k < used_signatures.len() {
                    let u = used_signatures.get(k);
                    let u_sig = u.signature();
                    let u_sig_bits = u_sig.bits();
                    let el_sig_bits = el_sig.bits();
                    if u_sig_bits == el_sig_bits {
                        concat_panic!(
                            "MultiVec named ",
                            name,
                            " already has ",
                            el,
                            ". Do not \
                            define MultiVectors using redundant or duplicate BasisSignatures. \
                            Don't forget that reordered or sign flipped BasisElements can share \
                            the same BasisSignature. "
                        )
                    }
                    k += 1;
                }
                used_signatures.push(*el);
                grades = grades.const_bitor(Grades::from_sig(el_sig));
                j += 1;
            }
            i += 1;
        }
        MultiVec { name, grades, element_groups }
    }
}

#[macro_export]
macro_rules! multi_vec {
    // grouped using tuples
    ($mv_name:ident<$anti_scalar:ident> => $( ($($basis_element:ident),+ $(,)?)),+ $(,)?) => {
        {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        }
    };
    // grouped using arrays
    ($mv_name:ident<$anti_scalar:ident> => $( [$($basis_element:ident),+ $(,)?]),+ $(,)?) => {
        {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        }
    };
    // ungrouped list of BasisElement
    ($mv_name:ident<$anti_scalar:ident> => $( $basis_element:ident ),+ $(,)?) => {
        {
            // Allocations are not allowed in static/const, but can't be bothered to make a
            // compatible version when specifying groups is fine instead.
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let elements: std::vec::Vec<$crate::algebra::basis::BasisElement> = vec![
                $($basis_element),+,
            ];
            $crate::algebra::multivector::MultiVec::<{$anti_scalar}>::new(name, elements)
        }
    };
    // Elegant and sparse
    ($mv_name:ident<$anti_scalar:ident> as $( $($basis_element:ident),+ $(,)?)|+ ) => {
        {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        }
    };
}

#[macro_export]
macro_rules! multi_vecs {
    // Grouped using tuples
    ($anti_scalar:ident; $( $mv_name:ident => $( ($($basis_element:ident),+ $(,)?)),+ $(,)? );+ $(;)?) => {
        $(
        pub static $mv_name: $crate::algebra::multivector::MultiVec<{$anti_scalar}> = {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name,)+
            ])
        }
    };
    // Grouped using arrays
    ($anti_scalar:ident; $( $mv_name:ident => $( [$($basis_element:ident),+ $(,)?]),+ $(,)? );+ $(;)?) => {
        $(
        pub static $mv_name: $crate::algebra::multivector::MultiVec<{$anti_scalar}> = {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name, )+
            ])
        }
    };
    // Elegant and sparse
    ($anti_scalar:ident; $( $mv_name:ident as $( $($basis_element:ident),+ $(,)?)|+ );+ $(;)?) => {
        $(
        pub static $mv_name: $crate::algebra::multivector::MultiVec<{$anti_scalar}> = {
            use $crate::elements::*;
            let name: &'static str = stringify!($mv_name);
            let groups: $crate::utility::ConstVec<
                $crate::algebra::multivector::BasisElementGroup,
                { $crate::algebra::multivector::QTY_GROUPS }
            > = {
                use $crate::algebra::multivector::TupleToGroup;
                let mut cv = $crate::utility::ConstVec::new();
                $(cv.push(($($basis_element),+,).tuple_to_group());)+
                cv
            };
            $crate::algebra::multivector::multivec_by_groups::<{$anti_scalar}>(name, groups)
        };
        )+
        pub fn register_multi_vecs(ga: std::sync::Arc<$crate::algebra::GeometricAlgebra<{$anti_scalar}>>) -> $crate::algebra::multivector::DeclareMultiVecs<{$anti_scalar}> {
            $crate::algebra::multivector::DeclareMultiVecs::declare(ga, [
                $(&$mv_name, )+
            ])
        }
    };
}

#[const_trait]
pub trait TupleToGroup {
    fn tuple_to_group(self) -> BasisElementGroup;
}
impl const TupleToGroup for (BasisElement,) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G1(self.0)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G2(self.0, self.1)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G3(self.0, self.1, self.2)
    }
}
impl const TupleToGroup for (BasisElement, BasisElement, BasisElement, BasisElement) {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G4(self.0, self.1, self.2, self.3)
    }
}
impl const TupleToGroup for [BasisElement; 4] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G4(self[0], self[1], self[2], self[3])
    }
}
impl const TupleToGroup for [BasisElement; 3] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G3(self[0], self[1], self[2])
    }
}
impl const TupleToGroup for [BasisElement; 2] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G2(self[0], self[1])
    }
}
impl const TupleToGroup for [BasisElement; 1] {
    fn tuple_to_group(self) -> BasisElementGroup {
        BasisElementGroup::G1(self[0])
    }
}

mod test_stuff {
    use crate::algebra::basis::elements::*;
    use crate::algebra::multivector::{MultiVec, TupleToGroup};
    use crate::utility::ConstVec;

    static Circle: MultiVec<{ e12345 }> = MultiVec::<e12345>::new_by_groups("Circle", {
        let mut cv = ConstVec::new();
        cv.push((e423, e431, e412, e321).tuple_to_group());
        cv.push((e415, e425, e435).tuple_to_group());
        cv.push((e235, e315, e125).tuple_to_group());
        cv
    });

    static Circle2: MultiVec<{ e12345 }> = multi_vec!(Circle<e12345> => (e423, e431, e412, e321), (e415, e425, e435), (e235, e315, e125));
    static Circle3: MultiVec<{ e12345 }> = multi_vec!(Circle<e12345> => [e423, e431, e412, e321], [e415, e425, e435], [e235, e315, e125]);
    static Dipole2: MultiVec<{ e12345 }> = multi_vec!(Dipole<e12345> as e41, e42, e43 | e23, e31, e12 | e15, e25, e35, e45);

    #[test]
    fn test_construction() {
        println!("{Circle:?}");
        println!("{Circle}");

        let circle_again = &Circle2;
        println!("{circle_again}");

        let circle_again = &Circle3;
        println!("{circle_again}");

        let dipole_again = &Dipole2;
        println!("{dipole_again}");
    }
}

pub struct DeclareMultiVecs<const AntiScalar: BasisElement> {
    ga: Arc<GeometricAlgebra<AntiScalar>>,
    declared: Vec<(Grades, BTreeSet<BasisSignature>, &'static MultiVec<AntiScalar>)>,
    unique_names: BTreeSet<&'static str>,
    documentation: BTreeMap<String, String>,
}

impl<const AntiScalar: BasisElement> DeclareMultiVecs<AntiScalar> {
    pub fn declare<const N: usize>(ga: Arc<GeometricAlgebra<AntiScalar>>, multi_vecs: [&'static MultiVec<AntiScalar>; N]) -> Self {
        let mut unique_names = BTreeSet::new();
        for mv in multi_vecs.iter() {
            let n = mv.name;
            if unique_names.contains(n) {
                panic!("MultiVec names must be unique, but this one shows up multiple times: {n}");
            }
            unique_names.insert(n);
        }

        let mut nb = ga.named_bases.write();
        let mut declared = vec![];
        if !multi_vecs.is_empty() {
            println!("\n// Manually Specified MultiVecs:");
        }
        for multi_vec in multi_vecs {
            let nice = multi_vec.macro_expression();
            println!("{nice}");
            let mut grades = Grades::none;
            let mut sig = BTreeSet::new();
            for el in multi_vec.elements() {
                if !AntiScalar.signature().contains(el.signature()) {
                    panic!("Element does not fit in anti_scalar {AntiScalar}: {el} in {multi_vec}");
                }
                match nb.accept_name(el) {
                    Ok(_) => {}
                    Err(err) => panic!("Could not accept BasisElement {el}: {err}"),
                }
                grades |= el.grades();
                sig.insert(el.signature());
            }
            declared.push((grades, sig, multi_vec));
        }
        drop(nb);
        let mut slf = DeclareMultiVecs {
            ga,
            declared,
            unique_names,
            documentation: Default::default(),
        };
        slf.sort_declarations();
        slf
    }

    fn sort_declarations(&mut self) {
        self.declared.sort_unstable_by(|(a_grade, a_sig, a_mv), (b_grade, b_sig, b_mv)| {
            a_grade
                .cmp(b_grade)
                .then_with(|| a_sig.len().cmp(&b_sig.len()).then_with(|| a_sig.cmp(b_sig).then_with(|| a_mv.name.cmp(b_mv.name))))
        });
    }

    pub fn new(ga: Arc<GeometricAlgebra<AntiScalar>>) -> Self {
        DeclareMultiVecs {
            ga,
            declared: vec![],
            unique_names: BTreeSet::new(),
            documentation: Default::default(),
        }
    }

    pub fn finished(self) -> Arc<MultiVecRepository<AntiScalar>> {
        MultiVecRepository::new(self)
    }

    pub fn variants<S1: Into<String>, S2: Into<String>, F1: SigSetFilter, F2: SigFilter, F3: SigSetFilter>(
        &mut self,
        prefix: S1,
        suffix: S2,
        filter_vecs_in: F1,
        filter_elements: F2,
        filter_vecs_out: F3,
        documentation: Option<&'static str>,
    ) {
        let mut add_these = vec![];
        let prefix = prefix.into();
        let suffix = suffix.into();
        for (_gr, sigs, mv) in self.declared.iter() {
            if !filter_vecs_in.filter_sig_set(sigs) {
                continue;
            }
            let old_name = mv.name;
            let new_name = format!("{prefix}{old_name}{suffix}");
            let mut did_filter_out_some = false;
            let mut new_groups = ConstVec::<BasisElementGroup, QTY_GROUPS>::new();
            let mut new_grades = Grades::none;
            let mut new_sigs = BTreeSet::new();
            let mut vacant_slots = 0;
            let mut vacant_1 = BTreeSet::new();
            let mut vacant_2 = BTreeSet::new();
            let mut vacant_3 = BTreeSet::new();
            for mvg in mv.element_groups.clone().into_iter() {
                let mut cv = ConstVec::new();
                for el in mvg.into_vec() {
                    if filter_elements.filter_sig(el.signature()) {
                        new_grades |= el.grades();
                        new_sigs.insert(el.signature());
                        cv.push(el);
                    } else {
                        did_filter_out_some = true;
                    }
                }
                let len = cv.len();
                let vacant_len = 4 - len;
                vacant_slots += vacant_len;
                let mut did_backfill = false;
                match (vacant_slots >= 4, vacant_len) {
                    (false, 1) => {
                        vacant_1.insert(new_groups.len());
                    }
                    (false, 2) => {
                        vacant_2.insert(new_groups.len());
                    }
                    (false, 3) => {
                        vacant_3.insert(new_groups.len());
                    }
                    (true, 1) => {
                        if let Some(group_to_fill_in_idx) = vacant_3.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in_idx];
                            group_to_fill_in.push(cv[0]);
                            group_to_fill_in.push(cv[1]);
                            group_to_fill_in.push(cv[2]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                        } else {
                            vacant_1.insert(new_groups.len());
                        }
                    }
                    (true, 2) => {
                        if let Some(group_to_fill_in) = vacant_2.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in];
                            group_to_fill_in.push(cv[0]);
                            group_to_fill_in.push(cv[1]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                        } else if let Some(group_to_fill_in_idx) = vacant_3.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in_idx];
                            group_to_fill_in.push(cv[0]);
                            group_to_fill_in.push(cv[1]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                            vacant_1.insert(group_to_fill_in_idx);
                        } else {
                            vacant_2.insert(new_groups.len());
                        }
                    }
                    (true, 3) => {
                        if let Some(group_to_fill_in) = vacant_1.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in];
                            group_to_fill_in.push(cv[0]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                        } else if let Some(group_to_fill_in_idx) = vacant_2.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in_idx];
                            group_to_fill_in.push(cv[0]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                            vacant_1.insert(group_to_fill_in_idx);
                        } else if let Some(group_to_fill_in_idx) = vacant_3.pop_first() {
                            let group_to_fill_in = &mut new_groups[group_to_fill_in_idx];
                            group_to_fill_in.push(cv[0]);
                            vacant_slots -= vacant_len;
                            did_backfill = true;
                            vacant_2.insert(group_to_fill_in_idx);
                        } else {
                            vacant_3.insert(new_groups.len());
                        }
                    }
                    _ => {}
                }
                if len > 0 && !did_backfill {
                    new_groups.push(BasisElementGroup::from_vec(cv));
                }
            }
            if !did_filter_out_some || new_sigs.is_empty() || !filter_vecs_out.filter_sig_set(&new_sigs) {
                continue;
            }
            add_these.push((new_grades, new_sigs, old_name, new_name, new_groups));
        }

        let mut intro = false;
        for (new_grades, new_sigs, old_name, new_name, new_groups) in add_these.into_iter() {
            let idx = self
                .declared
                .binary_search_by(|(gr, sig, _mv)| gr.cmp(&new_grades).then_with(|| sig.len().cmp(&new_sigs.len()).then_with(|| sig.cmp(&new_sigs))));
            let idx = match idx {
                Ok(_) => {
                    // already exists
                    continue;
                }
                Err(idx) => idx,
            };
            if self.unique_names.contains(new_name.as_str()) {
                // A panic might seem extreme, but the fact that this
                // implies semantic ambiguity feels very important.
                panic!(
                    "The generated variant {new_name} conflicts with an existing MultiVec of \
                    the same name but different signature."
                );
                // eprintln!("The generated variant {new_name} conflicts with an existing MultiVec \
                //     of the same name but different signature.");
                // continue;
            }
            if !intro {
                println!("\n// Variants: {prefix}{{Vector}}{suffix}");
                intro = true;
            }

            let new_name = Box::leak(new_name.into_boxed_str());
            self.unique_names.insert(new_name);
            let new_mv = MultiVec::<AntiScalar>::new_by_groups(new_name, new_groups);
            let new_mv = Box::leak(Box::new(new_mv));
            let nice = new_mv.macro_expression();
            println!("{nice}");
            self.declared.insert(idx, (new_grades, new_sigs, new_mv));
            if let Some(d) = documentation {
                let n = new_mv.name;
                let docs = self.documentation.entry(new_mv.name.to_string()).or_insert(format!("{n}.\n"));
                let mut d = d.to_string();
                d = d.replace("{type}", old_name);
                d = d.replace("{Vector}", old_name);
                docs.push_str(d.as_str());
            }
        }
    }

    pub fn generate_missing_duals(&mut self, documentation: Option<&'static str>) -> &mut Self {
        let mut add_these = vec![];
        for (_, _, mv) in self.declared.iter() {
            let old_name = mv.name;
            let new_name = format!("Anti{old_name}");
            let mut new_groups = ConstVec::new();
            let mut new_grades = Grades::none;
            let mut new_sigs = BTreeSet::new();
            for mvg in mv.element_groups.clone().into_iter() {
                let mut cv = ConstVec::new();
                for el in mvg.into_vec() {
                    let (_, d) = self.ga.right_dual(el);
                    new_grades |= d.grades();
                    new_sigs.insert(d.signature());
                    cv.push(d);
                }
                new_groups.push(BasisElementGroup::from_vec(cv));
            }
            add_these.push((new_grades, new_sigs, old_name, new_name, new_groups));
        }

        let mut intro = false;
        for (new_grades, new_sigs, old_name, new_name, new_groups) in add_these.into_iter() {
            let idx = self
                .declared
                .binary_search_by(|(gr, sig, _mv)| gr.cmp(&new_grades).then_with(|| sig.len().cmp(&new_sigs.len()).then_with(|| sig.cmp(&new_sigs))));
            let idx = match idx {
                Ok(_) => {
                    // already exists
                    continue;
                }
                Err(idx) => idx,
            };
            if self.unique_names.contains(new_name.as_str()) {
                // A panic might seem extreme, but the fact that this
                // implies semantic ambiguity feels very important.
                panic!(
                    "The generated variant {new_name} conflicts with an existing MultiVec of \
                    the same name but different signature."
                );
                // eprintln!("The generated variant {new_name} conflicts with an existing MultiVec \
                //     of the same name but different signature.");
                // continue;
            }
            if !intro {
                println!("\n// Variants: Anti{{Vector}}");
                intro = true;
            }
            let new_name = Box::leak(new_name.into_boxed_str());
            self.unique_names.insert(new_name);
            let new_mv = MultiVec::<AntiScalar>::new_by_groups(new_name, new_groups);
            let new_mv = Box::leak(Box::new(new_mv));
            let nice = new_mv.macro_expression();
            println!("{nice}");
            self.declared.insert(idx, (new_grades, new_sigs.clone(), new_mv));
            if let Some(d) = documentation {
                let mut idx = idx + 1;
                let mut closest_to = None;
                while idx < self.declared.len() {
                    let Some((gr, sigs, mv)) = self.declared.get(idx) else { break };
                    if !gr.contains(new_grades) {
                        idx += 1;
                        continue;
                    }
                    if sigs.is_superset(&new_sigs) {
                        closest_to = Some(mv.name);
                        break;
                    }
                    idx += 1;
                }

                let n = new_mv.name;
                let docs = self.documentation.entry(new_mv.name.to_string()).or_insert(format!("{n}.\n"));
                let mut d = d.to_string();
                d = d.replace("{type}", old_name);
                d = d.replace("{Vector}", old_name);
                let n = closest_to.unwrap_or("MultiVector");
                d = d.replace("{super}", n);
                d = d.replace("{Super}", n);
                docs.push_str(d.as_str());
            }
        }
        self
    }

    pub fn append_documentation<S: Into<String>>(&mut self, mv: &'static MultiVec<AntiScalar>, s: S) -> &mut Self {
        let n = mv.name;
        let docs = self.documentation.entry(mv.name.to_string()).or_insert(format!("{n}.\n"));
        let s = s.into();
        docs.push_str(s.as_str());
        self
    }
}

pub struct MultiVecRepository<const AntiScalar: BasisElement> {
    declarations: DeclareMultiVecs<AntiScalar>,
    uniform_grade_groups: BTreeMap<Grades, BTreeSet<&'static BasisElementGroup>>,
    mixed_grade_groups: BTreeMap<Grades, BTreeSet<&'static BasisElementGroup>>,
    wanted: Mutex<HashMap<BTreeSet<BasisSignature>, Vec<Arc<RawTraitImplementation>>>>,
    strongly_wanted: Mutex<HashMap<BTreeSet<BasisSignature>, Vec<Arc<RawTraitDefinition>>>>,
}

impl<const AntiScalar: BasisElement> MultiVecRepository<AntiScalar> {
    pub fn ga(&self) -> Arc<GeometricAlgebra<AntiScalar>> {
        self.declarations.ga.clone()
    }

    pub fn default(ga: Arc<GeometricAlgebra<AntiScalar>>) -> Arc<Self> {
        Self::new(DeclareMultiVecs::new(ga))
    }

    pub fn new(declarations: DeclareMultiVecs<AntiScalar>) -> Arc<Self> {
        let ga = declarations.ga.clone();
        let mut mvr = MultiVecRepository {
            declarations,
            uniform_grade_groups: Default::default(),
            mixed_grade_groups: Default::default(),
            wanted: Default::default(),
            strongly_wanted: Default::default(),
        };

        for (_, _, mv) in mvr.declarations.declared.iter() {
            let mut i = 0;
            while i < mv.element_groups.len() {
                let mvg = mv.element_groups.get(i);
                i += 1;

                let mut gr = Grades::none;
                for el in mvg.clone().into_vec() {
                    gr |= el.grades();
                }
                let qty_grades = gr.into_bits().count_ones();
                if qty_grades == 1 {
                    mvr.uniform_grade_groups
                        .entry(gr)
                        .and_modify(|v| {
                            v.insert(mvg);
                        })
                        .or_insert(BTreeSet::from([mvg]));
                } else {
                    mvr.mixed_grade_groups
                        .entry(gr)
                        .and_modify(|v| {
                            v.insert(mvg);
                        })
                        .or_insert(BTreeSet::from([mvg]));
                }
            }
        }

        // Generate fallback types.
        let all_elements: Vec<_> = ga.all_elements().map(|el| ga.fix_name_and_sign(el).1).collect();

        use crate::algebra::basis::elements::*;
        let mut has_fell_back = false;
        mvr.fallback(&mut has_fell_back, MultiVec::<AntiScalar>::new("Scalar", [scalar]));
        mvr.fallback(&mut has_fell_back, MultiVec::<AntiScalar>::new("AntiScalar", [AntiScalar]));

        // DualNum is kind of an enigma.
        // Very easy to imagine it should generally be scalar + AntiScalar.
        // But Eric recommends e5 + e12345 for CGA.
        // On the one hand this makes sense, because it multiplies by e5 to move from RGA to CGA.
        // But on the other hand this doesn't feel very general, or give any insight to other GAs.
        // For example, if we add more witt pairs or projective (to 0) dimensions.
        // So for now, we will not create DualNum as a fallback, and let each GA specify
        // its own DualNum instead.

        // mvr.fallback(&mut has_fell_back, MultiVec::<AntiScalar>::new_by_groups("DualNum", {
        //     let mut cv = ConstVec::new();
        //     cv.push(BasisElementGroup::G2(scalar, AntiScalar));
        //     cv
        // }));

        // 1..AntiScalar.grade() skips scalar and anti_scalar (since we already added them)
        for gr in 1..AntiScalar.grade() {
            let els: Vec<_> = all_elements.clone().into_iter().filter(|el| el.grade() == gr).collect();
            let mv = match gr {
                // 0 is Scalar, defined above
                1 => MultiVec::<AntiScalar>::new("Vector", els),
                2 => MultiVec::<AntiScalar>::new("BiVector", els),
                3 => MultiVec::<AntiScalar>::new("TriVector", els),
                4 => MultiVec::<AntiScalar>::new("QuadVector", els),
                5 => MultiVec::<AntiScalar>::new("VectorGr5", els),
                6 => MultiVec::<AntiScalar>::new("VectorGr6", els),
                7 => MultiVec::<AntiScalar>::new("VectorGr7", els),
                8 => MultiVec::<AntiScalar>::new("VectorGr8", els),
                9 => MultiVec::<AntiScalar>::new("VectorGr9", els),
                10 => MultiVec::<AntiScalar>::new("VectorGr10", els),
                11 => MultiVec::<AntiScalar>::new("VectorGr11", els),
                12 => MultiVec::<AntiScalar>::new("VectorGr12", els),
                13 => MultiVec::<AntiScalar>::new("VectorGr13", els),
                14 => MultiVec::<AntiScalar>::new("VectorGr14", els),
                15 => MultiVec::<AntiScalar>::new("VectorGr15", els),
                // 16 would be AntiScalar, defined above

                // This isn't possible because max grade of AntiScalar is 16
                _ => unreachable!("MultiVecs of D<0 or D>16 are not supported"),
            };
            mvr.fallback(&mut has_fell_back, mv);
        }
        // Full MultiVector
        {
            let mut remaining_els = BTreeMap::from_iter(all_elements.iter().map(|it| (it.signature(), *it)));
            let mut cv = ConstVec::new();
            cv.push(BasisElementGroup::G2(scalar, AntiScalar));
            remaining_els.remove(&scalar.signature());
            remaining_els.remove(&AntiScalar.signature());
            let cv = mvr.use_preferred_groups(cv, remaining_els);
            mvr.fallback(&mut has_fell_back, MultiVec::<AntiScalar>::new_by_groups("MultiVector", cv));
        }
        println!();
        mvr.declarations.sort_declarations();
        Arc::new(mvr)
    }

    fn use_preferred_groups(
        &self,
        mut cv: ConstVec<BasisElementGroup, QTY_GROUPS>,
        mut remaining_els: BTreeMap<BasisSignature, BasisElement>,
    ) -> ConstVec<BasisElementGroup, QTY_GROUPS> {
        let mut used_sigs = BTreeSet::new();
        let mut i = 0;
        while i < cv.len() {
            let used_group = cv.get(i);
            for el in used_group.into_iter() {
                used_sigs.insert(el.signature());
            }
            i += 1;
        }
        let all_groups = self.uniform_grade_groups.iter().map(|it| it.1).chain(self.mixed_grade_groups.iter().map(|it| it.1));
        for mvgs in all_groups {
            if remaining_els.is_empty() {
                break;
            }
            let mut mvgs = Vec::from_iter(mvgs);
            mvgs.sort_unstable_by(|a, b| {
                let a = a.into_vec();
                let b = b.into_vec();
                b.len().cmp(&a.len()).then_with(|| a.cmp(&b))
            });
            for mvg in mvgs.into_iter() {
                let can_use = (**mvg).clone().into_iter().all(|el| !used_sigs.contains(&el.signature()));
                if can_use {
                    for el in (*mvg).clone().into_iter() {
                        remaining_els.remove(&el.signature());
                        used_sigs.insert(el.signature());
                    }
                    cv.push((**mvg).clone());
                }
            }
        }
        let mut last_group = vec![];
        for (_, el) in remaining_els {
            last_group.push(el);
            if last_group.len() == 4 {
                cv.push(BasisElementGroup::G4(last_group[0], last_group[1], last_group[2], last_group[3]));
                last_group = vec![];
            }
        }
        if !last_group.is_empty() {
            cv.push(match last_group.len() {
                1 => BasisElementGroup::G1(last_group[0]),
                2 => BasisElementGroup::G2(last_group[0], last_group[1]),
                3 => BasisElementGroup::G3(last_group[0], last_group[1], last_group[2]),
                _ => unreachable!("last_group.len() cannot be <1 or >3 in this branch."),
            });
        }
        cv
    }

    fn fallback(&mut self, has_fell_back: &mut bool, multi_vec: MultiVec<AntiScalar>) {
        let mut signature = BTreeSet::new();
        let mut grades = Grades::none;
        for el in multi_vec.elements().iter() {
            grades |= el.grades();
            signature.insert(el.signature());
        }
        let idx = self.declarations.declared.binary_search_by(|(gr, sig, mv)| {
            match gr.cmp(&grades) {
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => {}
            }
            if sig.eq(&signature) {
                return std::cmp::Ordering::Equal;
            }
            sig.len().cmp(&signature.len()).then_with(|| sig.cmp(&signature).then_with(|| mv.name.cmp(multi_vec.name)))
        });
        let Err(insert_idx) = idx else { return };
        let multi_vec = Box::leak(Box::new(multi_vec));
        let mut i = 0;
        while i < multi_vec.element_groups.len() {
            let mvg = multi_vec.element_groups.get(i);
            i += 1;
            let mut gr = Grades::none;
            for el in mvg.clone().into_iter() {
                gr |= el.grades();
            }
            if gr.into_bits().count_ones() == 1 {
                self.uniform_grade_groups
                    .entry(gr)
                    .and_modify(move |s| {
                        s.insert(mvg);
                    })
                    .or_insert(BTreeSet::from([mvg]));
            } else {
                self.mixed_grade_groups
                    .entry(gr)
                    .and_modify(move |s| {
                        s.insert(mvg);
                    })
                    .or_insert(BTreeSet::from([mvg]));
            }
        }
        let nice_declaration = multi_vec.macro_expression();
        if !*has_fell_back {
            println!("\n// Required MultiVecs were generated:");
            *has_fell_back = true;
        }
        println!("{nice_declaration}");
        self.declarations.declared.insert(insert_idx, (grades, signature, multi_vec));
    }

    pub fn scalar(&self) -> &'static MultiVec<AntiScalar> {
        let signature = BTreeSet::from([BasisSignature::scalar]);
        self.get_exact(&signature).expect("Scalar should always be declared (if not explicitly, then implicitly)")
    }

    pub fn anti_scalar(&self) -> &'static MultiVec<AntiScalar> {
        let signature = BTreeSet::from([AntiScalar.signature()]);
        self.get_exact(&signature).expect("AntiScalar should always be declared (if not explicitly, then implicitly)")
    }

    pub fn dual_num(&self) -> &'static MultiVec<AntiScalar> {
        let signature = BTreeSet::from([BasisSignature::scalar, AntiScalar.signature()]);
        self.get_exact(&signature).expect("DualNum should always be declared (if not explicitly, then implicitly)")
    }

    pub fn full_multi_vector(&self) -> &'static MultiVec<AntiScalar> {
        // Maximum filled grades is sorted to the end,
        // then sorted by signatures lengths,
        // so the full multivector can only be at the end.
        self.declarations.declared.last().expect("Full MultiVec is always declared").2
    }

    pub(crate) fn get_at_least(&self, signature: &BTreeSet<BasisSignature>) -> (&'static MultiVec<AntiScalar>, bool) {
        let mut grades = Grades::none;
        for sig in signature.iter() {
            grades |= Grades::from_sig(*sig);
        }
        let grades = grades;

        let mut result = self.declarations.declared.last().expect("Full MultiVec is always declared");
        for tuple in self.declarations.declared.iter() {
            let (gr, sig, mv) = tuple;
            if !gr.contains(grades) {
                continue;
            }
            if !sig.is_superset(signature) {
                continue;
            }
            if sig.len() == signature.len() {
                return (*mv, true);
            }
            if result.0.into_bits().count_ones() > gr.into_bits().count_ones() {
                result = tuple;
                continue;
            }
            if result.1.len() > sig.len() {
                result = tuple;
            }
        }
        (result.2, false)
    }

    pub(crate) fn get_exact(&self, signature: &BTreeSet<BasisSignature>) -> Option<&'static MultiVec<AntiScalar>> {
        let mut grades = Grades::none;
        for sig in signature.iter() {
            grades |= Grades::from_sig(*sig);
        }
        let thing = self
            .declarations
            .declared
            .binary_search_by(|(gr, sig, _mv)| gr.cmp(&grades).then_with(|| sig.len().cmp(&signature.len()).then_with(|| sig.cmp(signature))))
            .ok()?;
        let mv = self.declarations.declared.get(thing)?.2;
        Some(mv)
    }

    pub fn all_classes(&self) -> impl Iterator<Item = &'static MultiVec<AntiScalar>> {
        let mut v = vec![];
        for mv in self.declarations.declared.iter() {
            v.push(mv.2);
        }
        v.into_iter()
    }

    pub fn qty_classes(&self) -> usize {
        self.declarations.declared.len()
    }

    pub(crate) fn note_wanted(&self, sig: BTreeSet<BasisSignature>, ti: Arc<RawTraitImplementation>) {
        let mut w = self.wanted.lock();
        let ti2 = ti.clone();
        w.entry(sig).and_modify(move |v| v.push(ti2)).or_insert(vec![ti]);
    }

    pub(crate) fn note_strongly_wanted(&self, sig: BTreeSet<BasisSignature>, td: Arc<RawTraitDefinition>) {
        let mut w = self.strongly_wanted.lock();
        let td2 = td.clone();
        w.entry(sig).and_modify(move |v| v.push(td2)).or_insert(vec![td]);
    }

    pub(crate) fn declarations(&self) -> Vec<&'static MultiVec<AntiScalar>> {
        self.declarations.declared.iter().map(|it| it.2).collect()
    }

    pub(crate) fn documentation(&self) -> &BTreeMap<String, String> {
        &self.declarations.documentation
    }
}

#[derive(Debug)]
pub struct DynamicMultiVector {
    vals: BTreeMap<BasisSignature, FloatExpr>,
}
impl DynamicMultiVector {
    pub fn zero() -> Self {
        DynamicMultiVector { vals: BTreeMap::new() }
    }

    pub fn grades(&self) -> Grades {
        let mut g = Grades::none;
        for el in self.vals.keys() {
            let el = BasisElement::from(*el);
            g |= el.grades();
        }
        g
    }

    // noinspection DuplicatedCode
    pub fn construct<const AntiScalar: BasisElement>(self, b: &TraitImplBuilder<AntiScalar, HasNotReturned>) -> Option<MultiVectorExpr> {
        if self.vals.is_empty() {
            return None;
        }
        let repo = b.mvs.clone();
        let mut vals = BTreeMap::new();
        let mut keys = BTreeSet::new();
        for (el, mut f) in self.vals.into_iter() {
            // Some calculations are less efficient without variables.
            // But some expressions can't be simplified down to 0 without variable inlining.
            // So we explore what happens with deep inlining,
            // even if we stick to using variables in the end.
            let mut orig_f = f.clone();
            f.deep_simplify();
            if let FloatExpr::Literal(0.0) = f {
                continue
            }

            let mut el = BasisElement::from(el);
            let (fix_f, fix_el) = repo.ga().fix_name_and_sign(el);
            if fix_f == 0.0 {
                continue;
            }
            if b.is_deep_inlining {
                f = f * fix_f;
            } else {
                orig_f.simplify();
                f = orig_f * fix_f;
            }
            el = fix_el;

            // println!("Constructing with non-zero element: {el}({f:?})");
            keys.insert(el.signature());
            vals.insert(el, f);
        }
        let (mv, exact) = repo.get_at_least(&keys);
        if !exact {
            b.note_wanted(keys)
        }
        let mv = MultiVector::from(mv);
        b.multivector_dependencies.lock().insert(mv);
        let result = mv.construct(|el| vals.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        if !vals.is_empty() {
            panic!("vals should be empty");
        }
        Some(result)
    }

    // TODO do I need to add tracing to this function? Would hate to find simplification bugs
    //  in such an early stage of trait implementation construction
    // noinspection DuplicatedCode
    pub fn construct_exact<const AntiScalar: BasisElement>(self, b: &TraitImplBuilder<AntiScalar, HasNotReturned>) -> Option<MultiVectorExpr> {
        if self.vals.is_empty() {
            return None;
        }
        let repo = b.mvs.clone();
        let mut vals = BTreeMap::new();
        for (el, mut f) in self.vals.into_iter() {
            // Some calculations are less efficient without variables.
            // But some expressions can't be simplified down to 0 without variable inlining.
            // So we explore what happens with deep inlining,
            // even if we stick to using variables in the end.
            let mut orig_f = f.clone();
            f.deep_simplify();

            match f {
                FloatExpr::Literal(0.0) => continue,
                FloatExpr::Product(v, _f) if v.is_empty() => panic!("Problem"),
                FloatExpr::Sum(v, _a) if v.is_empty() => panic!("Problem"),
                _ => {}
            }

            let mut el = BasisElement::from(el);
            let (fix_f, fix_el) = repo.ga().fix_name_and_sign(el);
            if fix_f == 0.0 {
                continue;
            }
            if b.is_deep_inlining {
                f = f * fix_f;
            } else {
                orig_f.simplify();
                f = orig_f * fix_f;
            }
            el = fix_el;

            // println!("Constructing with non-zero element: {el}({f:?})");
            vals.insert(el, f);
        }
        let keys = vals.keys().map(|el| el.signature()).collect();
        let mv = repo.get_exact(&keys);
        let Some(mv) = mv else {
            b.mvs.note_strongly_wanted(keys, b.trait_def.clone());
            return None;
        };
        let mv = MultiVector::from(mv);
        b.multivector_dependencies.lock().insert(mv);
        let result = mv.construct(|el| vals.remove(&el).unwrap_or(FloatExpr::Literal(0.0)));
        if !vals.is_empty() {
            panic!("vals should be empty");
        }
        Some(result)
    }
}
impl<FE: Into<FloatExpr>> AddAssign<(FE, BasisElement)> for DynamicMultiVector {
    fn add_assign(&mut self, rhs: (FE, BasisElement)) {
        let sign = rhs.1.coefficient() as f32;
        if sign == 0.0 {
            return;
        }
        let signature = rhs.1.signature();
        let thing = self.vals.entry(signature).or_insert(FloatExpr::Literal(0.0));
        *thing += rhs.0.into() * sign;
        if let FloatExpr::Literal(0.0) = thing {
            self.vals.remove(&signature);
        }
    }
}

impl Display for DynamicMultiVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut els: Vec<_> = self.vals.iter().collect();
        els.sort_by_key(|(el, _)| (*el).clone());
        write!(f, "DynamicMultiVector(")?;
        for (i, (el, e)) in els.into_iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{el}({e})")?;
        }
        write!(f, ")")
    }
}

//
