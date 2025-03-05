#![feature(const_option)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::marker::{ConstParamTy_, UnsizedConstParamTy};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Neg, Sub};
use const_panic::fmt::ShortString;
use const_panic::PanicFmt;

use crate::algebra::basis::generators::GeneratorElement;
use crate::algebra::basis::grades::{AntiGrades, Grades};
use crate::generator_squares;
use crate::utility::const_option::ConstOption;
use crate::utility::fstr::fstr;

pub mod arithmetic;
pub mod filter;
pub mod generators;
pub mod grades;
pub mod substitutes;

// Would love to use bitflags::bitflags!, but cannot because
// we need to implement ConstParamTy
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasisSignature(u16);
impl ConstParamTy_ for BasisSignature {}
impl UnsizedConstParamTy for BasisSignature {}

#[allow(non_upper_case_globals)]
impl BasisSignature {
    pub const scalar: BasisSignature = BasisSignature(0x0);
    pub const e0: BasisSignature = BasisSignature(0x1);
    pub const e1: BasisSignature = BasisSignature(0x2);
    pub const e2: BasisSignature = BasisSignature(0x4);
    pub const e3: BasisSignature = BasisSignature(0x8);
    pub const e4: BasisSignature = BasisSignature(0x10);
    pub const e5: BasisSignature = BasisSignature(0x20);
    pub const e6: BasisSignature = BasisSignature(0x40);
    pub const e7: BasisSignature = BasisSignature(0x80);
    pub const e8: BasisSignature = BasisSignature(0x100);
    pub const e9: BasisSignature = BasisSignature(0x200);
    pub const eA: BasisSignature = BasisSignature(0x400);
    pub const eB: BasisSignature = BasisSignature(0x800);
    pub const eC: BasisSignature = BasisSignature(0x1000);
    pub const eD: BasisSignature = BasisSignature(0x2000);
    pub const eE: BasisSignature = BasisSignature(0x4000);
    pub const eF: BasisSignature = BasisSignature(0x8000);
}

impl Default for BasisSignature {
    fn default() -> Self {
        BasisSignature::scalar
    }
}

impl PartialOrd for BasisSignature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(BasisSignature::const_cmp(self, other))
    }
}
impl Ord for BasisSignature {
    fn cmp(&self, other: &Self) -> Ordering {
        BasisSignature::const_cmp(self, other)
    }
}

impl Debug for BasisSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.bits();
        f.write_str(format!("BasisSignature(0b{num:016b})").as_str())
    }
}

impl Display for BasisSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.bits();
        if num == 0 {
            write!(f, "scalar")
        } else {
            write!(f, "e")?;
            for i in 0..16 {
                if num & (1 << i) != 0 {
                    write!(f, "{}", char::from_digit(i, 16).unwrap().to_ascii_uppercase())?;
                }
            }
            Ok(())
        }
    }
}

impl BasisSignature {
    // Strange return type is because of const evaluation compatibility
    const fn into_grade_1_signatures_const(self) -> (usize, [Option<BasisSignature>; 16]) {
        let mut result = [None; 16];
        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        while i < 16 {
            let sig = BasisSignature::from_bits_retain(u16::pow(2, i as u32));
            i += 1;
            if self.contains(sig) {
                result[j] = Some(sig);
                j += 1;
                len += 1;
            }
        }
        (len, result)
    }

    pub fn into_grade_1_signatures(self) -> Vec<BasisSignature> {
        self.into_grade_1_signatures_const().1.into_iter().filter_map(|it| it).collect()
    }

    // Strange return type is because of const evaluation compatibility
    const fn into_generator_elements_const(self) -> (usize, [Option<GeneratorElement>; 16]) {
        let mut result = [None; 16];
        let mut i = 0;
        let mut j = 0;
        let mut len = 0;
        let ge = GeneratorElement::array();
        while i < 16 {
            let sig = BasisSignature::from_bits_retain(u16::pow(2, i as u32));
            let ge = ge[i];
            i += 1;
            if self.contains(sig) {
                result[j] = Some(ge);
                j += 1;
                len += 1;
            }
        }
        (len, result)
    }

    pub fn into_generator_elements(self) -> Vec<GeneratorElement> {
        self.into_generator_elements_const().1.into_iter().filter_map(|it| it).collect()
    }

