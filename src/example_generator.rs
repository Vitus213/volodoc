use serde_json::json;
use crate::idl_parser::{Service, Method, Struct, Field};

pub fn generate_example_data(service: &Service) -> String {
    let mut examples = Vec::new();

    for method in &service.methods {
        let request_example = generate_example_struct(&method.request);
        let response_example = generate_example_struct(&method.response);

        examples.push(json!({
            "method": method.name,
            "request": request_example,
            "response": response_example,
        }));
    }

    serde_json::to_string_pretty(&examples).unwrap()
}

fn generate_example_struct(struct_def: &Struct) -> serde_json::Value {
    let mut example = serde_json::Map::new();
    for field in &struct_def.fields {
        example.insert(field.name.clone(), generate_example_value(&field.type));
    }
    serde_json::Value::Object(example)
}

fn generate_example_value(r#type: &str) -> serde_json::Value {
    match r#type {
        "string" => json!("example_string"),
        "int" => json!(123),
        "bool" => json!(true),
        _ => json!(null),
    }
}