use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fmt, io,
    time::{SystemTime, UNIX_EPOCH},
};
pub mod ErrorHandler;
pub fn getnow() -> std::time::Duration {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("error time went backward!");
    return since_epoch;
}
pub fn write(){

}
pub fn first_letter<'a>(word: &str) -> String {
    word.split(' ').flat_map(|x| x.chars().nth(0)).collect()
}

pub fn input(message: Option<&str>) -> Result<String, Vec<String>> {
    // .expect("Some commands could not be accepted"
    if let Some(msg) = message {
        print!("{}", msg);
    }
    let mut input = String::new();
    let mut err_collector = Vec::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n >= 50 {
                err_collector.push("Command line reached the maximum word length!".to_string())
            }
        }
        Err(error) => err_collector.push(error.to_string()),
    }

    if err_collector.len() != 0 {
        Err(err_collector)
    } else {
        Ok(input)
    }
}
pub struct Dot<'a> {
    dot_type: &'a str,
    dot_num: i32,
}
impl Dot<'_> {
    pub fn new<'a>(num: i32, tp: &'a str) -> Dot {
        Dot {
            dot_num: num,
            dot_type: tp,
        }
    }
    pub fn write_x(&self) {
        for _ in 0..self.dot_num {
            print!("{}", self.dot_type);
        }
        print!("\n");
    }
    pub fn write_y(&self) {
        for _ in 0..self.dot_num {
            println!("{}", self.dot_type);
        }
    }
    pub fn content_box(&self, content: &str) {
        self.write_x();
        println!("");
        let mid_cont = (content.len() as i32) / 2;
        let mid = self.dot_num / 2;
        for _ in 0..(mid - mid_cont) {
            // print!("A {}",G);
            print!(" ");
        }
        println!("{}", content);
        self.write_x();
        println!("");
    }
}
/*from init -> mid = (midcol - wordlen)*/
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Pathstack {
    stack: Rc<RefCell<Vec<String>>>,
    max: Option<u32>,
    count: RefCell<u32>,
    locked: RefCell<bool>,
}
// type Wrapper<'a> = std::slice::Iter<'a, String>;
impl Pathstack {
    pub fn new(maximum: Option<u32>) -> Pathstack {
        Pathstack {
            stack: Rc::new(RefCell::new(Vec::new())),
            count: RefCell::new(0),
            max: maximum,
            locked: RefCell::new(false),
        }
    }
    pub fn push(&self, item: String) {
        if *self.locked.borrow() == true {
            return;
        }
        let push_item = || self.stack.borrow_mut().push(item);
        let count = *self.count.borrow();
        let mut inner_count = self.count.borrow_mut();
        match self.max {
            Some(num) => {
                if count < num {
                    push_item();
                    *inner_count += 1;
                } else if count == num {
                    println!("stack reached maximum capacity!");
                    panic!("full limit");
                } else {
                    println!("The stack is locked! Cannot push items");
                }
            }
            None => {
                push_item();
                *inner_count += 1;
            }
        }
    }
    pub fn pop(&self) {
        let mut inner_count = self.count.borrow_mut();
        if *self.locked.borrow() == true {
            return;
        }
        self.stack.borrow_mut().pop();
        if *inner_count > 0{
            *inner_count -= 1;
        }else{
            return;
        }
        
    }
    pub fn is_empty(&self) -> bool {
        return *self.count.borrow() == 0;
    }
    pub fn peek(&self) -> String {
        if self.is_empty() {
            panic!("cannot peek into empty path!")
        }
        self.stack.borrow().get(0).unwrap().to_owned()
    }
    fn chain_ref(&self) -> &Self {
        self
    }
    pub fn lock(&self) {
        *self.locked.borrow_mut() = true;
    }
    pub fn unlock(&self) -> &Self {
        *self.locked.borrow_mut() = false;
        self.chain_ref()
    }
    // -> std::iter::Enumerate<Wrapper>
    pub fn get_stack<'a>(&self) -> Vec<String> {
        return self.stack.borrow().clone();
    }
}

pub fn split_chunk(line: &str, limit: usize) -> Vec<&str> {
    pub use std::str;
    line.as_bytes()
        .chunks(limit)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command<T, F> {
    pub execution: F,
    pub binder: T,
}
impl fmt::Display for Command<i32, &str> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(fmt, "{}.{}", self.binder, self.execution)
    }
}
pub fn make_cmd(execs: Vec<&str>) -> Vec<Command<i32, &str>> {
    let mut count: usize = 0;
    let mut cmd: Vec<Command<i32, &str>> = Vec::new();
    while count < execs.len() {
        let new = Command {
            execution: execs[count],
            binder: (count + 1) as i32,
        };
        cmd.push(new);
        count = count + 1
    }

    return cmd;
}
pub trait CmdComp: std::fmt::Debug + std::string::ToString {}
impl<T: std::fmt::Debug + std::string::ToString> CmdComp for T {}
pub fn match_command<'a, T, E>(
    param: &str,
    cmd_list: &'a Vec<Command<T, E>>,
) -> Option<&'a Command<T, E>>
where
    T: CmdComp,
    E: CmdComp,
{
    let input = param
        .trim()
        .split(".")
        .filter(|s| !s.is_empty() && !s.contains(" "))
        .map(|s| s.to_lowercase().to_string())
        .collect::<Vec<String>>();

    let p_get = |num: usize| {
        input
            .get(num)
            .unwrap_or_else(|| panic!("Insufficient length of parameters"))
    };
    let result = cmd_list.into_iter().find(|each_cmd| {
        if input.len() == 1 {
            let ip = p_get(0);

            each_cmd.binder.to_string() == *ip || each_cmd.execution.to_string() == *ip
        } else {
            let input_binder = p_get(0);
            let input_exec = p_get(1);

            each_cmd.binder.to_string() == *input_binder
                && each_cmd.execution.to_string() == *input_exec
        }
    });
    return result;
}

#[test]
fn test_verify() {
    let t = make_cmd(vec!["apple", "pencil", "dede"]);
    let c = Command {
        binder: "!",
        execution: "anananaa",
    };
    println!("this is cmd : P{:?}", match_command("1", &t));
}

