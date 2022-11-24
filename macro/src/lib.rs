//!# Native JSON for Rust
//!
//!This crate provides native JSON syntax for Rust, it brings with a powerful way of parsing JSON syntax into native Rust structs. You can declare the JSON object natively as you do with JavaScript, JSON in Rust was made easy!
//!
//!> Note: This crate is just a crude proc-macro (compiler plugin), for more features, please refer to [native-json](https://crates.io/crates/native-json)
//!
//!## Usage
//!Add dependencies to your Cargo.toml, `serde_json` is only needed if you want to stringify the JSON object.
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
//!    let text = serde_json::to_string_pretty(&json).unwrap();
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
extern crate proc_macro;
mod json;

use json::*;
use std::str::FromStr;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Declare or instantiate a native JSON object, please refere to module [json](index.html)
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let parser = parse_macro_input!(input as Json); 
    let block = parser.get_block();
    // Show me the code
    // println!("XXXXXXXX\n{}", block);
    return TokenStream::from_str(block.as_str()).unwrap();
}
