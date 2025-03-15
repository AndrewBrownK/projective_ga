use std::collections::HashSet;
use std::sync::Arc;

use anyhow::bail;

use crate::ast::traits::{RawTraitImplementation, TraitKey};

pub mod rust;
pub mod wesl;
pub mod sql;
pub mod edsl;
pub mod slang;
pub mod rust_on_gpu;

fn sort_trait_impls(trait_implementations: &mut Vec<Arc<RawTraitImplementation>>, mut already_ordered: HashSet<TraitKey>) -> anyhow::Result<()> {
    // Start with a sort by name, so we get stable outputs
    trait_implementations.sort_by(|a, b| {
        let a_key = a.definition.names.trait_key;
        let b_key = b.definition.names.trait_key;
        let key_cmp = a_key.cmp(&b_key);
        let std::cmp::Ordering::Equal = key_cmp else { return key_cmp };

        let owner_cmp = a.owner.cmp(&b.owner);
        let std::cmp::Ordering::Equal = owner_cmp else { return owner_cmp };

        let a_other = a.other_params.get(0);
        let b_other = b.other_params.get(0);
        a_other.cmp(&b_other)
    });

    // Ok now lets properly order the implementations that
    // are actually declared here.
    let mut needs_ordering: Vec<_> = trait_implementations.clone();
    let mut ordered_implementations = vec![];
    while !needs_ordering.is_empty() {
        let size_before = needs_ordering.len();
        let mut already_disqualified_this_pass = HashSet::new();
        needs_ordering.retain(|it| {
            let k = it.definition.names.trait_key;
            if already_ordered.contains(&k) {
                ordered_implementations.push(it.clone());
                return false;
            }
            if already_disqualified_this_pass.contains(&k) {
                return true;
            }
            let deps = it.definition.dependencies.lock();
            if deps.iter().all(|dep| already_ordered.contains(dep)) {
                already_ordered.insert(k);
                ordered_implementations.push(it.clone());
                return false;
            }
            already_disqualified_this_pass.insert(k);
            return true;
        });
        let size_after = needs_ordering.len();
        if size_before == size_after {
            bail!(
                "There is a missing dependency of a trait implementation. It needs to be \
                included/declared in this file, or else imported to this file."
            )
        }
    }
    *trait_implementations = ordered_implementations;
    Ok(())
}

//
