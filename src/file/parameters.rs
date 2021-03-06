extern crate serde;
extern crate serde_json;
use super::utility::ErrorHandler;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::hash_map;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
/**
Parameters-Scope:
    prefixes/consecutive commands:
        -select:[1,2,3,4,...]
        -[operation:(search,read,write,update)]:
        -mode:enum([default,save,new])

    affixes/proxy parameters:
        --c:case-sensitive/default
        --L[ower]:lower_case
        --U[pper]:upper_case
        --red/green/blue: --r/--g/--b
        --center/left/right

**/
#[derive(Debug, Deserialize)]
struct FileParam {
    pub short: String,
    long: String,
    description: String,
}
#[derive(Debug, Deserialize)]
enum Argtype {
    Prefixes,
    Affixes,
}
type StringFunct = Box<dyn FnMut(&str) -> String>;
#[derive(Debug, Deserialize)]
struct ParamConfigBuilder {
    limit: Option<i32>,
    arg_type: Argtype,
    commands: Vec<FileParam>,
}
struct ParamConfig {
    limit: i32,
    arg_type: Argtype,
    commands: Vec<FileParam>,
    prefix: String,
    executatble: hash_map::HashMap<String, StringFunct>,
    count: usize,
}
pub fn filter_param(parameters: &Vec<String>, input: &str) -> String {
    let mut result = String::new();
    if parameters.is_empty() == false {
        parameters
            .iter()
            .for_each(|param| match param.to_lowercase().as_str() {
                "--u" => result.push_str(&input.to_uppercase()),
                "--l" => result.push_str(&input.to_lowercase()),
                "--c" | "default" | _ => result = input.to_string(),
            })
    } else {
        result = input.to_string()
    }
    return result;
}

/*
    Trait and implementations descriptions
    Param_config_factory:
    -Trait:{
        fn create(--prefix:&str) <-Read configurations
        fn chainer(&mut self) <- enabling chaining
        fn encapasulate(&self,closure!) <- Generic static function!
        fn builtmatcher() <- Complete building!
    }
    Invoker:
    -Trait:{

    }
*/
trait ParamFactory {
    fn create(address: &str) -> Rc<RefCell<Self>>;
    fn chain_ref(&mut self) -> &mut Self;
    fn add_prefix(&mut self, prefix: &str) -> &mut Self;
    fn collect(self) -> ParamConfig;
}
impl ParamFactory for ParamConfigBuilder {
    fn create(address: &str) -> Rc<RefCell<Self>> {
        let get_ref = |res| Rc::new(RefCell::new(res));
        //Todo: handle this more appropriately
        let json_file = File::open(address).expect("Cannot read config file!");

        let mut buffer = String::new();
        let mut reader = std::io::BufReader::new(json_file);
        reader
            .read_to_string(&mut buffer)
            // should always read valid config file!
            .unwrap_or_else(|x| panic!("pac! :{}", x));

        let res: ParamConfigBuilder = serde_json::from_str(&buffer).expect("ahh");
        return get_ref(res);
    }
    fn add_prefix(&mut self, prefix: &str) -> &mut Self {
        if prefix.len() > 2 {
            panic!("prefix only accepts string with len of 2!");
        }
        self.commands.iter_mut().for_each(|par| {
            let split_char = |str: &str| {
                return str.chars().collect::<Vec<char>>();
            };

            let prepend = |tar: String| {
                let mut splited = split_char(&tar);
                splited.splice(0..0, prefix.chars());
                return splited.into_iter().collect::<String>().to_owned();
            };
            par.long = prepend(par.long.to_string());
            par.short = prepend(par.short.to_string());
        });
        self.chain_ref()
    }

    fn chain_ref(&mut self) -> &mut Self {
        return self;
    }
    fn collect(self) -> ParamConfig {
        let first: &FileParam = self.commands.get(1).unwrap();
        let prfx = first.short.chars().take(2).collect();
        ParamConfig {
            commands: self.commands,
            limit: self.limit.unwrap_or(9999999),
            arg_type: self.arg_type,
            prefix: prfx,
            executatble: hash_map::HashMap::new(),
            count: 0,
        }
    }
}
impl ParamConfig {
    pub fn add_execs(&mut self, execs: StringFunct) {
        if self.count < self.limit as usize {
            self.executatble
                .insert(self.commands[self.count].short.clone(), execs);
            self.count += 1;
        } else {
            panic!("Maximum amount of executables")
        }
    }
    pub fn exec(
        &mut self,
        command: &str,
        content: &str,
    ) -> Result<String, ErrorHandler::FileError> {
        match self.executatble.get_mut(command) {
            Some(ex) => Ok(ex(content)),
            None => Err(ErrorHandler::FileError::new()
                .set_message("Invalid parameters!")
                .describe("Parameter Error!")),
        }
    }
}
fn add_prefix(pb: Rc<RefCell<ParamConfigBuilder>>, prefix: Option<&str>) {
    match prefix {
        Some(val) => {
            if val.len() > 2 {
                panic!("Only accepts prefix of two symbols!");
            } else {
                pb.borrow_mut()
                    .commands
                    .iter_mut()
                    .for_each(|x| x.long = "wtf".to_string());
            }
        }
        None => {}
    }
}
// dependency:
/*
    Need arg: 1 params: -> get from self;
    Need user arg : 1 params -> get from usr(file_function);
    Need additional augment params -> proxy from args
*/

fn tex() {}
