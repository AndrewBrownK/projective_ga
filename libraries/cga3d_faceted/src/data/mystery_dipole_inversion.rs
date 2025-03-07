use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MysteryDipoleInversion.
/// TODO this is currently a mystery I'm investigating
#[repr(C)]
#[derive(Clone, Copy)]
pub union MysteryDipoleInversion {
    groups: MysteryDipoleInversionGroups,
    /// e23, e31, e12, e45, e4235, e4315, e4125, 0
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MysteryDipoleInversionGroups {
    /// e23, e31, e12, e45
    g0: Simd32x4,
    /// e4235, e4315, e4125
    g1: Simd32x3,
}
impl MysteryDipoleInversion {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e45: f32, e4235: f32, e4315: f32, e4125: f32) -> Self {
        Self {
            elements: [e23, e31, e12, e45, e4235, e4315, e4125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3) -> Self {
        Self {
            groups: MysteryDipoleInversionGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
}
const MYSTERY_DIPOLE_INVERSION_INDEX_REMAP: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
impl std::ops::Index<usize> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryDipoleInversion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl From<MysteryDipoleInversion> for [f32; 7] {
    fn from(vector: MysteryDipoleInversion) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6],
            ]
        }
    }
}
impl From<[f32; 7]> for MysteryDipoleInversion {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0],
        }
    }
}
impl std::fmt::Debug for MysteryDipoleInversion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryDipoleInversion")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e45", &self[3])
            .field("e4235", &self[4])
            .field("e4315", &self[5])
            .field("e4125", &self[6])
            .finish()
    }
}

impl MysteryDipoleInversion {
    pub const LEN: usize = 7;
}

impl nearly::NearlyEqEps<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {
    fn nearly_eq_eps(&self, other: &MysteryDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {
    fn nearly_eq_ulps(&self, other: &MysteryDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {}
impl nearly::NearlyEq<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {}
impl nearly::NearlyOrdUlps<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {
    fn nearly_lt_ulps(&self, other: &MysteryDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &MysteryDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {
    fn nearly_lt_eps(&self, other: &MysteryDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &MysteryDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {}
impl nearly::NearlyOrd<MysteryDipoleInversion, f32, f32> for MysteryDipoleInversion {}

impl MysteryDipoleInversion {
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

impl PartialOrd for MysteryDipoleInversion {
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
impl Ord for MysteryDipoleInversion {
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
impl PartialEq for MysteryDipoleInversion {
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
impl Eq for MysteryDipoleInversion {}
impl std::hash::Hash for MysteryDipoleInversion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MysteryDipoleInversion {}
unsafe impl bytemuck::Pod for MysteryDipoleInversion {}
impl encase::ShaderType for MysteryDipoleInversion {
    type ExtraMetadata = <MysteryDipoleInversionGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MysteryDipoleInversionGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MysteryDipoleInversionGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MysteryDipoleInversionGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MysteryDipoleInversionGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MysteryDipoleInversion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MysteryDipoleInversion", 7)?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.serialize_field("e4235", &self[crate::elements::e4235])?;
        state.serialize_field("e4315", &self[crate::elements::e4315])?;
        state.serialize_field("e4125", &self[crate::elements::e4125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MysteryDipoleInversion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MysteryDipoleInversionField {
            e23,
            e31,
            e12,
            e45,
            e4235,
            e4315,
            e4125,
        }
        struct MysteryDipoleInversionVisitor;
        impl<'de> Visitor<'de> for MysteryDipoleInversionVisitor {
            type Value = MysteryDipoleInversion;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MysteryDipoleInversion")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MysteryDipoleInversion, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e45 = None;
                let mut e4235 = None;
                let mut e4315 = None;
                let mut e4125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MysteryDipoleInversionField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e4235 => {
                            if e4235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4235"));
                            }
                            e4235 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e4315 => {
                            if e4315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4315"));
                            }
                            e4315 = Some(map.next_value()?);
                        }

                        MysteryDipoleInversionField::e4125 => {
                            if e4125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4125"));
                            }
                            e4125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MysteryDipoleInversion::from([0.0; 7]);
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                result[crate::elements::e4235] = e4235.ok_or_else(|| serde::de::Error::missing_field("e4235"))?;
                result[crate::elements::e4315] = e4315.ok_or_else(|| serde::de::Error::missing_field("e4315"))?;
                result[crate::elements::e4125] = e4125.ok_or_else(|| serde::de::Error::missing_field("e4125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e23", "e31", "e12", "e45", "e4235", "e4315", "e4125"];
        deserializer.deserialize_struct("MysteryDipoleInversion", FIELDS, MysteryDipoleInversionVisitor)
    }
}
impl std::ops::Index<crate::elements::e23> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e45> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e4235> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e4315> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e4125> for MysteryDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e4235> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e4235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e4315> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e4315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e4125> for MysteryDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e4125) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/mystery_dipole_inversion.rs");
