use crate::data::*;
use crate::simd::*;

/// DualNumOnOrigin.
/// This variant of DualNum intersects the Origin.
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub union DualNumOnOrigin {
    groups: DualNumOnOriginGroups,
    /// e12345, 0, 0, 0
    elements: [f32; 4],
}
#[derive(Clone, Copy, nearly::NearlyEq, nearly::NearlyOrd, bytemuck::Pod, bytemuck::Zeroable, encase::ShaderType, serde::Serialize, serde::Deserialize)]
pub struct DualNumOnOriginGroups {
    /// e12345
    g0: f32,
}
impl DualNumOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e12345: f32) -> Self {
        Self {
            elements: [e12345, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self {
            groups: DualNumOnOriginGroups { g0 },
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
const DUAL_NUM_ON_ORIGIN_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for DualNumOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DUAL_NUM_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for DualNumOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DUAL_NUM_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<DualNumOnOrigin> for [f32; 1] {
    fn from(vector: DualNumOnOrigin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for DualNumOnOrigin {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for DualNumOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("DualNumOnOrigin").field("e12345", &self[0]).finish()
    }
}

impl DualNumOnOrigin {
    pub const LEN: usize = 1;
}

impl DualNumOnOrigin {
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

impl PartialOrd for DualNumOnOrigin {
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
impl Ord for DualNumOnOrigin {
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
impl PartialEq for DualNumOnOrigin {
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
impl Eq for DualNumOnOrigin {}
impl std::hash::Hash for DualNumOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

impl std::ops::Index<crate::elements::e12345> for DualNumOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for DualNumOnOrigin {
    fn index_mut(&self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/dual_num_on_origin.rs");
