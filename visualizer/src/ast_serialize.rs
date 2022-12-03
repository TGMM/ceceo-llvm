#![allow(non_upper_case_globals)]

use ceceo_llvm_parser::ast::Node;
use serde::Serialize;

// TODO: Find a way to unify this, so it's not repeated between crates
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Atom {
    Num(i32),
    Symbol(String),
    Str(String),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ListOrAtomInfo {
    List(Vec<ListOrAtomInfo>),
    Atom(InfoStruct),
}

const Num: &str = "number";
const Symbol: &str = "symbol";
const Str: &str = "string";

#[derive(Clone, Serialize)]
pub struct InfoStruct {
    r#type: String,
    value: Box<Atom>,
}

pub trait ToInfoStruct {
    fn to_info_struct(&self) -> ListOrAtomInfo;
}

fn nodes_to_info_structs(vn: &Vec<Node>) -> ListOrAtomInfo {
    let mut res: Vec<ListOrAtomInfo> = vec![];
    vn.iter().for_each(|n| res.push(n.to_info_struct()));

    return ListOrAtomInfo::List(res);
}

impl ToInfoStruct for Vec<Node> {
    fn to_info_struct(&self) -> ListOrAtomInfo {
        nodes_to_info_structs(self)
    }
}

impl ToInfoStruct for Node {
    fn to_info_struct(&self) -> ListOrAtomInfo {
        return match self {
            Node::Atom(a) => match a {
                ceceo_llvm_parser::ast::Atom::Num(num) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Num.to_string(),
                    value: Box::new(Atom::Num(*num)),
                }),
                ceceo_llvm_parser::ast::Atom::Symbol(symbol) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Symbol.to_string(),
                    value: Box::new(Atom::Symbol(symbol.to_owned())),
                }),
                ceceo_llvm_parser::ast::Atom::Str(str) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Str.to_string(),
                    value: Box::new(Atom::Str(str.to_owned())),
                }),
            },
            Node::List(list) => {
                return nodes_to_info_structs(list);
            }
        };
    }
}
