#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::missing_errors_doc)]

pub mod ast_serialize;

use ast_serialize::ToInfoStruct;
use parser::parse_ceceo;

fn main() {
    let node_list = parse_ceceo("(+ (* 2 2) (+ 5 1))").unwrap();
    let json_node_list = serde_json::to_string(&node_list.to_info_struct()).unwrap();

    println!("{json_node_list}");
}
