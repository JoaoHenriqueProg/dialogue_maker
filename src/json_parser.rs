// V 2
// https://github.com/JoaoHenriqueProg/j_json_parser

pub struct Parser {
    cur_text: String,
    cur_i: usize,
}

#[derive(Clone, Debug)]
pub struct JsonObject {
    children: Vec<(String, JsonType)>,
}

impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject {
            children: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.stringify());
    }

    pub fn stringify(&self) -> String {
        return self.priv_stringify(0);
    }

    fn priv_stringify(&self, indent: u8) -> String {
        let mut to_return: String = "{".to_string();

        for to_spit in &self.children {
            to_return.push('\n');

            for _ in 0..(indent + 1) * 2 {
                to_return.push(' ');
            }

            match &to_spit.1 {
                JsonType::Object(val) => {
                    to_return.push_str(&format!(
                        "\"{}\": {},",
                        to_spit.0,
                        val.priv_stringify(indent + 1)
                    ));
                }

                JsonType::Bool(val) => {
                    to_return.push_str(&format!("\"{}\": {},", to_spit.0, val));
                }
                JsonType::Number(val) => {
                    to_return.push_str(&format!("\"{}\": {},", to_spit.0, val));
                }
                JsonType::String(val) => {
                    to_return.push_str(&format!("\"{}\": \"{}\",", to_spit.0, val));
                }
                JsonType::Array(val) => {
                    to_return.push_str(&format!(
                        "\"{}\": {},",
                        to_spit.0,
                        &self.priv_stringify_array(val, indent + 1)
                    ));
                }
                JsonType::Null => {
                    to_return.push_str(&format!("\"{}\": null,", to_spit.0));
                }
            }
        }

        to_return.push('\n');
        for _ in 0..indent * 2 {
            to_return.push(' ');
        }

        to_return.push_str("}");
        return to_return;
    }

    fn priv_stringify_array(&self, array_to_stringify: &Vec<JsonType>, indent: u8) -> String {
        let mut to_return: String = "[".to_string();

        for to_spit in array_to_stringify {
            to_return.push('\n');

            for _ in 0..(indent + 1) * 2 {
                to_return.push(' ');
            }

            match &to_spit {
                JsonType::Object(val) => {
                    to_return.push_str(&format!("{},", val.priv_stringify(indent + 1)));
                }
                JsonType::Bool(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::Number(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::String(val) => {
                    to_return.push_str(&format!("{},", val));
                }
                JsonType::Array(val) => {
                    to_return
                        .push_str(&format!("{},", &self.priv_stringify_array(val, indent + 1)));
                }
                JsonType::Null => {
                    to_return = "null,".to_string();
                }
            }
        }

        to_return.push('\n');
        for _ in 0..indent * 2 {
            to_return.push(' ');
        }

        to_return.push_str("]");
        return to_return;
    }

    fn get_index_of_key<T: ToString>(&self, key: T) -> i64 {
        let mut i = 0;
        for child in self.children.clone() {
            if child.0 == key.to_string() {
                return i;
            }
            i += 1;
        }
        return -1;
    }

    pub fn get<T: ToString>(&self, key: T) -> Result<JsonType, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        } else {
            return Ok(self.children[i as usize].1.clone());
        }
    }

    pub fn get_bool<T: ToString>(&self, key: T) -> Result<bool, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match self.children[i as usize].1 {
            JsonType::Bool(val) => {
                return Ok(val);
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_number<T: ToString>(&self, key: T) -> Result<f64, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match self.children[i as usize].1 {
            JsonType::Number(val) => {
                return Ok(val);
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_string<T: ToString>(&self, key: T) -> Result<String, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match &self.children[i as usize].1 {
            JsonType::String(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_array<T: ToString>(&self, key: T) -> Result<Vec<JsonType>, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match &self.children[i as usize].1 {
            JsonType::Array(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_obj<T: ToString>(&self, key: T) -> Result<JsonObject, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match &self.children[i as usize].1 {
            JsonType::Object(val) => {
                return Ok(val.clone());
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn get_obj_ref<'a, T: ToString>(&'a mut self, key: T) -> Result<&'a mut JsonObject, JsonError> {
        let i = self.get_index_of_key(key.to_string());

        if i == -1 {
            return Err(JsonError::KeyNotFound);
        }
        match &mut self.children[i as usize].1 {
            JsonType::Object(val) => {
                Ok(val)
            }
            _ => {
                return Err(JsonError::WrongTypeValueRequest);
            }
        }
    }

    pub fn set_bool<T: ToString>(&mut self, new_key: T, new_value: bool) {
        let to_add = (new_key.to_string(), JsonType::Bool(new_value));
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }
    pub fn set_number<T: ToString>(&mut self, new_key: T, new_value: f64) {
        let to_add = (new_key.to_string(), JsonType::Number(new_value));
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }
    pub fn set_string<T: ToString>(&mut self, new_key: T, new_value: T) {
        let to_add = (new_key.to_string(), JsonType::String(new_value.to_string()));
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }
    pub fn set_array<T: ToString>(&mut self, new_key: T, new_value: Vec<JsonType>) {
        let to_add = (new_key.to_string(), JsonType::Array(new_value));
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }
    pub fn set_null<T: ToString>(&mut self, new_key: T) {
        let to_add = (new_key.to_string(), JsonType::Null);
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }

    pub fn push_obj<T: ToString>(&mut self, new_key: T) {
        let to_add = (new_key.to_string(), JsonType::Object(JsonObject::new()));
        let i = self.get_index_of_key(new_key);

        if i == -1 {
            self.children.push(to_add);
        } else {
            self.children[i as usize] = to_add;
        }
    }
}

#[derive(Clone, Debug)]
pub enum JsonType {
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonType>),
    Object(JsonObject),
    Null,
}

#[derive(Debug)]
pub enum JsonError {
    KeyNotFound,
    WrongTypeValueRequest,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            cur_text: "".to_string(),
            cur_i: 0,
        }
    }

    fn cur_char(&self) -> char {
        // self.print_cur_char_loc();
        self.cur_text.chars().nth(self.cur_i).unwrap()
    }

    fn get_substr(&mut self, len: usize) -> String {
        let to_return = self.cur_text.chars().skip(self.cur_i).take(len).collect();

        self.cur_i += len;

        return to_return;
    }

    // Only supports one line json, redo or completely remove later
    fn print_cur_char_loc(&self) {
        let chars: Vec<char> = self.cur_text.chars().skip(0).take(self.cur_i).collect();
        let slice: String = chars.into_iter().collect();
        println!("{}", slice);
        for _ in 0..self.cur_i {
            print!(" ");
        }
        print!("A\n");
    }

    fn expect_char(&self, to_expect: char) {
        if self.cur_char() != to_expect {
            panic!("Expected: '{}' but got: '{}'!", to_expect, self.cur_char())
        }
    }

    fn ignore_white_space(&mut self) {
        while self.cur_char() == ' '
            || self.cur_char() == '\n'
            || self.cur_char() == '\t'
            || self.cur_char() == '\r'
        {
            self.cur_i += 1;
        }
    }

    pub fn load<T: ToString>(&mut self, new_text: T) {
        self.cur_text = new_text.to_string();
        self.cur_i = 0;
    }

    fn parse_string(&mut self) -> String {
        self.expect_char('"');

        self.cur_i += 1;

        let mut to_return = "".to_string();
        loop {
            if self.cur_char() == '"' {
                break;
            }

            to_return.push(self.cur_char());

            self.cur_i += 1;
        }

        self.cur_i += 1;
        self.ignore_white_space();

        return to_return;
    }

    fn parse_bool(&mut self) -> bool {
        let keyword_len;

        if self.cur_char() == 'f' {
            keyword_len = 5;
        } else {
            keyword_len = 4;
        }

        let chars = self.get_substr(keyword_len);

        if chars == "true" {
            return true;
        } else if chars == "false" {
            return false;
        } else {
            panic!("Expected true or false, got: {}", chars);
        }
    }

    fn parse_number(&mut self) -> f64 {
        let mut stringed_number = "".to_string();

        loop {
            match self.cur_char() {
                ' ' | ',' | '\n' | '\t' | ']' | '}' => {
                    break;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    if self.cur_char() == '.' {
                        if stringed_number.contains(".") {
                            panic!("Tried to put two '.' in a number!")
                        } else {
                            stringed_number.push('.');
                        }
                    } else {
                        stringed_number.push(self.cur_char());
                    }
                }

                _ => {
                    panic!("Something went wrong in number parsing!")
                }
            }

            self.cur_i += 1;
        }

        return stringed_number.parse().unwrap();
    }

    fn parse_array(&mut self) -> Vec<JsonType> {
        let mut to_return: Vec<JsonType> = Vec::new();

        self.expect_char('[');

        self.cur_i += 1;

        loop {
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    to_return.push(JsonType::Bool(result));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    let result = self.parse_number();
                    to_return.push(JsonType::Number(result));
                }

                '"' => {
                    let result = self.parse_string();
                    to_return.push(JsonType::String(result));
                }

                '[' => {
                    let result = self.parse_array();
                    to_return.push(JsonType::Array(result));
                }

                'n' => {
                    let chars = self.get_substr(4);

                    if chars == "null" {
                        to_return.push(JsonType::Null);
                    } else {
                        panic!("Expected null found something else")
                    }
                }

                ']' => {}

                '{' => {
                    let result = self.parse_object();
                    to_return.push(JsonType::Object(result));
                }

                _ => {
                    unimplemented!("Strange path in array parser")
                }
            }

            self.ignore_white_space();

            if self.cur_char() == ']' {
                break;
            }

            self.expect_char(',');
            self.cur_i += 1;
        }
        self.cur_i += 1;

        return to_return;
    }

    fn parse_object(&mut self) -> JsonObject {
        let mut to_return: JsonObject = JsonObject::new();

        self.expect_char('{');

        self.cur_i += 1;

        if self.cur_char() == '}' {
            self.cur_i += 1;
            return to_return;
        }

        loop {
            self.ignore_white_space();

            let new_key = self.parse_string();
            if new_key == "" {
                panic!("Empty key!");
            }

            self.expect_char(':');
            self.cur_i += 1;
            self.ignore_white_space();

            match self.cur_char() {
                't' | 'f' => {
                    let result = self.parse_bool();
                    to_return.children.push((new_key, JsonType::Bool(result)));
                }

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                    let result = self.parse_number();
                    to_return.children.push((new_key, JsonType::Number(result)));
                }

                '"' => {
                    let result = self.parse_string();
                    to_return.children.push((new_key, JsonType::String(result)));
                }

                '[' => {
                    let result = self.parse_array();
                    to_return.children.push((new_key, JsonType::Array(result)));
                }

                'n' => {
                    let chars = self.get_substr(4);

                    if chars == "null" {
                        to_return.children.push((new_key, JsonType::Null));
                    } else {
                        panic!("Expected null found something else");
                    }
                }

                '}' => {}

                '{' => {
                    let result = self.parse_object();
                    to_return.children.push((new_key, JsonType::Object(result)));
                }

                _ => {
                    unimplemented!("Strange path")
                }
            }

            self.ignore_white_space();

            if self.cur_char() == '}' {
                self.cur_i += 1;
                break;
            }

            self.expect_char(',');
            self.cur_i += 1;

            self.ignore_white_space();

            if self.cur_char() == '}' {
                self.cur_i += 1;
                break;
            }
        }

        return to_return;
    }

    pub fn parse(&mut self) -> JsonObject {
        if self.cur_i != 0 {
            panic!("Please load a new json file!")
        }

        self.ignore_white_space();
        self.parse_object()
    }
}
