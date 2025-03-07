use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// VersorOdd
#[repr(C)]
#[derive(Clone, Copy)]
pub union VersorOdd {
    groups: VersorOddGroups,
    /// e41, e42, e43, scalar, e23, e31, e12, e45, e15, e25, e35, e1234, e4235, e4315, e4125, e3215
    elements: [f32; 16],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct VersorOddGroups {
    /// e41, e42, e43, scalar
    g0: Simd32x4,
    /// e23, e31, e12, e45
    g1: Simd32x4,
    /// e15, e25, e35, e1234
    g2: Simd32x4,
    /// e4235, e4315, e4125, e3215
    g3: Simd32x4,
}
impl VersorOdd {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        e41: f32,
        e42: f32,
        e43: f32,
        scalar: f32,
        e23: f32,
        e31: f32,
        e12: f32,
        e45: f32,
        e15: f32,
        e25: f32,
        e35: f32,
        e1234: f32,
        e4235: f32,
        e4315: f32,
        e4125: f32,
        e3215: f32,
    ) -> Self {
        Self {
            elements: [e41, e42, e43, scalar, e23, e31, e12, e45, e15, e25, e35, e1234, e4235, e4315, e4125, e3215],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4, g2: Simd32x4, g3: Simd32x4) -> Self {
        Self {
            groups: VersorOddGroups { g0, g1, g2, g3 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
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
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
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
}
const VERSOR_ODD_INDEX_REMAP: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
impl std::ops::Index<usize> for VersorOdd {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[VERSOR_ODD_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for VersorOdd {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[VERSOR_ODD_INDEX_REMAP[index]] }
    }
}
impl From<VersorOdd> for [f32; 16] {
    fn from(vector: VersorOdd) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
                vector.elements[8], vector.elements[9], vector.elements[10], vector.elements[11], vector.elements[12], vector.elements[13], vector.elements[14],
                vector.elements[15],
            ]
        }
    }
}
impl From<[f32; 16]> for VersorOdd {
    fn from(array: [f32; 16]) -> Self {
        Self {
            elements: [
                array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5], array[3], array[4], array[5], array[6],
            ],
        }
    }
}
impl std::fmt::Debug for VersorOdd {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("VersorOdd")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("scalar", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e45", &self[7])
            .field("e15", &self[8])
            .field("e25", &self[9])
            .field("e35", &self[10])
            .field("e1234", &self[11])
            .field("e4235", &self[12])
            .field("e4315", &self[13])
            .field("e4125", &self[14])
            .field("e3215", &self[15])
            .finish()
    }
}

impl VersorOdd {
    pub const LEN: usize = 16;
}

impl nearly::NearlyEqEps<VersorOdd, f32, f32> for VersorOdd {
    fn nearly_eq_eps(&self, other: &VersorOdd, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<VersorOdd, f32, f32> for VersorOdd {
    fn nearly_eq_ulps(&self, other: &VersorOdd, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<VersorOdd, f32, f32> for VersorOdd {}
impl nearly::NearlyEq<VersorOdd, f32, f32> for VersorOdd {}
impl nearly::NearlyOrdUlps<VersorOdd, f32, f32> for VersorOdd {
    fn nearly_lt_ulps(&self, other: &VersorOdd, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &VersorOdd, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<VersorOdd, f32, f32> for VersorOdd {
    fn nearly_lt_eps(&self, other: &VersorOdd, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &VersorOdd, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<VersorOdd, f32, f32> for VersorOdd {}
impl nearly::NearlyOrd<VersorOdd, f32, f32> for VersorOdd {}

impl VersorOdd {
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

impl PartialOrd for VersorOdd {
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
impl Ord for VersorOdd {
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
impl PartialEq for VersorOdd {
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
impl Eq for VersorOdd {}
impl std::hash::Hash for VersorOdd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for VersorOdd {}
unsafe impl bytemuck::Pod for VersorOdd {}
impl encase::ShaderType for VersorOdd {
    type ExtraMetadata = <VersorOddGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <VersorOddGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <VersorOddGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <VersorOddGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <VersorOddGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for VersorOdd {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("VersorOdd", 16)?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for VersorOdd {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum VersorOddField {
            e41,
            e42,
            e43,
            scalar,
            e23,
            e31,
            e12,
            e45,
            e15,
            e25,
            e35,
            e1234,
            e4235,
            e4315,
            e4125,
            e3215,
        }
        struct VersorOddVisitor;
        impl<'de> Visitor<'de> for VersorOddVisitor {
            type Value = VersorOdd;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct VersorOdd")
            }
            fn visit_map<V>(self, mut map: V) -> Result<VersorOdd, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut scalar = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e45 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e1234 = None;
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;
                let mut e3215 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        VersorOddField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        VersorOddField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        VersorOddField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        VersorOddField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }

                        VersorOddField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        VersorOddField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        VersorOddField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        VersorOddField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }

                        VersorOddField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        VersorOddField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        VersorOddField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        VersorOddField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }

                        VersorOddField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        VersorOddField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        VersorOddField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }

                        VersorOddField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = VersorOdd::from([0.0; 16]);
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "e41", "e42", "e43", "scalar", "e23", "e31", "e12", "e45", "e15", "e25", "e35", "e1234", "e4235", "e4315", "e4125", "e3215",
        ];
        deserializer.deserialize_struct("VersorOdd", FIELDS, VersorOddVisitor)
    }
}
impl std::ops::Index<crate::elements::e41> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e45> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e15> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e25> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e35> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e1234> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e4235> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e4315> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e4125> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::Index<crate::elements::e3215> for VersorOdd {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[15]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for VersorOdd {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[15]
    }
}
include!("./impls/versor_odd.rs");
