use std::collections::HashMap;

use syn::{
    parse::{Parse, ParseStream},
    *,
};

//------------------- JSON Syntax ------------------------------
//
// object = { pair, ...}
// pair = key : value
// key = identifier
// array  = [value, ...]
// value =  object | array | expression
// expression = string | number | identifier

const ATTRIBUTES: &str = "#[derive(Serialize, Deserialize, Debug, Clone)]\n";

#[derive(PartialEq, Clone, Copy)]
pub enum ValueType {
    NULL,
    OBJECT,
    ARRAY,
    EXPRESSION,
    DECLARE,
}

pub struct Array {
    pub items: Vec<Value>,
}

pub struct Pair {
    pub key: Ident,
    pub value: Value,
}

pub struct Object {
    pub name: String,
    pub pairs: Vec<Pair>,
}

pub struct Json {
    pub value: Value,
    pub id: i32,
    objects: Vec<Object>,
    arrays: Vec<Array>,
    expressions: Vec<String>,
}

pub struct Value {
    pub t: ValueType,
    pub i: usize,
}

impl Array {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl Object {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            pairs: Vec::new(),
        }
    }
}

struct ClassDict {
    map: HashMap<String, Value>,
}

impl ClassDict {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: &String, value: &Value) {
        let v = Value {
            t: value.t,
            i: value.i,
        };
        self.map.insert(key.clone(), v);
    }
}

impl Json {
    pub fn new() -> Self {
        return Self {
            value: Value {
                t: ValueType::NULL,
                i: 0,
            },
            id: 0,
            objects: Vec::new(),
            arrays: Vec::new(),
            expressions: Vec::new(),
        };
    }

    pub fn get_object(&self, v: &Value) -> &Object {
        return &self.objects[v.i];
    }

    pub fn get_object_mut(&mut self, v: &Value) -> &mut Object {
        return &mut self.objects[v.i];
    }

    pub fn get_array(&self, v: &Value) -> &Array {
        return &self.arrays[v.i];
    }

    pub fn get_expression(&self, v: &Value) -> &String {
        return &self.expressions[v.i];
    }

    fn append_object(&mut self, v: Object) -> Value {
        self.objects.push(v);
        let i = self.objects.len() - 1;
        let t = ValueType::OBJECT;
        return Value { t, i };
    }

    fn append_array(&mut self, v: Array) -> Value {
        self.arrays.push(v);
        let i = self.arrays.len() - 1;
        let t = ValueType::ARRAY;
        return Value { t, i };
    }

    fn append_expression(&mut self, v: String) -> Value {
        self.expressions.push(v);
        let i = self.expressions.len() - 1;
        let t = ValueType::EXPRESSION;
        return Value { t, i };
    }

    // terminal
    fn parse_expression(&mut self, input: ParseStream) -> Result<Value> {
        let mut span = input.span();

        // expression with generic is allowed
        let output = input.step(|cursor| {
            let mut rest = *cursor;
            let mut nested = 0;
            let mut s = "".to_owned();
            while let Some((tt, next)) = rest.token_tree() {
                span = tt.span();
                let token = tt.to_string();
                if token == "<" {
                    nested += 1;
                } else if token == ">" {
                    nested -= 1;
                }
                s += &token;

                // peek
                let mut peek = "".to_owned();
                if let Some((lookhead, _)) = next.token_tree() {
                    peek = lookhead.to_string();
                }

                // terminal
                if nested == 0 && (peek == "," || next.eof()) {
                    return Ok((s, next));
                }

                rest = next;
            }
            return Err(cursor.error("expression was not terminated!"));
        });

        if output.is_err() {
            return Err(Error::new(span, "expected expression"));
        }

        let s = output.unwrap_or("".to_owned());
        let value = self.append_expression(s);

        return Ok(value);
    }

    fn parse_pair(&mut self, input: ParseStream) -> Result<Pair> {
        // key
        let key: Ident = input.parse()?;
        // :
        input.parse::<Token![:]>()?;
        // value
        let value = self.parse_value(&input)?;

        return Ok(Pair { key, value });
    }

    fn parse_declare(&mut self, input: ParseStream) -> Result<Value> {
        let name: Ident = input.parse()?;
        let mut value = self.parse_object(input)?;

        // using declared name
        let mut object = self.get_object_mut(&value);
        object.name = name.to_string();
        value.t = ValueType::DECLARE;

        return Ok(value);
    }

