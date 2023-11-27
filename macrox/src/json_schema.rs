use std::collections::HashMap;

use heck::{AsPascalCase, AsSnakeCase};
use serde::{Deserialize, Serialize};

/// input data
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    pub properties: Option<HashMap<String, Schema>>,
}

/// output structure
pub struct St {
    /// structure name
    name: String,
    /// a list of structure fields
    fields: Vec<Fd>,
}

/// structure field
pub struct Fd {
    /// field name
    name: String,
    /// field type
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

impl Schema {
    pub fn into_st_vec(&self) -> Vec<St> {
        let mut structs = Vec::new();

        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(k.as_str(), v))
                    .collect();

                structs.push(St::new(pascal_case(self.title.as_ref().unwrap()), fields));

                structs
            }
            _ => panic!("Unsupported type: {:?}", self),
        }
    }
}

fn process_type(k: &str, v: &Schema) -> Fd {
    match v.ty.as_str() {
        "object" => {
            // need to create a new St, field type is the St name
            todo!()
        }
        "integer" => Fd::new(snake_case(k), "i64"),
        "float" => Fd::new(snake_case(k), "f64"),
        "string" => Fd::new(snake_case(k), "String"),
        _ => panic!("Unsupported type: {:?}", v),
    }
}

fn pascal_case(s: &str) -> String {
    AsPascalCase(s).to_string()
}

fn snake_case(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

mod tests {
    use super::*;

    const PERSON1: &str = include_str!("../fixtures/person1.json");
    #[test]
    fn schema_should_convert_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();

        let mut structs = schema.into_st_vec();
        assert_eq!(structs.len(), 1);

        let st = structs.pop().unwrap();
        assert_eq!(st.name, "Person");
        assert_eq!(st.fields.len(), 2);
        // assert_eq!(st.fields[0].name, "first_name");
        let mut fields: Vec<_> = st.fields.iter().map(|f| f.name.clone()).collect();
        fields.sort();
        assert_eq!(&fields[..], &["first_name", "last_name"]);
        assert_eq!(st.fields[0].ty, "String");
        assert_eq!(st.fields[1].ty, "String");
    }
}
