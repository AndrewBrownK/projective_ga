use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatPoint
#[repr(C)]
#[derive(Clone, Copy)]
pub union FlatPoint {
    groups: FlatPointGroups,
    /// e15, e25, e35, e45
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct FlatPointGroups {
    /// e15, e25, e35, e45
    g0: Simd32x4,
}
impl FlatPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e15: f32, e25: f32, e35: f32, e45: f32) -> Self {
        Self { elements: [e15, e25, e35, e45] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self { groups: FlatPointGroups { g0 } }
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
const FLAT_POINT_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for FlatPoint {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLAT_POINT_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlatPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLAT_POINT_INDEX_REMAP[index]] }
    }
}
impl From<FlatPoint> for [f32; 4] {
    fn from(vector: FlatPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for FlatPoint {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for FlatPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("FlatPoint")
            .field("e15", &self[0])
            .field("e25", &self[1])
            .field("e35", &self[2])
            .field("e45", &self[3])
            .finish()
    }
}

impl FlatPoint {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<FlatPoint, f32, f32> for FlatPoint {
    fn nearly_eq_eps(&self, other: &FlatPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<FlatPoint, f32, f32> for FlatPoint {
    fn nearly_eq_ulps(&self, other: &FlatPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<FlatPoint, f32, f32> for FlatPoint {}
impl nearly::NearlyEq<FlatPoint, f32, f32> for FlatPoint {}
impl nearly::NearlyOrdUlps<FlatPoint, f32, f32> for FlatPoint {
    fn nearly_lt_ulps(&self, other: &FlatPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &FlatPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<FlatPoint, f32, f32> for FlatPoint {
    fn nearly_lt_eps(&self, other: &FlatPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &FlatPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<FlatPoint, f32, f32> for FlatPoint {}
impl nearly::NearlyOrd<FlatPoint, f32, f32> for FlatPoint {}

impl FlatPoint {
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

impl PartialOrd for FlatPoint {
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
impl Ord for FlatPoint {
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
impl PartialEq for FlatPoint {
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
impl Eq for FlatPoint {}
impl std::hash::Hash for FlatPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for FlatPoint {}
unsafe impl bytemuck::Pod for FlatPoint {}
impl encase::ShaderType for FlatPoint {
    type ExtraMetadata = <FlatPointGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <FlatPointGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <FlatPointGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <FlatPointGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <FlatPointGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for FlatPoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlatPoint", 4)?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlatPoint {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum FlatPointField {
            e15,
            e25,
            e35,
            e45,
        }
        struct FlatPointVisitor;
        impl<'de> Visitor<'de> for FlatPointVisitor {
            type Value = FlatPoint;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlatPoint")
            }
            fn visit_map<V>(self, mut map: V) -> Result<FlatPoint, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e45 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FlatPointField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        FlatPointField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        FlatPointField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        FlatPointField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = FlatPoint::from([0.0; 4]);
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e15", "e25", "e35", "e45"];
        deserializer.deserialize_struct("FlatPoint", FIELDS, FlatPointVisitor)
    }
}
impl std::ops::Index<crate::elements::e15> for FlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e25> for FlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e35> for FlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e45> for FlatPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for FlatPoint {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for FlatPoint {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for FlatPoint {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for FlatPoint {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/flat_point.rs");
