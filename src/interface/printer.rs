use console::{Attribute, Color};
use std::io::{self, Write};

pub struct TermCfg {
    color: Color,
    background: Color,
    attribute: Attribute,
}
pub type Printer<'a> = Box<dyn FnMut(&str) -> io::Result<()> + 'a>;

impl TermCfg {
    pub fn new() -> Self {
        TermCfg {
            color: Color::Green,
            background: Color::White,
            attribute: Attribute::Bold,
        }
    }
    pub fn set_attr(mut self, attr: Attribute) -> Self {
        self.attribute = attr;
        self
    }
    fn generate_buffer(&self) -> io::BufWriter<io::Stdout> {
        let std_out = io::stdout();
        let mut handle = io::BufWriter::new(std_out);
        handle.buffer().windows(4);
        return handle;
    }
    fn styling(&self, color: Color, content: &str) -> console::StyledObject<String> {
        console::set_colors_enabled(true);
        let em_style = console::Style::new().bright().attr(self.attribute);
        let colorized = match color {
            Color::Red => em_style.red(),
            Color::Blue => em_style.blue(),
            Color::Cyan => em_style.cyan(),
            Color::Green => em_style.green(),
            Color::Black | _ => em_style.black(),
        };
        let styled_content = colorized.apply_to(content.to_string());
        return styled_content;
    }
    pub fn gen_println<'a>(&'a self,color:Option<Color>) -> Printer<'a> {
        let mut handle = self.generate_buffer();
        let printer = move |content: &str| -> io::Result<()> {
            let styled_content = self.styling(
                match color {
                    Some(col) => col,
                    None => self.color,
                },
                content,
            );
            writeln!(handle, "{}", styled_content)
        };
        return Box::new(printer);
    }
    pub fn gen_print<'a>(&'a self,color:Option<Color>) -> Printer<'a> {
        let mut handle = self.generate_buffer();
        let printer = move |content: &str| -> io::Result<()> {
            let styled_content = self.styling(
                match color {
                    Some(col) => col,
                    None => self.color,
                },
                content,
            );
            write!(handle, "{}", styled_content)
        };
        return Box::new(printer);
    }
}
#[test]
fn testrun() {
    let ini = TermCfg::new();
    let mut p = ini.gen_println(None);
    loop {
        for _ in 0..20 {
            p("fuck");
        }
        break;
    }
}
