pub mod file;
pub mod utility;
pub fn make_exec(command: &str) -> () {
    
    match command.to_lowercase().as_str() {
        "sysmain" => {
        }
        "calculator" => {
        }
        "file" => {
            // println!("File module has been selected!");
            
            let fileconfig =
                file::Fileconfig::new(&command, utility::getnow);
            if let Err(e) = fileconfig {
                println!("File application Error : {}",e);
            }else{
                fileconfig.unwrap().run(command);
            }
            
            // println!("P_test {:#?}",&Fileconfig);
            // file::run()
        }
        _ => {}
    }
}
