use std::collections::HashSet;

use crate::cmeta::CMetaValue;

use super::cmeta;

pub fn merge_vec<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    result.extend(a);
    result.extend(b);
    result
}

pub fn merge_uses<Key: Into<String>, IterKey: IntoIterator<Item = Key>>(keys: IterKey) -> HashSet<String> {
    let res = HashSet::new();

    keys.into_iter().fold(res, |mut acc, key| {
        let uses = cmeta::CMeta::get_stack(key);
        if let Some(CMetaValue::Array(uses)) = uses {
            uses.iter().for_each(|use_| {
                if let CMetaValue::String(use_) = use_ {
                    acc.insert(use_.clone());
                }
            });
        }
        acc
    })
}
