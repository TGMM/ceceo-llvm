pub mod ast_serialize;

use ast_serialize::ToInfoStruct;
use ceceo_llvm_parser::parse_ceceo;

fn main() {
    let node_list = parse_ceceo("(+ (* 2 2) (+ 5 1))").unwrap();
    let json_node_list = serde_json::to_string(&node_list.to_info_struct()).unwrap();

    print!("{}\n", json_node_list);
}
