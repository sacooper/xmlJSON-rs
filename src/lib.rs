//! Structs for conversions from XML to JSON
//!
//! # Example
//! ```
//! use xmlJSON::XmlDocument;
//! use std::str::FromStr;
//! 
//! let test = "<note type=\"Reminder\">
//!                 test
//!             </note>";
//! let data = XmlDocument::from_str(test);


extern crate xml;
extern crate rustc_serialize;

use std::io::prelude::*;
use xml::reader::{EventReader,XmlEvent};
use std::str::FromStr;
use std::fmt;
use std::io::Cursor;
use rustc_serialize::json;
use std::collections::BTreeMap;

/// An XML Document
#[derive(Debug, Clone)]
pub struct XmlDocument { 
    /// Data contained within the parsed XML Document
    pub data: Vec<XmlData>
}

// Print as JSON
impl fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for d in self.data.iter() {
            s = format!("{}{}", s, d);
        }

        s.fmt(f)
    }
}

/// An XML Tag
///
/// For exammple:
///
/// ```XML
/// <foo bar="baz">
///     test text
///     <sub></sub>
/// </foo>
/// ```
#[derive(Debug, Clone)]
pub struct XmlData {
    /// Name of the tag (i.e. "foo")
    pub name: String,
    /// Key-value pairs of the attributes (i.e. ("bar", "baz"))
    pub attributes: Vec<(String, String)>,
    /// Data (i.e. "test text")
    pub data: Option<String>,
    /// Sub elements (i.e. an XML element of "sub")
    pub sub_elements: Vec<XmlData>
}

// Generate indentation
fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

// Get the attributes as a string 
fn attributes_to_string(attributes: &[(String, String)]) -> String {
    attributes.iter().fold(String::new(), |acc, &(ref k, ref v)|{
        format!("{} {}=\"{}\"", acc, k , v)
    })
}

// Format the XML data as a string
fn format(data: &XmlData, depth: usize) -> String {
    let sub =
        if data.sub_elements.is_empty() {
            String::new()
        } else {
            let mut sub = "\n".to_string();
            for elmt in data.sub_elements.iter() {
                sub = format!("{}{}", sub, format(elmt, depth + 1));
            }
            sub
        };

    let indt = indent(depth);

    let fmt_data = if let Some(ref d) = data.data {
        format!("\n{}{}", indent(depth+1), d)
    } else {
        "".to_string()
    };

    format!("{}<{}{}>{}{}\n{}</{}>", indt, data.name, attributes_to_string(&data.attributes), fmt_data, sub, indt, data.name)
}

impl fmt::Display for XmlData {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format(self, 0)) 
    }
}

// Get the XML atributes as a string
fn map_owned_attributes(attrs: Vec<xml::attribute::OwnedAttribute>) -> Vec<(String, String)> {
    attrs.into_iter().map(|attr|{
        (attr.name.local_name, attr.value)
    }).collect()
}

// Parse the data
fn parse(mut data: Vec<XmlEvent>, current: Option<XmlData>, mut current_vec: Vec<XmlData>, trim: bool) -> (Vec<XmlData>, Vec<XmlEvent>) {
    if let Some(elmt) = data.pop() {
        match elmt {
            XmlEvent::StartElement{name, attributes, ..} => {
                let inner = XmlData{
                    name: name.local_name,
                    attributes: map_owned_attributes(attributes),
                    data: None,
                    sub_elements: Vec::new()
                };

                let (inner, rest) = parse(data, Some(inner), Vec::new(), trim);

                if let Some(mut crnt) = current {
                    crnt.sub_elements.extend(inner);
                    parse(rest, Some(crnt), current_vec, trim)
                } else {
                    current_vec.extend(inner);
                    parse(rest, None, current_vec, trim)
                }
            },
            XmlEvent::Characters(chr) => {
                let chr = if trim { chr.trim().to_string() } else {chr};
                if let Some(mut crnt) = current {
                    crnt.data = Some(chr);
                    parse(data, Some(crnt), current_vec, trim)
                } else {
                    panic!("Invalid form of XML doc");
                }
            },
            XmlEvent::EndElement{name} => {
                if let Some(crnt) = current {
                    if crnt.name == name.local_name {
                        current_vec.push(crnt);
                        return (current_vec, data)
                    } else {
                        panic!(format!("Invalid end tag: expected {}, got {}", crnt.name, name.local_name)) 
                    }
                } else {
                    panic!(format!("Invalid end tag: {}", name.local_name))
                }
            }
            _ => parse(data, current, current_vec, trim)
        }
    } else {
        if current.is_some() {
            panic!("Invalid end tag");
        } else {
            (current_vec, Vec::new())
        }
    }
}

