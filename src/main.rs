use std::collections::BTreeMap;

use serde::*;
use uiua::primitive::{Primitive, PrimClass};

fn get_prim_class(prim: Primitive) -> &'static str {
    macro_rules! code_font {
        ($class:literal) => {
            concat!("code-font ", $class)
        };
    }

    match prim {
        Primitive::Transpose => code_font!("monadic-function-button trans"),
        prim if prim.class() == PrimClass::Stack && prim.modifier_args().is_none() => {
            code_font!("stack-function-button")
        }
        prim => {
            if let Some(m) = prim.modifier_args() {
                if m == 1 {
                    code_font!("modifier1-button")
                } else {
                    code_font!("modifier2-button")
                }
            } else {
                match prim.args() {
                    Some(0) => code_font!("noadic-function-button"),
                    Some(1) => code_font!("monadic-function-button"),
                    Some(2) => code_font!("dyadic-function-button"),
                    Some(3) => code_font!("triadic-function-button"),
                    _ => code_font!("variadic-function-button"),
                }
            }
        }
    }
}

fn main() {
    let mut map = BTreeMap::new();
    for prim in Primitive::non_deprecated() {
        if let Some(names) = prim.names() {
            if names.glyph.is_none() { // Don't include system functions
                continue;
            }
            map.insert(
                names.text,
                PrimData {
                    glyph: names.glyph,
                    description: prim
                        .doc()
                        .map(|doc| doc.short_text())
                        .unwrap_or_default()
                        .into(),
                    class: get_prim_class(prim)
                },
            );
        }
    }
    let json = serde_json::to_string_pretty(&map).unwrap();
    std::fs::write("uiua_functions.json", json).unwrap();
}

#[derive(Serialize)]
struct PrimData {
    #[serde(skip_serializing_if = "Option::is_none")]
    glyph: Option<char>,
    description: String,
    class: &'static str,
}
