use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlectorAtInfinity.
/// This variant of Flector exists in the Horizon.
#[repr(C)]
#[derive(Clone, Copy)]
pub union FlectorAtInfinity {
    groups: FlectorAtInfinityGroups,
    /// e15, e25, e35, e3215
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct FlectorAtInfinityGroups {
    /// e15, e25, e35, e3215
    g0: Simd32x4,
}
impl FlectorAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e15: f32, e25: f32, e35: f32, e3215: f32) -> Self {
        Self { elements: [e15, e25, e35, e3215] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: FlectorAtInfinityGroups { g0 },
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
}
const FLECTOR_AT_INFINITY_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLECTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlectorAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLECTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<FlectorAtInfinity> for [f32; 4] {
    fn from(vector: FlectorAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for FlectorAtInfinity {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for FlectorAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlectorAtInfinity")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .field("e3215", &self[3])
            .finish()
    }
}

impl FlectorAtInfinity {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {
    fn nearly_eq_eps(&self, other: &FlectorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {
    fn nearly_eq_ulps(&self, other: &FlectorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {}
impl nearly::NearlyEq<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {}
impl nearly::NearlyOrdUlps<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {
    fn nearly_lt_ulps(&self, other: &FlectorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &FlectorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {
    fn nearly_lt_eps(&self, other: &FlectorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &FlectorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {}
impl nearly::NearlyOrd<FlectorAtInfinity, f32, f32> for FlectorAtInfinity {}

impl FlectorAtInfinity {
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

impl PartialOrd for FlectorAtInfinity {
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
impl Ord for FlectorAtInfinity {
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
impl PartialEq for FlectorAtInfinity {
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
impl Eq for FlectorAtInfinity {}
impl std::hash::Hash for FlectorAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for FlectorAtInfinity {}
unsafe impl bytemuck::Pod for FlectorAtInfinity {}
impl encase::ShaderType for FlectorAtInfinity {
    type ExtraMetadata = <FlectorAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <FlectorAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <FlectorAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <FlectorAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <FlectorAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for FlectorAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlectorAtInfinity", 4)?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlectorAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum FlectorAtInfinityField {
            e15,
            e25,
            e35,
            e3215,
        }
        struct FlectorAtInfinityVisitor;
        impl<'de> Visitor<'de> for FlectorAtInfinityVisitor {
            type Value = FlectorAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlectorAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<FlectorAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e3215 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FlectorAtInfinityField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        FlectorAtInfinityField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        FlectorAtInfinityField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        FlectorAtInfinityField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = FlectorAtInfinity::from([0.0; 4]);
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e15", "e25", "e35", "e3215"];
        deserializer.deserialize_struct("FlectorAtInfinity", FIELDS, FlectorAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e15> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e25> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e35> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for FlectorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for FlectorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for FlectorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for FlectorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for FlectorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/flector_at_infinity.rs");
