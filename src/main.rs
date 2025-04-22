use std::env;
use std::fs;
use std::fs::read_dir;
use std::path::Path;
use std::process;

mod idl_parser;
mod doc_generator;
mod sample_data_generator;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 如果传入 --version，则输出版本号并退出
    if args.len() >= 2 && args[1] == "--version" {
        println!("volodoc version {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // 单文件模式：如果传入 doc_generator 且指定了文件路径，则只处理该文件
    if args.len() >= 2 {
        match args[1].as_str() {
            "doc_generator" => {
                if args.len() >= 3 {
                    // 只处理用户指定的单个文件
                    process_file(&args[2]);
                } else {
                    // 未指定具体的文件时，扫描 idl/ 目录下所有 .thrift 文件处理
                    process_directory("idl");
                }
                return;
            }
            _ => {
                eprintln!("不支持的命令：{}", args[1]);
                process::exit(1);
            }
        }
    }

    // 未传入参数时，也默认遍历 idl 目录下所有 .thrift 文件
    process_directory("idl");
}

/// 遍历指定的目录，处理其中所有以 .thrift 为后缀的文件
fn process_directory(idl_dir: &str) {
    let out_dir = "volodoc"; // 输出目录
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("创建 volodoc 文件夹失败");
    }
    
    let entries = read_dir(idl_dir).unwrap_or_else(|_| {
        eprintln!("读取 {} 目录失败", idl_dir);
        process::exit(1);
    });

    for entry in entries {
        let entry = entry.expect("读取目录项失败");
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "thrift" {
                let idl_path = path.to_str().unwrap().to_string();
                process_file(&idl_path);
            }
        }
    }
}

/// 处理单个 IDL 文件，生成 API 文档和示例数据
fn process_file(idl_path: &str) {
    let content = fs::read_to_string(idl_path).unwrap_or_else(|_| {
        eprintln!("读取 IDL 文件失败: {}", idl_path);
        process::exit(1);
    });

    let file = idl_parser::parse_idl(&content).unwrap_or_else(|e| {
        eprintln!("解析 IDL 失败: {}", e);
        process::exit(1);
    });

    // 生成 API 文档（Markdown 格式）
    let api_markdown = idl_parser::parse_handler(&file);
    // 生成示例数据
    let sample_data = sample_data_generator::generate_sample_data(&file);

    let filename = Path::new(idl_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let api_out_path = format!("volodoc/{}_api.md", filename);
    let sample_out_path = format!("volodoc/{}_test.md", filename);

    fs::write(&api_out_path, api_markdown)
        .unwrap_or_else(|e| { eprintln!("写入 API 文档失败: {}", e); process::exit(1); });
    fs::write(&sample_out_path, sample_data)
        .unwrap_or_else(|e| { eprintln!("写入示例数据失败: {}", e); process::exit(1); });

    println!("处理 {} 成功，生成文件:\n  {}\n  {}", idl_path, api_out_path, sample_out_path);
}