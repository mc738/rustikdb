use crate::blocks::Block;
use crate::data::{Data, DateTime};

struct Object {
    name: String,
    fields: Vec<Field>,
}

struct Field {
    key: String,
    value: FieldValue,
}

enum FieldType {
    Scalar(DataType),
    Array,
    Object,
    Collection,
}

enum FieldValue {
    Scalar(Data),
    Array(Vec<Data>),
    Object(Object),
    Collection(Vec<Object>),
}

impl Object {
    pub fn get_size(&self, include_field_names: bool) -> usize {
        self.fields
            .iter()
            .fold(0, |size, f| size + f.get_size(include_field_names))
    }
}

impl Field {
    pub fn deseralize_value() {}

    pub fn get_size(&self, include_field_names: bool) -> usize {
        let start = match include_field_names {
            true => self.name.len(),
            false => 0,
        };

        start + self.value.get_size(include_field_names)
    }
}

impl FieldValue {
    pub fn get_size(&self, include_field_names: bool) -> usize {
        match self {
            FieldValue::Scalar(v) => v.get_length(),
            FieldValue::Array(vs) => vs.iter().fold(0, |acc, t| acc + t),
            FieldValue::Object(o) => o.get_size(include_field_names),
            FieldValue::Collection(oc) => oc
                .iter()
                .fold(0, |acc, o| acc + o.get_size(include_field_names)),
        }
    }
}

impl FieldValue {
    pub fn deseralize(raw: &[u8], field_type: FieldType) -> FieldValue {
        match field_type {
            FieldType::Scalar(t) => match t {},
            FieldType::Array => todo!(),
            FieldType::Object => todo!(),
            FieldType::Collection => todo!(),
        }
    }

    pub fn get_type_code(&self) -> u8 {
        match self {
            FieldValue::Scalar(_) => 1,
            FieldValue::Array(_) => 2,
            FieldValue::Object(_) => 3,
            FieldValue::Collection(_) => 4,
            _ => 0,
        }
    }
}
