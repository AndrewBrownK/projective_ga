use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// PlaneOnOrigin.
/// This variant of Plane intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union PlaneOnOrigin {
    groups: PlaneOnOriginGroups,
    /// e4235, e4315, e4125, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct PlaneOnOriginGroups {
    /// e4235, e4315, e4125
    g0: Simd32x3,
}
impl PlaneOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e4235, e4315, e4125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: PlaneOnOriginGroups { g0 },
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
const PLANE_ON_ORIGIN_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for PlaneOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[PLANE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for PlaneOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[PLANE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<PlaneOnOrigin> for [f32; 3] {
    fn from(vector: PlaneOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for PlaneOnOrigin {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for PlaneOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("PlaneOnOrigin")
            .field("e4235", &self[0])
            .field("e4315", &self[1])
            .field("e4125", &self[2])
            .finish()
    }
}

impl PlaneOnOrigin {
    pub const LEN: usize = 3;
}

impl nearly::NearlyEqEps<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {
    fn nearly_eq_eps(&self, other: &PlaneOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {
    fn nearly_eq_ulps(&self, other: &PlaneOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {}
impl nearly::NearlyEq<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {}
impl nearly::NearlyOrdUlps<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {
    fn nearly_lt_ulps(&self, other: &PlaneOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &PlaneOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {
    fn nearly_lt_eps(&self, other: &PlaneOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &PlaneOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {}
impl nearly::NearlyOrd<PlaneOnOrigin, f32, f32> for PlaneOnOrigin {}

impl PlaneOnOrigin {
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

impl PartialOrd for PlaneOnOrigin {
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
impl Ord for PlaneOnOrigin {
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
impl PartialEq for PlaneOnOrigin {
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
impl Eq for PlaneOnOrigin {}
impl std::hash::Hash for PlaneOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for PlaneOnOrigin {}
unsafe impl bytemuck::Pod for PlaneOnOrigin {}
impl encase::ShaderType for PlaneOnOrigin {
    type ExtraMetadata = <PlaneOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <PlaneOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <PlaneOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <PlaneOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <PlaneOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for PlaneOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("PlaneOnOrigin", 3)?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlaneOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum PlaneOnOriginField {
            e4235,
            e4315,
            e4125,
        }
        struct PlaneOnOriginVisitor;
        impl<'de> Visitor<'de> for PlaneOnOriginVisitor {
            type Value = PlaneOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PlaneOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<PlaneOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        PlaneOnOriginField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        PlaneOnOriginField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        PlaneOnOriginField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = PlaneOnOrigin::from([0.0; 3]);
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e4235", "e4315", "e4125"];
        deserializer.deserialize_struct("PlaneOnOrigin", FIELDS, PlaneOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e4235> for PlaneOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4315> for PlaneOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4125> for PlaneOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for PlaneOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for PlaneOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for PlaneOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/plane_on_origin.rs");
