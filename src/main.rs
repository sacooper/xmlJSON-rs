extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use std::str::FromStr;
use rustc_serialize::json::ToJson;

fn main(){
    let x = XmlDocument::from_str("<note>test<blah2>TEST</blah2></note><blah>test1</blah>");
    println!("{}", x.unwrap().to_json())
}
