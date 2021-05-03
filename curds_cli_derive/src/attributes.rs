use syn::{Attribute, NestedMeta, Meta, MetaList, Lit, LitStr, parse_macro_input};

pub fn parse_attributes(key: &str, attributes: &Vec<Attribute>) -> Vec<LitStr> {
    let mut nested_metas = Vec::<LitStr>::new();
    for attribute in attributes {
        if attribute.path.is_ident(key) {
            match attribute.parse_meta().unwrap() {
                Meta::List(metalist) => nested_metas.extend(extract_string_literals(metalist)),
                _ => panic!("Unsupported name attribute type"),
            }
        }
    }
    nested_metas
}

fn extract_string_literals(metalist: MetaList) -> Vec<LitStr> {
    let mut nested_metas = Vec::<LitStr>::new();
    for nested in metalist.nested {
        match nested {
            NestedMeta::Lit(literal) => {
                match literal {
                    Lit::Str(string_literal) => nested_metas.push(string_literal),
                    _ => panic!("Only string literals are allowed")
                }
            }
            _ => panic!("Only string literals are allowed")
        }
    }
    nested_metas
}