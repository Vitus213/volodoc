use std::fs;
use std::path::Path;
use tera::{Context, Tera};
use crate::idl_parser::{Service, Struct};
use pilota_thrift_parser::File as DocFile;

// 引入 rust-embed，用于嵌入模板文件
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Asset;

/// 生成 API 文档（包含结构体和服务信息）并保存到 result 文件夹中，然后返回 Markdown 字符串
pub fn doc_handler(file: &DocFile, structs: &Vec<Struct>, services: &Vec<Service>) -> String {
    let api_doc = generate_api_doc(file, structs, services);
    
    // 这里写入文件的逻辑可以根据需要处理，这里仅返回生成后的 Markdown
    api_doc
}

/// 根据 file、structs、services 数据渲染模板并返回生成的 API Markdown 文档
/// 该模板中同时包含结构体和服务部分
pub fn generate_api_doc(_file: &DocFile, structs: &Vec<Struct>, services: &Vec<Service>) -> String {
    // 从嵌入的资源中加载模板内容
    let template_data = Asset::get("api_template.md").expect("找不到 api_template.md 模板");
    let template_str = std::str::from_utf8(template_data.data.as_ref()).expect("模板内容不是有效 utf8");

    // 创建一个 Tera 实例并手动添加模板
    let mut tera = Tera::default();
    tera.add_raw_template("api_template.md", template_str).expect("添加模板失败");

    // 构造上下文数据
    let mut context = Context::new();
    context.insert("package", "rs.volo.example");
    context.insert("structs", structs);
    context.insert("services", services);

    tera.render("api_template.md", &context).expect("渲染 API 模板失败")
}