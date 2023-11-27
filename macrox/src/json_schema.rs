use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};
use askama::Template;
use heck::{AsPascalCase, AsSnakeCase};
use litrs::Literal;
use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};

pub fn get_string_literal(input: TokenStream) -> Result<String> {
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only support string literal"))
    // let tt = input.into_iter().next().unwrap();

    // match tt {
    //     TokenTree::Literal(v) => {
    //         Ok(v.to_string())
    //     },
    //     _  => Err(anyhow!("Only support string literal")),
    // }
}

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructeTemplate {
    structs: Vec<St>,
}

impl StructeTemplate {
    fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema = serde_json::from_str::<Schema>(&content)?;
        Ok(Self {
            structs: schema.to_st_vec(),
        })
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

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
    pub fn to_st_vec(&self) -> Vec<St> {
        let mut structs = Vec::new();

        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_str(), v))
                    .collect();

                structs.push(St::new(pascal_case(self.title.as_ref().unwrap()), fields));

                structs
            }
            _ => panic!("Unsupported type"),
        }
    }
}

fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    let name = snake_case(k);
    match v.ty.as_str() {
        "object" => {
            // need to create a new St, field type is the St name
            let sts = v.to_st_vec();
            structs.extend(sts);

            Fd::new(name, gen_name(v.title.as_deref(), k))
        }
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "string" => Fd::new(name, "String"),
        _ => panic!("Unsupported type: {:?}", v),
    }
}

fn pascal_case(s: &str) -> String {
    AsPascalCase(s).to_string()
}

fn snake_case(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

fn gen_name(first: Option<&str>, second: &str) -> String {
    pascal_case(first.unwrap_or(second))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERSON1: &str = include_str!("../fixtures/person1.json");
    const PERSON2: &str = include_str!("../fixtures/person2.json");

    #[test]
    fn schema_should_convert_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();

        let mut structs = schema.to_st_vec();
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

    #[test]
    fn schema_with_nested_struct_should_convert_to_st() {
        let schema: Schema = serde_json::from_str(PERSON2).unwrap();

        let structs = schema.to_st_vec();
        assert_eq!(structs.len(), 2);
    }

    #[test]
    fn schema_render_should_works() {
        let template = StructeTemplate::render("fixtures/person1.json").unwrap();
        println!("{:#?}", template);
    }
}
