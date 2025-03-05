#![allow(non_upper_case_globals)]

use std::marker::{ConstParamTy, ConstParamTy_, UnsizedConstParamTy};

use crate::algebra::basis::{BasisElement, BasisSignature};
use std::cmp::PartialEq;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, BitOrAssign, Not};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ConstParamTy)]
pub struct Grades(u32);
pub type AntiGrades = Grades;

#[allow(non_upper_case_globals)]
impl Grades {
    pub const none: Grades = Grades(0x0);

    pub const fn into_bits(self) -> u32 {
        self.0
    }

    const fn from_bits(mut b: u32) -> Self {
        // Up to 16 dimensions (see BasisSignature) gives us 1 << 15
        // Plus the bit representing grade0 gives us 1 << 16
        // Then to enable all valid bits we go to 1 << 17 and then subtract 1
        b &= (0x1 << (16 + 1)) - 1;
        Grades(b)
    }

    pub const fn new(gr: u32) -> Self {
        // Grade 0 takes 1 bit of grades
        // So grade 0 = 0x1
        // Grade 1 = 0x2
        // and NO GRADES that is to say NOT EVEN GRADE 0 is represented as 0x0
        Grades::from_bits(1u32 << gr)
    }

    pub const fn from_sig(sig: BasisSignature) -> Self {
        let ones = sig.bits().count_ones();
        Grades::from_bits(1u32 << ones)
    }

    pub const fn const_bitor(self, rhs: Self) -> Self {
        Grades(self.0 | rhs.0)
    }

    pub const fn const_bitand(self, rhs: Self) -> Self {
        Grades(self.0 & rhs.0)
    }
    pub const fn const_not(self) -> Self {
        Grades(!self.0)
    }

    pub const fn contains(&self, other: Grades) -> bool {
        self.0 == self.0 | other.0
    }
}

pub const grade0: Grades = Grades(0x1);
pub const grade1: Grades = Grades(0x2);
pub const grade2: Grades = Grades(0x4);
pub const grade3: Grades = Grades(0x8);
pub const grade4: Grades = Grades(0x10);
pub const grade5: Grades = Grades(0x20);
pub const grade6: Grades = Grades(0x40);
pub const grade7: Grades = Grades(0x80);
pub const grade8: Grades = Grades(0x100);
pub const grade9: Grades = Grades(0x200);
pub const grade10: Grades = Grades(0x400);
pub const grade11: Grades = Grades(0x800);
pub const grade12: Grades = Grades(0x1000);
pub const grade13: Grades = Grades(0x2000);
pub const grade14: Grades = Grades(0x4000);
pub const grade15: Grades = Grades(0x8000);
pub const grade16: Grades = Grades(0x10000);

pub fn plane_based_k_reflections() -> [Grades; 17] {
    [
        grade0,
        grade1,
        grade0 | grade2,
        grade1 | grade3,
        grade0 | grade2 | grade4,
        grade1 | grade3 | grade5,
        grade0 | grade2 | grade4 | grade6,
        grade1 | grade3 | grade5 | grade7,
        grade0 | grade2 | grade4 | grade6 | grade8,
        grade1 | grade3 | grade5 | grade7 | grade9,
        grade0 | grade2 | grade4 | grade6 | grade8 | grade10,
        grade1 | grade3 | grade5 | grade7 | grade9 | grade11,
        grade0 | grade2 | grade4 | grade6 | grade8 | grade10 | grade12,
        grade1 | grade3 | grade5 | grade7 | grade9 | grade11 | grade13,
        grade0 | grade2 | grade4 | grade6 | grade8 | grade10 | grade12 | grade14,
        grade1 | grade3 | grade5 | grade7 | grade9 | grade11 | grade13 | grade15,
        grade0 | grade2 | grade4 | grade6 | grade8 | grade10 | grade12 | grade14 | grade16,
    ]
}
pub fn point_based_k_reflections<const AntiScalar: BasisElement>() -> [Grades; 17] {
    let mut grades = AntiScalar.grades();
    let mut result = [grades; 17];
    for k in 0..17 {
        if k > 0 {
            grades.0 = grades.0 >> 1;
            if k % 2 == 0 {
                grades.0 |= AntiScalar.grades().0;
            }
        }
        result[k] = grades;
    }
    result
}

impl BitOr for Grades {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Grades(self.0 | rhs.0)
    }
}
impl BitOrAssign for Grades {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}
impl BitAnd for Grades {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Grades(self.0 & rhs.0)
    }
}
impl Not for Grades {
    type Output = Self;

    fn not(self) -> Self::Output {
        Grades(!self.0)
    }
}

// Implement `ConstParamTy` for `Grades`
impl UnsizedConstParamTy for Grades {}
impl ConstParamTy_ for Grades {}

#[macro_export]
macro_rules! grade_constraint {
    ($g:ty, $h:ty) => {
        [(); <$crate::algebra::basis::grades::AddGradesImpl as $crate::algebra::basis::grades::AddGradesTrait<$g, $h>>::OUTPUT.into_bits() as usize]
    };
}

pub struct AddGradesImpl;
pub trait AddGradesTrait<const G: Grades, const H: Grades> {
    type Output;
    const OUTPUT: Self::Output;
}
impl<const G: Grades, const H: Grades> AddGradesTrait<G, H> for AddGradesImpl {
    type Output = Grades;
    const OUTPUT: Grades = G.const_bitor(H);
}
