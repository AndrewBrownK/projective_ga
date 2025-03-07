use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlectorOnOrigin.
/// This variant of Flector intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union FlectorOnOrigin {
    groups: FlectorOnOriginGroups,
    /// e45, e4235, e4315, e4125
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct FlectorOnOriginGroups {
    /// e45, e4235, e4315, e4125
    g0: Simd32x4,
}
impl FlectorOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e45: f32, e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e45, e4235, e4315, e4125],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: FlectorOnOriginGroups { g0 },
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
const FLECTOR_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlectorOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<FlectorOnOrigin> for [f32; 4] {
    fn from(vector: FlectorOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for FlectorOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for FlectorOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlectorOnOrigin")
            .field("e45", &self[0])
            .field("e4235", &self[1])
            .field("e4315", &self[2])
            .field("e4125", &self[3])
            .finish()
    }
}

impl FlectorOnOrigin {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {
    fn nearly_eq_eps(&self, other: &FlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {
    fn nearly_eq_ulps(&self, other: &FlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {}
impl nearly::NearlyEq<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {}
impl nearly::NearlyOrdUlps<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {
    fn nearly_lt_ulps(&self, other: &FlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &FlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {
    fn nearly_lt_eps(&self, other: &FlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &FlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {}
impl nearly::NearlyOrd<FlectorOnOrigin, f32, f32> for FlectorOnOrigin {}

impl FlectorOnOrigin {
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

impl PartialOrd for FlectorOnOrigin {
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
impl Ord for FlectorOnOrigin {
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
impl PartialEq for FlectorOnOrigin {
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
impl Eq for FlectorOnOrigin {}
impl std::hash::Hash for FlectorOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for FlectorOnOrigin {}
unsafe impl bytemuck::Pod for FlectorOnOrigin {}
impl encase::ShaderType for FlectorOnOrigin {
    type ExtraMetadata = <FlectorOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <FlectorOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <FlectorOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <FlectorOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <FlectorOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for FlectorOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlectorOnOrigin", 4)?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlectorOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum FlectorOnOriginField {
            e45,
            e4235,
            e4315,
            e4125,
        }
        struct FlectorOnOriginVisitor;
        impl<'de> Visitor<'de> for FlectorOnOriginVisitor {
            type Value = FlectorOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlectorOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<FlectorOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e45 = None;
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FlectorOnOriginField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }

                        FlectorOnOriginField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        FlectorOnOriginField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        FlectorOnOriginField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = FlectorOnOrigin::from([0.0; 4]);
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e45", "e4235", "e4315", "e4125"];
        deserializer.deserialize_struct("FlectorOnOrigin", FIELDS, FlectorOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e45> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4235> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4315> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4125> for FlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for FlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for FlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for FlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for FlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/flector_on_origin.rs");
