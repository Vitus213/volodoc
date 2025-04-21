use pilota_thrift_parser::parser::Parser;
use pilota_thrift_parser::{File, Item};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File as StdFile;
use std::io::Write;

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

    let mut doc = String::new();

    // 命名空间
    if let Some(pkg) = &file.package {
        let ns = pkg.segments.iter().map(|seg| seg.to_string()).collect::<Vec<_>>().join(".");
        doc.push_str("# API 文档\n\n## 命名空间\n\n");
        doc.push_str(&format!("- **{}**\n\n---\n\n", ns));
    }

    // 结构体
    doc.push_str("## 结构体\n\n");
    for s in &structs {
        doc.push_str(&format!("### {}\n\n", s.name));
        doc.push_str("| 字段名 | 类型 | 必填 | 说明 |\n");
        doc.push_str("|--------|------|------|------|\n");
        for f in &s.fields {
            let required = if f.attribute.contains("Required") { "是" } else { "否" };
            doc.push_str(&format!("| {} | {} | {} | |\n", f.name, f.r#type, required));
        }
        doc.push_str("\n---\n\n");
    }

    // 服务
    doc.push_str("## 服务\n\n");
    for service in &services {
        doc.push_str(&format!("### {}\n\n#### 方法\n\n", service.name));
        for method in &service.methods {
            doc.push_str(&format!("##### {}\n\n", method.name));
            doc.push_str(&format!("- **请求参数**：{}\n", method.request.name));
            for f in &method.request.fields {
                let required = if f.attribute.contains("Required") { "是" } else { "否" };
                doc.push_str(&format!("    - {}: {} ({})\n", f.name, f.r#type, required));
            }
            doc.push_str(&format!("\n- **返回结果**：{}\n", method.response.name));
            for f in &method.response.fields {
                let required = if f.attribute.contains("Required") { "是" } else { "否" };
                doc.push_str(&format!("    - {}: {} ({})\n", f.name, f.r#type, required));
            }
            doc.push_str("\n---\n\n");
        }
    }
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
    // 先收集所有结构体，方便查找
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
                // 请求结构体
                let req_struct = if let Some(arg) = func.arguments.get(0) {
                    let type_str = format!("{:?}", arg.ty);

                    structs
                        .get(&type_str)
                        .map(|st| struct_to_struct(st))
                        .unwrap_or_else(|| Struct {
                            name: type_str.clone(),
                            fields: vec![],
                        })
                } else {
                    Struct::default()
                };

                // 返回结构体
                let resp_type_str = format!("{:?}", func.result_type);
                let resp_struct = structs
                    .get(&resp_type_str)
                    .map(|st| struct_to_struct(st))
                    .unwrap_or_else(|| Struct {
                        name: resp_type_str.clone(),
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
fn struct_to_struct(st: &pilota_thrift_parser::StructLike) -> Struct {
    Struct {
        name: st.name.0.to_string(),
        fields: st
            .fields
            .iter()
            .map(|f| Field {
                name: f.name.0.to_string(),
                r#type: format!("{:?}", f.ty),
                attribute: format!("{:?}", f.attribute),
            })
            .collect(),
    }
}
