//!# Native JSON for Rust
//!
//!This crate provides native JSON syntax for Rust, it brings with a powerful way of parsing JSON syntax into native Rust structs. You can declare the JSON object natively as you do with JavaScript, JSON in Rust was made easy!
//!
//!## Usage
//!Add dependencies to your `Cargo.toml`.
//!```toml
//![dependencies]
//!native-json = "1.1"
//!serde = {version = "1.0", features = ["derive"] }
//!serde_json = "1.0"
//!```
//!
//!## Example of using native JSON object
//!```rust
//!use native_json::json;
//!use std::collections::HashMap;
//!use serde::{Deserialize, Serialize};
//!
//!fn main()
//!{
//!    let mut json = json!{
//!        name: "native json",
//!        style: {
//!            color: "red",
//!            size: 12,
//!            bold: true,
//!            range: null
//!        },
//!        array: [5,4,3,2,1],
//!        vector: vec![1,2,3,4,5],
//!        hashmap: HashMap::from([ ("a", 1), ("b", 2), ("c", 3) ]);,
//!        students: [
//!            {name: "John", age: 18},
//!            {name: "Jack", age: 21},
//!        ],
//!    };
//!
//!    // Native access
//!    json.style.size += 1;
//!    json.students[0].age += 2;
//!
//!    // Debug
//!    println!("{:#?}", t);
//!
//!    // Stringify
//!    let text = json.stringify(4);
//!    println!("{}", text);
//!}
//!```
//!## Declare a named JSON struct
//!
//!With JSON decalre syntax, you can declare nested native JSON object in place.
//!
//!### JSON Declare Syntax
//!```rust
//!json!{
//!JSON_OBJECT_NAME {
//!    name : type,
//!    array: [type],
//!    object: {
//!        name: type,
//!        ...
//!    },
//!    ...
//!}}
//!```
//!
//!The native-json will generate native Rust structs for you, each object is named by object hierarchy path, concatenated with underscore.
//!
//!  1. `JSON_OBJECT_NAME.object` was converted to `JSON_OBJECT_NAME_object`
//!  2. `JSON_OBJECT_NAME.array's item` was converted to `JSON_OBJECT_NAME_array_item`
//!
//!## Example of using named JSON object
//!
//!```rust
//!use native_json::json;
//!use serde::{Deserialize, Serialize};
//!use std::collections::HashMap;
//!
//!json!{ School {
//!    name: String,
//!    students: [
//!        { name: String, age: u16 },
//!        ...
//!    ],
//!    map: HashMap<String, String>,
//!    nullable: Option<String>
//!}}
//!
//!fn main()
//!{
//!    let mut school = School::new();
//!
//!    school.name = "MIT".to_string();
//!    school.map.insert("Tom".to_owned(), "Profile".to_owned());
//!
//!    // using initializer
//!    let mut john = School_students_item::new();
//!    john.name = "John".to_owned();
//!    john.age = 18;
//!    school.students.push(john);
//!
//!    // using struct
//!    let jack = School_students_item { name: "Jack".to_string(), age: 21 };
//!    school.students.push(jack);
//!
//!    // show
//!    println!("{:#?}", school);
//!}
//!```
//!
pub use native_json_macro::*;
pub use serde::{Deserialize, Serialize};
pub use serde_json::from_str as parse;
pub use serde_json::Error;

// #[serde(default, skip_serializing_if = "is_default")]
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub trait JSON: Serialize {
    /// Return a concise JSON string
    fn string(&self) -> anyhow::Result<String> {
        self.stringify(0)
    }

    /// Stringify a native-json object
    ///
    /// indent
    ///
    /// - 0 : output concise JSON string
    /// - N : pretty output with N spaces indentation
    fn stringify(&self, indent: usize) -> anyhow::Result<String> {
        // concise
        if indent == 0 {
            return Ok(serde_json::to_string(self)?);
        }

        // pretty
        let spaces = vec![' ' as u8; indent];
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        self.serialize(&mut ser)?;
        let output = String::from_utf8(ser.into_inner())?;

        Ok(output)
    }
}

impl<T> JSON for T where T: Serialize {}
