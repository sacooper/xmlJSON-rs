# xmlJSON

[![Crates.io](https://img.shields.io/crates/l/xmlJSON.svg)](https://crates.io/crates/xmlJSON)
[![Crates.io](https://img.shields.io/crates/v/xmlJSON.svg)](https://crates.io/crates/xmlJSON)
[![Crates.io](https://img.shields.io/crates/d/xmlJSON.svg)](https://crates.io/crates/xmlJSON)
[![Build Status](https://travis-ci.org/sacooper/xmlJSON-rs.svg?branch=master)](https://travis-ci.org/sacooper/xmlJSON-rs.svg)

Convert XML data to JSON. Note that this is not yet well tested. Use at your own risk.

## Status

[Docs](http://sacooper.io/xmlJSON-rs/xmlJSON/index.html)

## Usage
Add this to your Cargo.toml:
```rust
[dependencies]
xmlJSON = "0.2.0"
```

Structs for conversions from XML to JSON
```rust
extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use rustc_serialize::json;
use std::str::FromStr;

let s = "<test lang=\"rust\">An XML Document <testElement>A test
element</testElement></test>"
let document : XmlDocument = XmlDocument::from_str(s).unwrap();
let jsn : json::Json = document.to_json(); 
```

The resulting Json will be of the form

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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## TODO:
- Add better testing
- Add documentation
- Add conversion from JSON to XML, and allowing for writing JSON as XML
