# xmlJSON
Convert XML data to JSON. Note that this is not yet well tested. Use at your own risk.

## Status
[![Build Status](https://travis-ci.org/sacooper/xmlJSON-rs.svg?branch=master)](https://travis-ci.org/sacooper/xmlJSON-rs.svg)

## Usage
Add this to your Cargo.toml:
```rust
[dependencies]
xmlJSON = "*"
```

Structs for conversions from XML to JSON
//!
```rust
extern crate xmlJSON;
extern crate rustc_serialize;
//!
use xmlJSON::XmlDocument;
use rustc_serialize::json;
use std::str::FromStr;
//!
let s = "<test lang=\"rust\">An XML Document <testElement>A test
element</testElement></test>"
let document : XmlDocument = XmlDocument::from_str(s).unwrap();
let jsn : json::Json = document.to_json(); 
```
//!
The resulting Json will be of the form
//!
```javascript
{
    "test": {
        "$": {
            "lang": "rust"
        },
        "_" : "An Xml Document",
        "testElement": {
            "_" : "A test element" 
        }
    }
}
```

and add this to your crate root:
```rust
extern crate xmlJSON;
```

## TODO:
- Add better testing
- Add documentation
- Add conversion from JSON to XML, and allowing for writing JSON as XML
