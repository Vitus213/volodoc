use pilota_thrift_parser::parser::Parser;
use pilota_thrift_parser::{File, Item};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File as StdFile;
use std::io::Write;

// 如果项目的模块结构允许，可以使用如下方式引入新的模块：

use crate::markdown_generator::generate_markdown;

#[derive(Debug, Serialize)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize)]
pub struct Method {
    pub name: String,
    pub request: Struct,
    pub response: Struct,
}

#[derive(Debug, Serialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Default for Struct {
    fn default() -> Self {
        Struct {
            name: "".to_string(),
            fields: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub attribute: String,
}

/// 传入解析后的 File，整理出一份新的 API 文档（Markdown 格式字符串）
pub fn parse_handler(file: &File) -> String {
    let structs = collect_structs(file);
    let services = extract_services(file);

    let doc = generate_markdown(file, &structs, &services);
    let _ = std::fs::write("api_doc.md", &doc);
    doc
}

/// 解析 Thrift IDL 文件内容
/// 该函数会解析 Thrift IDL 内容并将其结果保存到本地文件，同时返回解析后的 `File` 对象
pub fn parse_idl(idl_content: &str) -> Result<File, String> {
    match File::parse(idl_content) {
        Ok((_, file)) => {
            // 将解析结果保存到文件，方便后续调试
            let debug_str = format!("{:#?}", file);
            let mut output = StdFile::create("output").map_err(|e| e.to_string())?;
            output
                .write_all(debug_str.as_bytes())
                .map_err(|e| e.to_string())?;

            Ok(file)
        }
        Err(e) => Err(format!("解析 IDL 失败: {:?}", e)),
    }
}

/// 从解析后的 `File` 对象中提取服务信息
/// /// 收集所有结构体及其字段信息
pub fn collect_structs(file: &File) -> Vec<Struct> {
    file.items
        .iter()
        .filter_map(|item| {
            if let Item::Struct(st) = item {
                Some(struct_to_struct(st))
            } else {
                None
            }
        })
        .collect()
}

/// 遍历所有的服务定义，并提取出每个服务的名称和方法定义
pub fn extract_services(file: &File) -> Vec<Service> {
    // 先收集所有结构体，方便查找，注意这里 key 为结构体名称
    let structs: HashMap<String, _> = file
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Struct(st) = item {
                Some((st.name.0.to_string(), st))
            } else {
                None
            }
        })
        .collect();

    let mut services = Vec::new();
    for item in &file.items {
        if let Item::Service(s) = item {
            let mut methods = Vec::new();

            for func in &s.functions {
                // 请求结构体：提取简化名称后，再匹配是否存在对应的结构体
                let req_struct = if let Some(arg) = func.arguments.first() {
                    let type_str = simplify_type(&arg.ty);
                    structs.get(&type_str)
                        .map(|st| struct_to_struct(st))
                        .unwrap_or_else(|| Struct {
                            name: type_str,
                            fields: vec![],
                        })
                } else {
                    Struct::default()
                };

                // 返回结构体：同样提取简化名称
                let resp_type_str = simplify_type(&func.result_type);
                let resp_struct = structs.get(&resp_type_str)
                    .map(|st| struct_to_struct(st))
                    .unwrap_or_else(|| Struct {
                        name: resp_type_str,
                        fields: vec![],
                    });

                methods.push(Method {
                    name: func.name.0.to_string(),
                    request: req_struct,
                    response: resp_struct,
                });
            }
     
            services.push(Service {
                name: s.name.0.to_string(),
                methods,
            });
        }
    }
    services
}

/// 将 `pilota_thrift_parser::StructLike` 转换为自定义的 `Struct` 类型
/// 该函数将 Thrift 结构体的字段和类型转换为简化后的数据结构
fn simplify_type(ty: &impl std::fmt::Debug) -> String {
    let debug_str = format!("{:?}", ty);

    // 处理 Map 类型：例如
    // "Map { key: Type(String, Annotations([])), value: Type(String, Annotations([])) }"
    if debug_str.contains("Map") && debug_str.contains("key:") && debug_str.contains("value:") {
        let key_type = {
            if let Some(key_pos) = debug_str.find("key:") {
                if let Some(type_pos) = debug_str[key_pos..].find("Type(") {
                    let key_type_start = key_pos + type_pos + "Type(".len();
                    if let Some(comma_pos) = debug_str[key_type_start..].find(',') {
                        debug_str[key_type_start..key_type_start+comma_pos]
                            .trim()
                            .to_lowercase()
                    } else {
                        "".to_string()
                    }
                } else { "".to_string() }
            } else { "".to_string() }
        };

        let value_type = {
            if let Some(val_pos) = debug_str.find("value:") {
                if let Some(val_type_pos) = debug_str[val_pos..].find("Type(") {
                    let value_type_start = val_pos + val_type_pos + "Type(".len();
                    if let Some(val_comma_pos) = debug_str[value_type_start..].find(',') {
                        debug_str[value_type_start..value_type_start+val_comma_pos]
                            .trim()
                            .to_lowercase()
                    } else {
                        "".to_string()
                    }
                } else { "".to_string() }
            } else { "".to_string() }
        };

        if !key_type.is_empty() && !value_type.is_empty() {
            return format!("map<{}, {}>", key_type, value_type);
        }
    }

    // 处理 List 类型，例如 "List(Type(String, Annotations([])))"
    if debug_str.contains("List(") {
       if let Some(list_pos) = debug_str.find("List(") {
           let list_content = &debug_str[list_pos + "List(".len()..];
           if let Some(end) = list_content.find(')') {
              // 递归调用 simplify_type 处理内部类型
              let inner = &list_content[..end];
              let inner_type = simplify_type(&inner);
              return format!("list<{}>", inner_type.to_lowercase());
           }
       }
    }

    // 处理 Path 格式类型：例如
    // "Path(Path { segments: [Ident("Item")] })" => 返回 "Item"
    if debug_str.contains("Path") {
        if let Some(ident_pos) = debug_str.find("Ident(") {
            let start = ident_pos + "Ident(".len();
            if let Some(end) = debug_str[start..].find(')') {
                let type_name = &debug_str[start..start+end];
                return type_name.trim_matches('"').to_string();
            }
        }
    }
    
    // 处理 Type(...) 格式，例如 "Type(I64, Annotations([]))" => 返回 "I64"
    if debug_str.starts_with("Type(") {
        if let Some(comma) = debug_str.find(',') {
            // 跳过前面的 "Type(" (5字符)
            let inner = &debug_str[5..comma];
            return inner.trim().to_string();
        }
    }
    
    // 默认直接返回 debug 字符串
    debug_str
}

fn struct_to_struct(st: &pilota_thrift_parser::StructLike) -> Struct {
    Struct {
        name: st.name.0.to_string(),
        fields: st
            .fields
            .iter()
            .map(|f| Field {
                name: f.name.0.to_string(),
                r#type: simplify_type(&f.ty),
                attribute: format!("{:?}", f.attribute),
            })
            .collect(),
    }
}
