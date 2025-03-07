use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// SphereOnOrigin.
/// This variant of Sphere intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union SphereOnOrigin {
    groups: SphereOnOriginGroups,
    /// e4235, e4315, e4125, e1234
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct SphereOnOriginGroups {
    /// e4235, e4315, e4125, e1234
    g0: Simd32x4,
}
impl SphereOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e4235: f32, e4315: f32, e4125: f32, e1234: f32) -> Self {
        Self {
            elements: [e4235, e4315, e4125, e1234],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: SphereOnOriginGroups { g0 },
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
const SPHERE_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for SphereOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[SPHERE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for SphereOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[SPHERE_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<SphereOnOrigin> for [f32; 4] {
    fn from(vector: SphereOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for SphereOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for SphereOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("SphereOnOrigin")
            .field("e4235", &self[0])
            .field("e4315", &self[1])
            .field("e4125", &self[2])
            .field("e1234", &self[3])
            .finish()
    }
}

impl SphereOnOrigin {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<SphereOnOrigin, f32, f32> for SphereOnOrigin {
    fn nearly_eq_eps(&self, other: &SphereOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<SphereOnOrigin, f32, f32> for SphereOnOrigin {
    fn nearly_eq_ulps(&self, other: &SphereOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<SphereOnOrigin, f32, f32> for SphereOnOrigin {}
impl nearly::NearlyEq<SphereOnOrigin, f32, f32> for SphereOnOrigin {}
impl nearly::NearlyOrdUlps<SphereOnOrigin, f32, f32> for SphereOnOrigin {
    fn nearly_lt_ulps(&self, other: &SphereOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &SphereOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<SphereOnOrigin, f32, f32> for SphereOnOrigin {
    fn nearly_lt_eps(&self, other: &SphereOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &SphereOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<SphereOnOrigin, f32, f32> for SphereOnOrigin {}
impl nearly::NearlyOrd<SphereOnOrigin, f32, f32> for SphereOnOrigin {}

impl SphereOnOrigin {
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

impl PartialOrd for SphereOnOrigin {
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
impl Ord for SphereOnOrigin {
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
impl PartialEq for SphereOnOrigin {
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
impl Eq for SphereOnOrigin {}
impl std::hash::Hash for SphereOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for SphereOnOrigin {}
unsafe impl bytemuck::Pod for SphereOnOrigin {}
impl encase::ShaderType for SphereOnOrigin {
    type ExtraMetadata = <SphereOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <SphereOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <SphereOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <SphereOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <SphereOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for SphereOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SphereOnOrigin", 4)?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for SphereOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum SphereOnOriginField {
            e4235,
            e4315,
            e4125,
            e1234,
        }
        struct SphereOnOriginVisitor;
        impl<'de> Visitor<'de> for SphereOnOriginVisitor {
            type Value = SphereOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SphereOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<SphereOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;
                let mut e1234 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        SphereOnOriginField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        SphereOnOriginField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        SphereOnOriginField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }

                        SphereOnOriginField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = SphereOnOrigin::from([0.0; 4]);
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e4235", "e4315", "e4125", "e1234"];
        deserializer.deserialize_struct("SphereOnOrigin", FIELDS, SphereOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e4235> for SphereOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e4315> for SphereOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e4125> for SphereOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e1234> for SphereOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for SphereOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for SphereOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for SphereOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for SphereOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/sphere_on_origin.rs");
