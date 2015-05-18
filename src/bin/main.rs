extern crate xmlJSON;
extern crate rustc_serialize;

use rustc_serialize::json::ToJson;
use xmlJSON::XmlDocument;
use std::str::FromStr;

fn main() {
    println!("{}", XmlDocument::from_str("<test type=\"HOPE\">god please work<testing123>WE'LL SEE</testing123></test>").unwrap().to_json());
}
