use crate::value::Value;

#[derive(Debug, Clone)]
pub struct TaggedValue {
    pub tag: String,
    pub value: Value,
}

impl TaggedValue {
    pub fn new(tag: impl Into<String>, value: Value) -> Self {
        Self { tag: tag.into(), value }
    }
}

