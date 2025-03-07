use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MultiVector
#[repr(C)]
#[derive(Clone, Copy)]
pub union MultiVector {
    groups: MultiVectorGroups,
    /// scalar, e1234, 0, 0, e1, e2, e3, e4, e41, e42, e43, 0, e23, e31, e12, 0, e423, e431, e412, e321
    elements: [f32; 20],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MultiVectorGroups {
    /// scalar, e1234
    g0: Simd32x2,
    /// e1, e2, e3, e4
    g1: Simd32x4,
    /// e41, e42, e43
    g2: Simd32x3,
    /// e23, e31, e12
    g3: Simd32x3,
    /// e423, e431, e412, e321
    g4: Simd32x4,
}
impl MultiVector {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        scalar: f32,
        e1234: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e4: f32,
        e41: f32,
        e42: f32,
        e43: f32,
        e23: f32,
        e31: f32,
        e12: f32,
        e423: f32,
        e431: f32,
        e412: f32,
        e321: f32,
    ) -> Self {
        Self {
            elements: [scalar, e1234, 0.0, 0.0, e1, e2, e3, e4, e41, e42, e43, 0.0, e23, e31, e12, 0.0, e423, e431, e412, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x2, g1: Simd32x4, g2: Simd32x3, g3: Simd32x3, g4: Simd32x4) -> Self {
        Self {
            groups: MultiVectorGroups { g0, g1, g2, g3, g4 },
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
    pub fn group2(&self) -> Simd32x3 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x3 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g3 }
    }
    #[inline(always)]
    pub fn group4(&self) -> Simd32x4 {
        unsafe { self.groups.g4 }
    }
    #[inline(always)]
    pub fn group4_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g4 }
    }
}
const MULTI_VECTOR_INDEX_REMAP: [usize; 16] = [0, 1, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 16, 17, 18, 19];
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
impl From<MultiVector> for [f32; 16] {
    fn from(vector: MultiVector) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8], vector.elements[9],
                vector.elements[10], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[16], vector.elements[17], vector.elements[18],
                vector.elements[19],
            ]
        }
    }
}
impl From<[f32; 16]> for MultiVector {
    fn from(array: [f32; 16]) -> Self {
        Self {
            elements: [
                array[0], array[1], 0.0, 0.0, array[1], array[2], array[3], array[4], array[2], array[3], array[4], 0.0, array[3], array[4], array[5], 0.0, array[4], array[5],
                array[6], array[7],
            ],
        }
    }
}
impl std::fmt::Debug for MultiVector {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MultiVector")
            .field("scalar", &self[0])
            .field("e1234", &self[1])
            .field("e1", &self[2])
            .field("e2", &self[3])
            .field("e3", &self[4])
            .field("e4", &self[5])
            .field("e41", &self[6])
            .field("e42", &self[7])
            .field("e43", &self[8])
            .field("e23", &self[9])
            .field("e31", &self[10])
            .field("e12", &self[11])
            .field("e423", &self[12])
            .field("e431", &self[13])
            .field("e412", &self[14])
            .field("e321", &self[15])
            .finish()
    }
}

impl MultiVector {
    pub const LEN: usize = 16;
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
        let mut state = serializer.serialize_struct("MultiVector", 16)?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
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
            e1234,
            e1,
            e2,
            e3,
            e4,
            e41,
            e42,
            e43,
            e23,
            e31,
            e12,
            e423,
            e431,
            e412,
            e321,
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
                let mut e1234 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e4 = None;
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e321 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MultiVectorField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }

                        MultiVectorField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
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

                        MultiVectorField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MultiVector::from([0.0; 16]);
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["scalar", "e1234", "e1", "e2", "e3", "e4", "e41", "e42", "e43", "e23", "e31", "e12", "e423", "e431", "e412", "e321"];
        deserializer.deserialize_struct("MultiVector", FIELDS, MultiVectorVisitor)
    }
}
impl std::ops::Index<crate::elements::scalar> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1234> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
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
impl std::ops::Index<crate::elements::e41> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e42> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e43> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e23> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e31> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e12> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e423> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e431> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e412> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::Index<crate::elements::e321> for MultiVector {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[15]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
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
impl std::ops::IndexMut<crate::elements::e41> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MultiVector {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[15]
    }
}
include!("./impls/multi_vector.rs");
