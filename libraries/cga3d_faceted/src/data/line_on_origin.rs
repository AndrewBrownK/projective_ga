use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// LineOnOrigin.
/// This variant of Line intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union LineOnOrigin {
    groups: LineOnOriginGroups,
    /// e415, e425, e435, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct LineOnOriginGroups {
    /// e415, e425, e435
    g0: Simd32x3,
}
impl LineOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32) -> Self {
        Self {
            elements: [e415, e425, e435, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineOnOriginGroups { g0 },
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
const LINE_ON_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for LineOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for LineOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<LineOnOrigin> for [f32; 3] {
    fn from(vector: LineOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for LineOnOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for LineOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("LineOnOrigin").field("e415", &self[0]).field("e425", &self[1]).field("e435", &self[2]).finish()
    }
}

impl LineOnOrigin {
    pub const LEN: usize = 3;
}

impl nearly::NearlyEqEps<LineOnOrigin, f32, f32> for LineOnOrigin {
    fn nearly_eq_eps(&self, other: &LineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<LineOnOrigin, f32, f32> for LineOnOrigin {
    fn nearly_eq_ulps(&self, other: &LineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<LineOnOrigin, f32, f32> for LineOnOrigin {}
impl nearly::NearlyEq<LineOnOrigin, f32, f32> for LineOnOrigin {}
impl nearly::NearlyOrdUlps<LineOnOrigin, f32, f32> for LineOnOrigin {
    fn nearly_lt_ulps(&self, other: &LineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &LineOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<LineOnOrigin, f32, f32> for LineOnOrigin {
    fn nearly_lt_eps(&self, other: &LineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &LineOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<LineOnOrigin, f32, f32> for LineOnOrigin {}
impl nearly::NearlyOrd<LineOnOrigin, f32, f32> for LineOnOrigin {}

impl LineOnOrigin {
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

impl PartialOrd for LineOnOrigin {
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
impl Ord for LineOnOrigin {
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
impl PartialEq for LineOnOrigin {
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
impl Eq for LineOnOrigin {}
impl std::hash::Hash for LineOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for LineOnOrigin {}
unsafe impl bytemuck::Pod for LineOnOrigin {}
impl encase::ShaderType for LineOnOrigin {
    type ExtraMetadata = <LineOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <LineOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <LineOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <LineOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <LineOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for LineOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LineOnOrigin", 3)?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum LineOnOriginField {
            e415,
            e425,
            e435,
        }
        struct LineOnOriginVisitor;
        impl<'de> Visitor<'de> for LineOnOriginVisitor {
            type Value = LineOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LineOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<LineOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        LineOnOriginField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        LineOnOriginField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        LineOnOriginField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = LineOnOrigin::from([0.0; 3]);
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e415", "e425", "e435"];
        deserializer.deserialize_struct("LineOnOrigin", FIELDS, LineOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e415> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for LineOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for LineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for LineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for LineOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/line_on_origin.rs");
