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
pub use super::interface::{self, components, printer, template_engine, text_processing};
pub use super::utility::{self, ErrorHandler::FileError};
use parameters::filter_param;
use printer::TermCfg;
use std::{
    fs::File,
    io::prelude::*,
    io::{self},
    time::Duration,
};
use template_engine::TemplateBuilder;
use template_engine::TemplateEngine;
type Params = Vec<String>;
use std::collections::hash_map::HashMap;
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
            //^<\w++>$
            let p_reg = regex::Regex::new(r"^\--+[a-zA-Z]+").unwrap_or_else(|x| throw_reg_panic(x));
            let quote_word = regex::Regex::new(r#"(["'])((\\{2})*|(.*?[^\\](\\{2})*))"#)
                .unwrap_or_else(|x| throw_reg_panic(x));
            let match_inside_brac = regex::Regex::new(r"^\[(.*)\]$").unwrap();

            p_vec
                .filter(|vec_s| {
                    // println!("hm {}")
                    match_inside_brac.is_match(vec_s)
                        | p_reg.is_match(vec_s)
                        | quote_word.is_match(vec_s)
                })
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
    fn parse_quotation(&self, param: &Vec<String>) -> Vec<String> {
        let quoted = |st: &str| st.starts_with("\"") && st.ends_with("\"");
        param
            .into_iter()
            .filter(|st| quoted(st))
            .map(|quote_par| {
                text_processing::CrumpCluster::break_chunk(&quote_par)
                    .delete(0, Some(1))
                    .delete(quote_par.len() - 1, Some(quote_par.len()))
                    .merge_crump()
            })
            .collect::<Vec<String>>()
    }
    fn parse_bracket(&self, param: &Vec<String>) -> Vec<String> {
        let match_brack: &[_] = &['[', ']', '\"'];
        param
            .iter()
            // .filter(|general_param| match_inside_brac.is_match(general_param))
            .flat_map(|bk_par| {
                let split_brack = bk_par
                    .trim_matches(match_brack)
                    .split_whitespace()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                return split_brack;
            })
            .collect::<Vec<String>>()
        //   .filter(|bracketed|);
    }
    pub fn run(&self) -> Result<(), FileError> {
        let init_ptr = TermCfg::new().set_attr(console::Attribute::Bold);
        let print = init_ptr.gen_print(Some(console::Color::Blue));
        let mut print_ln = init_ptr.gen_println(Some(console::Color::Blue));

        let mut err_collector: Vec<FileError> = Vec::new();
        let display_txt = |txt: &str| {
            let filtertxt = filter_param(&self.parameters, txt);
            let mut tmp_engine = template_engine::TemplateFactory::init()
                .parse_in_template(&filtertxt)
                .create_movable()
                .collect();

            let template = tmp_engine
                .padding(vec![6, 6, 6, 6]);
            template.display();
        };
        match self.query.as_str() {
            "update" => {
                // self.write(params[0], params[1].parse::<i32>().unwrap());
                println!("what is your ct?");
                let elim_quote = self.parse_quotation(&self.parameters);
                self.update(&elim_quote[1], elim_quote[0].clone().as_str());
            }
            "search" => {
                let unquote = self.parse_bracket(&self.parameters);
                println!("{}", &format!("<->statistics of word {:?}<->", unquote));
                let mut p = init_ptr.gen_println(Some(console::Color::Blue));
                for quoted in unquote {
                    match self.search(&quoted) {
                        Ok(found_map) => {
                            print!("Highligted-Text: \n");
                            let full_content = self.read().unwrap();
                            let total_line = found_map.len();

                            let mut key_holder = Vec::new();

                            found_map.iter().for_each(|(key, _)| key_holder.push(key));
                            let mut count = 0;
                            let mut crumps = full_content
                                .lines()
                                .into_iter()
                                .enumerate()
                                .map(|(idx, x)| {
                                    (idx as i64, text_processing::CrumpCluster::break_chunk(x))
                                })
                                .collect::<Vec<(i64, text_processing::CrumpCluster)>>();
                            while count < found_map.len() {
                                // each_indx.iter().for_each(|x|)
                                crumps.iter_mut().for_each(|(loc, crump)| {
                                    if loc == key_holder[count] {
                                        let locations = found_map.get(loc).unwrap();
                                        locations.into_iter().for_each(|(start, end)| {
                                            crump.delete(*start, Some(*end));
                                            crump.insert(
                                                *start,
                                                &console::style(quoted.clone()).red().to_string()
                                            );
                                        });
                                    }
                                });
                                count += 1;
                            }
                            let fully_merged = crumps
                                .iter()
                                .map(|(_, crump)| {
                                    let merged = crump.merge_crump();
                                    return merged;
                                })
                                .collect::<String>();
                            display_txt(&fully_merged);
                            // display_txt(&fully_merged, "+/");
                            if total_line <= 1 {
                                p(&"No word found in the text!")?;
                            } else {
                                p(&format!(
                                    "->Number of line that contain word /{}/: {}",
                                    quoted,total_line
                                ))?;
                            }
                        }
                        Err(file_err) => err_collector.push(file_err),
                    }
                }
            }
            "read" => {
                let result = self.read();
                match result {
                    Ok(txt) => display_txt(&txt),
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
    fn search(&self, target: &str) -> Result<HashMap<i64, Vec<(usize, usize)>>, FileError>;
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
        println!("muttip {:?}", mutation);
        let mut count = 0;
        for n in mutation {
            let new_path = format!("output -- {} [{}]", self.path, count);
            let mut newfile = File::create(new_path).unwrap();
            newfile.write_all(n.as_bytes()).unwrap();
            count += 1;
        }
    }
    // regex for search: ^"[a-zA-Z-\s]+"
    fn search(&self, target: &str) -> Result<HashMap<i64, Vec<(usize, usize)>>, FileError> {
        let mut err_clt = String::new();
        // let found_map = Vec::new();
        let mut found_map: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();
        if self.parameters.is_empty() {
            err_clt.push_str("No params!")
        }
        let mut content = String::new();
        match self.read() {
            Ok(ct) => content.push_str(&ct),
            Err(read_error) => err_clt.push_str(&read_error.message),
        }
        let mut count: i64 = 0;
        let mut line_found = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let each_line = line.trim();
            let word_group = each_line.split_whitespace().collect::<Vec<&str>>();
            let reg = regex::Regex::new(&format!(r"{}", target)).unwrap();
            let mut indx_vec = Vec::new();
            for found in reg.find_iter(line) {
                let key_indx = (found.start(), found.end());
                indx_vec.push(key_indx);
            }

            if word_group.len() >= 1 && word_group.into_iter().any(|word| word.contains(target)) {
                line_found.push(line_num);
                found_map.insert(line_num as i64, indx_vec);
                count += 1;
            }
        }
        if err_clt.len() > 0 {
            let bruh = FileError::new().set_message(&err_clt.clone());
            return Err(bruh);
        } else {
            return Ok(found_map);
        }
        /**/
    }
}
impl Clone for Fileconfig {
    fn clone(&self) -> Self {
        return Fileconfig {
            name: self.name.clone(),
            access_at: self.access_at,
            query: self.query.clone(),
            parameters: self.parameters.clone(),
            // content:Option<String>,
            content: self.content.clone(),
            path: self.path.clone(),
        };
    }
}
#[test]
fn test() {
    let match_inside_brac = regex::Regex::new(r"^\[(.*)\]$").unwrap();
    let test = "[Apple sauce bananan ba;;;a]";
    println!("t {}", test);
    let x: &[_] = &['[', ']'];
    println!(
        "test  {:?} ",
        (match_inside_brac.is_match(test), test.trim_matches(x))
    );
}

