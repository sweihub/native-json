//!# Native JSON for Rust
//!
//!This crate provides native JSON syntax for Rust, it brings with a powerful way of parsing JSON syntax into native Rust structs. You can declare the JSON object natively as you do with JavaScript, JSON in Rust was made easy!
//!
//!## Usage
//!Add dependencies to your `Cargo.toml`.
//!```toml
//![dependencies]
//!native-json = "1.2"
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
//!With JSON declare syntax, you can declare nested native JSON object in place.
//!
//!### JSON Declare Syntax
//!```rust
//!json!{
//!JSON_OBJECT_NAME {
//!    state: i32?,    // optional field
//!    type_: String,  // suffix underscore will be removed when serialize & deserialize
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
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

pub use native_json_macro::*;
pub use serde::de::DeserializeOwned;
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
        let buf = Vec::new();
        let spaces = vec![' ' as u8; indent];
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        self.serialize(&mut ser)?;
        let output = String::from_utf8(ser.into_inner())?;

        Ok(output)
    }
}

impl<T> JSON for T where T: Serialize {}

/// Deserialize from file
pub fn read<T, P: AsRef<Path>>(path: P) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}

/// Serialize into file
pub fn write<T, P: AsRef<Path>>(path: P, value: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    let writer = BufWriter::new(file);
    Ok(serde_json::to_writer_pretty(writer, value)?)
}

pub struct Writer<'a> {
    path: &'a Path,
    indent: usize,
}

impl<'a> Writer<'a> {
    /// Indentation
    pub fn indent(mut self, n: usize) -> Self {
        self.indent = n;
        self
    }

    /// Write the value into file
    pub fn write<T>(&self, value: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let spaces = vec![' ' as u8; self.indent];
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path)?;
        let writer = BufWriter::new(file);
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
        let mut ser = serde_json::Serializer::with_formatter(writer, formatter);
        Ok(value.serialize(&mut ser)?)
    }
}

/// Build a file writer
pub fn writer<'a, P>(path: &'a P) -> Writer<'a>
where
    P: AsRef<Path>,
{
    Writer {
        path: path.as_ref(),
        indent: 2,
    }
}
