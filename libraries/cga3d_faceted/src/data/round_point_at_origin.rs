use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundPointAtOrigin.
/// This variant of RoundPoint is centered on the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union RoundPointAtOrigin {
    groups: RoundPointAtOriginGroups,
    /// e4, e5, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct RoundPointAtOriginGroups {
    /// e4, e5
    g0: Simd32x2,
}
impl RoundPointAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4: f32, e5: f32) -> Self {
        Self { elements: [e4, e5, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: Simd32x2) -> Self {
        Self {
            groups: RoundPointAtOriginGroups { g0 },
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
}
const ROUND_POINT_AT_ORIGIN_INDEX_REMAP: [usize; 2] = [0, 1];
impl std::ops::Index<usize> for RoundPointAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROUND_POINT_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for RoundPointAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROUND_POINT_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<RoundPointAtOrigin> for [f32; 2] {
    fn from(vector: RoundPointAtOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1]] }
    }
}
impl From<[f32; 2]> for RoundPointAtOrigin {
    fn from(array: [f32; 2]) -> Self {
        Self {
            elements: [array[0], array[1], 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for RoundPointAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("RoundPointAtOrigin").field("e4", &self[0]).field("e5", &self[1]).finish()
    }
}

impl RoundPointAtOrigin {
    pub const LEN: usize = 2;
}

impl nearly::NearlyEqEps<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {
    fn nearly_eq_eps(&self, other: &RoundPointAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {
    fn nearly_eq_ulps(&self, other: &RoundPointAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {}
impl nearly::NearlyEq<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {}
impl nearly::NearlyOrdUlps<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {
    fn nearly_lt_ulps(&self, other: &RoundPointAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &RoundPointAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {
    fn nearly_lt_eps(&self, other: &RoundPointAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &RoundPointAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {}
impl nearly::NearlyOrd<RoundPointAtOrigin, f32, f32> for RoundPointAtOrigin {}

impl RoundPointAtOrigin {
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

impl PartialOrd for RoundPointAtOrigin {
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
impl Ord for RoundPointAtOrigin {
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
impl PartialEq for RoundPointAtOrigin {
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
impl Eq for RoundPointAtOrigin {}
impl std::hash::Hash for RoundPointAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for RoundPointAtOrigin {}
unsafe impl bytemuck::Pod for RoundPointAtOrigin {}
impl encase::ShaderType for RoundPointAtOrigin {
    type ExtraMetadata = <RoundPointAtOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <RoundPointAtOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <RoundPointAtOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <RoundPointAtOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <RoundPointAtOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for RoundPointAtOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RoundPointAtOrigin", 2)?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoundPointAtOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum RoundPointAtOriginField {
            e4,
            e5,
        }
        struct RoundPointAtOriginVisitor;
        impl<'de> Visitor<'de> for RoundPointAtOriginVisitor {
            type Value = RoundPointAtOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RoundPointAtOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<RoundPointAtOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e4 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        RoundPointAtOriginField::e4 => {
                            if e4.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4"));
                            }
                            e4 = Some(map.next_value()?);
                        }

                        RoundPointAtOriginField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = RoundPointAtOrigin::from([0.0; 2]);
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e4", "e5"];
        deserializer.deserialize_struct("RoundPointAtOrigin", FIELDS, RoundPointAtOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e4> for RoundPointAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e5> for RoundPointAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for RoundPointAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for RoundPointAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[1]
    }
}
include!("./impls/round_point_at_origin.rs");
