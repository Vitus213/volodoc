use std::env;
use std::process;

use idl_parser::parse_idl;

mod idl_parser;
//mod doc_generator;
//mod example_generator;
mod utils;
mod markdown_generator;

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_idl_file>", args[0]);
        process::exit(1);
    }

    let idl_file_path = &args[1];

    // 解析 IDL 文件
    let idl_content = match utils::read_idl_file(idl_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let file = parse_idl(&idl_content).unwrap();
    let doc =idl_parser::parse_handler(&file);
    println! ("{}", doc );






    // // 解析 IDL 文件内容
    // let parsed_idl = match idl_parser::parse_idl(&idl_content) {
    //     Ok(parsed) => parsed,
    //     Err(e) => {
    //         eprintln!("Failed to parse IDL file: {}", e);
    //         process::exit(1);
    //     }
    // };
    

    // 生成 API 文档
    // let api_doc = doc_generator::generate_api_doc(&parsed_idl);
    // println!("Generated API Documentation:\n{}", api_doc);

    // // 生成示例数据
    // let example_data = example_generator::generate_example_data(&parsed_idl);
    // println!("Generated Example Data:\n{}", example_data);



}