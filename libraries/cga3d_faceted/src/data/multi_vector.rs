use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MultiVector
#[repr(C)]
#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// scalar, e12345, 0, 0, e1, e2, e3, e4, e5, 0, 0, 0, e41, e42, e43, e45, e15, e25, e35, 0, e23, e31, e12, 0, e415, e425, e435, e321, e423, e431, e412, 0, e235, e315, e125, 0, e1234, e4235, e4315, e4125, e3215, 0, 0, 0
    elements: [f32; 44],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MultiVectorGroups {
    /// scalar, e12345
    g0: Simd32x2,
    /// e1, e2, e3, e4
    g1: Simd32x4,
    /// e5
    g2: f32,
    /// e41, e42, e43, e45
    g3: Simd32x4,
    /// e15, e25, e35
    g4: Simd32x3,
    /// e23, e31, e12
    g5: Simd32x3,
    /// e415, e425, e435, e321
    g6: Simd32x4,
    /// e423, e431, e412
    g7: Simd32x3,
    /// e235, e315, e125
    g8: Simd32x3,
    /// e1234, e4235, e4315, e4125
    g9: Simd32x4,
    /// e3215
    g10: f32,
}
impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        scalar: f32,
        e12345: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e4: f32,
        e5: f32,
        e41: f32,
        e42: f32,
        e43: f32,
        e45: f32,
        e15: f32,
        e25: f32,
        e35: f32,
        e23: f32,
        e31: f32,
        e12: f32,
        e415: f32,
        e425: f32,
        e435: f32,
        e321: f32,
        e423: f32,
        e431: f32,
        e412: f32,
        e235: f32,
        e315: f32,
        e125: f32,
        e1234: f32,
        e4235: f32,
        e4315: f32,
        e4125: f32,
        e3215: f32,
    ) -> Self {
        Self {
            elements: [
                scalar, e12345, 0.0, 0.0, e1, e2, e3, e4, e5, 0.0, 0.0, 0.0, e41, e42, e43, e45, e15, e25, e35, 0.0, e23, e31, e12, 0.0, e415, e425, e435, e321, e423, e431, e412,
                0.0, e235, e315, e125, 0.0, e1234, e4235, e4315, e4125, e3215, 0.0, 0.0, 0.0,
            ],
        }
    }
    pub const fn from_groups(
        g0: Simd32x2,
        g1: Simd32x4,
        g2: f32,
        g3: Simd32x4,
        g4: Simd32x3,
        g5: Simd32x3,
        g6: Simd32x4,
        g7: Simd32x3,
        g8: Simd32x3,
        g9: Simd32x4,
        g10: f32,
    ) -> Self {
        Self {
            groups: MultiVectorGroups {
                g0,
                g1,
                g2,
                g3,
                g4,
                g5,
                g6,
                g7,
                g8,
                g9,
                g10,
            },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x2 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x2 {
        unsafe { &mut self.groups.g0 }
    }
    #[inline(always)]
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
    #[inline(always)]
    pub fn group2(&self) -> f32 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g3 }
    }
    #[inline(always)]
    pub fn group4(&self) -> Simd32x3 {
        unsafe { self.groups.g4 }
    }
    #[inline(always)]
    pub fn group4_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g4 }
    }
    #[inline(always)]
    pub fn group5(&self) -> Simd32x3 {
        unsafe { self.groups.g5 }
    }
    #[inline(always)]
    pub fn group5_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g5 }
    }
    #[inline(always)]
    pub fn group6(&self) -> Simd32x4 {
        unsafe { self.groups.g6 }
    }
    #[inline(always)]
    pub fn group6_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g6 }
    }
    #[inline(always)]
    pub fn group7(&self) -> Simd32x3 {
        unsafe { self.groups.g7 }
    }
    #[inline(always)]
    pub fn group7_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g7 }
    }
    #[inline(always)]
    pub fn group8(&self) -> Simd32x3 {
        unsafe { self.groups.g8 }
    }
    #[inline(always)]
    pub fn group8_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g8 }
    }
    #[inline(always)]
    pub fn group9(&self) -> Simd32x4 {
        unsafe { self.groups.g9 }
    }
    #[inline(always)]
    pub fn group9_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g9 }
    }
    #[inline(always)]
    pub fn group10(&self) -> f32 {
        unsafe { self.groups.g10 }
    }
    #[inline(always)]
    pub fn group10_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g10 }
    }
}
const MULTI_VECTOR_INDEX_REMAP: [usize; 32] = [0, 1, 4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 36, 37, 38, 39, 40];
impl std::ops::Index<usize> for MultiVector {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MULTI_VECTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MultiVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MULTI_VECTOR_INDEX_REMAP[index]] }
    }
}
impl From<MultiVector> for [f32; 32] {
    fn from(vector: MultiVector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[12],
                vector.elements[13], vector.elements[14], vector.elements[15], vector.elements[16], vector.elements[17], vector.elements[18], vector.elements[20],
                vector.elements[21], vector.elements[22], vector.elements[24], vector.elements[25], vector.elements[26], vector.elements[27], vector.elements[28],
                vector.elements[29], vector.elements[30], vector.elements[32], vector.elements[33], vector.elements[34], vector.elements[36], vector.elements[37],
                vector.elements[38], vector.elements[39], vector.elements[40],
            ]
        }
    }
}
impl From<[f32; 32]> for MultiVector {
    fn from(array: [f32; 32]) -> Self {
        Self {
            elements: [
                array[0], array[1], 0.0, 0.0, array[1], array[2], array[3], array[4], array[2], 0.0, 0.0, 0.0, array[3], array[4], array[5], array[6], array[4], array[5],
                array[6], 0.0, array[5], array[6], array[7], 0.0, array[6], array[7], array[8], array[9], array[7], array[8], array[9], 0.0, array[8], array[9], array[10], 0.0,
                array[9], array[10], array[11], array[12], array[10], 0.0, 0.0, 0.0,
            ],
        }
    }
}
impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("scalar", &self[0])
            .field("e12345", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e3", &self[4])
            .field("e4", &self[5])
            .field("e5", &self[6])
            .field("e41", &self[7])
            .field("e42", &self[8])
            .field("e43", &self[9])
            .field("e45", &self[10])
            .field("e15", &self[11])
            .field("e25", &self[12])
            .field("e35", &self[13])
            .field("e23", &self[14])
            .field("e31", &self[15])
            .field("e12", &self[16])
            .field("e415", &self[17])
            .field("e425", &self[18])
            .field("e435", &self[19])
            .field("e321", &self[20])
            .field("e423", &self[21])
            .field("e431", &self[22])
            .field("e412", &self[23])
            .field("e235", &self[24])
            .field("e315", &self[25])
            .field("e125", &self[26])
            .field("e1234", &self[27])
            .field("e4235", &self[28])
            .field("e4315", &self[29])
            .field("e4125", &self[30])
            .field("e3215", &self[31])
            .finish()
    }
}

