use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundPoint
#[repr(C)]
#[derive(Clone, Copy)]
pub union RoundPoint {
    groups: RoundPointGroups,
    /// e1, e2, e3, e4, e5, 0, 0, 0
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct RoundPointGroups {
    /// e1, e2, e3, e4
    g0: Simd32x4,
    /// e5
    g1: f32,
}
impl RoundPoint {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e1: f32, e2: f32, e3: f32, e4: f32, e5: f32) -> Self {
        Self {
            elements: [e1, e2, e3, e4, e5, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: f32) -> Self {
        Self {
            groups: RoundPointGroups { g0, g1 },
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
    #[inline(always)]
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const ROUND_POINT_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];
impl std::ops::Index<usize> for RoundPoint {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ROUND_POINT_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for RoundPoint {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ROUND_POINT_INDEX_REMAP[index]] }
    }
}
impl From<RoundPoint> for [f32; 5] {
    fn from(vector: RoundPoint) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}
impl From<[f32; 5]> for RoundPoint {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for RoundPoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("RoundPoint")
            .field("e1", &self[0])
            .field("e2", &self[1])
            .field("e3", &self[2])
            .field("e4", &self[3])
            .field("e5", &self[4])
            .finish()
    }
}

impl RoundPoint {
    pub const LEN: usize = 5;
}

impl nearly::NearlyEqEps<RoundPoint, f32, f32> for RoundPoint {
    fn nearly_eq_eps(&self, other: &RoundPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<RoundPoint, f32, f32> for RoundPoint {
    fn nearly_eq_ulps(&self, other: &RoundPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<RoundPoint, f32, f32> for RoundPoint {}
impl nearly::NearlyEq<RoundPoint, f32, f32> for RoundPoint {}
impl nearly::NearlyOrdUlps<RoundPoint, f32, f32> for RoundPoint {
    fn nearly_lt_ulps(&self, other: &RoundPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &RoundPoint, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<RoundPoint, f32, f32> for RoundPoint {
    fn nearly_lt_eps(&self, other: &RoundPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &RoundPoint, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<RoundPoint, f32, f32> for RoundPoint {}
impl nearly::NearlyOrd<RoundPoint, f32, f32> for RoundPoint {}

impl RoundPoint {
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

impl PartialOrd for RoundPoint {
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
impl Ord for RoundPoint {
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
impl PartialEq for RoundPoint {
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
impl Eq for RoundPoint {}
impl std::hash::Hash for RoundPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for RoundPoint {}
unsafe impl bytemuck::Pod for RoundPoint {}
impl encase::ShaderType for RoundPoint {
    type ExtraMetadata = <RoundPointGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <RoundPointGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <RoundPointGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <RoundPointGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <RoundPointGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for RoundPoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RoundPoint", 5)?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoundPoint {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum RoundPointField {
            e1,
            e2,
            e3,
            e4,
            e5,
        }
        struct RoundPointVisitor;
        impl<'de> Visitor<'de> for RoundPointVisitor {
            type Value = RoundPoint;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RoundPoint")
            }
            fn visit_map<V>(self, mut map: V) -> Result<RoundPoint, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e4 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        RoundPointField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        RoundPointField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        RoundPointField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }

                        RoundPointField::e4 => {
                            if e4.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4"));
                            }
                            e4 = Some(map.next_value()?);
                        }

                        RoundPointField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = RoundPoint::from([0.0; 5]);
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e1", "e2", "e3", "e4", "e5"];
        deserializer.deserialize_struct("RoundPoint", FIELDS, RoundPointVisitor)
    }
}
impl std::ops::Index<crate::elements::e1> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e2> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e3> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e4> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e5> for RoundPoint {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for RoundPoint {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for RoundPoint {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for RoundPoint {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for RoundPoint {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for RoundPoint {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/round_point.rs");
