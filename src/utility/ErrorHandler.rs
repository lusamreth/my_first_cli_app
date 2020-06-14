use console;
use std::fmt::Write;
#[derive(Debug)]
pub enum ErrorType {
    Warning,
    Fatal,
}
#[derive(Debug)]
pub struct FileError {
    pub kind: String,
    pub message: String,
    error_type: ErrorType,
}
#[derive(Debug)]
pub struct Accumulator(pub Vec<FileError>);
impl Accumulator {
    pub fn init() -> Self {
        return Accumulator(Vec::new());
    }
    pub fn flush(&mut self) {
        let mut tag: Vec<&str> = Vec::new();
        let error_tage = console::Style::new()
            .apply_to("Error")
            .bold()
            .white()
            .bg(console::Color::Yellow)
            .to_string();

        let mut set_color = |typa: &ErrorType, tag: &mut Vec<&str>| {
            let setting = console::Style::new().bright();
            match typa {
                ErrorType::Fatal => {
                    tag.push("Fatal");
                    setting.red()
                }
                ErrorType::Warning => {
                    tag.push("Warning");
                    setting.yellow()
                }
            }
        };
        for x in self.0.iter() {
            let mut buffer = String::new();
            let msg_to_display = set_color(&x.error_type, &mut tag)
                .apply_to(&x.message)
                .to_string();

            let full_tag = console::style(format!("{} Error", tag.iter().next().unwrap()))
                .on_red()
                .bright();
            writeln!(buffer, "{} {}", full_tag, msg_to_display)
                .expect("Cannot write Error to the display!");
            println!("{}", buffer);
        }
    }
}
impl FileError {
    pub fn new() -> Self {
        return FileError {
            kind: String::from("file"),
            message: String::new(),
            error_type: ErrorType::Fatal,
        };
    }
    fn chain(self) -> Self {
        return self;
    }
    pub fn set_type(mut self, e: ErrorType) -> Self {
        self.error_type = e;
        self.chain()
    }
    pub fn describe(mut self, kind: &str) -> Self {
        self.kind = String::new();
        self.kind.push_str(kind);
        self.chain()
    }
    pub fn set_message(mut self, msg: &str) -> Self {
        self.message.push_str(msg);
        self.chain()
    }
    pub fn panic(self) {
        let setting = console::Style::new();
        let error_tag = setting
            .apply_to("Error")
            .bold()
            .white()
            .bg(console::Color::Yellow)
            .to_string();

        let styled_msg = setting.bright().red().apply_to(self.message).to_string();
        print!("{}: {}", error_tag, styled_msg);
        std::process::exit(1)
    }
}
impl From<std::io::Error> for FileError {
    fn from(error: std::io::Error) -> Self {
        let new_error = FileError::new()
            .describe("io")
            .set_message(&error.to_string());
        return new_error;
    }
}
#[test]
fn test_err_handler() {
    let mut Handler = Accumulator::init();
    for _ in 0..2 {
        let newmsg = FileError::new()
            .set_message("bro wtf")
            .set_type(ErrorType::Warning)
            .describe("file");
        Handler.0.push(newmsg);
    }
    Handler.flush();
   
}