impl MultiVector {
    pub const LEN: usize = 32;
}

impl nearly::NearlyEqEps<MultiVector, f32, f32> for MultiVector {
    fn nearly_eq_eps(&self, other: &MultiVector, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_ne_eps(a, b, eps) {
                return false;
            }
            i += 1;
        }
        true
    }
}
impl nearly::NearlyEqUlps<MultiVector, f32, f32> for MultiVector {
    fn nearly_eq_ulps(&self, other: &MultiVector, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_ne_ulps(a, b, ulps) {
                return false;
            }
            i += 1;
        }
        true
    }
}
impl nearly::NearlyEqTol<MultiVector, f32, f32> for MultiVector {}
impl nearly::NearlyEq<MultiVector, f32, f32> for MultiVector {}
impl nearly::NearlyOrdUlps<MultiVector, f32, f32> for MultiVector {
    fn nearly_lt_ulps(&self, other: &MultiVector, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }

    fn nearly_gt_ulps(&self, other: &MultiVector, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }
}
impl nearly::NearlyOrdEps<MultiVector, f32, f32> for MultiVector {
    fn nearly_lt_eps(&self, other: &MultiVector, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }

    fn nearly_gt_eps(&self, other: &MultiVector, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }
}
impl nearly::NearlyOrdTol<MultiVector, f32, f32> for MultiVector {}
impl nearly::NearlyOrd<MultiVector, f32, f32> for MultiVector {}

impl MultiVector {
    pub fn clamp_zeros(mut self, tolerance: nearly::Tolerance<f32>) -> Self {
        for i in 0..Self::LEN {
            let f = self[i];
            if nearly::nearly!(0.0 == f, tol = tolerance) {
                self[i] = 0.0;
            }
        }
        self
    }
}

