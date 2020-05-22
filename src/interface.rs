pub use std::{ fmt, io};
extern crate cpx2;
pub use cpx2::controllers::{self, utility};
pub fn run() {

    let _breaker = 0;
    let main_menu = make_cmd(vec!["file", "calculator", "sysmain"]);
    for comd in &main_menu{
        println!("{}",comd);
    }
    controllers::make_exec("helllo abana lols");
    let mut path_stack = utility::Pathstack::new(None);
    let dot = utility::Dot::new(50, "+");
    loop {
        let raw = utility::input(Some("->")).unwrap();
        let feedback = raw.trim();
        if feedback != ""{
            println!("{}", feedback);
        }
        
        let mut main_path = String::new();
        // println!("||path {}", main_path);
        match feedback{
            "/" => {
                // print!("path fallback!");
                path_stack.stack.pop();
                path_stack.unlock();
            },
            "menu" => {
                for comd in &main_menu{
                    println!("{}",comd);
                }
            }
            _ => ()
        }
        let feedback_int = feedback.parse::<i32>().unwrap_or_default();
        let accepted_menu = main_menu.iter().find(|x| {
            x.execution == feedback || x.binder == feedback_int
        });
        if path_stack.stack.is_empty() == false{
            controllers::make_exec(&path_stack.stack[0]);
            
        }
        if let Some(cmd) = accepted_menu { 
            path_stack.push(cmd.execution.to_string());
            path_stack.lock();
            // input_tracker.push(tes.execution);
            let msg = format!("{} module has been selected!",cmd.execution);
            dot.content_box(&msg);
        }
        

        // println!("this is d {:#?}",d);
        for path in path_stack.stack.iter(){
            main_path.push_str(path);
        }
        println!("path:// {}",main_path);
        
        
    }
}
/*
    when command get pushed in, lock the cycle; Stop receiver
    / - mean go back to path
*/
/* if input == binder then return execution */
#[derive(Debug)]
struct Command<T, F> {
    execution: F,
    binder: T,
}
impl fmt::Display for Command<i32, &str> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(fmt, "{}.{}", self.binder, self.execution)
    }
}
fn make_cmd(execs: Vec<&str>) -> Vec<Command<i32, &str>> {
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

fn receptor(arg: String, cmds: Vec<String>) {
    for cmd in cmds.iter() {
        let f = utility::first_letter(cmd);
        println!("{:#?}", (f, &arg));
    }
}

#[test]
fn testo(){
    let mut newpath = utility::Pathstack::new(None);
    newpath.push("minecraft".to_string());
    newpath.lock();
    newpath.push("item is locked".to_string());
    let mut stk = newpath.stack.iter();
    stk.next();
    stk.next();
    println!("{:#?}",newpath);
}
#[test]
fn testo2(){
    let xd = utility::Dot::new(50,"-");    
    println!("");
    xd.content_box("ap")
}