impl XmlDocument {
    pub fn from_reader<R>(source : R, trim: bool) -> Self where R : Read {        
        let mut parser = EventReader::new(source);
        let mut events : Vec<XmlEvent> = parser.into_iter().map(|x| x.unwrap() ).collect();
        events.reverse();
        let (data, _) = parse(events, None, Vec::new(), trim);
        XmlDocument{ data: data }
    }
}

/// Error when parsing XML
#[derive(Debug, Clone, PartialEq)]
pub struct ParseXmlError;

impl fmt::Display for ParseXmlError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        "Could not parse string to XML".fmt(f)
    }
}

// Generate an XML document from a string
impl FromStr for XmlDocument {
    type Err = ParseXmlError;

    fn from_str(s: &str) -> Result<XmlDocument, ParseXmlError> {
        Ok(XmlDocument::from_reader(Cursor::new(s.to_string().into_bytes()), true))
    }

}

// Convert the data to an key-value pair of JSON
fn to_kv(data: &XmlData) -> (String, json::Json) {
    use rustc_serialize::json::ToJson;

    let mut map: BTreeMap<String, json::Json> = BTreeMap::new();

    if data.data.is_some(){
        map.insert("_".to_string(), data.data.clone().unwrap().to_json());
    }

    for (k, v) in data.sub_elements.iter().map(|x|{to_kv(x)}){
        let existed = if let Some(it) = map.get_mut(&k) {
            use rustc_serialize::json::Json;
            match it {
                &mut Json::Array(ref mut a) => {
                    a.push(v.to_json());
                },
                n => {
                    let x = vec![n.clone(), v.to_json()];
                    *n = x.to_json();
                }
            }
            true
        } else {
            false
        };

        if !existed {
            map.insert(k.clone(), v.to_json()); 
        }
    }

    let mut attr : BTreeMap<String, json::Json> = BTreeMap::new();
    for &(ref k, ref v) in data.attributes.iter() {
        let existed = if let Some(it) = attr.get_mut(k) {
            use rustc_serialize::json::Json;
            match it {
                &mut Json::Array(ref mut a) => {
                    a.push(v.to_json());
                },
                n => {
                    let x = vec![n.clone(), v.to_json()];
                    *n = x.to_json();
                }
            }
            true
        } else {
            false
        };

        if !existed {
            attr.insert(k.clone(), v.to_json()); 
        }
    }


    if !attr.is_empty() {
        map.insert("$".to_string(), attr.to_json());
    }

    (data.name.clone(), map.to_json())
}

impl json::ToJson for XmlDocument {
    fn to_json(&self) -> json::Json {
        let mut map: BTreeMap<String, json::Json> = BTreeMap::new();

        for (k, v) in self.data.iter().map(|x|{to_kv(x)}) {
            map.insert(k, v);
        }

        map.to_json() 
    }
}


#[cfg(test)]
mod tests {
    use super::XmlDocument;
    use std::io::Cursor;
    use std::str::FromStr;

    #[test]
    fn test_basic_xml(){
        let test = "<note type=\"Reminder\">
                        test
                    </note>".to_string();
        let data = XmlDocument::from_reader(Cursor::new(test.into_bytes()), true);
        assert_eq!(data.data.len(), 1);

        let ref data = data.data[0];
        assert_eq!(data.name, "note");

        let mut attrs = Vec::new();
        attrs.push(("type".to_string(), "Reminder".to_string()));

        assert_eq!(data.attributes, attrs);

        assert!(data.sub_elements.is_empty());

        assert_eq!(data.data, Some("test".to_string()));
    }

    #[test]
    fn test_from_str(){
        let test = "<note type=\"Reminder\">
                        test
                    </note>";
        let data = XmlDocument::from_str(test);
        assert!(data.is_ok());
        let data = data.unwrap();

        assert_eq!(data.data.len(), 1);

        let ref data = data.data[0];
        assert_eq!(data.name, "note");

        let mut attrs = Vec::new();
        attrs.push(("type".to_string(), "Reminder".to_string()));

        assert_eq!(data.attributes, attrs);

        assert!(data.sub_elements.is_empty());

        assert_eq!(data.data, Some("test".to_string()));
    }
}