impl PartialOrd for MultiVector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return Some(result),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}
impl Ord for MultiVector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return result,
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl PartialEq for MultiVector {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            if a != b {
                return false;
            }
        }
        true
    }
}
impl Eq for MultiVector {}
impl std::hash::Hash for MultiVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MultiVector {}
unsafe impl bytemuck::Pod for MultiVector {}
impl encase::ShaderType for MultiVector {
    type ExtraMetadata = <MultiVectorGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MultiVectorGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MultiVectorGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MultiVectorGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MultiVectorGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MultiVector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MultiVector", 32)?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.serialize_field("e12345", &self[crate::elements::e12345])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MultiVector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MultiVectorField {
            scalar,
            e12345,
            e1,
            e2,
            e3,
            e4,
            e5,
            e41,
            e42,
            e43,
            e45,
            e15,
            e25,
            e35,
            e23,
            e31,
            e12,
            e415,
            e425,
            e435,
            e321,
            e423,
            e431,
            e412,
            e235,
            e315,
            e125,
            e1234,
            e4235,
            e4315,
            e4125,
            e3215,
        }
        struct MultiVectorVisitor;
        impl<'de> Visitor<'de> for MultiVectorVisitor {
            type Value = MultiVector;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MultiVector")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MultiVector, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut scalar = None;
                let mut e12345 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e4 = None;
                let mut e5 = None;
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e45 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e1234 = None;
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;
                let mut e3215 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MultiVectorField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }

                        MultiVectorField::e12345 => {
                            if e12345.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12345"));
                            }
                            e12345 = Some(map.next_value()?);
                        }

                        MultiVectorField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        MultiVectorField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        MultiVectorField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }

                        MultiVectorField::e4 => {
                            if e4.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4"));
                            }
                            e4 = Some(map.next_value()?);
                        }

                        MultiVectorField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }

                        MultiVectorField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        MultiVectorField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        MultiVectorField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        MultiVectorField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }

                        MultiVectorField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        MultiVectorField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        MultiVectorField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        MultiVectorField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        MultiVectorField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        MultiVectorField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        MultiVectorField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        MultiVectorField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        MultiVectorField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        MultiVectorField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        MultiVectorField::e423 => {
                            if e423.is_some() {
                                return Err(serde::de::Error::duplicate_field("e423"));
                            }
                            e423 = Some(map.next_value()?);
                        }

                        MultiVectorField::e431 => {
                            if e431.is_some() {
                                return Err(serde::de::Error::duplicate_field("e431"));
                            }
                            e431 = Some(map.next_value()?);
                        }

                        MultiVectorField::e412 => {
                            if e412.is_some() {
                                return Err(serde::de::Error::duplicate_field("e412"));
                            }
                            e412 = Some(map.next_value()?);
                        }

                        MultiVectorField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        MultiVectorField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        MultiVectorField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        MultiVectorField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }

                        MultiVectorField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        MultiVectorField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        MultiVectorField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }

                        MultiVectorField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MultiVector::from([0.0; 32]);
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                result[crate::elements::e12345] = e12345.ok_or_else(|| serde::de::Error::missing_field("e12345"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "scalar", "e12345", "e1", "e2", "e3", "e4", "e5", "e41", "e42", "e43", "e45", "e15", "e25", "e35", "e23", "e31", "e12", "e415", "e425", "e435", "e321", "e423", "e431",
            "e412", "e235", "e315", "e125", "e1234", "e4235", "e4315", "e4125", "e3215",
        ];
        deserializer.deserialize_struct("MultiVector", FIELDS, MultiVectorVisitor)
    }
}
impl std::ops::Index<crate::elements::scalar> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e12345> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e1> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e2> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e3> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e4> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e5> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e41> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e42> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e43> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e45> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e15> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e25> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e35> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e23> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::Index<crate::elements::e31> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[15]
    }
}
impl std::ops::Index<crate::elements::e12> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[16]
    }
}
impl std::ops::Index<crate::elements::e415> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[17]
    }
}
impl std::ops::Index<crate::elements::e425> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[18]
    }
}
impl std::ops::Index<crate::elements::e435> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[19]
    }
}
impl std::ops::Index<crate::elements::e321> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[20]
    }
}
impl std::ops::Index<crate::elements::e423> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[21]
    }
}
impl std::ops::Index<crate::elements::e431> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[22]
    }
}
impl std::ops::Index<crate::elements::e412> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[23]
    }
}
impl std::ops::Index<crate::elements::e235> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[24]
    }
}
impl std::ops::Index<crate::elements::e315> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[25]
    }
}
impl std::ops::Index<crate::elements::e125> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[26]
    }
}
impl std::ops::Index<crate::elements::e1234> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[27]
    }
}
impl std::ops::Index<crate::elements::e4235> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[28]
    }
}
impl std::ops::Index<crate::elements::e4315> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[29]
    }
}
impl std::ops::Index<crate::elements::e4125> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[30]
    }
}
impl std::ops::Index<crate::elements::e3215> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[31]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[15]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[16]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[17]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[18]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[19]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[20]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[21]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[22]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[23]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[24]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[25]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[26]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[27]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[28]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[29]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[30]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[31]
    }
}
include!("./impls/multi_vector.rs");
