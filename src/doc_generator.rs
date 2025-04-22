use std::fs;
use std::path::Path;
use tera::{Tera, Context};
use crate::idl_parser::{Service, Struct};
use pilota_thrift_parser::File as DocFile;

/// 生成 API 文档（包含结构体和服务信息）并保存到 result 文件夹中，然后返回 Markdown 字符串
pub fn doc_handler(file: &DocFile, structs: &Vec<Struct>, services: &Vec<Service>) -> String {
    let api_doc = generate_api_doc(file, structs, services);
    
    let result_dir = "result";
    if !Path::new(result_dir).exists() {
        fs::create_dir_all(result_dir).expect("创建 result 文件夹失败");
    }
    
    let api_file_path = format!("{}/api_doc.md", result_dir);
    fs::write(&api_file_path, &api_doc).expect("写入 API markdown 文件失败");
    
    api_doc
}

/// 根据 file、structs、services 数据渲染模板并返回生成的 API Markdown 文档
/// 该模板中同时包含结构体和服务部分
pub fn generate_api_doc(_file: &DocFile, structs: &Vec<Struct>, services: &Vec<Service>) -> String {
    let tera = Tera::new("src/templates/**/*").expect("加载模板失败");
    let mut context = Context::new();
    // 传入 package 信息（可根据实际需求动态获取）
    context.insert("package", "rs.volo.example");
    context.insert("structs", structs);
    context.insert("services", services);
    tera.render("api_template.md", &context).expect("渲染 API 模板失败")
}