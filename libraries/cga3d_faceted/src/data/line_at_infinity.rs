use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// LineAtInfinity.
/// This variant of Line exists in the Horizon.
#[repr(C)]
#[derive(Clone, Copy)]
pub union LineAtInfinity {
    groups: LineAtInfinityGroups,
    /// e235, e315, e125, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct LineAtInfinityGroups {
    /// e235, e315, e125
    g0: Simd32x3,
}
impl LineAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3) -> Self {
        Self {
            groups: LineAtInfinityGroups { g0 },
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
const LINE_AT_INFINITY_INDEX_REMAP: [usize; 3] = [0, 1, 2];
impl std::ops::Index<usize> for LineAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[LINE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for LineAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[LINE_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<LineAtInfinity> for [f32; 3] {
    fn from(vector: LineAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2]] }
    }
}
impl From<[f32; 3]> for LineAtInfinity {
    fn from(array: [f32; 3]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0],
        }
    }
}
impl std::fmt::Debug for LineAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("LineAtInfinity")
            .field("e235", &self[0])
            .field("e315", &self[1])
            .field("e125", &self[2])
            .finish()
    }
}

impl LineAtInfinity {
    pub const LEN: usize = 3;
}

impl nearly::NearlyEqEps<LineAtInfinity, f32, f32> for LineAtInfinity {
    fn nearly_eq_eps(&self, other: &LineAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<LineAtInfinity, f32, f32> for LineAtInfinity {
    fn nearly_eq_ulps(&self, other: &LineAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<LineAtInfinity, f32, f32> for LineAtInfinity {}
impl nearly::NearlyEq<LineAtInfinity, f32, f32> for LineAtInfinity {}
impl nearly::NearlyOrdUlps<LineAtInfinity, f32, f32> for LineAtInfinity {
    fn nearly_lt_ulps(&self, other: &LineAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &LineAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<LineAtInfinity, f32, f32> for LineAtInfinity {
    fn nearly_lt_eps(&self, other: &LineAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &LineAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<LineAtInfinity, f32, f32> for LineAtInfinity {}
impl nearly::NearlyOrd<LineAtInfinity, f32, f32> for LineAtInfinity {}

impl LineAtInfinity {
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

impl PartialOrd for LineAtInfinity {
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
impl Ord for LineAtInfinity {
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
impl PartialEq for LineAtInfinity {
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
impl Eq for LineAtInfinity {}
impl std::hash::Hash for LineAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for LineAtInfinity {}
unsafe impl bytemuck::Pod for LineAtInfinity {}
impl encase::ShaderType for LineAtInfinity {
    type ExtraMetadata = <LineAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <LineAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <LineAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <LineAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <LineAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for LineAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LineAtInfinity", 3)?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum LineAtInfinityField {
            e235,
            e315,
            e125,
        }
        struct LineAtInfinityVisitor;
        impl<'de> Visitor<'de> for LineAtInfinityVisitor {
            type Value = LineAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LineAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<LineAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        LineAtInfinityField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        LineAtInfinityField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        LineAtInfinityField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = LineAtInfinity::from([0.0; 3]);
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e235", "e315", "e125"];
        deserializer.deserialize_struct("LineAtInfinity", FIELDS, LineAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e235> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e315> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e125> for LineAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for LineAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for LineAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for LineAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[2]
    }
}
include!("./impls/line_at_infinity.rs");
