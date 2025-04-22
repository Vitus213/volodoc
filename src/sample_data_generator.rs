use std::collections::HashMap;
use pilota_thrift_parser::File;
use serde_json::{json, Value};

// 假定你已在 idl_parser 模块中定义了 Service、Struct、Field、extract_services 和 collect_structs 函数
use crate::idl_parser::{Struct as MyStruct, extract_services, collect_structs};

/// 根据字段类型生成示例值
fn get_sample_value(field_type: &str, struct_map: &HashMap<String, MyStruct>) -> Value {
    match field_type.to_lowercase().as_str() {
        "i64" | "int" | "integer" => json!(123),
        "string" => json!("example"),
        "bool" | "boolean" => json!(true),
        _ => {
            if field_type.starts_with("map<") {
                // 示例：针对 map<string, string> 返回 {"example_key": "example_value"}
                json!({"example_key": "example_value"})
            } else if field_type.starts_with("list<") {
                // 示例：返回一个数组，数组内的元素由 inner 类型生成
                let inner = field_type.trim_start_matches("list<").trim_end_matches('>');
                json!([get_sample_value(inner, struct_map)])
            } else if struct_map.contains_key(field_type) {
                // 如果当前类型是已定义的结构体，递归生成示例数据
                generate_sample_for_struct(struct_map.get(field_type).unwrap(), struct_map)
            } else {
                // 默认生成字符串示例
                json!("example")
            }
        }
    }
}

/// 根据结构体定义生成示例 JSON 对象（递归支持嵌套结构体）
/// 可选字段（optional）在测试中生成 null。
fn generate_sample_for_struct(s: &MyStruct, struct_map: &HashMap<String, MyStruct>) -> Value {
    let mut map = serde_json::Map::new();
    for field in &s.fields {
        if field.attribute.to_lowercase().contains("optional") {
            // 对于可选类型直接输出 null，不生成示例数据
            map.insert(field.name.clone(), json!(null));
        } else {
            // 必填字段生成实际示例数据
            let sample = get_sample_value(&field.r#type, struct_map);
            map.insert(field.name.clone(), sample);
        }
    }
    Value::Object(map)
}

/// 根据解析后的 File 对象生成示例数据
///
/// 生成的 JSON 对象按照服务 -> 方法 -> { request, response } 的层级组织，
/// 如果找不到对应的结构体，则对应示例数据为 null。
pub fn generate_sample_data(file: &File) -> String {
    // 通过已有函数提取所有服务
    let services = extract_services(file);
    // 收集所有结构体定义，便于查找，注意 collect_structs 需返回 Vec<MyStruct>
    let structs = collect_structs(file);
    let mut struct_map: HashMap<String, MyStruct> = HashMap::new();
    for s in &structs {
        struct_map.insert(s.name.clone(), (*s).clone());
    }

    let mut result = serde_json::Map::new();
    // 针对每个服务生成示例数据
    for service in services {
        let mut service_obj = serde_json::Map::new();
        for method in service.methods {
            let request_value = if struct_map.contains_key(&method.request.name) {
                generate_sample_for_struct(struct_map.get(&method.request.name).unwrap(), &struct_map)
            } else {
                json!(null)
            };
            let response_value = if struct_map.contains_key(&method.response.name) {
                generate_sample_for_struct(struct_map.get(&method.response.name).unwrap(), &struct_map)
            } else {
                json!(null)
            };
            // 使用方法名称作为 key
            service_obj.insert(
                method.name,
                json!({
                    "request": request_value,
                    "response": response_value,
                }),
            );
        }
        result.insert(service.name, Value::Object(service_obj));
    }
    Value::Object(result).to_string()
}