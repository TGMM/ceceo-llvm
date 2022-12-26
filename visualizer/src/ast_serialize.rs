#![allow(non_upper_case_globals)]

use parser::ast::{Atom, Node};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(untagged, remote = "Atom")]
pub enum AtomS {
    Num(i32),
    Symbol(String),
    Str(String),
    Bool(bool),
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
    #[serde(with = "AtomS")]
    value: Box<Atom>,
}

pub trait ToInfoStruct {
    fn to_info_struct(&self) -> ListOrAtomInfo;
}

fn nodes_to_info_structs(vn: &[Node]) -> ListOrAtomInfo {
    let mut res: Vec<ListOrAtomInfo> = vec![];
    vn.iter().for_each(|n| res.push(n.to_info_struct()));

    return ListOrAtomInfo::List(res);
}

impl ToInfoStruct for Vec<Vec<Node>> {
    fn to_info_struct(&self) -> ListOrAtomInfo {
        ListOrAtomInfo::List(self.iter().map(|n| nodes_to_info_structs(n)).collect())
    }
}

impl ToInfoStruct for Vec<Node> {
    fn to_info_struct(&self) -> ListOrAtomInfo {
        nodes_to_info_structs(self)
    }
}

impl ToInfoStruct for Node {
    fn to_info_struct(&self) -> ListOrAtomInfo {
        // TODO: Quote atom and list should display a ' before they're printed
        // and their values inside the json should reflect their new type
        return match self {
            Node::Atom(a) | Node::QuoteAtom(a) => match a {
                Atom::Num(num) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Num.to_string(),
                    value: Box::new(Atom::Num(*num)),
                }),
                Atom::Symbol(symbol) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Symbol.to_string(),
                    value: Box::new(Atom::Symbol(symbol.to_owned())),
                }),
                Atom::Str(str) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: Str.to_string(),
                    value: Box::new(Atom::Str(str.to_owned())),
                }),
                Atom::Bool(b) => ListOrAtomInfo::Atom(InfoStruct {
                    r#type: b.to_string(),
                    value: Box::new(Atom::Bool(b.to_owned())),
                }),
            },
            Node::List(list) | Node::QuoteList(list) => {
                return nodes_to_info_structs(list);
            }
        };
    }
}
