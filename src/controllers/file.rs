pub use crate::controllers::utility;
use chrono::prelude::*;
pub use console;
use std::{
    error::Error,
    fmt,
    fs::File,
    io::prelude::*,
    io::{self, SeekFrom},
    time::Duration,
};
#[derive(Debug)]
pub struct Fileconfig<'a> {
    name: &'a str,
    access_at: Duration,
    query: &'a str,
    // parameters:Vec<&'a str>,
    // content:Option<String>,
    content: &'a str,
    path: &'a str,
}
// conditions:
/*
    exec trait: [alias:(file/any)] [operations] [path/name] [parameters/--options]
    Read:Content _ none
    Write:Content _ String : if use multi line content ->
        check len()::Enum -> i32|&str
    Update:Content _ String
    Delete:Content  _ none

    Some file operations need parameters and some don't;
*/

impl Fileconfig<'_> {
    pub fn new(param: &str, timestamp: fn() -> Duration) -> Result<Fileconfig, &'static str> {
        let mut command_chunk = Vec::new();
        for res in param.trim().split_whitespace() {
            command_chunk.push(res);
        }
        if param == "None" {
            panic!("Execution abort! Need parameters")
        } else if command_chunk.len() < 3 {
            return Err("Insufficient parameters to run file operations!");
        }
        println!("{:#?}", command_chunk);
        let result = Fileconfig {
            name: command_chunk[2],
            query: command_chunk[1],

            path: command_chunk[2],
            access_at: timestamp(),
            content: "None",
        };

        Ok(result)
    }
    pub fn run(&self, _p: &str) {
        let params = _p.split_whitespace().collect::<Vec<&str>>();
        match self.query.to_lowercase().as_str() {
            "update" => {
                // self.write(params[0], params[1].parse::<i32>().unwrap());
            }
            "search" => {
                // self.search()
            }
            "read" => {
                let result = self.read().unwrap();
                println!("res {}", result);
            }
            _ => {}
        }
    }
}

type OriResult<T> = Result<T, Box<dyn Error>>;
/*positions : [{ Number of line to modify / word to replace / newdoc }]*/
pub trait TextPos {
    fn modify(&self, content: String, new_str: &str) -> Vec<String>;
}
pub fn parser(content: &str) {
    let len = 100;
    let dot = utility::Dot::new(len, "+");
    dot.write_x();
    let print_preset = |line: &str, count: i32| {
        let formated_str = format!("{}|   {}", count, line);
        println!(
            "{}",
            console::pad_str(
                formated_str.as_str(),
                len as usize,
                console::Alignment::Left,
                None
            )
        );
    };
    let mut count_line = 0;
    for line in content.lines() {
        if line.len() > 100 {
            let muti_lines = utility::split_chunk(line, 100);
            for line in muti_lines {
                print_preset(line, count_line);
            }
        } else {
            print_preset(line, count_line);
        }
        count_line += 1;
    }
    let wrapper = console::pad_str(
        content.lines().collect::<Vec<&str>>()[2],
        50,
        console::Alignment::Left,
        None,
    );
    // let txt_line = console::measure_text_width(content.lines().collect::<Vec<&str>>()[2]);
    println!("txt l {}", wrapper);
    dot.write_x();
}
// [x1,x2,"string"]
impl TextPos for i32 {
    fn modify(&self, content: String, target: &str) -> Vec<String> {
        unimplemented!();
    }
}
// replace all word within that target across all content
impl TextPos for &str {
    fn modify(&self, content: String, new_str: &str) -> Vec<String> {
        if self.contains(" ") {
            let multi_tar = self.split_whitespace().collect::<Vec<&str>>();
            let emp = multi_tar
                .iter()
                .map(|x| {
                    let xt = content.replace(*x, new_str);
                    if xt != content {
                        return xt;
                    } else {
                        "None".to_string()
                    }
                })
                .filter(|x| *x != "None".to_string())
                .collect::<Vec<String>>();
            // println!("special emp {:#?}",emp);
            return emp;
        } else {
            let mut result: Vec<String> = Vec::new();
            result.push(content.replace(self, new_str));
            return result;
        }
    }
}
pub trait Operation {
    fn read(&self) -> OriResult<String>;
    fn update<T>(&self, new_content: &str, target: T)
    where
        T: TextPos;
    fn search(&self, target: &str);
}
#[derive(Debug)]
struct Emptyfile;
impl fmt::Display for Emptyfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error empty file!")
    }
}
impl Error for Emptyfile {
    fn description(&self) -> &str {
        "File contains empty string , Cannot read content!"
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
fn checkempty(result: &str) -> OriResult<String> {
    if result.is_empty() {
        Err(Box::new(Emptyfile))
    } else {
        Ok(result.to_string())
    }
}

impl Operation for Fileconfig<'_> {
    fn read(&self) -> OriResult<String> {
        let file = File::open(self.path)?;
        let mut buffer = io::BufReader::new(file);
        let mut result = String::new();
        buffer.read_to_string(&mut result)?;
        checkempty(&result)
        // Ok(result)
    }
    // use for string only
    fn update<T: TextPos>(&self, new_content: &str, target: T) {
        // use std::io::Seek;

        /* if target is multiple start spit out different result to different file! */
        let existed_content = self.read().expect("Cannot open that file");
        let mutation = target.modify(existed_content.to_string(), new_content);
        let mut count = 0;
        for n in mutation {
            let new_path = format!("output -- {} [{}]", self.path, count);
            let mut newfile = File::create(new_path).unwrap();
            newfile.write_all(n.as_bytes()).unwrap();
            count += 1;
        }
    }

    fn search(&self, target: &str) {
        let content = self.read().unwrap();
        let mut count = 0;
        let mut line_found = Vec::new();
        for (line_num, x) in content.lines().enumerate() {
            let word_group = x.split_whitespace().collect::<Vec<&str>>();
            if word_group.len() >= 1 && word_group.into_iter().any(|word| word.contains(target)) {
                line_found.push(line_num + 1);
                count += 1;
            }
        }
        /**/
        println!("found target in line {:?}",line_found);
        println!("total word found :{}", count);
    }
}
pub fn run() {
    let test_date = Utc::now();
}
