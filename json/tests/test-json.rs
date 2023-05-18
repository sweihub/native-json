use native_json::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type Pod<T = (), E = anyhow::Error> = core::result::Result<T, E>;

#[test]
fn json_reserved_keywords() -> Pod {
    // `keyword_` will be rename to `keyword`
    // declare
    json! {
     Order {
        type_: String
     }
    }

    let mut order = Order::new();
    order.type_ = "LIMIT".into();
    let s = order.string()?;
    assert!(s == "{\"type\":\"LIMIT\"}");

    // generic
    let mut json = json! {
        type_ : "Action"
    };

    let s = json.string()?;
    assert!(s == "{\"type\":\"Action\"}");

    json.type_ = "".into();
    json = serde_json::from_str(&s)?;
    assert!(json.type_ == "Action");

    Ok(())
}

#[test]
fn json_instance() {
    let mut json = json! {
        name: "native json",
        students:  [
            {name: "John", age: 17},
            {name: "Jack", age: 20}
        ],
        array: [1,2,3,4,5],
        vector: vec![5,4,3,2,1],
        hashmap: HashMap::from([("a", 1), ("b", 2), ("c", 3)]),
        rect: {x: 10, y: 10, width: 100, height: 50}
    };

    json.students[0].age += 1;
    json.rect.x += 10;
    json.name = "Native JSON";
}

#[test]
fn json_declare() {
    json! {
    School {
        name: String,
        students:[{
            name: String,
            age: i32,
            tutor: {
                name: String,
                course: String
              }
            },
            ...
        ],
        nullable: Option<String>,
        map: HashMap<String, i32>
    }}

    let mut school = School::new();
    school.name = "MIT".to_owned();

    let mut newbie = School_students_item::new();
    let tutor = School_students_item_tutor {
        name: "Don Markuson".to_owned(),
        course: "Math".to_owned(),
    };
    newbie.name = "John".to_owned();
    newbie.age = 17;
    newbie.tutor = tutor;
    school.students.push(newbie);
}

#[test]
fn json_serialize() -> Pod {
    let mut json = json! {
        name: "native json",
        point: { x: 10, y: 20},
        array: [1,2,3,4,5],
        vector: vec![1,2,3,4,5,6]
    };

    // stringify
    let s = json.stringify(4)?;
    json.name = "";
    json = native_json::parse(&s)?;
    assert_eq!(json.name, "native json");

    // concise string
    let s2 = json.string()?;
    json.name = "modified";
    json = native_json::parse(&s2)?;
    assert_eq!(json.name, "native json");

    Ok(())
}

#[test]
fn json_test_array_with_custom_type() -> Result<(), std::io::Error> {
    json! {
        Student { name: String, age: u32}
    }

    json! {
        Class {
            name: String,
            students: [Student]
        }
    }

    let mut c = Class::new();
    c.name = "High School".into();
    let mut student = Student::new();
    student.name = "Tom Jackson".into();
    student.age = 25;
    c.students.push(student);

    Ok(())
}

#[test]
fn json_test_inline_comment() {
    println!("should compile");
    json! {
    AggTrage {
        e: String,   // Event type: aggTrade
        E: i64,      // Event time
        s: String,   // Symbol
        a: i64,      // Aggregate trade ID
        p: String,   // Price
        q: String,   // Quantity
        f: i64,      // First trade ID
        l: i64,      // Last trade ID
        T: i64,      // Trade time
        m: bool,     // Is the buyer the market maker?
        c: char,     // test only
    }}
}
