use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatPointAtInfinity.
/// This variant of FlatPoint exists in the Horizon.
#[repr(C)]
#[derive(Clone, Copy)]
pub union FlatPointAtInfinity {
    groups: FlatPointAtInfinityGroups,
    /// e15, e25, e35, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct FlatPointAtInfinityGroups {
    /// e15, e25, e35
    g0: Simd32x3,
}
impl FlatPointAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e15: f32, e25: f32, e35: f32) -> Self {
        Self { elements: [e15, e25, e35, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: FlatPointAtInfinityGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g0 }
    }
}
const FLAT_POINT_AT_INFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for FlatPointAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLAT_POINT_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlatPointAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLAT_POINT_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<FlatPointAtInfinity> for [f32; 3] {
    fn from(vector: FlatPointAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for FlatPointAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for FlatPointAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlatPointAtInfinity")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .finish()
    }
}

impl FlatPointAtInfinity {
    pub const LEN: usize = 3;
}

impl nearly::NearlyEqEps<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {
    fn nearly_eq_eps(&self, other: &FlatPointAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {
    fn nearly_eq_ulps(&self, other: &FlatPointAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {}
impl nearly::NearlyEq<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {}
impl nearly::NearlyOrdUlps<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {
    fn nearly_lt_ulps(&self, other: &FlatPointAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &FlatPointAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {
    fn nearly_lt_eps(&self, other: &FlatPointAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &FlatPointAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {}
impl nearly::NearlyOrd<FlatPointAtInfinity, f32, f32> for FlatPointAtInfinity {}

impl FlatPointAtInfinity {
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

impl PartialOrd for FlatPointAtInfinity {
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
impl Ord for FlatPointAtInfinity {
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
impl PartialEq for FlatPointAtInfinity {
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
impl Eq for FlatPointAtInfinity {}
impl std::hash::Hash for FlatPointAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for FlatPointAtInfinity {}
unsafe impl bytemuck::Pod for FlatPointAtInfinity {}
impl encase::ShaderType for FlatPointAtInfinity {
    type ExtraMetadata = <FlatPointAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <FlatPointAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <FlatPointAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <FlatPointAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <FlatPointAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for FlatPointAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlatPointAtInfinity", 3)?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlatPointAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum FlatPointAtInfinityField {
            e15,
            e25,
            e35,
        }
        struct FlatPointAtInfinityVisitor;
        impl<'de> Visitor<'de> for FlatPointAtInfinityVisitor {
            type Value = FlatPointAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlatPointAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<FlatPointAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FlatPointAtInfinityField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        FlatPointAtInfinityField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        FlatPointAtInfinityField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = FlatPointAtInfinity::from([0.0; 3]);
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e15", "e25", "e35"];
        deserializer.deserialize_struct("FlatPointAtInfinity", FIELDS, FlatPointAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e15> for FlatPointAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e25> for FlatPointAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e35> for FlatPointAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for FlatPointAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for FlatPointAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for FlatPointAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/flat_point_at_infinity.rs");
