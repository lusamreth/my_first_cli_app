use super::file;
use super::utility::{self, ErrorHandler};
pub use std::{fmt, io};
mod state_manager;
use state_manager::Buffer;

pub fn run() {
    let main_menu = utility::make_cmd(vec!["file", "calculator", "sysmain"]);
    for comd in &main_menu {
        println!("{}", comd);
    }
    // controllers::make_exec("helllo abana lols");
    let path_stack = utility::Pathstack::new(None);
    let dot = utility::Dot::new(50, "+");
    let mut config_queue: Vec<String> = Vec::new();
    let mut state_queue = state_manager::BufferState::new(state_manager::Constate::Save, Some(10));
    loop {
        let raw = utility::input(Some("->")).unwrap();
        let feedback = raw.trim();
        if feedback != "" {
            println!("{}", feedback);
        }
        let mut main_path = String::new();
        let mut ui_error_acc = ErrorHandler::Accumulator::init();
        let mut flush = |error: ErrorHandler::FileError| {
            ui_error_acc.0.push(error);
        };
        let mut bufferpop = || {
            if state_queue.count() == 0 {
                return;
            }
            let selector = format!("/{}:n/", state_queue.count() - 1);
            match Buffer::remove(&mut state_queue, &selector) {
                Ok(removed) => {}
                Err(e) => {
                    flush(ErrorHandler::FileError::new().set_message("No buffer to remove from!"))
                }
            }
        };
        match feedback {
            "/" => {
                bufferpop();
                path_stack.unlock();
                path_stack.pop();
            }
            "menu" => {
                for comd in &main_menu {
                    println!("{}", comd);
                }
            }
            _ => (),
        }
        // save_stat function to store cfg
        let save_stat = || {
            let file_op = vec!["save", "new", "default"];
            let list = utility::make_cmd(file_op);
            //render list
            print!("\n");
            list.iter().for_each(|f| print!("{}|", f));
            // render_list(&list);
            return list;
        };

        if path_stack.is_empty() == false {
            match &*path_stack.peek() {
                "file" => {
                    let param = format!("{} {}", path_stack.peek(), feedback);
                    let filecfg = file::Fileconfig::new(&param, utility::getnow);
                    if let Err(x) = filecfg {
                        println!("we got error: {}", x);
                    } else {
                        path_stack.unlock();
                        Buffer::save(
                            &mut state_queue,
                            "name".to_string(),
                            filecfg.unwrap(),
                            &path_stack,
                        );
                        let selector = format!("/{}:n/", state_queue.count() - 1);
                        let config_holder = Buffer::select(&state_queue, &selector);

                        if let Err(error) = config_holder {
                            println!("{}", error)
                        } else {
                            match config_holder.unwrap().run() {
                                Ok(()) => println!("bruh"),
                                Err(fe) => {
                                    flush(fe);
                                    path_stack.unlock();
                                    path_stack.pop();
                                    Buffer::remove(&mut state_queue, &selector)
                                        .unwrap_or_else(|x| panic!("{}", x));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let push_stk = |exec_com: &str| {
            path_stack.push(exec_com.to_string());
            path_stack.lock();
            let msg = format!("{} module has been selected!", exec_com);
            dot.content_box(&msg);
            println!("Choose your operations");
        };
        let feedback_int = feedback.parse::<i32>().unwrap_or_default();
        let accepted_menu = main_menu
            .iter()
            .filter(|x| x.execution == feedback || x.binder == feedback_int)
            .for_each(|each_cmd| push_stk(each_cmd.execution));

        if feedback == "save" {
            println!("this is save mode")
        }
        path_stack
            .get_stack()
            .iter()
            .for_each(|path| main_path.push_str(path));
        println!("path:// {}", main_path);
        ui_error_acc.flush();
    }
}
/*
    when command get pushed in, lock the cycle; Stop receiver
    / - mean go back to path
*/
