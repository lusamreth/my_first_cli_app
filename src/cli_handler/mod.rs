use super::file;
use super::interface::{
    components::{self, border},
    printer,
    template_engine::Padding,
};
use super::utility::{self, ErrorHandler};
pub use std::{fmt, io};
mod state_manager;
use console::{self, Color};
use state_manager::Buffer;
use std::thread;

pub fn run() {
    #[macro_use]
    let main_menu = utility::make_cmd(vec!["Save", "Import(not yet implemented)", "Default"]);
    let map_to_temp = main_menu
        .iter()
        .map(|x| {
            let to_p = format!("{}.{}", x.binder, x.execution);
            return to_p;
        })
        .collect::<Vec<String>>();
    let pad = Padding::create().input(vec![5, 5, 3, 3]).to_owned();
    let ix = border(
        "+=",
        map_to_temp,
        file::components::BorderWeight::Light,
        pad,
    );
    ix.iter().for_each(|x| println!("{}", x));
    let path_stack = utility::Pathstack::new(None);
    let dot = utility::Dot::new(50, "+");
    let state_queue = state_manager::BufferState::new(state_manager::Constate::Save, Some(10));
    let mut mx: u16 = 0;
    // let (print_str,print_line) =
    loop {
        let raw = utility::input(Some(">>")).unwrap();
        let pstr = init_print();
        let print = pstr.gen_print(None);
        let mut print_ln = pstr.gen_println(None);

        let feedback = raw.trim();

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
            match Buffer::remove(&state_queue, &selector) {
                Ok(()) => {}
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
                    print_ln(&format!("{}", comd));
                }
            }
            _ => (),
        }
        // save_stat function to store cfg
        let mut op_runner = |config_holder: Option<file::Fileconfig>| {
            let selector = format!("/{}:n/", state_queue.count() - 1);
            match config_holder.unwrap().run() {
                Ok(()) => {}
                Err(fe) => {
                    flush(fe);
                    path_stack.unlock();
                    path_stack.pop();
                    state_queue
                        .remove(&selector)
                        .unwrap_or_else(|x| panic!("{}", x));
                }
            }
        };

        let init_config = || {
            let param = format!("{} {}", path_stack.peek(), feedback);
            let filecfg = file::Fileconfig::new(&param, utility::getnow);
            return filecfg;
        };
        if path_stack.is_empty() == false {
            match &*path_stack.peek() {
                "Save" => {
                    let filecfg = init_config();
                    if let Err(x) = filecfg {
                        println!("we got error: {}", x);
                    } else {
                        path_stack.unlock();
                        state_queue.save("name".to_string(), filecfg.unwrap(), &path_stack);
                        let selector = format!("/{}:n/", state_queue.count() - 1);
                        let config_holder = state_queue.select(&selector);
                        op_runner(config_holder);
                    }
                }
                "Default" => {
                    println!("selected");
                    let filecfg = init_config();
                    match filecfg {
                        Ok(cfg) => {
                            match cfg.run() {
                                Ok(()) => {}
                                Err(st) => flush(st),
                            };
                        }
                        Err(efcf) => {
                            flush(ErrorHandler::FileError::new().set_message(efcf));
                        }
                    }
                }
                _ => {}
            }
        }
        let push_stk = |exec_com: &str| {
            let st = format!("{} module has been selected!", exec_com);
            let msg = vec![st];
            border(
                "+=",
                msg,
                file::components::BorderWeight::Light,
                Padding::create(),
            )
            .iter()
            .for_each(|x| println!("{}", x));
            let z = exec_com.to_string();
            path_stack.push(z.clone());
            path_stack.lock();
        };
        let feedback_int = feedback.parse::<i32>().unwrap_or_default();
        match utility::match_command(feedback, &main_menu) {
            Some(comd) => push_stk(comd.execution),
            None => {}
        }

        // print_ln(&format!("this is matched {:?}", matched));
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
fn init_print<'a>() -> printer::TermCfg {
    let init = printer::TermCfg::new().set_attr(console::Attribute::Italic);
    return init;
}
