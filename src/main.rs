// Check folder contains transloco.config.js
// Read langs array
// Read rootTranslationPath and scopePathMap to create a map with "folder name" => path
// * rootTranslationPath starts from src folder whereas scopePathMap start from current directory

use std::{collections::HashMap, fs, path::Path};

use rslint_parser::{
    ast::{Literal, LiteralProp},
    parse_module, AstNode, SyntaxKind, SyntaxNodeExt,
};

fn main() {
    // Check folder contains transloco.config.js
    let config = std::fs::read_to_string("transloco.config.js")
        .expect("transloco.config.js file not found in current directory");

    let parse = parse_module(&config, 0);

    let script = parse.syntax().first_child().unwrap();
    let module = script.first_child().unwrap();
    let config_object = module
        .child_with_kind(rslint_parser::SyntaxKind::OBJECT_EXPR)
        .unwrap();

    let mut langs = Vec::<String>::new();
    let mut scopes = HashMap::<String, String>::new();
    for node in config_object.children().into_iter() {
        if node.kind().eq(&SyntaxKind::LITERAL_PROP) {
            let prop = LiteralProp::cast(node.to_owned()).expect("Mismatched literal prop");
            match prop.key() {
                Some(key) => {
                    if key.text().eq("rootTranslationsPath") {
                        let mut path = "src/".to_owned();
                        let value = prop
                            .value()
                            .expect("Missing rootTranslationsPath value")
                            .text()
                            .replace("'", "");
                        path.push_str(&value);

                        scopes.insert(String::from("assets"), path);
                    }
                    if key.text().eq("langs") {
                        node.child_with_kind(SyntaxKind::ARRAY_EXPR)
                            .expect("Missing langs array field")
                            .children()
                            .for_each(|lang| {
                                let lang =
                                    Literal::cast(lang.to_owned()).expect("Missing lang value");
                                let value = lang.to_string().replace("'", "");
                                langs.push(value);
                            });
                    }
                    if key.text().eq("scopePathMap") {
                        let paths = node
                            .child_with_kind(SyntaxKind::OBJECT_EXPR)
                            .expect("Missing scopePathMap object");
                        paths.children().for_each(|path_node| {
                            let folder = path_node
                                .child_with_kind(SyntaxKind::NAME)
                                .expect("Missing scopePathMap node name")
                                .to_string();
                            let path = path_node
                                .child_with_kind(SyntaxKind::LITERAL)
                                .expect("Missing scopePathMap node value")
                                .to_string()
                                .replace("'", "");

                            scopes.insert(folder, path);
                        });
                    }
                }
                _ => (),
            }
        }
    }

    println!("Found {} scopes: {:#?}", scopes.len(), scopes);
    println!("Found {} languages: {:#?}", langs.len(), langs);

    for (k, v) in scopes {
        if Path::new(&v).exists() {
            for lang in &langs {
                let mut path = v.to_owned();
                path.push_str("/");
                path.push_str(&lang);
                path.push_str(".json");
                if Path::new(&path).exists() {
                    println!("Found file {}", path);
                    let mut dest = "export_langs".to_owned();
                    dest.push_str("/");
                    dest.push_str(&k);
                    fs::create_dir_all(&dest).expect("Error creating directory");
                    dest.push_str("/");
                    dest.push_str(&lang);
                    dest.push_str(".json");
                    fs::copy(path, dest).expect("Something went wrong copying the file");
                } else {
                    println!("Missing language {}", path);
                }
            }
        } else {
            println!("Scope {} not found", k);
        }
    }
}
