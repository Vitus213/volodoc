use crate::idl_parser::{Service, Struct};
use pilota_thrift_parser::File;

/// 根据传入的 File、结构体列表和服务列表生成 Markdown 文档字符串
pub fn generate_markdown(file: &File, structs: &Vec<Struct>, services: &Vec<Service>) -> String {
    let mut doc = String::new();

    // 命名空间
    if let Some(pkg) = &file.package {
        let ns = pkg.segments
            .iter()
            .map(|seg| seg.to_string())
            .collect::<Vec<_>>()
            .join(".");
        doc.push_str("# API 文档\n\n## 命名空间\n\n");
        doc.push_str(&format!("- **{}**\n\n---\n\n", ns));
    }

    // 结构体
    doc.push_str("## 结构体\n\n");
    for s in structs {
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
    for service in services {
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

    doc
}