    // object := { key: value, ...}
    fn parse_object(&mut self, input: ParseStream) -> Result<Value> {
        let inner;
        let mut content;

        content = input;
        if input.peek(syn::token::Brace) {
            braced!(inner in input);
            content = &inner;
        }

        let mut object = Object::new();
        object.name = format!("Object{}", self.id);
        self.id += 1;

        loop {
            let pair = self.parse_pair(content)?;
            object.pairs.push(pair);
            if !content.peek(Token![,]) {
                break;
            }
            content.parse::<Token![,]>()?;
            if content.is_empty() {
                break;
            }
        }

        let v = self.append_object(object);
        return Ok(v);
    }

    // array := [value, ...]
    fn parse_array(&mut self, input: ParseStream) -> Result<Value> {
        let mut array = Array::new();

        let inner;
        let mut content = input;

        if input.peek(syn::token::Bracket) {
            bracketed!(inner in input);
            content = &inner;
        }

        loop {
            let value = self.parse_value(content)?;
            array.items.push(value);
            if !content.peek(Token![,]) {
                break;
            }
            content.parse::<Token![,]>()?;
            if content.is_empty() {
                break;
            }
        }

        let value = self.append_array(array);
        return Ok(value);
    }

    // value ：= object | array | expression
    fn parse_value(&mut self, input: ParseStream) -> Result<Value> {
        if input.peek(syn::token::Brace) {
            return self.parse_object(input);
        } else if input.peek(syn::token::Bracket) {
            return self.parse_array(input);
        }
        return self.parse_expression(input);
    }

    pub fn get_generics(&self) -> String {
        let mut defines = Vec::new();
        defines.push("".to_owned());

        for obj in &self.objects {
            let mut i = 0;
            let mut types = Vec::new();
            let mut fields = Vec::new();
            for pair in &obj.pairs {
                let t = format!("T{}", {
                    i += 1;
                    i
                });
                let f = format!("{}:{}", pair.key.to_string(), t);
                types.push(t);
                fields.push(f);
            }
            let define = format!(
                "struct {}<{}> {{ {} }}",
                obj.name,
                types.join(","),
                fields.join(",")
            );
            defines.push(define);
        }

        return defines.join(ATTRIBUTES);
    }

    pub fn get_code(&self) -> String {
        let name = "".to_owned();
        let code = self.gen_code(&self.value, &name);
        return code;
    }

    pub fn get_block(&self) -> String {
        if self.value.t == ValueType::DECLARE {
            let path = "".to_owned();
            let (name, declare) = self.gen_declare(path, &self.value);
            let mut code = declare;
            // objects which require initializers
            let mut dict = ClassDict::new();
            dict = self.get_dict(dict, &name, &self.value);
            for (key, value) in &dict.map {
                let init = self.gen_initializer(key, value);
                let implement = format!(
                    "impl {} {{\n    pub fn new() -> Self {{\n        {}\n    }}\n}}\n",
                    key, init
                );
                code += &implement;
            }

            return code;
        } else {
            let prototypes = self.get_generics();
            let code = self.get_code();
            let block = format!("{{ {}\n{} }}", prototypes, code);
            return block;
        }
    }

    fn gen_code(&self, value: &Value, object_type: &String) -> String {
        let mut code;
        let none = "".to_owned();
        match value.t {
            ValueType::OBJECT => {
                let obj = self.get_object(value);
                let mut fields = Vec::new();
                for pair in &obj.pairs {
                    let v = self.gen_code(&pair.value, object_type);
                    let f = format!("{}:{}", pair.key.to_string(), v);
                    fields.push(f);
                }
                let name = if object_type.is_empty() {
                    &obj.name
                } else {
                    object_type
                };
                code = format!("{} {{ {} }}", name, fields.join(","));
            }
            ValueType::ARRAY => {
                let array = self.get_array(value);
                let mut item_type = &none;
                // use the first item type
                if array.items.len() > 0 && matches!(array.items[0].t, ValueType::OBJECT) {
                    let obj = self.get_object(&array.items[0]);
                    item_type = &obj.name;
                }
                let items: Vec<_> = array
                    .items
                    .iter()
                    .map(|x| {
                        let c = self.gen_code(x, &item_type);
                        c
                    })
                    .collect();
                code = format!("[{}]", items.join(","));
            }
            ValueType::EXPRESSION => {
                let expr = self.get_expression(value);
                code = expr.clone();
                if code.eq("null") || code.eq("None") {
                    code = "Option::<String>::None".to_owned();
                }
            }
            ValueType::DECLARE => {
                code = "TODO: declare".to_owned();
            }
            ValueType::NULL => {
                code = "Option::<String>::None".to_owned();
            }
        }
        return code;
    }

