use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum StringOrVecString {
    String(String),
    VecString(Vec<String>),
}

impl StringOrVecString {
    pub fn push(&mut self, s: String) {
        match self {
            StringOrVecString::String(s1) => {
                let v = vec![s1.clone(), s];
                *self = StringOrVecString::VecString(v);
            }
            StringOrVecString::VecString(v) => {
                v.push(s);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TOrVecT<T> {
    T(T),
    VecT(Vec<T>),
}

impl<T: Clone> TOrVecT<T> {
    pub fn push(&mut self, s: T) {
        match self {
            TOrVecT::T(s1) => {
                let v = vec![s1.clone(), s];
                *self = TOrVecT::VecT(v);
            }
            TOrVecT::VecT(v) => {
                v.push(s);
            }
        }
    }
}
