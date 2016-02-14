extern crate xmlJSON;
use xmlJSON::XmlDocument;
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
