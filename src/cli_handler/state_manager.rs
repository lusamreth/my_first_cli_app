use std::collections::hash_map::HashMap;
use super::utility;
type Value = String;
#[derive(PartialEq, Debug, Clone)]
pub enum Constate {
    Save,
    Default,
    New(Value),
}
/* if input == binder then return execution */
#[derive(Debug)]
pub enum BufferResult<T, E> {
    Value(T),
    Err(E),
}

impl<T, E: std::fmt::Debug> BufferResult<T, E> {
    pub fn get_value(&self) -> Result<&T, &E> {
        match self {
            BufferResult::Value(value) => {
                return Ok(value);
            }
            BufferResult::Err(error) => Err(error),
        }
    }
}

pub trait Buffer<T> {
    fn verify(&self, opt: &str) -> Option<Constate>;
    fn save(&mut self, name: String, content: T, path_stack: &utility::Pathstack) -> ();
    fn remove(&mut self, index: &str) -> Result<(), String>;
    fn select(&self, index: &str) -> Result<&T, String>;
    fn clear(&mut self) -> ();
    fn automate<N>(&self, command: &str) -> BufferResult<&T, String>;
    fn count(&self) -> usize;
}
#[derive(Debug, Clone)]
pub struct BufferState<T> {
    exec_type: Constate,
    data_map: HashMap<String, T>,
}
impl<A> BufferState<A> {
    pub fn new<'a>(ext: Constate,limit:Option<usize>) -> BufferState<A> {
        let new_map = match limit{
            Some(size) => HashMap::with_capacity(size),
            None => HashMap::new()
        };
        BufferState {
            exec_type: ext,
            data_map:new_map ,
        }
    }
}
impl<T: std::fmt::Debug + PartialEq> Buffer<T> for BufferState<T> {
    fn verify(&self, opt: &str) -> Option<Constate> {
        let in_scope = match opt.to_lowercase().as_str() {
            "save" => Constate::Save,
            "new" => Constate::New("new".to_string()),
            "default" | _ => Constate::Default,
        };
        if in_scope != self.exec_type {
            None
        } else {
            Some(in_scope)
        }
    }
    fn save(&mut self, name: String, content: T, path_stack: &utility::Pathstack) -> () {
        if self.exec_type != Constate::Save {
            return;
        }
        path_stack.unlock();
        let mut num = 0;
        let ct_str = name;
        let pstack = path_stack.get_stack();
        let mut cfg_naming_char: String = String::new();
        for _ in 1..pstack.len() {
            num += 1
        }
        pstack.iter().for_each(|each_path| {
            let param_char = ct_str
                .chars()
                .nth(0)
                .unwrap_or_else(|| panic!("The content parameter is empty!"));
            match each_path.chars().nth(1) {
                Some(p_char) => {
                    // if the first character is the same
                    if p_char == param_char {
                        num += 1;
                        // cfg_naming_char = param_char + each_path.chars().next().unwrap();
                        cfg_naming_char =
                            format!("{}{}", param_char, each_path.chars().next().unwrap());
                    } else {
                        cfg_naming_char = format!("{}", param_char);
                    }
                }
                None => {}
            }
        });
        let format_cfg = |param_char| format!("/cfg/{}:{}/", num, param_char);
        path_stack.push(String::from(format_cfg(&cfg_naming_char)));

        self.data_map
            .insert(format_cfg(&cfg_naming_char)[4..].to_string(), content);
        path_stack.lock();
    }
    fn clear(&mut self) -> () {
        self.data_map.clear();
        self.exec_type = Constate::Default;
    }
    fn remove(&mut self, index: &str) -> Result<(), String> {
        if self.data_map.len() > 0 {
            let rmt = self.data_map.remove(index);
            if let None = rmt {
                Err(String::from(
                    "Cannot perform remove operation! index not found!",
                ))
            } else {
                Ok(())
            }
        }else{
            Err(String::from("Cannot Delete empty map!"))
        }
       
    }
    fn select(&self, index: &str) -> Result<&T, String> {
        let rmt = self.data_map.get(index);
        if let Some(x) = rmt {
            Ok(&x)
        } else {
            Err(String::from(
                "There are not any correct index in the table! ReEnter the correct one!",
            ))
        }
    }
    fn count(&self) -> usize{
        self.data_map.len()
    }
    fn automate<N>(&self, command: &str) -> BufferResult<&T, String> {
        let divide = command.split_whitespace().collect::<Vec<&str>>();
        let operation = divide
            .get(0)
            .unwrap_or_else(|| panic!("require the operation!"));
        let parameters = divide.get(1).unwrap();

        // let mut store = |result| collector.push(result);
        match operation.to_lowercase().as_str() {
            "select" => match self.select(parameters) {
                Ok(res) => BufferResult::Value(res),
                Err(x) => BufferResult::Err(x),
            },
            _ => BufferResult::Err("ayyyy".to_string()),
        }
        // return collector;
    }
}
#[derive(Debug, PartialOrd, PartialEq)]
struct Example {
    name: String,
    color: i32,
}
impl Example {
    fn some_fn(&self) {
        println!("holaaa");
    }
}

use std::rc::Rc;
#[test]
fn rc_test() {
    let stt = Example {
        name: String::from("bomb"),
        color: 69,
    };
    let x = Rc::new(stt);
    let y = x.clone();
    let b = y.clone();
    b.some_fn();
    y.some_fn();
}
