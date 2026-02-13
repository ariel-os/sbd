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
