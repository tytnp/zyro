use std::{env, fs};
use std::path::Path;
use syn::{Fields, Item};


fn main() {
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let parse_path = Path::new(&current_dir).parent().unwrap().join("entity").join("src").join("model");
    for entry in fs::read_dir(parse_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let content = &fs::read_to_string(path).unwrap();
            let ast = syn::parse_file(&content).unwrap();
            for item in ast.items {
                if let Item::Struct(struct_item) = item {
                    println!("Found struct: {}", struct_item.ident);
                    // 遍历结构体的字段
                    if let Fields::Named(fields) = &struct_item.fields {
                        for field in &fields.named {
                            // 输出字段的名称和类型
                            let field_name = &field.ident.as_ref().expect("Unnamed field");
                            let field_type = &field.ty;

                            // Convert type to a string
                            let type_string = quote::quote! {#field_type}.to_string();

                            println!("  Field: {} - Type: {}", field_name, type_string);
                        }
                    }
                }
            }
        }
    }
}




