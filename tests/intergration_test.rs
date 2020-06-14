extern crate console;
extern crate cpx2;
use std::collections::hash_map::HashMap;

#[test]
fn textpos() {
    let styler = console::Style::new();
    let aye = styler.underlined().apply_to("xdddddd").cyan();
    println!("this is console {}", console::style("quite").cyan());
    println!("another val {}", aye);
}
type Value = String;
#[derive(PartialEq, Debug, Clone)]
pub enum Constate {
    Save,
    Default,
    New(Value),
}
#[test]
fn save_or_nay() {
    let aye = Constate::New("read foo.txt --l --x".to_string());
    match aye {
        Constate::Save => {
            println!("this is save");
            // invoke the save function
        }
        Constate::Default => {
            println!("Use default settings");
            // leave it ask temporary
        }
        Constate::New(val) => {
            println!("new value created {}", val);
            // cache the new_config
        }
    }
}
/* if input == binder then return execution */
#[derive(Debug)]
enum BufferResult<T, E> {
    Value(std::rc::Rc<T>),
    Err(E),
}
use std::rc::Rc;
impl BufferResult<&Example, String> {
    fn get_value(&self) -> Option<&Rc<&Example>>{
        match self {
            BufferResult::Value(value) => {
                println!("this is value! {:?}", value);
                return Some(value);
            }
            _ => None
        }
    }
}

trait Buffer<T> {
    fn verify(&self, opt: &str) -> Option<Constate>;
    fn save(&mut self, name: String, content: T, path_stack_mock: &mut Vec<String>) -> ();
    fn remove(&mut self, index: &str) -> Result<(), String>;
    fn select(&self, index: &str) -> Result<&T, String>;
    fn clear(&mut self) -> ();
    fn automate<N>(&self, command: &str) -> BufferResult<&T, String>;
}
/*
    Save: [Save : name],
    Delete: [Delete : index(num/str)],
    Clear: [()],
    Select: [Select: index(num/str)]
*/
#[derive(Debug, Clone)]
struct BufferState<T> {
    exec_type: Constate,
    data_map: HashMap<String, T>,
}
impl<A> BufferState<A> {
    fn new<'a>(ext: Constate) -> BufferState<A> {
        BufferState {
            exec_type: ext,
            data_map: HashMap::new(),
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
    fn save(&mut self, name: String, content: T, path_stack_mock: &mut Vec<String>) -> () {
        if self.exec_type != Constate::Save {
            return;
        }
        let mut num = 0;
        let ct_str = name;

        let mut cfg_naming_char: String = String::new();
        for _ in 1..path_stack_mock.len() {
            num += 1
        }
        path_stack_mock.iter().for_each(|each_path| {
            println!("ep : {}", each_path.chars().nth(0).unwrap());
            let param_char = ct_str
                .chars()
                .nth(0)
                .unwrap_or_else(|| panic!("The content parameter is empty!"));
            match each_path.chars().nth(1) {
                Some(p_char) => {
                    // if the first character is the same
                    println!("hmm pcha {}", param_char);
                    if p_char == param_char {
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
        let format_cfg = |param_char| format!(" cfg/{}:{}/", num, param_char);
        println!("cfg {}", cfg_naming_char);
        path_stack_mock.push(String::from(format_cfg(cfg_naming_char)));
        println!("path : {:?}", path_stack_mock);
        let piter: &str = &path_stack_mock
            .get(num + 1)
            .unwrap_or_else(|| panic!("Unknown buffer error!"));
        self.data_map.insert(piter.trim()[3..].to_string(), content);
    }
    fn clear(&mut self) -> () {
        self.data_map.clear();
        self.exec_type = Constate::Default;
    }
    fn remove(&mut self, index: &str) -> Result<(), String> {
        let rmt = self.data_map.remove(index);
        if let None = rmt {
            Err(String::from(
                "Cannot perform remove operation! index not found!",
            ))
        } else {
            Ok(())
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
    fn automate<N>(&self, command: &str) -> BufferResult<&T, String> {
        let divide = command.split_whitespace().collect::<Vec<&str>>();
        let operation = divide[0];
        let parameters = divide[1];

        // let mut store = |result| collector.push(result);
        match operation.to_lowercase().as_str() {
            "select" => BufferResult::Value(std::rc::Rc::new(self.select(parameters).unwrap())),
            _ => BufferResult::Err("bpi".to_string()),
        }
        // return collector;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Example {
    name: String,
    color: i32,
}
impl Example {
    pub fn do_a(&self){
       println!("this is do sth {}",self.name);
    }
}
#[test]
fn test_buffer() {
    // let hm = HashMap::new();
    let mut state1 = BufferState::new(Constate::Save);
    assert_eq!(Constate::Save, state1.verify("save").unwrap());
    let mut mock_path = Vec::new();
    mock_path.push("cca".to_string());
    state1.save(
        "boconut".to_string(),
        Example {
            name: String::from("heyppp"),
            color: 69,
        },
        &mut mock_path,
    );
    state1.save(
        "coconut".to_string(),
        Example {
            name: String::from("heyaaappp"),
            color: 69,
        },
        &mut mock_path,
    );
    let umm = state1.verify("save");
    match umm {
        Some(token) => {}
        None => println!("Error! invalid cmd!"),
    }
    let mut storage = Vec::new();
    
    let test_result_enum = state1.automate::<String>("select /0:b/");
    storage.push(test_result_enum.get_value());
    let ohn = storage.get(0).unwrap();
    let oh_yah =  ohn.unwrap();
    // unwrap from Buffer ahhhhhhhhhhhhhhhh
    println!("{:#?}",Example::do_a(oh_yah));
    println!("P :{:#?}", state1.data_map);
}
#[test]
 fn rc_cell(){
    let hmm = Rc::new(Example{
        name:"aahha".to_string(),
        color:60
    });
    println!("hm {:?}",Example::do_a(&hmm));
 }
 