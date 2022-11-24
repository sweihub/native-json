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
pub use serde_json::Error;

use std::{fs::read_to_string, path::Path};
use serde::{Deserialize, Serialize};

// normalize error
fn get<T, E: ToString>(v: Result<T, E>) -> Result<T, String> 
{
    match v {
        Ok(a) => { Ok(a) },
        Err(e) => { Err(e.to_string()) }
    }
}

pub trait JSON<'a>: Serialize + Deserialize<'a> {
    /// Parse JSON from string
    fn parse<T: AsRef<str>>(&mut self, s: &'a T) -> Result<(), serde_json::Error> {
        *self = serde_json::from_str(s.as_ref())?;
        Ok(())
    }

    /// Return a concise JSON string
    fn to_string(&self) -> String {
       return self.stringify(0);
    }

     /// Stringify a native-json object
    ///
    /// indent
    ///
    /// - 0 : output concise JSON string
    /// - N : pretty output with N spaces indentation
    fn stringify(&self, indent: usize) -> String   
    {
        let output;

        // concise
        if indent == 0 {
            match serde_json::to_string(self) {
                Ok(s) => {
                    output = s;
                }
                Err(e) => {
                    return format!("{{ error : \"{}\" }}", e.to_string());
                }
            }
            return output;
        }

        // pretty
        let spaces = vec![' ' as u8; indent];
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&spaces);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        if let Err(e) = self.serialize(&mut ser) {
            return format!("{{ error : \"{}\" }}", e.to_string());
        }

        match String::from_utf8(ser.into_inner()) {
            Ok(s) => {
                output = s;
            }
            Err(e) => {
                return format!("{{ error : \"{}\" }}", e.to_string());
            }
        }

        return output;
    }
    
     /// Deserialize JSON from file
     /// 
     /// Due to the serde lifetime issue, the content should have same lifetime as the JSON
     /// object itself, the content will be borrowed as mutable zoombie.
     fn read<F: AsRef<Path>>(&mut self, file: F, content: &'a mut String) -> Result<(), String>     
     {
        *content = get(read_to_string(file))?;
        let decoder = serde_json::from_str(content.as_str());
        *self = get(decoder)?;
 
         return Ok(());
     }

      /// Serialize JSON into file
    fn write<F: AsRef<Path>>(&self, file: F) -> std::io::Result<()>
    {
        let content = self.stringify(4);
        std::fs::write(file, content)?;
        return Ok(());
    }
}

impl<'a, T> JSON<'a> for T where T: Serialize + Deserialize<'a> {}
