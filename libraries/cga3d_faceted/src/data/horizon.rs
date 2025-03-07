use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Horizon
#[repr(C)]
#[derive(Clone, Copy)]
pub union Horizon {
    groups: HorizonGroups,
    /// e3215, 0, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct HorizonGroups {
    /// e3215
    g0: f32,
}
impl Horizon {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e3215: f32) -> Self {
        Self { elements: [e3215, 0.0, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: HorizonGroups { g0 } }
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
const HORIZON_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for Horizon {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[HORIZON_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Horizon {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[HORIZON_INDEX_REMAP[index]] }
    }
}
impl From<Horizon> for [f32; 1] {
    fn from(vector: Horizon) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for Horizon {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for Horizon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("Horizon").field("e3215", &self[0]).finish()
    }
}

impl Horizon {
    pub const LEN: usize = 1;
}

impl nearly::NearlyEqEps<Horizon, f32, f32> for Horizon {
    fn nearly_eq_eps(&self, other: &Horizon, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<Horizon, f32, f32> for Horizon {
    fn nearly_eq_ulps(&self, other: &Horizon, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<Horizon, f32, f32> for Horizon {}
impl nearly::NearlyEq<Horizon, f32, f32> for Horizon {}
impl nearly::NearlyOrdUlps<Horizon, f32, f32> for Horizon {
    fn nearly_lt_ulps(&self, other: &Horizon, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &Horizon, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<Horizon, f32, f32> for Horizon {
    fn nearly_lt_eps(&self, other: &Horizon, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &Horizon, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<Horizon, f32, f32> for Horizon {}
impl nearly::NearlyOrd<Horizon, f32, f32> for Horizon {}

impl Horizon {
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

impl PartialOrd for Horizon {
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
impl Ord for Horizon {
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
impl PartialEq for Horizon {
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
impl Eq for Horizon {}
impl std::hash::Hash for Horizon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for Horizon {}
unsafe impl bytemuck::Pod for Horizon {}
impl encase::ShaderType for Horizon {
    type ExtraMetadata = <HorizonGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <HorizonGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <HorizonGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <HorizonGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <HorizonGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for Horizon {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Horizon", 1)?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for Horizon {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum HorizonField {
            e3215,
        }
        struct HorizonVisitor;
        impl<'de> Visitor<'de> for HorizonVisitor {
            type Value = Horizon;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Horizon")
            }
            fn visit_map<V>(self, mut map: V) -> Result<Horizon, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e3215 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        HorizonField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = Horizon::from([0.0; 1]);
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e3215"];
        deserializer.deserialize_struct("Horizon", FIELDS, HorizonVisitor)
    }
}
impl std::ops::Index<crate::elements::e3215> for Horizon {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for Horizon {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/horizon.rs");
