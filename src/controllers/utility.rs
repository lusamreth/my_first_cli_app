use std::{
    fmt, io,
    time::{SystemTime, UNIX_EPOCH},
};
pub fn getnow() -> std::time::Duration {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("error time went backward!");
    return since_epoch;
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
pub fn writedot(dot_num: i32, dot_type: Option<&'static str>) {
    let mut initial = 0;
    match dot_type {
        Some(x) => {
            for _ in initial..dot_num {
                print!("{}", x);
            }
            print!("\n");
        }
        None => {
            for _ in initial..dot_num {
                println!("+");
            }
            print!("\n");
        }
    }
}
impl Dot<'_> {
    pub fn new<'a>(num:i32,tp:&'a str) -> Dot{
        Dot{
           dot_num:num,
           dot_type:tp 
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
    pub fn content_box(&self,content:&str){
        self.write_x();
        println!("");
        let mid_cont = (content.len() as i32) / 2;
        let mid =  self.dot_num  / 2;
        for _ in 0..(mid - mid_cont){
            // print!("A {}",G);
            print!(" ");
        }
        println!("{}",content);
        self.write_x();
        println!("");
    }
}
/*from init -> mid = (midcol - wordlen)*/
#[derive(Debug)]
pub struct Pathstack {
    pub stack: Vec<String>,
    max: Option<u32>,
    count: u32,
    locked: bool,
}
impl Pathstack {
    pub fn new(maximum: Option<u32>) -> Pathstack {
        Pathstack {
            stack: Vec::new(),
            count: 0,
            max: maximum,
            locked: false,
        }
    }
    pub fn push(&mut self, item: String) {
        if self.locked == false {
            match self.max {
                Some(num) => {
                    if self.count < num {
                        self.stack.push(item);
                        self.count += 1;
                    } else if self.count == num {
                        println!("stack reached maximum capacity!");
                        panic!("full limit");
                    } else {
                        println!("The stack is locked! Cannot push items");
                    }
                }
                None => {
                    self.stack.push(item);
                    self.count += 1;
                }
            }
        }
    }
    pub fn lock(&mut self) {
        self.locked = true
    }
    pub fn unlock(&mut self) {
        self.locked = false
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