    fn get_instance(&self, class: &String) -> String {
        const PRIMITIVES: [&str; 16] = [
            "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
            "bool", "char", "isize", "usize"
        ];

        for c in &PRIMITIVES {
            if c == class {
                return format!("0 as {}", class);
            }
        }

        if class.find("Option<").is_some() {
            return "None".to_owned();
        }

        let mut c = class.as_str();

        // generic
        if let Some(i) = class.find("<") {
            c = &class[0..i];
        } else if class == "str" || class == "&str" {
            c = "std::string::String";
        }

        // type must have new() initializer
        return format!("{}::new()", c);
    }

    // dict of (path, object)
    fn get_dict(&self, mut dict: ClassDict, path: &String, value: &Value) -> ClassDict {
        match value.t {
            ValueType::DECLARE | ValueType::OBJECT => {
                // initializer for object
                dict.set(path, value);
                let object = self.get_object(value);
                for pair in &object.pairs {
                    let child = path.clone() + "_" + &pair.key.to_string();
                    dict = self.get_dict(dict, &child, &pair.value);
                }
            }
            ValueType::ARRAY => {
                // initializer for array item
                let array = self.get_array(value);
                let child = path.clone() + "_item";
                dict = self.get_dict(dict, &child, &array.items[0]);
            }
            ValueType::EXPRESSION => {}
            ValueType::NULL => {}
        }

        return dict;
    }

    fn gen_initializer(&self, path: &String, value: &Value) -> String {
        let mut code = "".to_owned();

        match value.t {
            ValueType::DECLARE | ValueType::OBJECT => {
                let object = self.get_object(value);
                let mut fields = Vec::new();
                for pair in &object.pairs {
                    let child = path.clone() + "_" + &pair.key.to_string();
                    let c = self.gen_initializer(&child, &pair.value);
                    let f = format!("{}: {}", pair.key.to_string(), c);
                    fields.push(f);
                }
                code += format!("{} {{ {} }}", path, fields.join(",")).as_str();
            }
            ValueType::ARRAY => {
                code = "std::vec::Vec::new()".to_owned();
            }
            ValueType::EXPRESSION => {
                let expr = self.get_expression(value);
                code = self.get_instance(&expr);
            }
            ValueType::NULL => {}
        }

        return code;
    }

    fn gen_declare(&self, mut path: String, value: &Value) -> (String, String) {
        // class of current node
        let mut class = "".to_owned();
        let mut code = "".to_owned();
        match value.t {
            ValueType::DECLARE | ValueType::OBJECT => {
                let object = self.get_object(value);
                if path.is_empty() {
                    path = object.name.clone();
                }
                class = path;
                let mut fields = Vec::new();
                for pair in &object.pairs {
                    let child = class.clone() + "_" + &pair.key.to_string();
                    let (n, c) = self.gen_declare(child, &pair.value);
                    code += &c;
                    // collapse to "key: type"
                    let f = format!("pub {}:{}", pair.key.to_string(), n);
                    fields.push(f);
                }
                let c = format!("pub struct {} {{ {} }}\n", class, fields.join(","));
                code += ATTRIBUTES;
                code += &c;
            }
            ValueType::ARRAY => {
                // array: [type]
                let array = self.get_array(value);
                let child = path + "_item";
                let (n, c) = self.gen_declare(child, &array.items[0]);
                code += &c;
                class = format!("std::vec::Vec<{}>", n);
            }
            ValueType::EXPRESSION => {
                // expression is type
                let v = self.get_expression(value);
                class = v.clone();
                if class == "str" || class == "&str" {
                    class = "String".to_owned();
                }
            }
            ValueType::NULL => {}
        }

        return (class, code);
    }
}

impl Parse for Json {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut json = Json::new();

        if input.peek2(syn::token::Brace) {
            // declare := identifier { ... }
            json.value = json.parse_declare(input)?;
        } else if input.peek2(syn::token::Colon) {
            // value := object | array
            json.value = json.parse_object(input)?;
        } else {
            // array := [value, ...]
            json.value = json.parse_array(input)?;
        }

        return Ok(json);
    }
}