    pub const fn const_cmp(&self, other: &BasisSignature) -> Ordering {
        let a = self.bits();
        let b = other.bits();
        let aco = a.count_ones();
        let bco = b.count_ones();
        if aco < bco {
            return Ordering::Less;
        }
        if aco > bco {
            return Ordering::Greater;
        }
        let ra = a.reverse_bits();
        let rb = b.reverse_bits();
        if rb < ra {
            return Ordering::Less;
        }
        if rb > ra {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    pub const fn contains(&self, other: BasisSignature) -> bool {
        self.0 == (self.0 | other.0)
    }

    pub const fn from_bits_retain(bits: u16) -> BasisSignature {
        BasisSignature(bits)
    }

    pub const fn bits(&self) -> u16 {
        self.0
    }

    pub const fn empty() -> Self {
        BasisSignature(0)
    }

    pub const fn union(&self, rhs: BasisSignature) -> Self {
        BasisSignature(self.0 | rhs.0)
    }

    pub const fn intersection(&self, rhs: BasisSignature) -> Self {
        BasisSignature(self.0 & rhs.0)
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub const fn remove(&mut self, rhs: BasisSignature) {
        self.0 &= !rhs.0
    }
}

impl BitOr for BasisSignature {
    type Output = BasisSignature;

    fn bitor(self, rhs: Self) -> Self::Output {
        BasisSignature(self.0.bitor(rhs.0))
    }
}
impl BitOrAssign for BasisSignature {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl BitAnd for BasisSignature {
    type Output = BasisSignature;

    fn bitand(self, rhs: Self) -> Self::Output {
        BasisSignature(self.0.bitand(rhs.0))
    }
}
impl BitAndAssign for BasisSignature {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl Sub<BasisSignature> for BasisSignature {
    type Output = BasisSignature;

    fn sub(self, rhs: BasisSignature) -> Self::Output {
        BasisSignature(self.0 - rhs.0)
    }
}

impl PanicFmt for BasisSignature {
    type This = Self;
    type Kind = const_panic::IsCustomType;
    const PV_COUNT: usize = 17;
}
impl BasisSignature {
    pub const fn to_panicvals(self, _: const_panic::FmtArg) -> [const_panic::PanicVal<'static>; BasisSignature::PV_COUNT] {
        let mut result = {
            [
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
            ]
        };
        if self.0 == 0 {
            result[0] = const_panic::PanicVal::write_str("scalar");
            return result;
        }
        result[0] = const_panic::PanicVal::write_str("e");

        let hex_dec = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];
        let mut i = 0;
        let mut j = 1;
        while j < result.len() {
            let bit = 1 << i;
            if self.0 & bit == bit {
                result[j] = const_panic::PanicVal::write_str(hex_dec[i]);
            }
            i += 1;
            j += 1;
        }
        result
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BasisElement {
    // BasisElements will mathematically operate
    // under the assumption that the primary bases
    // are ordered normally, like e123456 etc
    coefficient: i8,
    signature: BasisSignature,

    // However you can also override the name of a
    // basis element (like "e3215" or "e_plus" or "e_minus"
    // or "anti_scalar" or "pseudo_scalar" or "pss" or "i")
    // As long as you also record the sign with
    // respect to the normally ordered element.
    // This way we can tell what to do with all of
    // (for example) +e412, -e412, +e124, and -e124
    display_name: ConstOption<BasisElementDisplayName>,
}
impl UnsizedConstParamTy for BasisElement {}
impl ConstParamTy_ for BasisElement {}

impl Neg for BasisElement {
    type Output = BasisElement;

    fn neg(self) -> Self::Output {
        let mut s = self;
        s.coefficient *= -1;
        s
    }
}
impl PanicFmt for BasisElement {
    type This = Self;
    type Kind = const_panic::IsCustomType;
    const PV_COUNT: usize = 23;
}
impl BasisElement {
    pub const fn to_panicvals(self, fmt: const_panic::FmtArg) -> [const_panic::PanicVal<'static>; BasisElement::PV_COUNT] {
        let mut result = {
            [
                const_panic::PanicVal::write_str("BasisElement("),
                // 1: Sign
                const_panic::PanicVal::write_str(""),
                // 2-18: BasisSignature
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                // 19-21: display name
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(""),
                const_panic::PanicVal::write_str(")"),
            ]
        };
        let sign = self.coefficient;
        if sign == 0 {
            result[1] = const_panic::PanicVal::write_str("0");
        }
        if sign == -1 {
            result[1] = const_panic::PanicVal::write_str("-");
        }
        let sig = BasisSignature::to_panicvals(self.signature, fmt);
        let mut s = 0;
        while s < sig.len() {
            result[2 + s] = sig[s];
            s += 1;
        }
        if let ConstOption::Some(dn) = self.display_name {
            result[19] = const_panic::PanicVal::write_str(" displayed_as ");
            let mut neg = false;
            neg |= (sign == -1) && !dn.negate_display;
            neg |= (sign == 1) && dn.negate_display;
            if neg {
                result[20] = const_panic::PanicVal::write_str("-");
            }
            if sign == 0 {
                result[21] = const_panic::PanicVal::write_str("0");
            } else {
                result[21] = const_panic::PanicVal::write_short_str(ShortString::new(dn.display_name.as_str()));
            }
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PanicFmt)]
pub struct BasisElementDisplayName {
    display_name: fstr<20>,
    negate_display: bool,
}
impl UnsizedConstParamTy for BasisElementDisplayName {}
impl ConstParamTy_ for BasisElementDisplayName {}

impl PartialOrd for BasisElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(BasisElement::const_cmp(self, other))
    }
}
impl Ord for BasisElement {
    fn cmp(&self, other: &Self) -> Ordering {
        BasisElement::const_cmp(self, other)
    }
}
impl Debug for BasisElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BasisElement(")?;
        let sign = self.coefficient;
        match sign {
            0 => write!(f, "0"),
            1 => write!(f, "{}", self.signature),
            -1 => write!(f, "-{}", self.signature),
            c => write!(f, "{}*{}", c, self.signature),
        }?;
        let n = &self.display_name;
        write!(f, ", {n:?})")
    }
}
impl Display for BasisElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sign = self.coefficient;
        if let ConstOption::Some(dn) = self.display_name {
            if dn.negate_display {
                sign = sign * -1;
            }
            return match sign {
                0 => write!(f, "0"),
                1 => write!(f, "{}", dn.display_name),
                -1 => write!(f, "-{}", dn.display_name),
                c => write!(f, "{}*{}", c, dn.display_name),
            };
        }
        match sign {
            0 => write!(f, "0"),
            1 => write!(f, "{}", self.signature),
            -1 => write!(f, "-{}", self.signature),
            c => write!(f, "{}*{}", c, self.signature),
        }
    }
}
impl Default for BasisElement {
    fn default() -> Self {
        Self {
            coefficient: 0,
            signature: BasisSignature::scalar,
            display_name: ConstOption::None,
        }
    }
}

impl From<BasisSignature> for BasisElement {
    fn from(signature: BasisSignature) -> Self {
        Self {
            coefficient: 1,
            signature,
            display_name: ConstOption::None,
        }
    }
}

impl BasisElement {
    const fn const_cmp(&self, other: &Self) -> Ordering {
        match BasisSignature::const_cmp(&self.signature, &other.signature) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => {}
        }
        let ac = self.coefficient;
        let bc = other.coefficient;
        if ac < bc {
            return Ordering::Less;
        }
        if ac > bc {
            return Ordering::Greater;
        }
        Ordering::Equal
    }

    pub const fn coefficient(&self) -> i8 {
        self.coefficient
    }

    pub(crate) fn set_coefficient(mut self, c: i8) -> Self {
        self.coefficient = c;
        self
    }

    pub const fn signature(&self) -> BasisSignature {
        self.signature
    }

    pub const fn zero() -> Self {
        Self {
            coefficient: 0,
            signature: BasisSignature::scalar,
            display_name: ConstOption::None,
        }
    }

    pub const fn scalar() -> Self {
        Self {
            coefficient: 1,
            signature: BasisSignature::scalar,
            display_name: ConstOption::None,
        }
    }

    pub const fn negate(mut self) -> Self {
        self.coefficient *= -1;
        self
    }

    pub const fn grade(&self) -> u32 {
        self.signature.bits().count_ones()
    }

    pub const fn anti_grade(&self, anti_scalar: BasisElement) -> u32 {
        anti_scalar.grade() - self.grade()
    }
    pub const fn grades(&self) -> Grades {
        Grades::from_sig(self.signature)
    }
    pub const fn anti_grades(&self, anti_scalar: BasisElement) -> AntiGrades {
        AntiGrades::from_sig(BasisSignature(anti_scalar.signature.0 - self.signature.0))
    }

    pub const fn parsed_display_name(s: &'static str) -> Option<Self> {
        let mut result = BasisElement::zero();
        result.coefficient = 1;
        let mut display_name = BasisElementDisplayName {
            display_name: fstr::<20>::from_str(s)
                .expect("BasisElement name must not be too long"),
            negate_display: false,
        };

        // This might not look very idiomatic at first glance,
        // but keep in mind we are operating in a const fn.

        let s = s.as_bytes();
        // if s == b"zero" {
        //     display_name.display_name = "zero";
        //     result.display_name = Some(display_name);
        //     result.coefficient = 0;
        //     return Some(result)
        // }
        // if s == b"scalar" {
        //     display_name.display_name = "scalar";
        //     result.display_name = Some(display_name);
        //     result.coefficient = 1;
        //     return Some(result)
        // }
        let mut i = 0;
        let mut reached_elements = false;
        while i < s.len() {
            let c = s[i];
            i += 1;
            let next_basis: u16 = match (reached_elements, c) {
                (false, b'-') => {
                    // It might seem difficult or confusing to arbitrate if this
                    // negative sign should negate the representation or the display name,
                    // but at the time of writing (getting CGA geometric product tests to run)
                    // it is convenient to parse product results like -e31 or whatever,
                    // and in that case we need the leading negative sign to negate the
                    // representation and not the display, as shown below. If we ever get a
                    // compelling case/motivation to interpret it another way, then be sure
                    // to run and resolve tests in order to complete an accurate refactor.

                    // display_name.negate_display = !display_name.negate_display;
                    result.coefficient *= -1;
                    continue;
                }
                (false, b'0') => {
                    result.coefficient = 0;
                    result.signature = BasisSignature::empty();
                    // For parsing here, it would be weird for someone to specify stuff
                    // after a 0, like "0e123". So if you want to be weird, do that
                    // with a manually set display_name instead of me figure out
                    // strange cases in const evaluation.
                    return if i == s.len() {
                        result.display_name = ConstOption::Some(display_name);
                        Some(result)
                    } else {
                        None
                    };
                }
                (false, b'1') => {
                    result.signature = BasisSignature::empty();
                    return if i == s.len() {
                        result.display_name = ConstOption::Some(display_name);
                        Some(result)
                    } else {
                        None
                    };
                }
                (false, b'e') => {
                    reached_elements = true;
                    continue;
                }
                (true, b'0') => 0x0,
                (true, b'1') => 0x1,
                (true, b'2') => 0x2,
                (true, b'3') => 0x3,
                (true, b'4') => 0x4,
                (true, b'5') => 0x5,
                (true, b'6') => 0x6,
                (true, b'7') => 0x7,
                (true, b'8') => 0x8,
                (true, b'9') => 0x9,
                (true, b'A') => 0xA,
                (true, b'B') => 0xB,
                (true, b'C') => 0xC,
                (true, b'D') => 0xD,
                (true, b'E') => 0xE,
                (true, b'F') => 0xF,
                _ => return None,
            };
            let sig_addition = BasisSignature::from_bits_retain(1u16 << next_basis);
            let sig_existing = result.signature;

            // Reject double bases.
            if sig_existing.contains(sig_addition) {
                return None;
            }
            // Alright so the slot is open for this primary element.
            // Now the only question is how many primary elements we have to swap to be in order.
            let sig_behind_this_basis = BasisSignature::from_bits_retain(u16::MAX - (sig_addition.bits() - 1));
            let yes_swap = sig_existing.intersection(sig_behind_this_basis);
            let exp = yes_swap.bits().count_ones();
            let negate = -1 == i8::pow(-1, exp);
            if negate {
                result.coefficient = result.coefficient * -1;
                display_name.negate_display = !display_name.negate_display;
            }
            // Alright and finally actually update the signature
            result.signature = sig_existing.union(sig_addition);
        }
        result.display_name = ConstOption::Some(display_name);
        Some(result)
    }

    pub const fn with_name(mut self, display_name: &'static str, odd_permutation: bool) -> Self {
        let dn = BasisElementDisplayName {
            display_name: fstr::<20>::from_str(display_name)
                .expect("BasisElement name must not be too long"),
            negate_display: odd_permutation,
        };
        self.display_name = ConstOption::Some(dn);
        self
    }

    pub const fn anon(mut self) -> Self {
        self.display_name = ConstOption::None;
        self
    }
    pub const fn is_anon(&self) -> bool {
        self.display_name.is_none()
    }

    pub const fn reverse(&self) -> Self {
        let gr = self.grade();
        if gr == 0 {
            return *self;
        }
        let exp = gr * (gr - 1) / 2;
        let mut copy = *self;
        copy.coefficient = i8::pow(-1i8, exp) * copy.coefficient;
        copy
    }

    pub const fn anti_reverse(&self, anti_scalar: BasisElement) -> Self {
        let r = self.right_complement(anti_scalar);
        let r = r.reverse();
        r.left_complement(anti_scalar)
    }

    pub const fn right_complement(&self, anti_scalar: BasisElement) -> BasisElement {
        if !anti_scalar.signature.contains(self.signature) {
            panic!(
                "Cannot take the right complement of a BasisElement with respect to an \
                AntiScalar that does not contain it."
            )
        }
        if self.coefficient == 0 {
            return BasisElement::zero();
        }

        let new_sig = anti_scalar.signature.bits() - self.signature.bits();

        // Negative coefficient anti_scalar is allowed
        let mut answer = BasisElement::scalar();
        answer.coefficient = self.coefficient * anti_scalar.coefficient;
        answer.signature = BasisSignature::from_bits_retain(new_sig);

        // The following gr and ag calculation for sign doesn't work for negative anti_scalars
        // So let's just do the tested approach

        // // Now we can test if we have to actually worry about the sign.
        // let d = anti_scalar.grade();
        // let gr = self.grade();
        // let ag = d - gr;
        //
        // // Hmmm it is so inconvenient to define these,
        // // maybe I should get them as a dependency instead
        // const fn is_even(num: u8) -> bool { num % 2 == 0 }
        // const fn is_odd(num: u8) -> bool { num % 2 != 0 }
        //
        // if is_odd(d) || is_even(gr) || is_even(ag) {
        //     return answer;
        // }

        // Okay we should double-check the sign.
        let test = self.wedge(answer);
        if test.coefficient == anti_scalar.coefficient {
            return answer;
        }
        if test.coefficient == -anti_scalar.coefficient {
            answer.coefficient = -1 * answer.coefficient;
            return answer;
        }

        // This basically shouldn't happen unless the i8 coefficient somehow gets corrupted
        // panic!("Cannot figure out right_complement for strange element: {self:?} anti_scalar: {anti_scalar:?}")

        // Limited/no formatting options in const eval
        panic!("Cannot figure out right_complement for strange element")
    }

    pub const fn left_complement(&self, anti_scalar: BasisElement) -> BasisElement {
        if self.coefficient == 0 {
            return BasisElement::zero();
        }
        let mut rc = self.right_complement(anti_scalar);

        // let d = anti_scalar.grade();
        // let gr = self.grade();
        // let ag = d - gr;
        // let exp = gr * ag;
        // rc.coefficient = rc.coefficient * i8::pow(-1, exp as u32);

        // Okay we should double-check the sign.
        let test = rc.wedge(*self);
        if test.coefficient == anti_scalar.coefficient {
            return rc;
        }
        if test.coefficient == -anti_scalar.coefficient {
            rc.coefficient = -1 * rc.coefficient;
            return rc;
        }

        // This basically shouldn't happen unless the i8 coefficient somehow gets corrupted
        // panic!("Cannot figure out right_complement for strange element: {self:?} anti_scalar: {anti_scalar:?}")

        // Limited/no formatting options in const eval
        panic!("Cannot figure out left_complement for strange element")
    }

    /// Wedge product
    pub const fn wedge(&self, other: BasisElement) -> BasisElement {
        // Implementation may look a bit strange because it is const compatible

        let (a_len, a) = self.signature.into_grade_1_signatures_const();
        let (b_len, b) = other.signature.into_grade_1_signatures_const();
        let mut sign = self.coefficient * other.coefficient;

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
                Ordering::Equal => return BasisElement::zero(),
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

    /// AntiWedge product
    pub const fn anti_wedge(&self, other: BasisElement, anti_scalar: BasisElement) -> BasisElement {
        let s = self.right_complement(anti_scalar);
        let o = other.right_complement(anti_scalar);
        let w = s.wedge(o);
        w.left_complement(anti_scalar)
    }

    pub const fn is_wedge_on(&self, other: BasisElement) -> bool {
        self.signature.contains(other.signature)
    }

    pub const fn const_from(sig: BasisSignature) -> Self {
        BasisElement {
            coefficient: 1,
            signature: sig,
            display_name: ConstOption::None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BasisElementNames {
    zero: Option<BasisElementDisplayName>,
    elements: HashMap<BasisSignature, BasisElementDisplayName>,
}
impl BasisElementNames {
    pub fn new() -> Self {
        BasisElementNames {
            zero: None,
            elements: HashMap::new(),
        }
    }

    pub fn contaminate(a: &mut BasisElement, b: &mut BasisElement) {
        if a.signature != b.signature {
            return;
        }
        match (a.display_name, b.display_name) {
            (ConstOption::None, ConstOption::None) => {}
            (ConstOption::Some(an), ConstOption::None) => b.display_name = ConstOption::Some(an.clone()),
            (ConstOption::None, ConstOption::Some(bn)) => a.display_name = ConstOption::Some(bn.clone()),
            (ConstOption::Some(an), ConstOption::Some(bn)) => {
                if an != bn {
                    panic!("Conflicting display names found for {a:?} and {b:?}")
                }
            }
        }
    }

    /// This check will return true if you should attempt to accept_name ("should" including the
    /// case of revealing a conflict in a Result::Err). This will return false if there is no reason
    /// to invoke accept_name (no name on the BasisElement, or it is already the same as what we
    /// have).
    pub fn may_accept(&self, el: BasisElement) -> bool {
        let a = match &el.display_name {
            ConstOption::None => return false,
            ConstOption::Some(a) => a,
        };
        let Some(b) = self.elements.get(&el.signature) else {
            return true;
        };
        // This case will result in Result::Err, but we want that revealed
        a != b
    }

    /// Give a name to a BasisElement, if one exists.
    pub fn provide_name(&self, mut el: BasisElement) -> BasisElement {
        let existing = if el.coefficient == 0 { self.zero } else { self.elements.get(&el.signature).cloned() };
        if let Some(dn) = existing {
            el.display_name = ConstOption::Some(dn);
        }
        return el;
    }

    /// Give a name and sign to a BasisElement, if the direction of your BasisElement is not of
    /// critical relied upon yet. In other words, this will give the positive direction of any
    /// BasisElement, according to the name's preference as an odd or even permutation.
    pub fn provide_name_and_sign(&self, mut el: BasisElement) -> BasisElement {
        let existing = if el.coefficient == 0 { self.zero } else { self.elements.get(&el.signature).cloned() };
        el.coefficient = 1;
        if let Some(dn) = existing {
            el.display_name = ConstOption::Some(dn);
            if dn.negate_display {
                el.coefficient = -1;
            }
        }
        return el;
    }

    /// Add a name on a BasisElement (if it exists) to the BasisElementNames.
    pub fn accept_name(&mut self, el: BasisElement) -> anyhow::Result<()> {
        let ConstOption::Some(el_dn) = el.display_name else { return Ok(()) };
        let sig = el.signature;
        let existing = if el.coefficient == 0 { self.zero } else { self.elements.get(&sig).cloned() };
        if let Some(dn) = existing {
            if el_dn != dn {
                anyhow::bail!("BasisElementNames cannot accept name {el_dn:?} because it already has {dn:?} for the same signature {sig:?}")
            }
        } else {
            self.elements.insert(sig, el_dn);
        }
        Ok(())
    }
}


#[test]
fn test_metric() {
    use elements::*;
    let rga = generator_squares!(1 => e1, e2, e3; 0 => e4);

    // NOTE that this is CGA without substitute bases, so it has a diagonal metric
    // instead of the metric shown on page 185.
    let cga = generator_squares!(1 => e1, e2, e3, e4; -1 => e5);

    let zero = BasisElement::zero().anon();

    assert_eq!(scalar, rga.apply_metric(scalar));

    assert_eq!(e1, rga.apply_metric(e1));
    assert_eq!(e2, rga.apply_metric(e2));
    assert_eq!(e3, rga.apply_metric(e3));
    assert_eq!(zero, rga.apply_metric(e4));

    assert_eq!(e12, rga.apply_metric(e12));
    assert_eq!(e23, rga.apply_metric(e23));
    assert_eq!(e31.anon(), rga.apply_metric(e31));
    assert_eq!(zero, rga.apply_metric(e41));
    assert_eq!(zero, rga.apply_metric(e42));
    assert_eq!(zero, rga.apply_metric(e43));

    assert_eq!(e123, rga.apply_metric(e123));
    assert_eq!(zero, rga.apply_metric(e412));
    assert_eq!(zero, rga.apply_metric(e423));
    assert_eq!(zero, rga.apply_metric(e431));

    assert_eq!(zero, rga.apply_metric(e1234));

    //

    assert_eq!(scalar, cga.apply_metric(scalar));

    assert_eq!(e1, cga.apply_metric(e1));
    assert_eq!(e2, cga.apply_metric(e2));
    assert_eq!(e3, cga.apply_metric(e3));
    assert_eq!(e4, cga.apply_metric(e4));
    assert_eq!(-e5, cga.apply_metric(e5));

    assert_eq!(e12, cga.apply_metric(e12));
    assert_eq!(e23, cga.apply_metric(e23));
    assert_eq!(e31.anon(), cga.apply_metric(e31));
    assert_eq!(e41.anon(), cga.apply_metric(e41));
    assert_eq!(e42.anon(), cga.apply_metric(e42));
    assert_eq!(e43.anon(), cga.apply_metric(e43));
    assert_eq!(-e15, cga.apply_metric(e15));
    assert_eq!(-e25, cga.apply_metric(e25));
    assert_eq!(-e35, cga.apply_metric(e35));
    assert_eq!(-e45, cga.apply_metric(e45));

    assert_eq!(e123, cga.apply_metric(e123));
    assert_eq!(e412.anon(), cga.apply_metric(e412));
    assert_eq!(e423.anon(), cga.apply_metric(e423));
    assert_eq!(e431.anon(), cga.apply_metric(e431));
    assert_eq!(-e125, cga.apply_metric(e125));
    assert_eq!(-e235, cga.apply_metric(e235));
    assert_eq!(-e315.anon(), cga.apply_metric(e315));
    assert_eq!(-e415.anon(), cga.apply_metric(e415));
    assert_eq!(-e425.anon(), cga.apply_metric(e425));
    assert_eq!(-e435.anon(), cga.apply_metric(e435));

    assert_eq!(e1234, cga.apply_metric(e1234));
    assert_eq!(-e1235, cga.apply_metric(e1235));
    assert_eq!(-e1245, cga.apply_metric(e1245));
    assert_eq!(-e1345, cga.apply_metric(e1345));
    assert_eq!(-e2345, cga.apply_metric(e2345));

    assert_eq!(-e12345, cga.apply_metric(e12345));
}

#[allow(non_upper_case_globals, dead_code)]
pub(crate) mod elements {
    use crate::algebra::basis::*;

    // List a bunch of generated elements

    include!(concat!(env!("OUT_DIR"), "/generated_elements.rs"));

    // And we'll add some custom bases as seen in the wild.
    // (But let's not go overboard by using code generation on this part...
    //  The number of permutations is the factorial of the number of dimensions.)

    const fn const_parse(s: &'static str) -> BasisElement {
        BasisElement::parsed_display_name(s).expect("Failed to parse const BasisElement")
    }

    // Eric Lengyel's bases:
    pub const e41: BasisElement = const_parse("e41");
    pub const e42: BasisElement = const_parse("e42");
    pub const e43: BasisElement = const_parse("e43");
    pub const e31: BasisElement = const_parse("e31");
    pub const e423: BasisElement = const_parse("e423");
    pub const e431: BasisElement = const_parse("e431");
    pub const e412: BasisElement = const_parse("e412");
    pub const e321: BasisElement = const_parse("e321");
    pub const e415: BasisElement = const_parse("e415");
    pub const e425: BasisElement = const_parse("e425");
    pub const e435: BasisElement = const_parse("e435");
    pub const e315: BasisElement = const_parse("e315");
    pub const e4235: BasisElement = const_parse("e4235");
    pub const e4315: BasisElement = const_parse("e4315");
    pub const e4125: BasisElement = const_parse("e4125");
    pub const e3215: BasisElement = const_parse("e3215");

    // biVector.net pga
    pub const e021: BasisElement = const_parse("e021");
    pub const e032: BasisElement = const_parse("e032");

    // There is room for people to be weird and make these
    // generators square to unconventional values, but that
    // risk of breaking semantics is worth providing the
    // convenience of these already being defined.
    pub const e_inf: BasisElement = eD.with_name("e_inf", false);
    pub const e_plus: BasisElement = eE.with_name("e_plus", false);
    pub const e_minus: BasisElement = eF.with_name("e_minus", false);
    pub const eI: BasisElement = eD.with_name("eI", false);
    pub const eP: BasisElement = eE.with_name("eP", false);
    pub const eM: BasisElement = eF.with_name("eM", false);

    // And so for that matter... Let's provide some wedges for the common dimensions.

    // e_inf wedges

    pub const e0I: BasisElement = const_parse("e0D").with_name("e0I", false);
    pub const e1I: BasisElement = const_parse("e1D").with_name("e1I", false);
    pub const e2I: BasisElement = const_parse("e2D").with_name("e2I", false);
    pub const e3I: BasisElement = const_parse("e3D").with_name("e3I", false);

    pub const e01I: BasisElement = const_parse("e01D").with_name("e01I", false);
    pub const e02I: BasisElement = const_parse("e02D").with_name("e02I", false);
    pub const e03I: BasisElement = const_parse("e03D").with_name("e03I", false);
    pub const e12I: BasisElement = const_parse("e12D").with_name("e12I", false);
    pub const e13I: BasisElement = const_parse("e13D").with_name("e13I", false);
    pub const e23I: BasisElement = const_parse("e23D").with_name("e23I", false);

    pub const e123I: BasisElement = const_parse("e123D").with_name("e123I", false);
    pub const e023I: BasisElement = const_parse("e023D").with_name("e023I", false);
    pub const e013I: BasisElement = const_parse("e013D").with_name("e013I", false);
    pub const e012I: BasisElement = const_parse("e012D").with_name("e012I", false);

    pub const e0123I: BasisElement = const_parse("e0123D").with_name("e0123I", false);

    // e_plus

    pub const e0P: BasisElement = const_parse("e0E").with_name("e0P", false);
    pub const e1P: BasisElement = const_parse("e1E").with_name("e1P", false);
    pub const e2P: BasisElement = const_parse("e2E").with_name("e2P", false);
    pub const e3P: BasisElement = const_parse("e3E").with_name("e3P", false);

    pub const e01P: BasisElement = const_parse("e01E").with_name("e01P", false);
    pub const e02P: BasisElement = const_parse("e02E").with_name("e02P", false);
    pub const e03P: BasisElement = const_parse("e03E").with_name("e03P", false);
    pub const e12P: BasisElement = const_parse("e12E").with_name("e12P", false);
    pub const e13P: BasisElement = const_parse("e13E").with_name("e13P", false);
    pub const e23P: BasisElement = const_parse("e23E").with_name("e23P", false);

    pub const e123P: BasisElement = const_parse("e123E").with_name("e123P", false);
    pub const e023P: BasisElement = const_parse("e023E").with_name("e023P", false);
    pub const e013P: BasisElement = const_parse("e013E").with_name("e013P", false);
    pub const e012P: BasisElement = const_parse("e012E").with_name("e012P", false);

    pub const e0123P: BasisElement = const_parse("e0123E").with_name("e0123P", false);

    // e_minus

    pub const e0M: BasisElement = const_parse("e0F").with_name("e0M", false);
    pub const e1M: BasisElement = const_parse("e1F").with_name("e1M", false);
    pub const e2M: BasisElement = const_parse("e2F").with_name("e2M", false);
    pub const e3M: BasisElement = const_parse("e3F").with_name("e3M", false);

    pub const e01M: BasisElement = const_parse("e01F").with_name("e01M", false);
    pub const e02M: BasisElement = const_parse("e02F").with_name("e02M", false);
    pub const e03M: BasisElement = const_parse("e03F").with_name("e03M", false);
    pub const e12M: BasisElement = const_parse("e12F").with_name("e12M", false);
    pub const e13M: BasisElement = const_parse("e13F").with_name("e13M", false);
    pub const e23M: BasisElement = const_parse("e23F").with_name("e23M", false);

    pub const e123M: BasisElement = const_parse("e123F").with_name("e123M", false);
    pub const e023M: BasisElement = const_parse("e023F").with_name("e023M", false);
    pub const e013M: BasisElement = const_parse("e013F").with_name("e013M", false);
    pub const e012M: BasisElement = const_parse("e012F").with_name("e012M", false);

    pub const e0123M: BasisElement = const_parse("e0123F").with_name("e0123M", false);

    // e_plus and e_minus

    pub const ePM: BasisElement = const_parse("eEF").with_name("ePM", false);

    pub const e0PM: BasisElement = const_parse("e0EF").with_name("e0PM", false);
    pub const e1PM: BasisElement = const_parse("e1EF").with_name("e1PM", false);
    pub const e2PM: BasisElement = const_parse("e2EF").with_name("e2PM", false);
    pub const e3PM: BasisElement = const_parse("e3EF").with_name("e3PM", false);

    pub const e01PM: BasisElement = const_parse("e01EF").with_name("e01PM", false);
    pub const e02PM: BasisElement = const_parse("e02EF").with_name("e02PM", false);
    pub const e03PM: BasisElement = const_parse("e03EF").with_name("e03PM", false);
    pub const e12PM: BasisElement = const_parse("e12EF").with_name("e12PM", false);
    pub const e13PM: BasisElement = const_parse("e13EF").with_name("e13PM", false);
    pub const e23PM: BasisElement = const_parse("e23EF").with_name("e23PM", false);

    pub const e123PM: BasisElement = const_parse("e123EF").with_name("e123PM", false);
    pub const e023PM: BasisElement = const_parse("e023EF").with_name("e023PM", false);
    pub const e013PM: BasisElement = const_parse("e013EF").with_name("e013PM", false);
    pub const e012PM: BasisElement = const_parse("e012EF").with_name("e012PM", false);

    pub const e0123PM: BasisElement = const_parse("e0123EF").with_name("e0123PM", false);

    #[test]
    fn test_parse_custom_bases() {
        use std::fmt::Write;
        let cases = [
            (e41, "e41", true, e14.negate()),
            (e42, "e42", true, e24.negate()),
            (e43, "e43", true, e34.negate()),
            (e31, "e31", true, e13.negate()),
            (e423, "e423", false, e234),
            (e431, "e431", true, e134.negate()),
            (e412, "e412", false, e124),
            (e321, "e321", true, e123.negate()),
            (e415, "e415", true, e145.negate()),
            (e425, "e425", true, e245.negate()),
            (e435, "e435", true, e345.negate()),
            (e315, "e315", true, e135.negate()),
            (e4235, "e4235", false, e2345),
            (e4315, "e4315", true, e1345.negate()),
            (e4125, "e4125", false, e1245),
            (e3215, "e3215", true, e1235.negate()),
        ];
        for (custom_element, correct_name, display_is_negated, ordered_element) in cases {
            assert_eq!(
                custom_element.signature(),
                ordered_element.signature(),
                "Custom BasisElement {custom_element:?} does not match signature of {ordered_element:?}"
            );
            assert_eq!(
                custom_element.coefficient(),
                ordered_element.coefficient(),
                "Custom BasisElement {custom_element:?} does not match coefficient of {ordered_element:?}"
            );
            let dn = custom_element.display_name.expect("Parsed BasisElements should have custom names");
            assert_eq!(dn.negate_display, display_is_negated, "Custom BasisElement {custom_element:?} has incorrect display negation");
            let mut n = String::new();
            write!(n, "{custom_element}").expect("BasisElements must implement Display without fail");
            assert_eq!(n.as_str(), correct_name, "Custom BasisElement {custom_element:?} does not display to \"{correct_name}\"");

            // Negated display
            let mut custom_element = custom_element;
            custom_element.coefficient = -1 * custom_element.coefficient;
            let correct_name = format!("-{correct_name}");
            let mut n = String::new();
            write!(n, "{custom_element}").expect("BasisElements must implement Display without fail");
            assert_eq!(n, correct_name, "Custom BasisElement {custom_element:?} does not display to \"{correct_name}\"");
        }
    }
}
