#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_idl() {
        let idl_content = r#"
            service ExampleService {
                string example_method(1: string param1)
            }
        "#;

        let parsed_idl = idl_parser::parse_idl(idl_content).unwrap();
        assert_eq!(parsed_idl.name, "ExampleService");
        assert_eq!(parsed_idl.methods.len(), 1);
    }

    #[test]
    fn test_generate_api_doc() {
        let service = idl_parser::Service {
            name: "ExampleService".to_string(),
            methods: vec![idl_parser::Method {
                name: "example_method".to_string(),
                request: idl_parser::Struct {
                    fields: vec![idl_parser::Field {
                        name: "param1".to_string(),
                        r#type: "string".to_string(),
                    }],
                },
                response: idl_parser::Struct {
                    fields: vec![idl_parser::Field {
                        name: "result".to_string(),
                        r#type: "string".to_string(),
                    }],
                },
            }],
        };

        let api_doc = doc_generator::generate_api_doc(&service);
        assert!(api_doc.contains("ExampleService"));
        assert!(api_doc.contains("example_method"));
    }
}