use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiFlatOrigin.
/// This variant of MysteryCircle is the Dual to FlatOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MysteryCircle,
/// but an imaginary radius, and a spacial presence in the shape of a
/// FlatOrigin with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiFlatOrigin {
    groups: AntiFlatOriginGroups,
    /// e321, 0, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiFlatOriginGroups {
    /// e321
    g0: f32,
}
impl AntiFlatOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e321: f32) -> Self {
        Self { elements: [e321, 0.0, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self {
            groups: AntiFlatOriginGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_FLAT_ORIGIN_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for AntiFlatOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_FLAT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiFlatOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_FLAT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiFlatOrigin> for [f32; 1] {
    fn from(vector: AntiFlatOrigin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for AntiFlatOrigin {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiFlatOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiFlatOrigin").field("e321", &self[0]).finish()
    }
}

impl AntiFlatOrigin {
    pub const LEN: usize = 1;
}

impl nearly::NearlyEqEps<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {
    fn nearly_eq_eps(&self, other: &AntiFlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {
    fn nearly_eq_ulps(&self, other: &AntiFlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {}
impl nearly::NearlyEq<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {}
impl nearly::NearlyOrdUlps<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {
    fn nearly_lt_ulps(&self, other: &AntiFlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiFlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {
    fn nearly_lt_eps(&self, other: &AntiFlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiFlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {}
impl nearly::NearlyOrd<AntiFlatOrigin, f32, f32> for AntiFlatOrigin {}

impl AntiFlatOrigin {
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

impl PartialOrd for AntiFlatOrigin {
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
impl Ord for AntiFlatOrigin {
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
impl PartialEq for AntiFlatOrigin {
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
impl Eq for AntiFlatOrigin {}
impl std::hash::Hash for AntiFlatOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiFlatOrigin {}
unsafe impl bytemuck::Pod for AntiFlatOrigin {}
impl encase::ShaderType for AntiFlatOrigin {
    type ExtraMetadata = <AntiFlatOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiFlatOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiFlatOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiFlatOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiFlatOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiFlatOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiFlatOrigin", 1)?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiFlatOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiFlatOriginField {
            e321,
        }
        struct AntiFlatOriginVisitor;
        impl<'de> Visitor<'de> for AntiFlatOriginVisitor {
            type Value = AntiFlatOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiFlatOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiFlatOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e321 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiFlatOriginField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiFlatOrigin::from([0.0; 1]);
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e321"];
        deserializer.deserialize_struct("AntiFlatOrigin", FIELDS, AntiFlatOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e321> for AntiFlatOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiFlatOrigin {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/anti_flat_origin.rs");
