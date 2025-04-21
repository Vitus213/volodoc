use tera::{Tera, Context};
use crate::idl_parser::{Service, Method, Struct, Field};

pub fn generate_api_doc(service: &Service) -> String {
    // 加载模板文件
    let tera = Tera::new("src/templates/**/*").unwrap();

    // 准备模板上下文
    let mut context = Context::new();
    context.insert("service", service);

    // 渲染模板
    let rendered = tera.render("api_template.md", &context).unwrap();
    rendered
}