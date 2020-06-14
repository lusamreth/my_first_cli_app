/*               File_config_functionalities_pmz                 */
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
mod parameters;
pub use super::interface;
pub use super::utility::{self, ErrorHandler::FileError};
use chrono::prelude::*;
use parameters::filter_param;
use std::{
    fs::File,
    io::prelude::*,
    io::{self},
    time::Duration,
};
use unicode_segmentation::UnicodeSegmentation;
type Params = Vec<String>;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Fileconfig {
    name: String,
    access_at: Duration,
    query: String,
    parameters: Params,
    // content:Option<String>,
    content: String,
    path: String,
}
impl Fileconfig {
    pub fn new(param: &str, timestamp: fn() -> Duration) -> Result<Fileconfig, &'static str> {
        let mut command_chunk = Vec::new();
        for res in param.trim().split_whitespace() {
            command_chunk.push(res.to_owned());
        }
        if command_chunk.len() < 3 {
            return Err("Insufficient parameters to run file operations!");
        }
        let capture = |index: usize| command_chunk.get(index).unwrap().to_owned();

        let parameters: Vec<String> = if command_chunk.len() > 3 {
            let v_param = command_chunk[3..command_chunk.len()].to_owned();
            let p_vec = v_param.into_iter().map(|p_str| String::from(p_str));

            // let tup = (p_reg,quote_word);
            //^"[a-zA-Z-\s]+"
            let throw_reg_panic =
                |regex_err: regex::Error| panic!("Verification Errors! : {}", regex_err);

            let p_reg = regex::Regex::new(r"^\--+[a-zA-Z]+").unwrap_or_else(|x| throw_reg_panic(x));
            let quote_word =
                regex::Regex::new(r#""[a-zA-Z]+\d+""#).unwrap_or_else(|x| throw_reg_panic(x));

            p_vec
                .filter(|vec_s| p_reg.is_match(vec_s) | quote_word.is_match(vec_s))
                .collect::<Vec<String>>()
        } else {
            Vec::new()
        };

        let result = Fileconfig {
            name: capture(2),
            query: capture(1),
            path: capture(2),
            access_at: timestamp(),
            parameters: parameters,
            content: String::from("None"),
        };

        Ok(result)
    }
    pub fn run(&self) -> Result<(), FileError> {
        let mut err_collector: Vec<FileError> = Vec::new();
        match self.query.as_str() {
            "update" => {
                // self.write(params[0], params[1].parse::<i32>().unwrap());
            }
            "search" => {
                // self.search()
                let target = self
                    .parameters
                    .get(0)
                    .unwrap_or(&String::from("Lorem"))
                    .to_owned();
                let filtered_target = filter_param(&self.parameters, &target);
                self.search(&target);
            }
            "read" => {
                let result = self.read();
                match result {
                    Ok(txt) => {
                        let filtertxt = filter_param(&self.parameters, txt.as_str());
                        // parser(&filtertxt);
                        let bordered_box = interface::border(
                            "++",
                            interface::parse_in_template(&filtertxt),
                            interface::BorderWeight::Bolder,
                        );
                        bordered_box.iter().for_each(|line| println!("{}", line))
                    }
                    Err(file_err) => {
                        err_collector.push(file_err);
                    }
                }
            }
            _ => err_collector.push(FileError::new().set_message("Invalid operation!")),
        }
        if err_collector.len() > 0 {
            Err(err_collector.into_iter().next().unwrap())
        } else {
            Ok(())
        }
    }
}

type OriResult<T> = Result<T, FileError>;
/*positions : [{ Number of line to modify / word to replace / newdoc }]*/
pub trait TextPos {
    fn modify(&self, content: String, new_str: &str) -> Vec<String>;
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
    fn search(&self, target: &str) -> Result<(), FileError>;
}

fn checkempty(result: &str) -> OriResult<String> {
    if result.is_empty() {
        let empty_err = FileError::new().set_message("The Folder is Empty inside");
        Err(empty_err)
    } else {
        Ok(result.to_string())
    }
}
impl Operation for Fileconfig {
    fn read(&self) -> OriResult<String> {
        let file = File::open(&self.path)?;
        let mut buffer = io::BufReader::new(file);
        let mut result = String::new();
        buffer.read_to_string(&mut result)?;
        checkempty(&result)
    }
    // use for string only
    fn update<T: TextPos>(&self, new_content: &str, target: T) {
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
    // regex for search: ^"[a-zA-Z-\s]+"
    fn search(&self, target: &str) -> Result<(), FileError> {
        let mut err_clt = String::new();
        if self.parameters.is_empty() {
            err_clt.push_str("No params!")
        }
        let mut content = String::new();
        match self.read() {
            Ok(ct) => content.push_str(&ct),
            Err(read_error) => err_clt.push_str(&read_error.message),
        }
        let mut count = 0;
        let mut line_found = Vec::new();
        for (line_num, x) in content.lines().enumerate() {
            let each_word = x.trim();
            let word_group = each_word.split_whitespace().collect::<Vec<&str>>();

            if word_group.len() >= 1 && word_group.into_iter().any(|word| word.contains(target)) {
                line_found.push(line_num + 1);
                count += 1;
            }
        }
        if err_clt.len() > 0 {
            let bruh = FileError::new().set_message(&err_clt.clone());
            return Err(bruh);
        } else {
            return Ok(());
        }
        /**/
        println!("found target in line {:?}", line_found);
        println!("total word found :{}", count);
    }
}
#[test]
fn test_grapheme() {
    let som = "ab3123123 sdasd asdasd asd ";
    let split_whitespace = som.unicode_words().collect::<Vec<&str>>();
    let grap = UnicodeSegmentation::graphemes(som, true)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", (grap, split_whitespace));
}
