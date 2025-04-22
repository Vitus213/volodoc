use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod idl_parser;
mod sample_data_generator;
mod markdown_generator;
mod  doc_generator;


fn main() {
    // 从命令行参数中获取 IDL 文件路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <idl 文件路径>", args[0]);
        process::exit(1);
    }
    let idl_path = &args[1];
    let idl_content = match fs::read_to_string(idl_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("读取 IDL 文件失败: {}", e);
            process::exit(1);
        }
    };

    // 解析 IDL 文件
    let file = match idl_parser::parse_idl(&idl_content) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("解析 IDL 失败: {}", e);
            process::exit(1);
        }
    };

    // 生成 API 文档（Markdown 格式）和示例数据（JSON 格式）
    let api_doc = idl_parser::parse_handler(&file);
    let sample_data = sample_data_generator::generate_sample_data(&file);

    // 确保保存结果的文件夹存在（文件夹名为 result）
    let result_dir = "result";
    if !Path::new(result_dir).exists() {
        if let Err(e) = fs::create_dir_all(result_dir) {
            eprintln!("创建文件夹 {} 失败: {}", result_dir, e);
            process::exit(1);
        }
    }

    // 保存 API 文档到 result/api_doc.md
    let api_doc_path = format!("{}/api_doc.md", result_dir);
    if let Err(e) = fs::write(&api_doc_path, &api_doc) {
        eprintln!("写入 API 文档失败: {}", api_doc_path);
        process::exit(1);
    }
    println!("API 文档生成成功，文件：{}", api_doc_path);

    // 保存示例数据到 result/sample_data.json
    let sample_data_path = format!("{}/sample_data.json", result_dir);
    if let Err(e) = fs::write(&sample_data_path, &sample_data) {
        eprintln!("写入示例数据失败: {}", sample_data_path);
        process::exit(1);
    }
    println!("示例数据生成成功，文件：{}", sample_data_path);
}